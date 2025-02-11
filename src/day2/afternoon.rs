// 16.4
use std::fs::File;
use std::io::Read;

// 16.7
use std::collections::HashMap;

// 16.8
/// Counter counts the number of times each value of type T has been seen.
use std::hash::Hash;

struct Counter<K> {
  values: HashMap<K, u64>,
}

impl<K: Eq + Hash> Counter<K> {
  /// Create a new Counter.
  fn new() -> Self {
      Counter {
          values: HashMap::new(),
      }
  }

  /// Count an occurrence of the given value.
  fn count(&mut self, value: K) {
      if self.values.contains_key(&value) {
          *self.values.get_mut(&value).unwrap() += 1;
      } else {
          self.values.insert(value, 1);
      }
  }

  /// Return the number of times the given value has been seen.
  fn times_seen(&self, value: K) -> u64 {
      self.values.get(&value).copied().unwrap_or_default()
  }
}

// 17.1
use std::cmp::Ordering;

struct Key {
  id: u32,
  metadata: Option<String>,
}
impl PartialEq for Key {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

#[derive(PartialEq)]
struct Citation {
  author: String,
  year: u32,
}
impl PartialOrd for Citation {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.author.partial_cmp(&other.author) {
      Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
      author_ord => author_ord,
    }
  }
}

// 17.2
#[derive(Debug, Copy, Clone)]
struct Point {
  x: i32,
  y: i32,
}

impl std::ops::Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self { x: self.x + other.x, y: self.y + other.y }
  }
}

// 17.5
use std::io::{BufRead, BufReader, Result}; // almost reimported Read, also had to take Result out

fn count_lines<R: Read>(reader: R) -> usize {
  let buf_reader = BufReader::new(reader);
  buf_reader.lines().count()
}

use std::io::Write;
fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
  writer.write_all(msg.as_bytes())?;
  writer.write_all("\n".as_bytes())
}

// 17.6
#[derive(Debug, Default)]
struct Derived {
  x: u32,
  y: String,
  z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
  fn default() -> Self {
    Self("John smith".into())
  }
}

// 17.7
struct RotDecoder<R: Read> {
  input: R,
  rot: u8,
}

impl<R: Read> Read for RotDecoder<R> {
  fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
    // let bytes: &[u8] = self.input.as_bytes();
    // let mut bytes = [0; 1000];
    let mut i: usize = 0;
    let n_bytes = self.input.read(buf)?;
    for l in &mut buf[..n_bytes] {
      if *l >= 65 && *l <= 90 {
        if *l + self.rot > 90 {
          *l = 65 + *l + self.rot - 90 - 1;
        } else {
          *l = *l + self.rot;
        }
      } else if *l >= 97 && *l <= 122 {
        if *l + self.rot > 122 {
          *l = 97 + *l + self.rot - 122 - 1;
        } else {
          *l = *l + self.rot;
        }
      }
      i += 1;
    }
    Ok(i)
  }
}

// 18.3
fn apply_and_log(func: impl FnOnce(String) -> String, name: &str, input: &str) {
  println!("{name}({input}) = {}", func(input.into()));
}

// 18.4
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

struct Filter<F: Fn(u8, &str) -> bool> {
  inner: StderrLogger,
  func: F,
}

impl<F: Fn(u8, &str) -> bool> Filter<F> {
  fn new(inn: StderrLogger, f: F) -> Self {
    Self { inner: inn, func: f }
  }

  fn log(&self, verbosity: u8, msg: &str) {
    if (self.func)(verbosity, msg) { self.inner.log(verbosity, msg) };
  }
}

pub fn run() -> Result<()> {
  // 16.3
  let name = "Löwe 老虎 Léopard Gepardi";
  let mut position: Option<usize> = name.find('é');
  println!("find returned {position:?}");
  assert_eq!(position.unwrap(), 14);
  position = name.find('Z');
  println!("find returned {position:?}");
  // assert_eq!(position.expect("Character not found"), 0);

  // 16.4
  // let file: Result<File, std::io::Error> = File::open("diary.txt");
  let file: Result<File> = File::open("diary.txt");
  match file {
    Ok(mut file) => {
      let mut contents = String::new();
      if let Ok(bytes) = file.read_to_string(&mut contents) {
        println!("dear diary: {contents}, ({bytes} bytes)");
      } else {
        println!("could not read file content");
      }
    }
    Err(err) => {
      println!("could not open the file: {err}");
    }
  }

  // 16.5
  let mut s1 = String::new();
  s1.push_str("hello");
  println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());
  let mut s2 = String::with_capacity(s1.len() + 1);
  s2.push_str(&s1);
  s2.push('!');
  println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());
  let s3 = String::from("🇨🇭");
  println!("s3: len = {}, numb chars = {}", s3.len(), s3.chars().count());

  // 16.6
  let mut v1 = Vec::new();
  v1.push(42);
  println!("v1: len = {}, cap = {}", v1.len(), v1.capacity());
  let mut v2 = Vec::with_capacity(v1.len() + 1);
  v2.extend(v1.iter());
  v2.push(9999);
  println!("v2: {v2:?}, len = {}, cap = {}", v2.len(), v2.capacity());
  let mut v3 = vec![0, 0, 1, 2, 3, 4];
  v3.retain(|x| x%2 == 0);
  println!("{v3:?}");
  v3.dedup();
  println!("{v3:?}");

  // 16.7
  let mut page_counts = HashMap::new();
  page_counts.insert("Adventures of Huckleberry Finn", 207);
  page_counts.insert("Grimms' Fairy Tails", 751);
  page_counts.insert("Pride and Prejudice", 303);
  if !page_counts.contains_key("Les Misérables") {
    println!(
      "we know about {} books but not Les Misérables",
      page_counts.len()
    );
  }
  for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
    match page_counts.get(book) {
      Some(count) => println!("{book} has {count} pages"),
      None        => println!("book {book} is unknown"),
    }
  }
  for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
    let page_count: &mut i32 = page_counts.entry(book).or_insert(0);
    *page_count += 1;
  }
  println!("pgs: {page_counts:?}");

  // 16.8
  println!("");
  let mut ctr = Counter::new();
  ctr.count(13);
  ctr.count(14);
  ctr.count(16);
  ctr.count(14);
  ctr.count(14);
  ctr.count(11);

  for i in 10..20 {
      println!("saw {} values equal to {}", ctr.times_seen(i), i);
  }

  let mut strctr = Counter::new();
  strctr.count("apple");
  strctr.count("orange");
  strctr.count("apple");
  println!("got {} apples", strctr.times_seen("apple"));

  // 17.1
  println!("");
  let k1 = Key { id: 1, metadata: Some("hello".to_string()) };
  let k2 = Key { id: 1, metadata: Some("hi".to_string()) };
  println!("k1 == k2: {}", k1 == k2);
  let c1 = Citation { author: "Mario1".to_string(), year: 2022 };
  let c2 = Citation { author: "Mario".to_string(), year: 2021 };
  println!("c1 >= c2: {}", c1 >= c2);

  // 17.2
  println!("");
  let p1 = Point { x: 10, y: 20 };
  let p2 = Point { x: 100, y: 200 };
  println!("{p1:?} + {p2:?} = {:?}", p1 + p2);

  // 17.3
  println!("");
  // let s = String::from("hello");
  // let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
  // let one = i16::from(true);
  // let bigger = i32::from(123_i16);
  // println!("{s}, {addr}, {one}, {bigger}");
  let s: String = "hello".into();
  let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
  let one: i16 = true.into();
  let bigger: i32 = 123_i16.into();
  println!("{s}, {addr}, {one}, {bigger}");

  // 17.4
  println!("");
  let value: i64 = 1000;
  println!("as u16: {}", value as u16);
  println!("as i16: {}", value as i16);
  println!("as u8: {}", value as u8);

  // 17.5
  println!("");
  let slice: &[u8] = b"foo\ngreat\nthing\n";
  println!("{} lines in slice", count_lines(slice));
  let file = std::fs::File::open(std::env::current_exe()?)?;
  println!("{} lines in file", count_lines(file));
  // Ok(())
  let mut buffer = Vec::new();
  log(&mut buffer, "hello")?;
  log(&mut buffer, "world")?;
  println!("logged: {buffer:?}");
  // Ok(())

  // 17.6
  println!("");
  let default_struct = Derived::default();
  println!("{default_struct:#?}");
  let almost_default =
    Derived { y: "y is set :0".into(), ..Derived::default() };
  println!("{almost_default:#?}");
  let nothing: Option<Derived> = None;
  println!("{:#?}", nothing.unwrap_or_default());

  // 17.7
  println!("");
  let mut rot = RotDecoder { input: "Gb trg gb gur bgure fvqr!".as_bytes(), rot: 13 };
  let mut result = String::new();
  rot.read_to_string(&mut result).unwrap();
  println!("{}", result);

  // 18.1
  println!("");
  let value = Some(13);
  dbg!(value.map(|num| format!("{num}")));
  let mut nums = vec![1, 10, 99, 24];
  nums.sort_by_key(|v| if v % 2 == 0 { (0, *v) } else { (1, *v) });
  dbg!(nums);

  // 18.2
  println!("");
  let mut max_value = 5;
  let clamp = move |v| {
    // max_value += 1;
    if v > max_value {
      max_value
    } else {
      v
    }
  };
  max_value = 4;
  println!("clamped values at {max_value}: {:?}", (0..10).map(clamp).collect::<Vec<_>>());

  // 18.3
  println!("");
  let suffix = "-itis";
  let add_suffix = |z| format!("{z}{suffix}");
  apply_and_log(&add_suffix, "add_suffix", "gastro");
  let mut v = Vec::new();
  let mut accumulate = |x| {
    v.push(x);
    v.join("/")
  };
  apply_and_log(&mut accumulate, "accumulate", "red");
  apply_and_log(&mut accumulate, "accumulate", "blue");
  apply_and_log(&mut accumulate, "accumulate", "green");
  let take_and_reverse = |mut prefix: String| {
    prefix.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
    prefix
  };
  apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");
  println!("");

  // 18.4
  let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
  logger.log(5, "FYI");
  logger.log(1, "yikes, something went wrong");
  logger.log(2, "uhoh");

  Ok(())
}