// 5.4
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
  return a*b + b*c + c*a;
}

// 5.5
fn takes_u32(x: u32) {
  println!("u32: {x}");
}

fn takes_i8(y: i8) {
  println!("i8: {y}");
}

// 5.6
fn fib(n: u32) -> u32 {
  if n < 2 {
    return n;
  } else {
    return fib(n-1) + fib(n-2);
  }
}

// 6.8
fn collatz_length(n: i32) -> u32 {
  let next;

  if n == 1 {
    return 1;
  } else {
    if n%2 != 0 {
      next = collatz_length(3*n + 1);
    } else {
      next = collatz_length(n/2);
    }
  }
  return next + 1;
}

pub fn run() {
  // 5.1
  println!("hello man, this is a 🌍️ world!\n");

  // 5.2
  let mut x: i32 = 1_000_000;
  println!("x is {x}");
  x = 20;
  println!("x now is {x}\n");

  // 5.4
  println!("inter: {}\n", interproduct(120, 100, 248));

  // 5.5
  let x1 = 10;
  let x2 = 30;
  takes_u32(x1);
  takes_i8(x2);
  println!();

  // 5.6
  let n = 20;
  println!("fib({n}) = {}\n", fib(n));

  // 6.1
  let myelem = 10;
  let x3 = {
    let y = 13;
    println!("myelem: {myelem}");
    y - myelem
  };
  println!("x3: {x3}");

  let a = 10;
  println!("\na: {a}");
  {
    let a = "hello";
    println!("inner scope a: {a}");
    let a = true;
    println!("shadowed a: {a}");
  }
  println!("final a: {a}");
  
  // 6.3
  let mut val = 1;
  match val {
    1 => println!("this is a one"),
    10 => println!("ten"),
    _ => {
      println!("fall through");
    }
  }

  // 6.4.1
  val = 200;
  while val >= 10 {
    val /= 2;
  }
  println!("final val: {val}");

  // 6.4.2
  for i in 1..5 {
    print!("{i} ");
  }
  println!();
  for elem in [1, 2, 3, 4, 5] {
    print!("{elem} ");
  }
  println!();

  // 6.4.3
  let mut i = 0;
  loop {
    i+=1;
    if i >= 100 {break};
  }
  println!("final loop i: {i}");

  // 6.5.1
  let s = [[1, 2, 3], [4, 6, 10], [43, 12, 4]];
  let mut elems_searched = 0;
  let myelem = 10;
  'outer: for i in 0..=2 {
    for j in 0..=2 {
      elems_searched += 1;
      if s[i][j] == myelem { break 'outer }
    }
  }
  println!("elems searched: {elems_searched}");

  // 6.8
  let n1 = 11;
  println!("\ncollatz for {n1}: {}", collatz_length(n1));
}