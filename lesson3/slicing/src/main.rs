/* https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/slicing.html
 *
 * Write some functions that take a sequence as an argument, and return a copy of that sequence:
 *
 * * with the first and last items exchanged.
 * * with every other item removed.
 * * with the first 4 and the last 4 items removed, and then every other item in the remaining sequence.
 * * with the elements reversed (just with slicing).
 * * with the last third, then first third, then the middle third in the new order.
 *
 * NOTE: These should work with ANY sequence – but you can use strings to test, if you like.
 */
fn exchange_first_last(a: &str) -> String {
  // Convert to a vector so we can change it
  let mut out: Vec<char> = a.chars().collect();

  // Swap the first and last character
  let len = out.len();
  out.swap(0, len - 1);

  // Convert into a String
  out.into_iter().collect()
}

fn remove_every_other(a: &str) -> String {
  let mut i = 0;

  // Filter every odd character
  a.chars().filter(|_| {
    i += 1;
    (i % 2) == 1
  }).collect()
}

fn remove_first_and_last_four(a: &str) -> String {
  let mut out = String::new();

  let end = a.len() - 4;
  out.push_str(&a[4..end]);

  return out;
}

fn reverse(a: &str) -> String {
  a.chars().into_iter().rev().collect()
}

fn rearrange_thirds(a: &str) -> String {
  let size = a.len() / 3;

  let mut out = String::new();
  out.push_str(&a[(size*2)..(size*3)]);
  out.push_str(&a[(size*0)..(size*1)]);
  out.push_str(&a[(size*1)..(size*2)]);

  return out;
}

fn main() {
  assert!(exchange_first_last("this is a string") == "ghis is a strint");
  assert!(remove_every_other("this is a string") == "ti sasrn");
  assert!(remove_first_and_last_four("this is a string") == " is a st");
  assert!(reverse("this is a string") == "gnirts a si siht");
  assert!(rearrange_thirds("this is a string!!") == "ring!!this is a st");

  println!("Tests passed");
}
