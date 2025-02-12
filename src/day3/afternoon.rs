// 23.1
#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
  Point(p1.0 + p2.0, p1.1 + p2.1)
}

pub fn run() {
  // 23.1
  let p1 = Point(2,3);
  let p2 = Point(4,6);
  let p3 = add(&p1, &p2);
  println!("\n{p1:?} + {p2:?} = {p3:?}");

  // 23.2

  // let x_ref = {
  //   let x = 10;
  //   &x
  // }
  // println!("{x_ref}");
  
  // let mut a: i32 = 10;
  // let b: &i32 = &a;
  // {
  //   let c: &mut i32 = &mut a;
  //   *c = 20;
  // }
  // println!("a: {a}");
  // println!("b: {b}");

  let mut a: i32 = 10;
  let b = &mut a;
  {
    // Instead of borrowing `a` directly, reborrow through `b`, effectively giving us two mutable references.
    let c = &mut *b;
    *c = 20;
  }
  println!("b: {b}");
  println!("a: {a}");

  // 23.3

  // let mut vec = vec![1, 2, 3, 4, 5, 6];
  // let elem = &vec[2];
  // vec.push(6);
  // println!("{elem}");

  // let mut vec = vec![1, 2, 3, 4, 5, 6];
  // for elem in &vec {
  //   vec.push(elem * 2);
  // }

}