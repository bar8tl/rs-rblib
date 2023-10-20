// create_sqlite3_tablelist: Drop & create a list of Sqlite3 DB tables
use rusqlite::Connection;

#[derive(Debug, Clone, Default)]
pub struct TlistTp {
  pub table: String,
  pub sqlst: String
}

pub fn create_sqlite3_tablelist(dbpath: &String, tlist: &Vec<TlistTp>) {
  let cnn = Connection::open(&dbpath).expect("Error opening DB");
  for tabl in tlist {
    cnn.execute(format!("DROP TABLE IF EXISTS {}", tabl.table).as_str(), [])
      .expect("Error clearing table");
    cnn.execute(tabl.sqlst.as_str(), []).expect("Error writing on table");
    println!("Table {} created...", tabl.table);
  }
}
