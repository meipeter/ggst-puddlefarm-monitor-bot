mod concurrent_test;
mod database;
use anyhow::Result;
use concurrent_test::{benchmark_concurrent_operations, demo_concurrent_usage};
use database::calls::ping_player;
use database::table::{
    batch_update_player_matchcounts, get_all_player_matchcounts, get_player_matchcount,
    insert_follow_table, insert_player_matchcount, query_follow_table, remove_all_follow_table,
    update_player_matchcount,
};

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

    println!("\n🎮 玩家匹配数量测试...");

    // 测试 PLAYER_MATCHCOUNT_TABLE 操作
    let test_uid = 240608152606560723;

    // 插入/更新玩家匹配数量
    insert_player_matchcount(test_uid, c)?;
    println!("插入玩家 {} 匹配数量: {}", test_uid, c);

    // 获取玩家匹配数量
    match get_player_matchcount(test_uid)? {
        Some(count) => println!("玩家 {} 当前匹配数量: {}", test_uid, count),
        None => println!("玩家 {} 匹配数量未找到", test_uid),
    }

    // 更新匹配数量
    let new_count = c + 10;
    update_player_matchcount(test_uid, new_count)?;
    println!("更新玩家 {} 匹配数量为: {}", test_uid, new_count);

    // 验证更新
    match get_player_matchcount(test_uid)? {
        Some(count) => println!("验证更新后匹配数量: {}", count),
        None => println!("更新后未找到匹配数量"),
    }

    // 批量更新测试
    let batch_updates = vec![(1001, 50), (1002, 75), (1003, 100)];
    batch_update_player_matchcounts(&batch_updates)?;
    println!("批量更新 {} 个玩家的匹配数量", batch_updates.len());

    // 获取所有匹配数量记录
    let all_counts = get_all_player_matchcounts()?;
    println!("数据库中共有 {} 个玩家匹配数量记录", all_counts.len());

    // 显示前5条记录作为示例
    for (i, (uid, count)) in all_counts.iter().take(5).enumerate() {
        println!("  {}. 玩家 {}: {} 场匹配", i + 1, uid, count);
    }

    Ok(())
}
