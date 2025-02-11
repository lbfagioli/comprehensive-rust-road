// 8.4
fn check_order(tuple: (i32, i32, i32)) -> bool {
  let (left, middle, right) = tuple;
  left < middle && middle < right
}

// 8.5
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
  let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
  for i in 0..=2 {
    for j in 0..=2 {
      new_matrix[i][j] = matrix[j][i];
    }
  }
  new_matrix
}

// 9.6
fn magnitude(vector: &[f64; 3]) -> f64 {
  let mut mag: f64 = 0.0;
  for i in vector {
    mag += i*i;
  }
  mag.sqrt()
}

fn normalize(vector: &mut [f64; 3]) {
  let mag = magnitude(vector);
  for i in vector {
    *i = *i/mag
  }
}

// 10.1
struct Person {
  name: String,
  age: u8,
}

fn describe(person: &Person) {
  println!("{} is {} years old", person.name, person.age);
}

// 10.2
struct Point(i32, i32);

// 10.3
// #[derive(Debug)]
// enum Direction {
//   Left,
//   Right,
// }

// #[derive(Debug)]
// enum PlayerMove {
//   Pass, // simple
//   Run(Direction), // tuple
//   Teleport { x: u32, y: u32 }, // struct [[variants]]
// }

// #[repr(u32)]
// enum Bar {
//   A,
//   B = 10000,
//   C,
// }

// 10.5
const DIGEST_VALUE: usize = 3;

// 10.6
static BANNER: &str = "welcome to this &str";

// 10.7
#[derive(Debug)]
struct Floor(#[allow(dead_code)] i32);

#[derive(Debug)]
enum Event {
  Lobby(#[allow(dead_code)] Floor, #[allow(dead_code)] Direction),
  CarArrived(#[allow(dead_code)] Floor),
  CarDoorOpened,
  FloorButtonPressed(#[allow(dead_code)] Floor),
  CarDoorClosed,
}

#[derive(Debug)]
enum Direction {
  Up,
  #[allow(dead_code)] Down,
}

fn car_arrived(floor: i32) -> Event {
  Event::CarArrived(Floor(floor))
}

fn car_door_opened() -> Event {
  Event::CarDoorOpened
}

fn car_door_closed() -> Event {
  Event::CarDoorClosed
}

fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
  Event::Lobby(Floor(floor), dir)
}

fn car_floor_button_pressed(floor: i32) -> Event {
  Event::FloorButtonPressed(Floor(floor))
}

pub fn run() {
  // 8.1
  let mut arr: [i8; 10] = [42; 10];
  arr[5] = 0;
  println!("\nout arr: {arr:?}");

  // 8.2
  let t: (i8, bool) = (7, true);
  println!("\nt.0: {}", t.0);
  println!("t.1: {}", t.1);

  // 8.4
  let mytup = (1, 5, 3);
  println!("{mytup:?} is {}", if check_order(mytup) { "ordered" } else { "unordered" });

  // 8.5
  let matrix = [
      [101, 102, 103], // <-- the comment makes rustfmt add a newline
      [201, 202, 203],
      [301, 302, 303],
  ];
  println!("\nmatrix: {:?}", matrix);
  let transposed = transpose(matrix);
  println!("transposed: {:?}", transposed);

  // 9.1
  let a: char = 'A';
  let b: char = 'B';
  let mut r: &char = &a;
  println!("\n*r: {}", *r);
  r = &b;
  println!("*r: {}", *r);

  // 9.2
  let mut point = (1, 2);
  let x_coord = &mut point.0;
  *x_coord = 20;
  println!("\npoint: {point:?}");

  // 9.3
  let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
  println!("\na: {a:?}");
  let s: &[i32] = &a[2..4];
  println!("s: {s:?}");

  // 9.4
  let s1: &str = "world";
  println!("s1: {s1}");
  let mut s2: String = String::from("Hello ");
  println!("s2: {s2}");
  s2.push_str(s1);
  println!("s2: {s2}");
  let s3: &str = &s2[s2.len() - s1.len()..];
  println!("s3: {s3}");
  println!("{:?}", b"abc");
  println!("{:?}", &[97, 98, 99]);

  // 9.5
  // let x_ref = {
  //   let x = 10;
  //   &x
  // };
  // println!("x: {x_ref}");

  // 9.6
  println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

  let mut v = [1.0, 2.0, 9.0];
  println!("Magnitude of {v:?}: {}", magnitude(&v));
  normalize(&mut v);
  println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));

  // 10.1
  let mut peter = Person { name: String::from("Peter"), age: 26 };
  describe(&peter);
  peter.age = 28;
  describe(&peter);
  let name = String::from("Avery");
  let age = 35;
  let avery = Person { name, age };
  describe(&avery);
  let jackie = Person { name: String::from("Jackie"), ..avery };
  describe(&jackie);

  // 10.2
  let p = Point(17, 23);
  println!("({}, {})", p.0, p.1);

  // 10.3
  // let playermove: PlayerMove = PlayerMove::Run(Direction::Left);
  // println!("on this move: {playermove:?}");
  // // 10.4
  // type Buri = Bar;
  // println!("A: {}", Bar::A as u32);
  // println!("B: {}", Bar::B as u32);
  // println!("C: {}", Bar::C as u32);
  // println!("C: {}", Buri::C as u32);

  //10.5
  println!("dige {}", DIGEST_VALUE);

  // 10.6
  println!("{BANNER}");

  // 10.7
  println!(
      "A ground floor passenger has pressed the up button: {:?}",
      lobby_call_button_pressed(0, Direction::Up)
  );
  println!("The car has arrived on the ground floor: {:?}", car_arrived(0));
  println!("The car door opened: {:?}", car_door_opened());
  println!(
      "A passenger has pressed the 3rd floor button: {:?}",
      car_floor_button_pressed(3)
  );
  println!("The car door closed: {:?}", car_door_closed());
  println!("The car has arrived on the 3rd floor: {:?}", car_arrived(3));
  println!();
}