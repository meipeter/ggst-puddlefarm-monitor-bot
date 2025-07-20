use crate::database::table::*;
use anyhow::Result;

use puddle_farm_api_client_openapi_client::apis::configuration::Configuration;
use puddle_farm_api_client_openapi_client::apis::default_api::*;

pub async fn ping_player(uid: Uid) -> Result<MatchCount> {
    let player = player_id_get(&Configuration::new(), uid as i64).await?;
    let match_count = player
        .ratings
        .as_ref()
        .into_iter()
        .flatten()
        .map(|p| p.match_count.unwrap_or(0) as i64)
        .sum::<i64>() as u64;

    Ok(match_count)
}
