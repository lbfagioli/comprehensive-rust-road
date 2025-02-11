use std::time::Duration;
// #[derive(Debug)]
// struct Some(i32);

// 12.2
struct Foo {
  x: (u32, u32),
  y: u32,
}

// 12.3
#[derive(Debug)]
enum Resulti {
  Ok(i32),
  Err(String),
}

fn divide_in_two(n: i32) -> Resulti {
  if n%2 == 0 {
    Resulti::Ok(n/2)
  } else {
    Resulti::Err(format!("cannot divide {n} into two equal parts"))
  }
}

// 12.4.1
fn sleep_for(secs: f32) {
  if let Ok(duration) = Duration::try_from_secs_f32(secs) {
    std::thread::sleep(duration);
    println!("slept for {duration:?}");
  }
}

// 12.4.3
// fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
//   // if let Some(s) = maybe_string {
//   //   if let Some(first_byte_char) = s.chars().next() {
//   //     if let Some(digit) = first_byte_char.to_digit(16) {
//   //       Ok(digit)
//   //     } else {
//   //       Err(String::from("not a hex digit"))
//   //     }
//   //   } else {
//   //     Err(String::from("got an empty string"))
//   //   }
//   // } else {
//   //   Err(String::from("got none"))
//   // }

//   let Some(s) = maybe_string else {
//     return Err(String::from("got none."));
//   };
//   let Some(first_byte_char) = s.chars().next() else {
//     return Err(String::from("got empty string."));
//   };
//   let Some(digit) = first_byte_char.to_digit(16) else {
//     return Err(String::from("not a hex digit"));
//   };
//   Ok(digit)
// }

// 12.5
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
  Add,
  Sub,
  Mul,
  #[allow(dead_code)] Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
  /// An operation on two subexpressions.
  Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

  /// A literal value
  Value(i64),
}

fn eval(e: Expression) -> i64 {
  match e {
    Expression::Value(val)             => val,
    Expression::Op { op, left, right } => match op {
      Operation::Add => eval(*left) + eval(*right),
      Operation::Sub => eval(*left) - eval(*right),
      Operation::Mul => eval(*left) * eval(*right),
      Operation::Div => eval(*left) / eval(*right),
    },
  }
}

#[test]
fn test_value() {
  assert_eq!(eval(Expression::Value(19)), 19);
}

#[test]
fn test_sum() {
  assert_eq!(
    eval(Expression::Op {
      op: Operation::Add,
      left: Box::new(Expression::Value(10)),
      right: Box::new(Expression::Value(20)),
    }),
    30
  );
}

#[test]
fn test_recursion() {
  let term1 = Expression::Op {
    op: Operation::Mul,
    left: Box::new(Expression::Value(10)),
    right: Box::new(Expression::Value(9)),
  };
  let term2 = Expression::Op {
    op: Operation::Mul,
    left: Box::new(Expression::Op {
      op: Operation::Sub,
      left: Box::new(Expression::Value(3)),
      right: Box::new(Expression::Value(4)),
    }),
    right: Box::new(Expression::Value(5)),
  };
  assert_eq!(
    eval(Expression::Op {
      op: Operation::Add,
      left: Box::new(term1),
      right: Box::new(term2),
    }),
    85
  );
}

#[test]
fn test_zeros() {
  assert_eq!(
    eval(Expression::Op {
      op: Operation::Add,
      left: Box::new(Expression::Value(0)),
      right: Box::new(Expression::Value(0))
    }),
    0
  );
  assert_eq!(
    eval(Expression::Op {
      op: Operation::Mul,
      left: Box::new(Expression::Value(0)),
      right: Box::new(Expression::Value(0))
    }),
    0
  );
  assert_eq!(
    eval(Expression::Op {
      op: Operation::Sub,
      left: Box::new(Expression::Value(0)),
      right: Box::new(Expression::Value(0))
    }),
    0
  );
}

// 13.1
#[derive(Debug)]
struct CarRace {
  name: String,
  laps: Vec<i32>,
}

impl CarRace {
  fn new(name: &str) -> Self {
    Self { name: String::from(name), laps: Vec::new() }
  }

  fn add_lap(&mut self, lap: i32) {
    self.laps.push(lap);
  }

  fn print_laps(&self) {
    println!("recorded {} laps for {}", self.laps.len(), self.name);
    for i in self.laps.iter() {
      println!("{i}");
    }
  }

  fn finish(self) {
    let len: i32 = self.laps.iter().sum();
    println!("race {} finished in a total lap time {}", self.name, len);
  }
}

// 13.2.1
trait Pet {
  fn talk(&self) -> String;

  fn greet(&self) {
    println!("hi, {}", self.talk());
  }
}

struct Dog {
  name: String,
}

impl Pet for Dog {
  fn talk(&self) -> String {
    format!("my name is {}", self.name)
  }
}

// 13.2.2
// trait Animal {
//   fn leg_count(&self) -> u32;
// }

// trait Pet: Animal {
//   fn name(&self) -> String;
// }

// struct Dog(String);

// impl Pet for Dog {
//   fn name(&self) -> String {
//     self.0.clone()
//   }
// }

// impl Animal for Dog {
//   fn leg_count(&self) -> u32 {
//     4
//   }
// }

// 13.2.3
#[derive(Debug)]
struct Meters(i32);

#[derive(Debug)]
struct MetersSquared(i32);

trait Multiply {
  type Output;
  fn multiply(&self, other: &Self) -> Self::Output;
}

impl Multiply for Meters {
  type Output = MetersSquared;
  fn multiply(&self, other: &Self) -> Self::Output {
    MetersSquared(self.0 * other.0)
  }
}

// 13.3
#[derive(Debug, Clone, Default)]
struct Player {
  name: String,
  strength: u8,
  hit_points: u8,
}

// 13.4
pub trait Logger {
  /// Log a message at the given verbosity level.
  fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
  fn log(&self, verbosity: u8, message: &str) {
      eprintln!("verbosity={verbosity}: {message}");
  }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter { // 14.3: VerbosityFilter<L> {
  max_verbosity: u8,
  inner: StderrLogger, // 14.3: L,
}

// TODO: Implement the `Logger` trait for `VerbosityFilter`.
impl Logger for VerbosityFilter { 
    fn log(&self, verbosity: u8, message: &str) {
    if self.max_verbosity >= verbosity {
      self.inner.log(verbosity, message);
    }
  }
}

// 14.1
fn pick<T>(n: i32, even: T, odd: T) -> T {
  if n%2 == 0 {
    even
  } else {
    odd
  }
}

// 14.2
fn duplicate<T: Clone>(a: T) -> (T, T) {
  (a.clone(), a.clone())
}

#[derive(Debug, Clone)]
struct NotClonable;

// 14.3: impl<L: Logger> Logger for VerbosityFilter<L> { 
// this will work for every logger L that implements Logger, so VerbosityFilter<i32> won't have this impl available

// 14.4
#[derive(Debug)]
struct Foofi(String);

/* https://doc.rust-lang.org/stable/std/convert/trait.From.html
 *
 * pub trait From<T>: Sized {
 *   fn from(value: T) -> Self;
 * }
 */

impl From<u32> for Foofi {
  fn from(from: u32) -> Foofi {
    Foofi(format!("converted from integer: {from}"))
  }
}

impl From<bool> for Foofi {
  fn from(from: bool) -> Foofi {
    Foofi(format!("converted from bool: {from}"))
  }
}

// 14.5
// fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
  x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
  (x+1, x-1)
}

// 14.6
struct Cat {
  lives: i8,
}

impl Pet for Cat {
  fn talk(&self) -> String {
    String::from("miau")
  }
}

fn generic(pet: &impl Pet) {
  println!("hi, who are you? {}", pet.talk());
}

fn dynamic(pet: &dyn Pet) {
  println!("hi, who are you? {}", pet.talk());
}

// 14.7
use std::cmp::Ordering;

fn min<T: Ord>(x: T, y: T) -> T {
  if x.cmp(&y) == Ordering::Less {
    x
  } else {
    y
  }
}

pub fn run() {
  // 11: Welcome
  println!("day 2 begins");

  // 12.1
  let input = 'x';
  match input {
    'q'                       => println!("quitting"),
    'a' | 's' | 'w' | 'd'     => println!("movement"),
    '0'..='9'                 => println!("digit detected"),
    key if key.is_lowercase() => println!("key: {key}"),
    _                         => println!("something else"),
  }
  let opt = Some(123);
  match opt {
    outer @ Some(inner) => {
      println!("outer: {outer:?}, inner: {inner}");
    }
    None => {}
  }

  // 12.2
  let foo = Foo { x: (2, 3), y: 2 };
  match &foo {
    Foo { x: (1, b), y } => println!("x.0: 1, b: {b}, y: {y}"),
    Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
    Foo { y, .. }        => println!("y = {y}, other fields ignored"),
  }

  // 12.3
  let n = 100;
  match divide_in_two(n) {
    Resulti::Ok(half) => println!("resulti is {half}"),
    Resulti::Err(err) => println!("{}", err),
  }

  // 12.4.1
  sleep_for(-10.0);
  sleep_for(0.01);

  // 12.4.2
  let mut name = String::from("Comprehensive rust 🦀");
  while let Some(c) = name.pop() {
    println!("char: {c}");
  }

  // 12.4.3
  // println!("result {:?}", hex_or_die_trying(Some(String::from("foo"))));

  // 12.5
  let expr = Expression::Op {
    op: Operation::Sub,
    left: Box::new(Expression::Value(20)),
    right: Box::new(Expression::Value(10)),
  };
  println!("expr: {expr:?}");
  println!("result: {:?}", eval(expr));

  // 13.1
  let mut race = CarRace::new("Monaco razors x10");
  race.add_lap(70);
  race.add_lap(68);
  race.print_laps();
  race.add_lap(69);
  race.print_laps();
  race.finish();
  // race.add_lap(42);

  // 13.2.1
  let dog = Dog { name: String::from("Fido") };
  dog.greet();

  // 13.2.2
  // let dogi = Dog(String::from("Doggie"));
  // println!("i have {} legs and my name is {}", dogi.leg_count(), dogi.name());

  // 13.2.3
  println!("{:?}", Meters(10).multiply(&Meters(20)));

  // 13.3
  let p1 = Player::default();
  let mut p2 = p1.clone();
  p2.name = String::from("Chad");
  println!("{p1:?} vs {p2:?}");

  // 13.4
  let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
  logger.log(5, "FYI");
  logger.log(2, "Uhoh");

  // 14.1
  println!("picked a number: {:?}", pick(97, 222, 333));
  println!("picked a string: {:?}", pick(28, "dog", "cat"));

  // 14.2
  let foo = String::from("foo");
  let pair = duplicate(foo);
  println!("{pair:?}");
  let not_clonable = NotClonable;
  let not2 = duplicate(not_clonable);
  println!("{not2:?}");

  // 14.4
  let from_int = Foofi::from(123);
  let from_bool = Foofi::from(true);
  println!("{from_int:?} and {from_bool:?}");

  // 14.5
  let many = add_42_millions(42_i8);
  println!("{many}");
  let many_more = add_42_millions(10_000_000);
  println!("{many_more}");
  let debuggable = pair_of(27);
  println!("debugable: {debuggable:?}");

  // 14.6
  let cat = Cat { lives: 9 };
  generic(&cat);
  generic(&dog);
  dynamic(&cat);
  dynamic(&dog);

  // 14.7
  assert_eq!(min(0, 10), 0);
  assert_eq!(min(500, 123), 123);

  assert_eq!(min('a', 'z'), 'a');
  assert_eq!(min('7', '1'), '1');

  assert_eq!(min("hello", "goodbye"), "goodbye");
  assert_eq!(min("bat", "armadillo"), "armadillo");
}