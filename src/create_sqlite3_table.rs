// create_sqlite3_table: Drop & create an individual Sqlite3 DB table
use rusqlite::Connection;

pub fn create_sqlite3_table(dbpath: &String, table: &String, sqlstm: &String) {
  let cnn = Connection::open(&dbpath).expect("Error opening DB");
  cnn.execute(format!("DROP TABLE IF EXISTS {}", table).as_str(), [])
    .expect("Error clearing table");
  cnn.execute(sqlstm.as_str(), []).expect("Error writing on table");
  println!("Table {} created.", table);
}
