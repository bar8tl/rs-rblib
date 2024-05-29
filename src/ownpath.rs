// ownpath.rs - Returns an absolute path from a project home directory and a relative
// path of file directories used during the executions (2019-07-01 bar8tl)
pub fn ownpath(phome: &String, rpath: &String) -> String {
  if phome.len() > 0 {
    if rpath.len() > 0 {
      let atokn: Vec<&str> = rpath.splitn(2, "\\").collect();
      if atokn[0] == "~" {
        format!("{}{}", phome, atokn[1])
      } else {
        rpath.clone()
      }
    } else {
      phome.clone()
    }
  } else {
    if rpath.len() > 0 {
      let atokn: Vec<&str> = rpath.splitn(2, "\\").collect();
      if atokn[0] == "~" {
        rpath.replace("~", ".")
      } else {
        rpath.clone()
      }
    } else {
      String::new()
    }
  }
}
