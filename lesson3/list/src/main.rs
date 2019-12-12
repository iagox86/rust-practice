// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/list_lab.html

use std::io::{self, Read, Write};
use std::io::prelude::*;

fn main() {

  // SERIES 1

  // Create a list that contains “Apples”, “Pears”, “Oranges” and “Peaches”.
  let mut v: Vec<&str> = vec!["Apples", "Pears", "Oranges", "Peaches"];

  // Display the list (plain old print() is fine…).
  println!("List: {:?}", v);

  // Ask the user for another fruit and add it to the end of the list.
  print!("Add a new fruit -> ");
  io::stdout().flush().unwrap();
  let new_fruit = io::stdin().lock().lines().next().unwrap().unwrap();
  v.push(&new_fruit);

  // Display the list.
  println!("List: {:?}", v);

  // Ask the user for a number and display the number back to the user and the fruit corresponding to that number (on a 1-is-first basis). Remember that Python uses zero-based indexing, so you will need to correct.
  print!("Get fruit from index -> ");
  io::stdout().flush().unwrap();
  let index: usize = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
  println!("You chose {}!", v[index - 1]);

  // Add another fruit to the beginning of the list using insert() and display the list.
  print!("Add a new fruit to the beginning -> ");
  io::stdout().flush().unwrap();
  let new_fruit = io::stdin().lock().lines().next().unwrap().unwrap();
  v.insert(0, &new_fruit);

  // Display all the fruits that begin with “P”, using a for loop.
  let v2: Vec<_> = v.clone().into_iter().filter(|e| e.chars().next().unwrap() == 'P').collect();
  println!("Fruits that start with P: {:?}", v2);

  // SERIES 2

  // Display the list.
  println!("And again: {:?}", v);

  // Remove the last fruit from the list.
  v.remove(v.len() - 1);

  // Display the list.
  println!("Last value removed: {:?}", v);

  // Ask the user for a fruit to delete, find it and delete it.
  print!("Remove a specific fruit -> ");
  io::stdout().flush().unwrap();
  let bad_fruit = io::stdin().lock().lines().next().unwrap().unwrap();
  let v: Vec<_> = v.into_iter().filter(|i| **i != bad_fruit).collect();

  println!("{} removed: {:?}", bad_fruit, v);

  // SERIES 3
  let mut v: Vec<_> = v.into_iter().filter(|f| {
    loop {
      print!("Do you like {}? -> ", f);
      io::stdout().flush().unwrap();

      let answer = io::stdin().lock().lines().next().unwrap().unwrap();
      match answer.as_ref() {
        "yes" => return true,
        "no" => return false,
        _ => println!("Use either yes or no!"),
      }
    }
  }).collect();

  println!("List is now: {:?}", v);

  // SERIES 4
  let reversed: Vec<String> = v.clone().into_iter().map(|f| String::from(f).chars().into_iter().rev().collect()).collect();
  println!("Reversed list: {:?}", reversed);

  v.remove(v.len() - 1);
  println!("Final list: {:?}", v);
}
