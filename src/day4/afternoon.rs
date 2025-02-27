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

// 31.3
static HELLO_WORLD: &str = "Hello world";

static mut COUNTER: i32 = 0;

fn add_to_counter(c: i32) {
  unsafe {
    COUNTER += c;
  }
}

// 31.4
#[repr(C)] // removing this didn't trigger a compiler error, so i don't know why it's here
union MyUnion { // unions are almost never used, the book says it interacts well with C libs
  i: u8,
  b: bool,
}

// 31.5.1
// the function is unsafe as it requires arguments to be valid pointers
unsafe fn swap(a: *mut u8, b: *mut u8) {
  let temp_var = *a;
  *a = *b;
  *b = temp_var;
}

// 31.5.2
use std::ffi::c_char;

unsafe extern "C" {
  // abs doesn't have any safety requirement
  safe fn abs(input: i32) -> i32;

  // Safety: s must be a pointer to a NUL terminated C string, valid, not modified during this call
  unsafe fn strlen(s: *const c_char) -> usize;
}

// 31.5.3
struct KeyPair {
  pk: [u16; 4], // 2 * 4 bytes = 8 bytes
  sk: [u16; 4],
}

const PK_BYTE_LEN: usize = 8;

// this is and unsound function, as it produces undefined behavior, but is safe
// this fn should be marked as unsafe as pk_ptr should meet certain criteria (point to a 8 elements arr for this case)
fn log_public_key(pk_ptr: *const u16) {
  let pk: &[u16] = unsafe { std::slice::from_raw_parts(pk_ptr, PK_BYTE_LEN) }; // second arg is number of elems, not bytes, so this shows undefined behavior
  println!("pk: {pk:?}");
}

// 31.6
use std::{mem, slice};

// safety: type must have a defined representation and no padding
pub unsafe trait IntoBytes {
  fn as_bytes(&self) -> &[u8] {
    let len = mem::size_of_val(self);
    let slf: *const Self = self;
    unsafe { slice::from_raw_parts(slf.cast::<u8>(), len) }
  }
}

// safety: u32 meets requirements
unsafe impl IntoBytes for u32 {}

// 31.7
mod ffi {
  use std::os::raw::{c_char, c_int};
  #[cfg(not(target_os = "macos"))]
  use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

  // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
  #[repr(C)]
  pub struct DIR {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
  }

  // Layout according to the Linux man page for readdir(3), where ino_t and
  // off_t are resolved according to the definitions in
  // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}.
  #[cfg(not(target_os = "macos"))]
  #[repr(C)]
  pub struct dirent {
    pub d_ino: c_ulong,
    pub d_off: c_long,
    pub d_reclen: c_ushort,
    pub d_type: c_uchar,
    pub d_name: [c_char; 256],
  }

  // Layout according to the macOS man page for dir(5).
  #[cfg(all(target_os = "macos"))]
  #[repr(C)]
  pub struct dirent {
    pub d_fileno: u64,
    pub d_seekoff: u64,
    pub d_reclen: u16,
    pub d_namlen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 1024],
  }

  unsafe extern "C" {
    pub unsafe fn opendir(s: *const c_char) -> *mut DIR;

    #[cfg(not(all(target_os = "macos", target_arch = "x86_64")))]
    pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

    // See https://github.com/rust-lang/libc/issues/414 and the section on
    // _DARWIN_FEATURE_64_BIT_INODE in the macOS man page for stat(2).
    //
    // "Platforms that existed before these updates were available" refers
    // to macOS (as opposed to iOS / wearOS / etc.) on Intel and PowerPC.
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    #[link_name = "readdir$INODE64"]
    pub unsafe fn readdir(s: *mut DIR) -> *const dirent;

    pub unsafe fn closedir(s: *mut DIR) -> c_int;
  }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
  path: CString,
  dir: *mut ffi::DIR,
}

impl DirectoryIterator {
  fn new(path: &str) -> Result<DirectoryIterator, String> {
    // Call opendir and return a Ok value if that worked,
    // otherwise return Err with a message.
    let ptr = unsafe { ffi::opendir(CString::new(path).unwrap().as_ptr()) }; // possible unattended error by unwrap
    if ptr.is_null() {
      return Err("could not open dir".to_string());
    } else {
      Ok(DirectoryIterator { path: CString::new(path).unwrap(), dir: ptr })
    }
  }
}

impl Iterator for DirectoryIterator {
  type Item = OsString;
  fn next(&mut self) -> Option<OsString> {
    // Keep calling readdir until we get a NULL pointer back.
    // readdir and from_ptr are unsafe
    unsafe {
      let dir_ent = ffi::readdir(self.dir);
      if dir_ent.is_null() {
        return None
      }
      Some(OsString::from(OsStr::from_bytes(CStr::from_ptr((*dir_ent).d_name.as_ptr()).to_bytes())))
    }
  }
}

impl Drop for DirectoryIterator {
  fn drop(&mut self) {
    // Call closedir as needed.
    // closedir is an unsafe fn
    unsafe { ffi::closedir(self.dir) };
    println!("dropped dir\n");
  }
}

pub fn run() -> Result<(), String> {
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
  println!();

  // 31.3
  println!("static var: {HELLO_WORLD}"); // safe static immnutable variable
  add_to_counter(42); // this requires unsafe block even though this is a single thread process
  unsafe {
    println!("counter: {COUNTER}");
  }
  println!();

  // 31.4
  let my_union = MyUnion { i: 42 };
  println!("union int: {}", unsafe { my_union.i });
  println!("union bool: {}", unsafe { my_union.b });
  println!();

  // 31.5.1
  let mut n1: u8 = 32;
  let mut n2: u8 = 51;
  // this unsafe function must be called into an unsafe block
  // SAFETY: pointers come from references, satisfying validity required by this function
  unsafe {
    swap(&mut n1, &mut n2);
  }
  println!("n1: {}, n2: {}", n1, n2);
  println!();

  // 31.5.2
  println!("abs of -5 according to C: {}", abs(-5));

  // Safety: argument for strlen is a pointer to a c valid string
  unsafe {
    println!("strlen: {}", strlen(c"String".as_ptr()));
  }
  println!();

  // 31.5.3
  println!();
  let key_pair = KeyPair { pk: [3, 1, 3, 4], sk: [0, 0, 1, 2] };
  log_public_key(key_pair.pk.as_ptr());
  println!();

  // 31.7
  let iter = DirectoryIterator::new(".")?;
  println!("files: {:#?}", iter.collect::<Vec<_>>());
  Ok(())
}