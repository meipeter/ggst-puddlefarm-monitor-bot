use anyhow::Result;
use lazy_static::lazy_static;
use log::trace;
#[allow(unused_variables, unused_imports, unused)]
use redb::{
    Database, MultimapTableDefinition, ReadOnlyMultimapTable, ReadableMultimapTable, ReadableTable,
    TableDefinition,
};
use std::sync::Arc;
pub type Uid = u64;
pub type MatchCount = u64;
pub const FOLLOWERS_TABLE: MultimapTableDefinition<Uid, Uid> =
    MultimapTableDefinition::new("followers_table"); //the first is the player been followed and the 2nd is the player following the first
pub const PING_STATUS_TABLE: TableDefinition<Uid, MatchCount> =
    TableDefinition::new("ping_status_table");

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

// 获取数据库实例的函数（如果需要直接访问）
pub fn get_database() -> &'static Arc<Database> {
    &DATABASE
}
