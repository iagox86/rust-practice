// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/comprehensions_lab.html
use std::collections::{HashSet, HashMap};

fn count_evens(l: &Vec<u32>) -> u32 {
  return l.into_iter().fold(0, |acc, x| acc + ((x + 1) % 2));
}

fn main() {
  let before = vec![
    "lambs",
    "sloths",
    "orangutans",
    "breakfast cereals",
    "fruit bats",
  ];

  println!("Capitalize:");
  println!(" Before: {:?}", before);
  let after: Vec<String> = before.into_iter().map(|v| v.to_uppercase()).collect();
  println!(" After:  {:?}", after);

  let before = vec![
    "spam",
    "sloths",
    "orangutans",
    "breakfast cereals",
    "fruit bats",
  ];

  println!("> 6 characters:");
  println!(" Before: {:?}", before);
  let after: Vec<&str> = before.into_iter().filter(|v| v.len() > 6).collect();
  println!(" After:  {:?}", after);

  let before = vec![
    (1, "Lumberjack"),
    (2, "Inquisition"),
    (4, "Spam"),
  ];
  println!("Unpacking tuples");
  println!(" Before: {:?}", before);
  let after: Vec<String> = before.into_iter().map(|(n, s)| String::from(s).repeat(n)).collect();
  println!(" After:  {:?}", after);

  let before1 = vec![
    "poached egg",
    "fried egg",
  ];
  let before2 = vec![
    "lite spam",
    "ham spam",
    "fried spam",
  ];

  println!("Unpacking tuples");
  println!(" Before1: {:?}", before1);
  println!(" Before2: {:?}", before2);
  let mut after: Vec<String> = Vec::new();
  for s1 in before1 {
    for s2 in before2.clone() {
      after.push(format!("{} and {}", s1, s2));
    }
  }
  println!(" After:  {:?}", after);

  let before = "aabbbcccc".chars();
  println!(" Before: {:?}", before);
  let after: HashSet<char> = before.into_iter().collect();
  println!(" After:  {:?}", after);

  let mut before: HashMap<&str, &str> = HashMap::new();
  before.insert("first",  "fear");
  before.insert("second", "surprise");
  before.insert("third",  "ruthless efficiency");
  before.insert("forth",  "fanatical devotion");

  println!(" Before: {:?}", before);
  let after: HashMap<String, &str> = before.into_iter().map(|(k, v)| (String::from(k).to_uppercase(), v)).collect();
  println!(" After:  {:?}", after);

  let list = vec![1, 2, 3, 4, 5];
  println!("Evens in {:?}: {}", list, count_evens(&list));
  let list = vec![1, 1, 1, 1, 1];
  println!("Evens in {:?}: {}", list, count_evens(&list));
  let list = vec![2, 2, 2, 2, 5];
  println!("Evens in {:?}: {}", list, count_evens(&list));

  let mut before: HashMap<&str, &str> = HashMap::new();
  before.insert("name", "Chris");
  before.insert("city", "Seattle");
  before.insert("cake", "chocolate");
  before.insert("fruit", "mango");
  before.insert("salad", "greek");
  before.insert("pasta", "lasagna");

  // Print the dict by passing it to a string format method, so that you get something like:
  // “Chris is from Seattle, and he likes chocolate cake, mango fruit, greek salad, and lasagna pasta”
  println!("{} is from {}, and he likes {}, {}, {}, and {}", before.get("name").unwrap(), before.get("city").unwrap(), before.get("cake").unwrap(), before.get("fruit").unwrap(), before.get("salad").unwrap(), before.get("pasta").unwrap());

  // Using a list comprehension, build a dictionary of numbers from zero to fifteen and the hexadecimal equivalent (string is fine). (the hex() function gives you the hexidecimal representation of a number as a string)
  // Do the previous entirely with a dict comprehension – should be a one-liner
  let hex_list: HashMap<String, u32> = (0..15).into_iter().map(|i| (format!("{:x}", i), i)).collect();
  println!("Hex numbers from 0 to 15: {:?}", hex_list);

  // Using the dictionary from item (1): Make a dictionary using the same keys but with the number of ‘a’s in each value. You can do this either by editing the dict in place, or making a new one. If you edit in place make a copy first!
  let after: HashMap<&str, u32> = before.into_iter().map(|(k, v)| (k, v.chars().into_iter().fold(0, |acc, c|  acc + match c { 'a' => 1, _ => 0}))).collect();
  println!("Number of a's in the before hash: {:?}", after);

  // Create sets s2, s3 and s4 that contain numbers from zero through twenty, divisible 2, 3 and 4.
  // Do this with one set comprehension for each set.
  let s2: HashSet<u32> = (0..21).into_iter().filter(|v| (v % 2) == 0).collect();
  let s3: HashSet<u32> = (0..21).into_iter().filter(|v| (v % 3) == 0).collect();
  let s4: HashSet<u32> = (0..21).into_iter().filter(|v| (v % 4) == 0).collect();

  println!("s2: {:?}", s2);
  println!("s3: {:?}", s3);
  println!("s4: {:?}", s4);

  // What if you had a lot more than 3? – Don’t Repeat Yourself (DRY).
  // create a sequence that holds all the divisors you might want – could be 2,3,4, or could be any other arbitrary divisors.
  // loop through that sequence to build the sets up – so no repeated code. you will end up with a list of sets – one set for each divisor in your sequence.
  // The idea here is that when you see three (Or more!) lines of code that are almost identical, then you you want to find a way to generalize that code and have it act on a set of inputs, so the actual code is only written once.
  // Extra credit: do it all as a one-liner by nesting a set comprehension inside a list comprehension. (OK, that may be getting carried away!)
  let s: HashMap<u32, HashSet<u32>> = (1..5).map(|v| (v, (0..21).into_iter().filter(|v2| (v2 % v) == 0).collect())).collect();
  println!();
  println!("s: {:?}", s);


}
