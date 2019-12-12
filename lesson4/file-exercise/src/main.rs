// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/file_lab.html

use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
  // Write a little script that reads that file and generates a list of all the languages that have been used.
  // What might be the best data structure to use to keep track of bunch of values (the languages) without duplication?
  let mut languages: HashSet<String> = HashSet::new();

  let file = File::open("./students.txt").unwrap();
  let reader = BufReader::new(file);
  let mut lines = reader.lines().enumerate();

  // Skip the first line
  lines.next();

  for (_, line) in lines {
    let line = line.unwrap();
    let mut split_line = line.split(":");

    // Drop the student
    split_line.next();
    let subjects = split_line.next().unwrap();

    for subject in subjects.split(",") {
      let subject = subject.trim();
      languages.insert(String::from(subject));
    }
  }

  println!("Languages: {:?}", languages);
}

fn part2() {
  let mut languages: HashMap<String, u32> = HashMap::new();

  let file = File::open("./students.txt").unwrap();
  let reader = BufReader::new(file);
  let mut lines = reader.lines().enumerate();

  // Skip the first line
  lines.next();

  for (_, line) in lines {
    let line = line.unwrap();
    let mut split_line = line.split(":");

    // Drop the student
    split_line.next();
    let subjects = split_line.next().unwrap();

    for subject in subjects.split(",") {
      let subject = subject.trim();

      languages.get_mut(subject)
        .map(|count|  *count += 1 )
        .unwrap_or_else(|| { languages.insert(String::from(subject), 1); });
    }
  }

  println!("Languages: {:?}", languages);
}

fn main() {
  part1();
  part2();
}
