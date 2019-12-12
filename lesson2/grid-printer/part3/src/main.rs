// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/grid_printer.html#part-3

use std::env;

fn divider(cell_count: usize, cell_size: usize) {
  let a = vec![" "; cell_size];

  for _ in 0..cell_count {
    print!("+{}", a.join("-"));
  }
  println!("+");
}

fn inside(cell_count: usize, cell_size: usize) {
  let a = vec![" "; cell_size];

  for _ in 0..cell_count {
    print!("|{}", a.join(" "));
  }
  println!("|");
}

fn grid(cell_count: usize, cell_size: usize) {
  for _ in 0..cell_count {
    divider(cell_count, cell_size);

    for _ in 0..cell_size {
      inside(cell_count, cell_size);
    }
  }

  divider(cell_count, cell_size);
}

fn main() {
  let mut args = env::args().skip(1);

  let cell_count = args.next().unwrap_or(String::from("3")).parse::<usize>().unwrap();
  let cell_size  = args.next().unwrap_or(String::from("4")).parse::<usize>().unwrap();

  grid(cell_count, cell_size);
}
