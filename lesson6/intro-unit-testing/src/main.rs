// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/unit_testing.html

use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn walnut_party(walnuts: u32, is_weekend: bool) -> bool {
  walnuts != 30 && walnuts != 39 && !(walnuts == 61 && !is_weekend)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    assert!(walnut_party(30, false) == false)
  }

  #[test]
  fn test_2() {
    assert!(walnut_party(50, false) == true)
  }

  #[test]
  fn test_3() {
    assert!(walnut_party(70, true) == true)
  }

  #[test]
  fn test_4() {
    assert!(walnut_party(30, true) == false)
  }

  #[test]
  fn test_5() {
    assert!(walnut_party(50, true) == true)
  }

  #[test]
  fn test_6() {
    assert!(walnut_party(60, false) == true)
  }

  #[test]
  fn test_7() {
    assert!(walnut_party(61, false) == false)
  }

  #[test]
  fn test_8() {
    assert!(walnut_party(40, false) == true)
  }

  #[test]
  fn test_9() {
    assert!(walnut_party(39, false) == false)
  }

  #[test]
  fn test_10() {
    assert!(walnut_party(40, true) == true)
  }

  #[test]
  fn test_11() {
    assert!(walnut_party(39, true) == false)
  }
}

fn main() {
}
