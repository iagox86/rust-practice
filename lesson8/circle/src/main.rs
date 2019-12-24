// https://uwpce-pythoncert.github.io/PythonCertDevel/exercises/circle_class.html

use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug)]
struct Circle {
  radius: f64
}

impl Circle {
  fn new(radius: f64) -> Self {
    Circle {
      radius: radius
    }
  }

  fn from_diameter(diameter: f64) -> Self {
    Self::new(diameter / 2.0)
  }

  fn diameter(&self) -> f64 {
    self.radius * 2.0
  }

  fn set_diameter(&mut self, diameter: f64) {
    self.radius = diameter / 2.0;
  }

  fn area(&self) -> f64 {
    self.radius * self.radius * 3.14
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Circle with radius: {}", self.radius)
  }
}

impl ops::Add for Circle {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self::new(self.radius + other.radius)
  }
}

impl ops::Mul<f64> for Circle {
  type Output = Self;

  fn mul(self, scalar: f64) -> Self {
    Self::new(self.radius * scalar)
  }
}

// Implementing this backwards is kinda silly, but following the exercise :)
impl ops::Mul<Circle> for f64 {
  type Output = Circle;

  fn mul(self, circle: Circle) -> Circle {
    circle * self
  }
}

impl cmp::PartialOrd for Circle {
  fn partial_cmp(&self, rhs: &Circle) -> Option<cmp::Ordering> {
    self.radius.partial_cmp(&rhs.radius)
  }
}

impl cmp::PartialEq for Circle {
  fn eq(&self, rhs: &Circle) -> bool {
    self.radius.eq(&rhs.radius)
  }
}

fn main() {
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_circle_with_radius() {
    let c = Circle::new(3.0);
    assert_eq!(c.radius, 3.0);
  }

  #[test]
  fn test_diameter() {
    let c = Circle::new(3.0);
    assert_eq!(c.diameter(), 6.0);
  }

  #[test]
  fn test_set_diameter() {
    let mut c = Circle::new(3.0);
    assert_eq!(c.diameter(), 6.0);

    c.set_diameter(11.0);
    assert_eq!(c.radius, 5.5);
  }

  #[test]
  fn test_area() {
    let c = Circle::new(10.0);
    assert_eq!(c.area(), 314.0);
  }

  #[test]
  fn test_new_diameter() {
    let c = Circle::from_diameter(3.0);
    assert_eq!(c.radius, 1.5);
  }

  #[test]
  fn test_display() {
    let c = Circle::new(5.0);

    assert_eq!("Circle with radius: 5", format!("{}", c));
  }

  #[test]
  fn test_add() {
    let c = Circle::new(5.0) + Circle::new(1.5);
    assert_eq!(6.5, c.radius);
  }

  #[test]
  fn test_mul() {
    let c = Circle::new(5.0) * 3.0;
    assert_eq!(15.0, c.radius);
  }

  #[test]
  fn test_silly_mul() {
    let c = 3.0 * Circle::new(5.0);
    assert_eq!(15.0, c.radius);
  }

  #[test]
  fn test_eq() {
    assert_eq!(true,  Circle::new(5.0) == Circle::new(5.0));
    assert_eq!(false, Circle::new(6.0) == Circle::new(5.0));
  }

  #[test]
  fn test_cmp() {
    assert_eq!(true, Circle::new(5.0) < Circle::new(6.0));
    assert_eq!(true, Circle::new(6.0) > Circle::new(5.0));

    assert_eq!(false, Circle::new(5.0) > Circle::new(6.0));
    assert_eq!(false, Circle::new(6.0) < Circle::new(5.0));
  }

  #[test]
  fn test_sort() {
    let mut v = vec![Circle::new(5.0), Circle::new(5.0), Circle::new(6.0), Circle::new(4.0)];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(v, vec![Circle::new(4.0), Circle::new(5.0), Circle::new(5.0), Circle::new(6.0)]);
  }

  #[test]
  fn test_uniq() {
    let mut v = vec![Circle::new(5.0), Circle::new(4.0), Circle::new(6.0), Circle::new(5.0)];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v.dedup();

    assert_eq!(v, vec![Circle::new(4.0), Circle::new(5.0), Circle::new(6.0)]);
  }
}
