//**********************************************************************************
// lib.rs: Rust library with common functions used in bar8tl programs
// (2022-04-06 bar8tl)
//**********************************************************************************
pub mod db;
pub mod params;

use std::fs;

// Renames a file ------------------------------------------------------------------
pub fn ren_file(mode: &str, curdr: &str, fnm: &str, fex: &str) {
  let oldnm = format!("{}{}.{}", curdr, fnm, fex);
  let mut newnm = oldnm.clone();
  if mode == "inp" {
    newnm = format!("{}inp_{}_processed.{}", curdr, fnm, fex);
  } else if mode == "out" {
    newnm = format!("{}out_{}.{}", curdr, fnm, fex);
  }
  fs::rename(oldnm, newnm).expect("File rename failure");
}

// Indicates if a char string matches one pattern ----------------------------------
pub fn pass_filter(ifilt: &String, filen: &str) -> bool {
  if filen == ifilt {
    return true;
  }
  true
}

// Performs rounding of floating point numbers to specific decimal positions -------
pub fn rb_round(x: f32, y: u32) -> f32 {
  let y = 10i32.pow(y) as f32;
  (x * y).round() / y
}

// Determines if a number is into one numbers list ---------------------------------
pub fn contains(s: &Vec<usize>, e: &usize) -> bool {
  for a in s {
    if a == e {
      return true;
    }
  }
  return false;
}

// Displays the data type of an specific object ------------------------------------
include!(".\\include\\typeof.rs");
