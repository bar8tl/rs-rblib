// number_in_list: Determines if a number is into one numbers list
pub fn number_in_list(s: &Vec<usize>, e: &usize) -> bool {
  for a in s {
    if a == e {
      return true;
    }
  }
  return false;
}
