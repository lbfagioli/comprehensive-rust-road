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

  // 30.3
  // std::fs::write("config.dat", "alice").unwrap();
  let username = read_username("config.dat");
  println!("username or error: {username:?}");

  // 30.4
  // std::fs::write("config.dat", "").unwrap();
  let username2 = read_username2("config.dat");
  println!("username or error: {username2:?}");

  // 30.5
  // std::fs::write("config.dat", "1i3").unwrap();
  match read_count("config.dat") {
    Ok(count) => println!("count is {count}"),
    Err(error) => println!("got an error: {error}"),
  }

  // 30.6
  // std::fs::write("config.dat", "").unwrap();
  let username3 = read_username3("config.dat");
  println!("username or error: {username3:?}");

  // 30.7
  // std::fs::write("config.dat", "").unwrap();
  match read_username4("config.dat") {
    Ok(username) => println!("user: {username}"),
    Err(err) => println!("got err: {err}"),
  }
}