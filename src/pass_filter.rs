// pass_filter: Indicates if a char string matches one pattern
pub fn pass_filter(ifilt: &String, filen: &str) -> bool {
  if filen == ifilt {
    return true;
  }
  true
}
