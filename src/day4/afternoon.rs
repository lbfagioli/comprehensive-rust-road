// 30.1
use std::panic;

// 30.2
use std::fs::File;
use std::io::Read;

// 30.3
fn read_username(path: &str) -> Result<String, std::io::Error> {
  let mut username_file = File::open(path)?;

  let mut username = String::new();
  username_file.read_to_string(&mut username)?;

  Ok(username)
}

// 30.4
#[derive(Debug)]
enum ReadUsernameError {
  IoError(std::io::Error),
  EmptyUsername(String),
}

impl std::error::Error for ReadUsernameError {}

impl std::fmt::Display for ReadUsernameError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::IoError(e) => write!(f, "I/O err: {e}"),
      Self::EmptyUsername(path) => write!(f, "Empty string in path {path}"),
    }
  }
}

impl From<std::io::Error> for ReadUsernameError {
  fn from(err: std::io::Error) -> Self {
    Self::IoError(err)
  }
}

fn read_username2(path: &str) -> Result<String, ReadUsernameError> {
  let mut username = String::with_capacity(100);
  File::open(path)?.read_to_string(&mut username)?;
  if username.is_empty() {
    return Err(ReadUsernameError::EmptyUsername(String::from(path)));
  }

  Ok(username)
}

// 30.5
fn read_count(path: &str) -> Result<i32, Box<dyn std::error::Error>> { // must impl Error in custom errors to do this
  let mut count_str = String::new();
  File::open(path)?.read_to_string(&mut count_str)?;
  let count: i32 = count_str.parse()?;
  Ok(count)
}

// 30.6
use thiserror::Error;

#[derive(Debug, Error)]
enum ReadUsernameError2 {
  #[error("I/O error")]
  IoError(#[from] std::io::Error),
  #[error("Found no username in {0}")]
  EmptyUsername(String),
}

fn read_username3(path: &str) -> Result<String, ReadUsernameError2> {
  let mut username = String::with_capacity(100);
  File::open(path)?.read_to_string(&mut username)?;
  if username.is_empty() {
    return Err(ReadUsernameError2::EmptyUsername(String::from(path)));
  }
  Ok(username)
}

// 30.7
use anyhow::{bail, Context, Result};

#[derive(Debug, Clone, Eq, Error, PartialEq)]
#[error("Did not find an user in {0}")]
struct EmptyUserError(String);

fn read_username4(path: &str) -> Result<String> {
  let mut username = String::with_capacity(100);
  File::open(path)
    .with_context(|| format!("failed to open {path}"))?
    .read_to_string(&mut username)
    .context("failed to read")?;
  if username.is_empty() {
    bail!(EmptyUserError(path.to_string()));
  }
  Ok(username)
}

// 30.8
/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
  Add,
  Sub,
  Mul,
  Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
  /// An operation on two subexpressions.
  Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

  /// A literal value
  Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

// The original implementation of the expression evaluator. Update this to
// return a `Result` and produce an error when dividing by 0.
fn eval(e: Expression) -> Result<i64, DivideByZeroError> {
  match e {
    Expression::Op { op, left, right } => {
      let Ok(left) = eval(*left) else { // should have done let x = eval()?;
        return Err(DivideByZeroError);
      };
      let Ok(right) = eval(*right) else { // should have done let x = eval()?;
        return Err(DivideByZeroError);
      };
      match op { // could wrap entire match in a Ok(match op {...}) instead of returning Ok(left op right) for every instance
        Operation::Add => Ok(left + right), // wrong
        Operation::Sub => Ok(left - right), // wrong
        Operation::Mul => Ok(left * right), // wrong
        Operation::Div => if right != 0 {
          Ok(left / right) // wrong
        } else {
          return Err(DivideByZeroError);
        },
      }
    }
    Expression::Value(v) => Ok(v),
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_error() {
    assert_eq!(
      eval(Expression::Op {
        op: Operation::Div,
        left: Box::new(Expression::Value(99)),
        right: Box::new(Expression::Value(0)),
      }),
      Err(DivideByZeroError)
    );
  }

  #[test]
  fn test_ok() {
    let expr = Expression::Op {
      op: Operation::Sub,
      left: Box::new(Expression::Value(20)),
      right: Box::new(Expression::Value(10)),
    };
    assert_eq!(eval(expr), Ok(10));
  }
}

pub fn run() {
  println!("\nday4::afternoon::run");

  // 30.1
  // let v = vec![0, 1, 4];
  // println!("v[100]: {}", v[100]); // panic at runtime by access out of bounds
  let result = panic::catch_unwind(|| "no problem here..");
  println!("result: {result:?}");
  let result = panic::catch_unwind(|| {
    panic!("oh no, an error!");
  });
  println!("result2: {result:?}");
  println!("keeps going despite panicking as unwind got caught");
  println!();

  // 30.2
  let file: Result<File, std::io::Error> = File::open("diary.txt");
  match file {
    Ok(mut actual_file) => {
      let mut contents = String::new();
      if let Ok(bytes) = actual_file.read_to_string(&mut contents) {
        println!("contents: {contents}, bytes: {bytes}");
      } else {
        println!("could not read file");
      }
    }
    Err(err) => println!("error reading file")
  }
  println!();

  // 30.3
  // std::fs::write("config.dat", "alice").unwrap();
  let username = read_username("config.dat");
  println!("username or error: {username:?}");
  println!();

  // 30.4
  // std::fs::write("config.dat", "").unwrap();
  let username2 = read_username2("config.dat");
  println!("username or error: {username2:?}");
  println!();

  // 30.5
  // std::fs::write("config.dat", "1i3").unwrap();
  match read_count("config.dat") {
    Ok(count) => println!("count is {count}"),
    Err(error) => println!("got an error: {error}"),
  }
  println!();

  // 30.6
  // std::fs::write("config.dat", "").unwrap();
  let username3 = read_username3("config.dat");
  println!("username or error: {username3:?}");
  println!();

  // 30.7
  // std::fs::write("config.dat", "").unwrap();
  match read_username4("config.dat") {
    Ok(username) => println!("user: {username}"),
    Err(err) => println!("got err: {err}"),
  }
  println!();

  // 31.2
  let mut s = String::from("careful");
  let r1 = &raw mut s;
  let r2 = r1 as *const String;

  unsafe {
    println!("r1: {}", *r1);
    *r1 = String::from("what could go wrong");
    println!("r2: {}", *r2);
  }

  // should never do this (kinda obvious):
  // let r3: &String = unsafe { &*r1 };
  // drop(s);
  // println!("r3 is {}", *r3);
}