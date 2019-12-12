// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/fib_and_lucas.html

fn sum_series(n: usize, n0: Option<usize>, n1: Option<usize>) -> usize {
  let n0 = n0.unwrap_or(0);
  let n1 = n1.unwrap_or(1);

  if n == 0 {
    return n0;
  } else if n == 1 {
    return n1;
  }

  sum_series(n - 1, Some(n0), Some(n1)) + sum_series(n - 2, Some(n0), Some(n1))
}

fn fibonacci(n: usize) -> usize {
  sum_series(n, None, None)
}

fn lucas(n: usize) -> usize {
  sum_series(n, Some(2), Some(1))
}

fn main() {
  assert!(fibonacci(0) == 0);
  assert!(fibonacci(1) == 1);
  assert!(fibonacci(2) == 1);
  assert!(fibonacci(3) == 2);
  assert!(fibonacci(4) == 3);
  assert!(fibonacci(5) == 5);
  assert!(fibonacci(6) == 8);
  assert!(fibonacci(7) == 13);

  assert!(lucas(0) == 2);
  assert!(lucas(1) == 1);

  assert!(lucas(4) == 7);

  assert!(sum_series(5, None, None) == fibonacci(5));
  assert!(sum_series(7, Some(0), Some(1)) == fibonacci(7));

  assert!(sum_series(5, Some(2), Some(1)) == lucas(5));

  assert!(sum_series(0, Some(3), Some(2)) == 3);
  assert!(sum_series(1, Some(3), Some(2)) == 2);
  assert!(sum_series(2, Some(3), Some(2)) == 5);
  assert!(sum_series(3, Some(3), Some(2)) == 7);
  assert!(sum_series(4, Some(3), Some(2)) == 12);
  assert!(sum_series(5, Some(3), Some(2)) == 19);

  println!("Tests passed");
}
