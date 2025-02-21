// 30.1
use std::panic;

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
}