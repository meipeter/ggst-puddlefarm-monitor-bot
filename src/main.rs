mod database;
use crate::database::table::PING_STATUS_TABLE;
use database::table::Db;
use redb::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let db = Db::new()?;
    db.insert_follow_table(1231, 1231123)?;
    db.insert_follow_table(1231, 1231125151153)?;
    let w = db.qurey_follow_table(1231)?;
    println!("{:?}", w);
    Ok(())
}
