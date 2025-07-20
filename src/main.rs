mod concurrent_test;
mod database;
mod simple_comparison;
use anyhow::Result;
use concurrent_test::{benchmark_concurrent_operations, demo_concurrent_usage};
use database::calls::ping_player;
use database::table::{insert_follow_table, query_follow_table, remove_all_follow_table};
use simple_comparison::{run_all_demos, show_code_examples};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    insert_follow_table(1231, 1231123)?;
    insert_follow_table(1231, 1231125151153)?;
    // remove_follow_table(1231, 1231123)?;
    // remove_follow_table(1231, 1231125151153)?;
    remove_all_follow_table(1231)?;
    let c = ping_player(240608152606560723).await?;
    insert_follow_table(240608152606560723, c)?;

    let w = query_follow_table(240608152606560723)?;
    println!("查询结果: {:?}", w);

    Ok(())
}
