/*
 * https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/fizz_buzz.html
 *
 * Fizz Buzz is a classic simple problem in computer science.
 *
 * It is often used as an exercise in interviews for programmers.
 *
 * Apparently a LOT of people applying for jobs as professional developers can’t do this in an interview:
 *
 * (http://c2.com/cgi/wiki?FizzBuzzTest)
 *
 * Now that we’ve psyched you out – it’s really pretty straightforward.
 *
 * Goal:
 * Write a program that prints the numbers from 1 to 100 inclusive.
 * But for multiples of three print “Fizz” instead of the number.
 * For the multiples of five print “Buzz” instead of the number.
 * For numbers which are multiples of both three and five print “FizzBuzz” instead.
*/

use std::env;

fn main() {
  let mut args = env::args().skip(1);

  let max = args.next().unwrap_or(String::from("100")).parse::<usize>().unwrap();

  for i in 1..(max+1) {
    if (i % 3 == 0) && (i % 5 == 0) {
      println!("FizzBuzz");
    } else if i % 3 == 0 {
      println!("Fizz");
    } else if i % 5 == 0 {
      println!("Buzz");
    } else {
      println!("{}", i);
    }
  }
}
