// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/dict_lab.html

use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn dictionaries1() {
  // Create a dictionary containing “name”, “city”, and “cake” for “Chris” from “Seattle” who likes “Chocolate” (so the keys should be: “name”, etc, and values: “Chris”, etc.)
  let mut map: HashMap<&str, &str> = HashMap::new();
  map.insert("name", "Chris");
  map.insert("city", "Seattle");
  map.insert("cake", "Chocolate");

  // Display the dictionary.
  println!("Initial dictionary: {:?}", map);

  // Delete the entry for “cake”.
  map.remove("cake");

  // Display the dictionary.
  println!("Without cake: {:?}", map);

  // Add an entry for “fruit” with “Mango” and display the dictionary.
  map.insert("fruit", "Mango");

  // Display the dictionary keys.
  println!("Keys: {:?}", map.keys());
  // Display the dictionary values.
  println!("Values: {:?}", map.values());
  // Display whether or not “cake” is a key in the dictionary (i.e. False) (now).
  println!("Contains cake (key): {:?}", map.contains_key("cake"));
  // Display whether or not “Mango” is a value in the dictionary (i.e. True).
  println!("Contains Mango (value): {:?}", map.values().any(|&v| v == "Mango"));
  println!("Contains Mangro (value): {:?}", map.values().any(|&v| v == "Mangro"));

}

fn dictionaries2() {
  // Using the dictionary from item 1: Make a dictionary using the same keys but with the number of ‘t’s in each value as the value (consider upper and lower case?).
  let mut map: HashMap<&str, &str> = HashMap::new();
  map.insert("name", "Chris");
  map.insert("city", "Seattle");
  map.insert("cake", "Chocolate");

  let mut map2: HashMap<&str, u32> = HashMap::new();
  println!("Originals: {:?}", map);
  for (k, v) in map {
    map2.insert(k, v.chars().fold(0, |acc, c| acc + (if c == 't' || c == 'T' { 1 } else { 0 })));
  }
  println!("Counts: {:?}", map2);
}

fn sets1() {
  let mut s2: HashSet<u32> = HashSet::new();
  let mut s3: HashSet<u32> = HashSet::new();
  let mut s4: HashSet<u32> = HashSet::new();

  for i in 0..21 {
    if (i % 2) == 0 { s2.insert(i); }
    if (i % 3) == 0 { s3.insert(i); }
    if (i % 4) == 0 { s4.insert(i); }
  }

  println!("Divisible by 2: {:?}", s2);
  println!("Divisible by 3: {:?}", s3);
  println!("Divisible by 4: {:?}", s4);

  println!("Is s3 a subset of s2? (false) -> {}", s3.is_subset(&s2));
  println!("Is s4 a subset of s2? (true)  -> {}", s4.is_subset(&s2));
}

fn sets2() {
  // Create a set with the letters in ‘Python’ and add ‘i’ to the set.
  let mut s1: HashSet<char> = HashSet::new();
  "Python".chars().into_iter().for_each(|c| { s1.insert(c); });
  println!("Python set: {:?}", s1);

  s1.insert('i');
  println!("Python set + 'i': {:?}", s1);

  // Create a frozenset with the letters in ‘marathon’.
  let mut s2: HashSet<char> = HashSet::new();
  "marathon".chars().into_iter().for_each(|c| { s2.insert(c); });

  let s2 = s2; // "Freeze" it
  println!("Marathon set: {:?}", s2);

  // Display the union and intersection of the two sets.
  println!("Union: {:?}", s1.union(&s2));
  println!("Intersection: {:?}", s1.intersection(&s2));
}

fn main() {
  dictionaries1();
  dictionaries2();
  sets1();
  sets2();
}
