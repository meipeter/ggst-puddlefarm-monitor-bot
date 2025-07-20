mod database;
use database::table::Db;
use redb::Error;
#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    let db = Db::new()?;
    db.insert_follow_table(1231, 1231123)?;
    db.insert_follow_table(1231, 1231125151153)?;
    // db.remove_follow_table(1231, 1231123)?;
    // db.remove_follow_table(1231, 1231125151153)?;
    db.remove_all_follow_table(1231)?;
    let w = db.qurey_follow_table(1231)?;
    println!("{:?}", w);
    Ok(())
}
