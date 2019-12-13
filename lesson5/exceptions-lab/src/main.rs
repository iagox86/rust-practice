// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/file_lab.html

use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};


fn safe_input() -> Option<String> {
  println!("Enter something!");

  // Deal with end-of-line with ?, then error with .ok()
  io::stdin().lock().lines().next()?.ok()
}

fn main() {
  println!("Test: {:?}", safe_input());
}
