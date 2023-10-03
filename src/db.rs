//**********************************************************************************
// db.rs: Database common routines (2022-04-06 bar8tl)
//**********************************************************************************
use rusqlite::Connection;

#[derive(Debug, Clone, Default)]
pub struct TlistTp {
  pub table: String,
  pub sqlst: String
}

// Drop and Create sqlite3 data base several tables --------------------------------
pub fn reset_tables(dbopt: &String, tlist: &Vec<TlistTp>) {
  let cnn = Connection::open(&dbopt).expect("Error opening DB");
  for tabl in tlist {
    cnn.execute(format!("DROP TABLE IF EXISTS {}", tabl.table).as_str(), [])
      .expect("Error clearing table");
    cnn.execute(tabl.sqlst.as_str(), []).expect("Error writing on table");
    println!("Table {} created...", tabl.table);
  }
}

// Drop and Create a single sqlite3 data base table --------------------------------
pub fn reset_table(dbopt: &String, table: &String, sqlst: &String) {
  let cnn = Connection::open(&dbopt).expect("Error opening DB");
  cnn.execute(format!("DROP TABLE IF EXISTS {}", table).as_str(), [])
    .expect("Error clearing table");
  cnn.execute(sqlst.as_str(), []).expect("Error writing on table");
  println!("Table {} created.", table);
}
