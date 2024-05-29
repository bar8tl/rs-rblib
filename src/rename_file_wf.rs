// rename_file_wf: Rename a file as part of a workflow process
use std::fs;

pub fn rename_file_wf(mode: &str, curdir: &str, filenm: &str, filetp: &str) {
  let oldnm = format!("{}{}.{}", curdir, filenm, filetp);
  let mut newnm = oldnm.clone();
  if mode == "inp" {
    newnm = format!("{}inp_{}_processed.{}", curdir, filenm, filetp);
  } else if mode == "out" {
    newnm = format!("{}out_{}.{}", curdir, filenm, filetp);
  }
  match fs::rename(oldnm, newnm) {
    Ok(())  => {},
    Err(..) => (),
  }
}
