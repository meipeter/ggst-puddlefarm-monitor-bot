mod apps;
mod concurrent_test;
mod database;
mod net;
use anyhow::Result;
use concurrent_test::{benchmark_concurrent_operations, demo_concurrent_usage};

use database::table::{
    batch_update_player_matchcounts, get_all_player_matchcounts, get_player_matchcount_table,
    insert_follow_table, insert_player_matchcount_table, query_follow_table,
    remove_all_follow_table, update_player_matchcount_table,
};
use net::api::ping_player;

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
