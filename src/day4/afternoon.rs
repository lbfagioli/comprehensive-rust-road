// 30.1
use std::panic;

// 30.2
use std::fs::File;
use std::io::Read;

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
}