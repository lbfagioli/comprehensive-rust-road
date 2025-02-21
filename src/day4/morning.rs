// 26.2
struct SliceIterator<'s, T> {
  slice: &'s [T],
  i: usize,
}

impl<'s, T> Iterator for SliceIterator<'s, T> {
  type Item = &'s T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.i == self.slice.len() {
      None
    } else {
      let next = &self.slice[self.i];
      self.i += 1;
      Some(next)
    }
  }
}

// 26.5
struct Grid {
  x_coords: Vec<i32>,
  y_coords: Vec<i32>,
}

impl IntoIterator for Grid {
  type Item = (i32, i32);
  type IntoIter = GridIter;
  fn into_iter(self) -> GridIter {
    GridIter { grid: self, i: 0, j: 0 }
  }
}
struct GridRef {
  x_coords: Vec<i32>,
  y_coords: Vec<i32>,
}

impl<'a> IntoIterator for &'a GridRef {
  type Item = (i32, i32);
  type IntoIter = GridRefIter<'a>;
  fn into_iter(self) -> GridRefIter<'a> {
    GridRefIter { grid: self, i: 0, j: 0 }
  }
}

struct GridIter {
  grid: Grid,
  i: usize,
  j: usize,
}

struct GridRefIter<'a> {
  grid: &'a GridRef,
  i: usize,
  j: usize,
}

impl Iterator for GridIter {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<(i32, i32)> {
    if self.i == self.grid.x_coords.len() {
      self.i = 0;
      self.j += 1;
      if self.j == self.grid.y_coords.len() {
        return None;
      }
    }

    let next = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
    self.i += 1;

    next

  }
}

impl<'a> Iterator for GridRefIter<'a> {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<(i32, i32)> {
    if self.i == self.grid.x_coords.len() {
      self.i = 0;
      self.j += 1;
      if self.j == self.grid.y_coords.len() {
        return None;
      }
    }

    let next = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
    self.i += 1;

    next

  }
}

// 26.6
/// Calculate the differences between elements of `values` offset by `offset`,
/// wrapping around from the end of `values` to the beginning.
///
/// Element `n` of the result is `values[(n+offset)%len] - values[n]`.
fn offset_differences(offset: usize, values: Vec<i32>) -> Vec<i32> {
  // An actual so better solution:
  // let a = values.iter();
  // let b = values.iter().cycle().skip(offset);
  // a.zip(b).map(|(a, b)| *b - *a).collect()
  values.clone()
    .into_iter()
    .enumerate()
    .map(|(i, x)| values[(i+offset) % values.len()] - x)
    .collect::<Vec<_>>()
}

#[test]
fn test_offset_one() {
  assert_eq!(offset_differences(1, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
  assert_eq!(offset_differences(1, vec![1, 3, 5]), vec![2, 2, -4]);
  assert_eq!(offset_differences(1, vec![1, 3]), vec![2, -2]);
}

#[test]
fn test_larger_offsets() {
  assert_eq!(offset_differences(2, vec![1, 3, 5, 7]), vec![4, 4, -4, -4]);
  assert_eq!(offset_differences(3, vec![1, 3, 5, 7]), vec![6, -2, -2, -2]);
  assert_eq!(offset_differences(4, vec![1, 3, 5, 7]), vec![0, 0, 0, 0]);
  assert_eq!(offset_differences(5, vec![1, 3, 5, 7]), vec![2, 2, 2, -6]);
}

#[test]
fn test_degenerate_cases() {
  assert_eq!(offset_differences(1, vec![0]), vec![0]);
  assert_eq!(offset_differences(1, vec![1]), vec![0]);
  let empty: Vec<i32> = vec![];
  assert_eq!(offset_differences(1, empty), vec![]);
}

// 27.1
mod foo {
  pub fn do_something() {
    println!("foo");
  }
}

mod bar {
  pub fn do_something() {
    println!("bar");
  }
}

// 27.3
mod outer {
  fn private() {
    println!("outer::private");
  }

  pub fn public() {
    println!("outer::public");
  }

  pub mod inner {
    fn private() {
      println!("outer::inner::private");
    }

    pub fn public() {
      println!("outer::inner::public");
      super::private();
    }
  }
}

// 27.4
use outer2::Foo;

mod outer2 {
  pub struct Foo {
    pub val: i32,
    is_big: bool,
  }

  impl Foo {
    pub fn new(val: i32) -> Self {
      Self { val, is_big: val > 100 }
    }
  }

  pub mod inner {
    use super::Foo;

    pub fn print_foo(foo: &Foo) {
      println!("is {} big? {}", foo.val, foo.is_big);
    }
  }
}

// 27.6
use super::widget::window::Window;
use super::widget::label::Label;
use super::widget::button::Button;
use super::widget::widget::Widget;

// 28.1
fn first_word(string: &str) -> &str {
  match string.find(" ") {
    Some(index) => &string[..index],
    None => &string
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    assert_eq!(first_word(""), "");
  }

  #[test]
  fn test_one_word() {
    assert_eq!(first_word("hello"), "hello");
  }

  #[test]
  fn test_multiple_words() {
    assert_eq!(first_word("fine or not?"), "fine");
  }
}

// 28.3
// #[deny(clippy::cast_possible_truncation)]

// 28.4
pub fn luhn(cc_number: &str) -> bool {
  let mut sum = 0;
  let mut double = false;
  let mut digit_count = 0;

  for c in cc_number.chars().rev() {
      if let Some(digit) = c.to_digit(10) {
          if double {
              let double_digit = digit * 2;
              sum +=
                  if double_digit > 9 { double_digit - 9 } else { double_digit };
          } else {
              sum += digit;
          }
          double = !double;
          digit_count += 1;
      } else if c != ' ' {
          return false;
      } else {
        continue;
      }
  }

  if digit_count < 2 {
    return false;
  }

  sum % 10 == 0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_valid_cc_number() {
      assert!(luhn("4263 9826 4026 9299"));
      assert!(luhn("4539 3195 0343 6467"));
      assert!(luhn("7992 7398 713"));
  }

  #[test]
  fn test_invalid_cc_number() {
      assert!(!luhn("4223 9826 4026 9299"));
      assert!(!luhn("4539 3195 0343 6476"));
      assert!(!luhn("8273 1232 7352 0569"));
  }

  #[test]
  fn test_first_mine() {
    assert_eq!(luhn("43"), false);
    assert_eq!(luhn(""), false);
    assert_eq!(luhn("0    0"), true);
  }

  #[test]
  fn test_non_digit_cc_number() {
      assert!(!luhn("foo"));
      assert!(!luhn("foo 0 0"));
  }

  #[test]
  fn test_empty_cc_number() {
      assert!(!luhn(""));
      assert!(!luhn(" "));
      assert!(!luhn("  "));
      assert!(!luhn("    "));
  }

  #[test]
  fn test_single_digit_cc_number() {
      assert!(!luhn("0"));
  }

  #[test]
  fn test_two_digit_cc_number() {
      assert!(luhn(" 0 0 "));
  }
}

pub fn run() {
  println!("so 'day4' starts..");

  // 26.2
  let slice = &[2, 4, 5, 3];
  let iter = SliceIterator { slice: slice, i: 0 };
  for elem in iter {
    println!("{elem}");
  }

  // 26.3
  println!();
  let result: i32 = (1..=10)
    .filter(|x| x%2 == 0)
    .map(|x| x*x)
    .sum();
  println!("sum of squared even nums up to 10: {result}");

  // 26.4
  println!();
  let primes = vec![2, 3, 5, 7];
  // let prime_squares = primes.into_iter().map(|elem| elem * elem).collect::<Vec<_>>();
  let prime_squares: Vec<_> = primes.into_iter().map(|elem| elem * elem).collect();
  println!("prime squares: {prime_squares:?}");

  // 26.5
  println!();
  let grid = Grid { x_coords: vec![1, 3, 5, 7], y_coords: vec![2, 4, 6, 8] };
  for elem in grid {
    println!("coords = {elem:?}");
  }
  // this won't compile as grid got moved due to into_iter() taking ownership
  // for elem in grid {
  //   println!("coords2 = {elem:?}");
  // }
  let grid_ref = GridRef { x_coords: vec![1, 3, 5, 7], y_coords: vec![2, 4, 6, 8] };
  for (x, y) in &grid_ref {
    println!("grid ref 1; x: {x}, y: {y}");
  }
  for (x, y) in &grid_ref {
    println!("grid ref 2; x: {x}, y: {y}");
  }

  // 27.1
  println!();
  foo::do_something();
  bar::do_something();

  // 27.3
  println!();
  outer::public();
  outer::inner::public();
  // outer::inner::private(); // this won't compile as inner::priv is not pub despite inner mod being pub

  // 27.4
  println!();
  let foo = Foo::new(42);
  // let foo2 = Foo { val: 34, is_big: true }; // compile error: is_big is private
  outer2::inner::print_foo(&foo);
  outer2::inner::print_foo(&foo);
  // println!("is {} big? {}", foo.val, foo.is_big); // compile error: same as before
  println!();

  // 27.6
  let mut window = Window::new("Rust GUI Demo 1.23");
  window.add_widget(Box::new(Label::new("This is a small text GUI demo.")));
  window.add_widget(Box::new(Button::new("Click me!")));
  window.draw();

  // 28.3
  let mut x = 3;
  while (x < 700000) {
    x *= 2;
  }
  println!("x should fit in u16: {}", x as u16);
}