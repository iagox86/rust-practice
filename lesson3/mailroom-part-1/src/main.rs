use std::io::{self, Read, Write};
use std::io::prelude::*;
use std::process;
use std::collections::HashMap;

enum Menu {
  SendThanks,
  CreateReport,
  Quit,
}

struct Details {
  amounts: Vec<f64>
}

struct Donations {
  donations: HashMap<String, Details>
}

impl Donations {
  fn new() -> Donations {
    return Donations {
      donations: HashMap::new(),
    }
  }

  fn add(&mut self, name: String, donation: f64) {
    let entry = self.donations.get_mut(&name);
    match entry {
      Some(v) => {
        // Add the donation to the amounts list
        v.amounts.push(donation);
      },
      _ => {
        // Create a new donations entry
        self.donations.insert(name.clone(), Details { amounts: vec![donation] });
      }
    }
  }

  fn list(&self) {
    for name in self.donations.keys() {
      println!("{name}", name=name);
    };
  }

  fn report(&self) {
    println!("{name:16} | {total:11} | {gifts:8} | {average:12}", name="Donor Name", total="Total Given", gifts="Num Gifts", average="Average Gift");
    println!("------------------------------------------------------------------");

    for (name, donations) in self.donations.iter() {
      let total: f64 = donations.amounts.clone().into_iter().sum();

      println!("{name:16} | {total:11.2} | {gifts:9} | {average:12.2}", name=name, total=total, gifts=donations.amounts.len(), average=(total / (donations.amounts.len() as f64)));
    };
  }

  fn get_donor_name(&self) -> String {
    loop {
      print!("Who is the generous donor? ('list' to list them all) -> ");
      io::stdout().flush().unwrap();
      let donor = io::stdin().lock().lines().next().unwrap().unwrap();
      if donor == "list" {
        self.list();
      } else {
        return donor;
      }
    }
  }

  fn get_amount() -> f64 {
    loop {
      print!("How much did they donate? -> ");
      io::stdout().flush().unwrap();
      let selection: Result<f64, _> = io::stdin().lock().lines().next().unwrap().unwrap().parse();
      match selection {
        Ok(v) => return v,
        _ => println!("Format error!"),
      }
    }
  }

  fn thankyou(&mut self) {
    let donor = self.get_donor_name();
    let amount = Donations::get_amount();

    println!();
    println!("Wow, {} donated {:.2}!", donor, amount);
    self.add(donor, amount);
  }
}


fn menu() -> Menu {
  loop {
    println!("Main Menu, please select an action");
    println!();
    println!("1) Send a Thank You");
    println!("2) Create a Report");
    println!("3) Quit");
    println!();
    print!("> ");
    io::stdout().flush().unwrap();
    let selection: Result<u8, _> = io::stdin().lock().lines().next().unwrap().unwrap().parse();

    match selection {
      Ok(1) => return Menu::SendThanks,
      Ok(2) => return Menu::CreateReport,
      Ok(3) => return Menu::Quit,
      _ => println!("Invalid selection!"),
    }
  }
}

fn main() {
  let mut donations = Donations::new();

  donations.add(String::from("John McAfee"), 1.0);
  donations.add(String::from("Ron"), 100.0);
  donations.add(String::from("Ron"), 1000.0);
  donations.add(String::from("Chris"), 1.0);
  donations.add(String::from("Chris"), 2.0);
  donations.add(String::from("Chris"), 3.0);
  donations.add(String::from("Person3"), 1.1);
  donations.add(String::from("Person3"), 1.2);
  donations.add(String::from("Person3"), 1.3);
  donations.add(String::from("Person4"), 100000.12);
  donations.add(String::from("Person4"), 12345.67);

  loop {
    match menu() {
      Menu::SendThanks => donations.thankyou(),
      Menu::CreateReport => donations.report(),
      Menu::Quit => process::exit(0),
    }

    println!();
  }
}
