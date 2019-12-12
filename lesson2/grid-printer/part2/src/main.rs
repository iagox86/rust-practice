// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/grid_printer.html#part-2

use std::env;

fn divider(n: usize) {
  let a = vec![" "; n];

  for _ in 0..2 {
    print!("+{}", a.join("-"));
  }
  println!("+");
}

fn inside(n: usize) {
  let a = vec![" "; n];

  for _ in 0..2 {
    print!("|{}", a.join(" "));
  }
  println!("|");
}

fn grid(n: usize) {
  divider(n);

  for _ in 0..n {
    inside(n);
  }

  divider(n);

  for _ in 0..n {
    inside(n);
  }

  divider(n);
}

fn main() {
  let n = env::args().skip(1).next().unwrap_or(String::from("3")).parse::<usize>().unwrap();
  println!("Arg = {}", n);

  grid((n / 2) + 1);
}
