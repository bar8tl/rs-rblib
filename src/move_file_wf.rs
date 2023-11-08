// move_file_wf: Move a file between directories as part of a workflow process
use std::fs;

pub fn move_file_wf(mode: &str, curdir: &str, pcddir: &str, filenm: &str,
  filetp: &str) {
  let oldnm = format!("{}{}.{}", curdir, filenm, filetp);
  if mode == "inp" {
    let newnm = format!("{}{}.{}", pcddir, filenm, filetp);
    match fs::rename(oldnm, newnm) {
      Ok(())  => {},
      Err(..) => (),
    }
  }
}
