// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/kata_fourteen.html

use std::collections::{HashMap};
use regex::Regex;
use rand::seq::SliceRandom;
use std::fs;

fn clean_word(word: &str) -> String {
  return String::from(word.to_lowercase().trim());
}

fn get_trigrams(data: &str) -> HashMap<String, Vec<String>> {
  // Split the words
  let re = Regex::new("[^a-zA-Z0-9']").unwrap();
  let mut words = re.split(&data);

  // Get the first two words
  let mut two_ago = clean_word(words.next().unwrap());
  while two_ago.len() == 0 {
    two_ago = clean_word(words.next().unwrap());
  }

  let mut one_ago = clean_word(words.next().unwrap());
  while one_ago.len() == 0 {
    one_ago = clean_word(words.next().unwrap());
  }

  // Build up the trigrams
  let mut trigrams: HashMap<String, Vec<String>> = HashMap::new();
  for word in re.split(&data) {
    let word = clean_word(word);

    // Skip empty words
    if word.len() == 0 {
      continue;
    }

    // Add it to the set, index by the first two
    let key = format!("{} {}", two_ago, one_ago);
    trigrams.get_mut(&key)
      .map(|words|  words.push(word.clone()) )
      .unwrap_or_else(|| { trigrams.insert(key, vec![word.clone()]); });

    // Move "forward"
    two_ago = one_ago;
    one_ago = word;
  }

  trigrams
}

fn markov_chain(trigrams: HashMap<String, Vec<String>>) {
  let current_pair: &String = trigrams.keys().collect::<Vec<&String>>().choose(&mut rand::thread_rng()).unwrap();

  let mut current_split = (*current_pair).split(" ");
  let mut word1 = current_split.next().unwrap();
  let mut word2 = current_split.next().unwrap();

  print!("{} {}", word1, word2);
  for _ in 0..100 {
    let key = format!("{} {}", word1, word2);
    let words = trigrams.get(&key);

    match words {
      Some(w) => {
        word1 = word2;
        word2 = w.choose(&mut rand::thread_rng()).unwrap();
        print!(" {}", word2);
      },
      _ => {
        println!("\n\nOut of words!");
        return;
      },
    }
  }
}

fn main() {
  // Read the file
  let data = fs::read_to_string("./text.txt").unwrap();
  let trigrams = get_trigrams(&data);
  markov_chain(trigrams);
}
