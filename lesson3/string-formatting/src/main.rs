// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/string_formatting.html
use inflector::string::singularize::to_singular;

fn task_one(filename: u32, float: f64, scientific: u64, long_float: f64) {
  // The first element is used to generate a filename that can help with file sorting. The idea behind the “file_002” is that if you have a bunch of files that you want to name with numbers that can be sorted, you need to “pad” the numbers with zeros to get the right sort order.
  println!("Filename with 0-padded three digits: file_{:03}", filename);

// The second element is a floating point number. You should display it with 2 decimal places shown.
  println!("Float to two decimal places: {:.2}", float);

// The third value is an integer, but could be any number. You should display it in scientific notation, with 2 decimal places shown.
  println!("Integer in scientific notation: {:e}", scientific as f64);

// The fourth value is a float with a lot of digits – display it in scientific notation with 3 significant figures.
  println!("Long float with three significant figures: {:.3e}", long_float);
}

fn task_two(filename: u32, float: f64, scientific: u64, long_float: f64) {
  // Using your results from Task One, repeat the exercise, but this time using an alternate type of format string (hint: think about alternative ways to use .format() (keywords anyone?), and also consider f-strings if you’ve not used them already).


  // The first element is used to generate a filename that can help with file sorting. The idea behind the “file_002” is that if you have a bunch of files that you want to name with numbers that can be sorted, you need to “pad” the numbers with zeros to get the right sort order.
  println!("Filename with 0-padded three digits: file_{filename:03}\nFloat to two decimal places: {float:.2}\nInteger in scientific notation: {scientific:e}\nLong float with three significant figures: {long_float:.3e}", filename=filename, float=float, scientific=(scientific as f64), long_float=long_float);
}

fn _task_three() {
  // Dynamically Building up format strings
  //
  // Unfortunately, that's not really a thing in Rust: https://stackoverflow.com/a/32580595
}

fn task_four() {
  // Given a 5 element tuple:
  // ( 4, 30, 2017, 2, 27)
  // use string formating to print:
  // '02 27 2017 04 30'
  // Hint: use index numbers to specify positions.
  let a = ( 4, 30, 2017, 2, 27);

  //println!("{4:.2} {5:} {3:} {1:.1} {2}", a.0, a.1, a.2, a.3, a.4);
  println!("Out:      {3:02} {4} {2} {0:02} {1}", a.0, a.1, a.2, a.3, a.4);
  println!("Expected: 02 27 2017 04 30");
}

fn task_five() {
  // Here’s a task for you: Given the following four element list:

  // ['oranges', 1.3, 'lemons', 1.1]
  let a = ("oranges", 1.3, "lemons", 1.1);

  // Write an f-string that will display:
  // The weight of an orange is 1.3 and the weight of a lemon is 1.1
  println!("The weight of an {orange} is {weight1} and the weight of a {lemon} is {weight2}", orange=to_singular(a.0.into()), weight1=a.1, lemon=to_singular(a.2.into()), weight2=a.3);

  // Now see if you can change the f-string so that it displays the names of the fruit in upper case, and the weight 20% higher (that is 1.2 times higher).
  println!("The weight of an {orange} is {weight1} and the weight of a {lemon} is {weight2}", orange=to_singular(a.0.into()).to_uppercase(), weight1=a.1 * 1.20, lemon=to_singular(a.2.into()).to_uppercase(), weight2=a.3 * 1.20);
}

fn task_six() {
  // Write some Python code to print a table of several rows, each with a name, an age and a cost. Make sure some of the costs are in the hundreds and thousands to test your alignment specifiers.
  println!("{name:32} {age:5} {cost:8.2}", name="Ron",             age=12,   cost=123.45);
  println!("{name:32} {age:5} {cost:8.2}", name="Christopher",     age=123,  cost=11123.45);
  println!("{name:32} {age:5} {cost:8.2}", name="Jim",             age=40,   cost=1223.345);
  println!("{name:32} {age:5} {cost:8.2}", name="Edgar Allen Poe", age=400,  cost=1.45);
  println!("{name:32} {age:5} {cost:8.2}", name="Justin Trudeau",  age=1222, cost=0.45);

  // And for an extra task, given a tuple with 10 consecutive numbers, can you work how to quickly print the tuple in columns that are 5 charaters wide? It can be done on one short line!
  let a = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
  println!("{:5} {:5} {:5} {:5} {:5} {:5} {:5} {:5} {:5} {:5} ", a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8, a.9);
}

fn main() {
  task_one(2, 123.4567, 10001, 1234567.89);
  println!();
  task_two(2, 123.4567, 10001, 1234567.89);
  println!();
  task_four();
  println!();
  //task_five();
  println!();
  task_six();
}
