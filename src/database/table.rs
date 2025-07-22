use anyhow::Result;
use bincode::config;
use lazy_static::lazy_static;
use log::trace;
use puddle_farm_api_client_openapi_client::models::PlayerResponse;

#[allow(unused_variables, unused_imports, unused, unused)]
use redb::{
    Database, MultimapTableDefinition, ReadOnlyMultimapTable, ReadableMultimapTable, ReadableTable,
    TableDefinition,
};
use std::sync::Arc;
pub type Uid = u64;
pub type MatchCount = u64;
pub const FOLLOWERS_TABLE: MultimapTableDefinition<Uid, Uid> =
    MultimapTableDefinition::new("followers_table"); //the first is the player been followed and the 2nd is the player following the first
pub const PLAYER_MATCHCOUNT_TABLE: TableDefinition<Uid, MatchCount> =
    TableDefinition::new("player_matchcount_table");
pub const PING_STATUS_TABLE: TableDefinition<Uid, MatchCount> =
    TableDefinition::new("ping_status_table");
pub const PLAYER_DATA_TABLE: TableDefinition<Uid, Vec<u8>> =
    TableDefinition::new("player_data_table");

// 使用 lazy_static 创建全局数据库实例
lazy_static! {
    static ref DATABASE: Arc<Database> =
        Arc::new(Database::create("./pdmb.redb").expect("Failed to create database"));
}
// 直接使用全局数据库实例的便捷函数
pub fn query_follow_table(uid: Uid) -> Result<Vec<Uid>> {
    let follower_list = DATABASE
        .begin_read()?
        .open_multimap_table(FOLLOWERS_TABLE)?
        .get(uid)?
        .map(|v| v.map(|g| g.value()))
        .collect::<Result<Vec<_>, _>>()?;
    trace!("query player {},followed by {:?}", uid, follower_list);
    Ok(follower_list)
}

pub fn insert_follow_table(followee: Uid, follower: Uid) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    wr_txn
        .open_multimap_table(FOLLOWERS_TABLE)?
        .insert(followee, follower)?;
    wr_txn.commit()?;
    trace!("insert player {},followed by {:?}", followee, follower);
    Ok(())
}

pub fn remove_follow_table(followee: Uid, follower: Uid) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    wr_txn
        .open_multimap_table(FOLLOWERS_TABLE)?
        .remove(followee, follower)?;
    wr_txn.commit()?;
    trace!("remove follower {:?} following {:?}", follower, followee);
    Ok(())
}

pub fn remove_all_follow_table(followee: Uid) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    wr_txn
        .open_multimap_table(FOLLOWERS_TABLE)?
        .remove_all(followee)?;
    wr_txn.commit()?;
    trace!("remove all follower {:?}", followee);
    Ok(())
}

// PLAYER_MATCHCOUNT_TABLE 操作函数 (一对一关系，使用 open_table)
pub fn get_player_matchcount_table(uid: Uid) -> Result<Option<MatchCount>> {
    let read_txn = DATABASE.begin_read()?;
    let table = read_txn.open_table(PLAYER_MATCHCOUNT_TABLE)?;
    let result = table.get(uid)?;
    match result {
        Some(access_guard) => {
            let match_count = access_guard.value();
            trace!("get player {} match count: {}", uid, match_count);
            Ok(Some(match_count))
        }
        None => {
            trace!("player {} match count not found", uid);
            Ok(None)
        }
    }
}

pub fn insert_player_matchcount_table(uid: Uid, match_count: MatchCount) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    {
        let mut table = wr_txn.open_table(PLAYER_MATCHCOUNT_TABLE)?;
        table.insert(uid, match_count)?;
    }
    wr_txn.commit()?;
    trace!("insert player {} match count: {}", uid, match_count);
    Ok(())
}

pub fn update_player_matchcount_table(uid: Uid, match_count: MatchCount) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    {
        let mut table = wr_txn.open_table(PLAYER_MATCHCOUNT_TABLE)?;
        table.insert(uid, match_count)?; // insert 会覆盖现有值
    }
    wr_txn.commit()?;
    trace!("update player {} match count: {}", uid, match_count);
    Ok(())
}

pub fn remove_player_matchcount_table(uid: Uid) -> Result<bool> {
    let wr_txn = DATABASE.begin_write()?;
    let removed = {
        let mut table = wr_txn.open_table(PLAYER_MATCHCOUNT_TABLE)?;
        table.remove(uid)?.is_some()
    };
    wr_txn.commit()?;
    if removed {
        trace!("remove player {} match count", uid);
    } else {
        trace!("player {} match count not found for removal", uid);
    }
    Ok(removed)
}

// 获取所有玩家的匹配数量（用于调试或统计）
pub fn get_all_player_matchcounts() -> Result<Vec<(Uid, MatchCount)>> {
    let read_txn = DATABASE.begin_read()?;
    let table = read_txn.open_table(PLAYER_MATCHCOUNT_TABLE)?;
    let mut results = Vec::new();

    // 遍历所有记录
    for result in table.iter()? {
        let (uid, match_count) = result?;
        results.push((uid.value(), match_count.value()));
    }

    trace!("retrieved {} player match count records", results.len());
    Ok(results)
}

// 批量插入/更新玩家匹配数量
pub fn batch_update_player_matchcounts(updates: &[(Uid, MatchCount)]) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    {
        let mut table = wr_txn.open_table(PLAYER_MATCHCOUNT_TABLE)?;
        for &(uid, match_count) in updates {
            table.insert(uid, match_count)?;
        }
    }
    wr_txn.commit()?;
    trace!("batch updated {} player match count records", updates.len());
    Ok(())
}

// 获取数据库实例的函数（如果需要直接访问）
pub fn get_database() -> &'static Arc<Database> {
    &DATABASE
}

// PLAYER_DATA_TABLE 操作函数
pub fn get_player_data(uid: Uid) -> Result<Option<PlayerResponse>> {
    let read_txn = DATABASE.begin_read()?;
    let table = read_txn.open_table(PLAYER_DATA_TABLE)?;
    let result = table.get(uid)?;

    match result {
        Some(access_guard) => {
            let serialized_data = access_guard.value();
            let player_response = deserialize_player_response(&serialized_data)?;
            trace!("get player {} data", uid);
            Ok(Some(player_response))
        }
        None => {
            trace!("player {} data not found", uid);
            Ok(None)
        }
    }
}

pub fn insert_player_data(uid: Uid, player: &PlayerResponse) -> Result<()> {
    let serialized_data = serialize_player_response(player)?;
    let wr_txn = DATABASE.begin_write()?;
    {
        let mut table = wr_txn.open_table(PLAYER_DATA_TABLE)?;
        table.insert(uid, serialized_data)?;
    }
    wr_txn.commit()?;
    trace!("insert player {} data", uid);
    Ok(())
}

pub fn update_player_data(uid: Uid, player: &PlayerResponse) -> Result<()> {
    let serialized_data = serialize_player_response(player)?;
    let wr_txn = DATABASE.begin_write()?;
    {
        let mut table = wr_txn.open_table(PLAYER_DATA_TABLE)?;
        table.insert(uid, serialized_data)?; // insert 会覆盖现有值
    }
    wr_txn.commit()?;
    trace!("update player {} data", uid);
    Ok(())
}

pub fn remove_player_data(uid: Uid) -> Result<bool> {
    let wr_txn = DATABASE.begin_write()?;
    let removed = {
        let mut table = wr_txn.open_table(PLAYER_DATA_TABLE)?;
        table.remove(uid)?.is_some()
    };
    wr_txn.commit()?;
    if removed {
        trace!("remove player {} data", uid);
    } else {
        trace!("player {} data not found for removal", uid);
    }
    Ok(removed)
}

// 获取所有玩家数据
pub fn get_all_player_data() -> Result<Vec<(Uid, PlayerResponse)>> {
    let read_txn = DATABASE.begin_read()?;
    let table = read_txn.open_table(PLAYER_DATA_TABLE)?;
    let mut results = Vec::new();

    for result in table.iter()? {
        let (uid, serialized_data) = result?;
        let player_response = deserialize_player_response(&serialized_data.value())?;
        results.push((uid.value(), player_response));
    }

    trace!("retrieved {} player data records", results.len());
    Ok(results)
}

// 批量插入/更新玩家数据
pub fn batch_update_player_data(updates: &[(Uid, &PlayerResponse)]) -> Result<()> {
    let wr_txn = DATABASE.begin_write()?;
    {
        let mut table = wr_txn.open_table(PLAYER_DATA_TABLE)?;
        for &(uid, player) in updates {
            let serialized_data = serialize_player_response(player)?;
            table.insert(uid, serialized_data)?;
        }
    }
    wr_txn.commit()?;
    trace!("batch updated {} player data records", updates.len());
    Ok(())
}
// PlayerResponse 序列化/反序列化函数
pub fn serialize_player_response(player: &PlayerResponse) -> Result<Vec<u8>> {
    let config = config::standard();
    bincode::serde::encode_to_vec(player, config).map_err(|e| anyhow::anyhow!("序列化失败: {}", e))
}

pub fn deserialize_player_response(data: &[u8]) -> Result<PlayerResponse> {
    let config = config::standard();
    let (player, _): (PlayerResponse, usize) = bincode::serde::decode_from_slice(data, config)
        .map_err(|e| anyhow::anyhow!("反序列化失败: {}", e))?;
    Ok(player)
}
