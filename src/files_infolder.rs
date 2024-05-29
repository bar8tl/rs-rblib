// files_infolder.rs - Get a list of files contained into one directory, specifying
// detail for each file (2021-07-01 bar8tl)
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FilelistTp {
  pub flpth: String,
  pub fldir: String,
  pub flide: String,
  pub flnam: String,
  pub flext: String
}

pub fn files_infolder(dir: &String, extsn: &String, prefx: &String) ->
  Vec<FilelistTp> {
  let mut flist: Vec<FilelistTp> = Default::default();
  for entry in fs::read_dir(dir).unwrap() {
    let entry = entry.unwrap().path(); // type of 'entry' is PathBuf
    if entry.is_dir() {
      continue;
    }
    let filid = Path::new(&entry).file_name().unwrap();
    let filnm = Path::new(&filid).file_stem().unwrap();
    let filex = Path::new(&filid).extension().unwrap();
    let flide = filid.to_str().unwrap().to_string(); // file name.type
    let flnam = filnm.to_str().unwrap().to_string(); // file name
    let flext = filex.to_str().unwrap().to_string(); // file type
    let flpth = format!("{}{}", &dir, flide);        // file path
    if (extsn.len() > 0 &&  flext != *extsn) ||
       (prefx.len() > 0 && !flnam.starts_with(prefx)) {
      continue;
    }
    flist.push(FilelistTp{flpth: flpth.clone(), fldir: dir.clone(),
      flide: flide.clone(), flnam: flnam.clone(), flext: flext.clone()});
  }
  return flist;
}
