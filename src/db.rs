//**********************************************************************************
// db.rs: Database common routines (2022-04-06 bar8tl)
//**********************************************************************************
use rusqlite::Connection;

#[derive(Debug, Clone, Default)]
pub struct TlistTp {
  pub table: String,
  pub sqlst: String
}

// Drop and Create sqlite3 data base tables ----------------------------------------
pub fn reset_tables(dbopt: &String, tlist: &Vec<TlistTp>) {
  let cnn = Connection::open(&dbopt).expect("Error opening DB");
  for tabl in tlist {
    reset_table(&cnn, tabl);
  }
  println!("Creation of dabatase {} completed.", dbopt);
}

pub fn reset_table(cnn: &Connection, tabl: &TlistTp) {
  cnn.execute(format!("DROP TABLE IF EXISTS {}", tabl.table).as_str(), [])
    .expect("Error clearing table");
  cnn.execute(tabl.sqlst.as_str(), []).expect("Error writing on table");
  println!("Table {} created...", tabl.table);
}
