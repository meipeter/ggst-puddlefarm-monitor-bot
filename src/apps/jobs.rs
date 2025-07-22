use crate::database::table::{
    batch_update_player_data, batch_update_player_matchcounts, get_all_player_data,
    get_all_player_matchcounts,
};
use anyhow::Result;
use log::error;
use puddle_farm_api_client_openapi_client::apis::configuration::Configuration;
use puddle_farm_api_client_openapi_client::apis::default_api::player_id_get;
use puddle_farm_api_client_openapi_client::models::PlayerResponse;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time;
pub async fn init_player_pulling() {
    let mut tick = time::interval(Duration::from_secs(120));

    println!("Player pulling initialized");
    loop {
        tokio::spawn(async {
            //拉取数据并写入数据库
            match pulling_players_response().await {
                Ok(list) => {
                    let list: Vec<(u64, &PlayerResponse)> = list
                        .iter()
                        .filter_map(|pr| {
                            let id = pr.id? as u64; // 把 Option<i64> → u64，None 的丢弃
                            Some((id, pr))
                        })
                        .collect();
                    if let Err(e) = batch_update_player_data(&list) {
                        error!("Batch update player data error: {}", e);
                    }
                }

                Err(e) => error!("Player pulling error: {}", e),
            }
            //更新游戏比赛数
            match get_all_player_data() {
                Ok(counts) => {
                    let counts: Vec<(u64, u64)> = counts
                        .iter()
                        .map(|(id, count)| {
                            (
                                id.clone(),
                                count
                                    .ratings
                                    .as_ref()
                                    .into_iter()
                                    .flatten()
                                    .map(|p| p.match_count.unwrap_or(0) as i64)
                                    .sum::<i64>() as u64,
                            )
                        })
                        .collect();
                    if let Err(e) = batch_update_player_matchcounts(&counts) {
                        error!("Batch update player matchcounts error: {}", e);
                    }
                }
                Err(e) => error!("Get all player matchcounts error: {}", e),
            }
        });
        tick.tick().await;
    }
}
async fn pulling_players_response() -> Result<Vec<PlayerResponse>> {
    let config = Configuration::default();
    let all_players = get_all_player_matchcounts()?;
    let len = all_players.len();
    let mut tick = time::interval(Duration::from_millis(60000 / len as u64));
    let mut set = JoinSet::new();
    for (id, _) in all_players {
        let config_clone = config.clone();
        set.spawn(async move { player_id_get(&config_clone, id as i64).await });
        tick.tick().await;
        println!("Player {} pulled", id);
    }
    let mut results = Vec::with_capacity(len);
    while let Some(res) = set.join_next().await {
        match res {
            Ok(val) => match val {
                Ok(player_response) => results.push(player_response),
                Err(e) => error!("Player API error: {}", e),
            },
            Err(e) => error!("Player pulling error: {}", e),
        }
    }
    Ok(results)
}
