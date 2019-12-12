// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/grid_printer.html

fn divider() {
  let a = [" "; 5];

  for _ in 0..2 {
    print!("+{}", a.join("-"));
  }
  println!("+");
}

fn inside() {
  let a = [" "; 5];

  for _ in 0..2 {
    print!("|{}", a.join(" "));
  }
  println!("|");
}

fn grid() {
  divider();

  for _ in 0..5 {
    inside();
  }

  divider();

  for _ in 0..5 {
    inside();
  }

  divider();
}

fn main() {
  grid();
}
