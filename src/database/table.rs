#[allow(unused_variables, unused_imports, unused)]
use redb::{
    Database, Error, MultimapTableDefinition, ReadOnlyMultimapTable, ReadableMultimapTable,
    ReadableTable, TableDefinition,
};
use std::{fs::write, sync::Arc};

use crate::database;
type Uid = u64;
type MatchCount = u64;
pub const FOLLOWERS_TABLE: MultimapTableDefinition<Uid, Uid> =
    MultimapTableDefinition::new("followers_table"); //the first is the player been followed and the 2nd is the player following the first
pub const PING_STATUS_TABLE: TableDefinition<Uid, MatchCount> =
    TableDefinition::new("ping_status_table");
pub struct Db {
    database: Arc<Database>,
}
impl Db {
    pub fn new() -> Result<Self, Error> {
        Ok(Db {
            database: Arc::new(Database::create("./pdmb.redb")?),
        })
    }
    pub fn qurey_follow_table(&self, uid: Uid) -> Result<Vec<Uid>, Error> {
        let follower_list = self
            .database
            .begin_read()?
            .open_multimap_table(FOLLOWERS_TABLE)?
            .get(uid)?
            .map(|v| v.map(|g| g.value()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(follower_list)
    }
    pub fn insert_follow_table(&self, followee: Uid, follower: Uid) -> Result<(), Error> {
        let wr_txn = self.database.begin_write()?;
        wr_txn
            .open_multimap_table(FOLLOWERS_TABLE)?
            .insert(followee, follower)?;
        wr_txn.commit()?;
        Ok(())
    }
}
