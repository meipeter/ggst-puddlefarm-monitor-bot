mod apps;
mod concurrent_test;
mod database;
mod examples;
mod net;
use anyhow::Result;

use database::table::insert_player_matchcount_table;

use crate::apps::jobs::init_player_pulling;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    insert_player_matchcount_table(210611180647101346, 0)?;
    insert_player_matchcount_table(230307003358952157, 0)?;
    insert_player_matchcount_table(231003095904663665, 0)?;
    init_player_pulling().await;
    Ok(())
}
