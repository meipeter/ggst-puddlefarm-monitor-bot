use crate::database::table::get_all_player_matchcounts;
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
            match pulling_players_response().await {
                Ok(list) => {
                    for player in list {
                        println!("{:?}", player.name)
                    }
                    //明天在这里写写入数据库
                }
                Err(e) => error!("Player pulling error: {}", e),
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
