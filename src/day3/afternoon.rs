// 23.1
#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
  Point(p1.0 + p2.0, p1.1 + p2.1)
}

// 23.4.1
use std::cell::Cell;

// 23.4.2
use std::cell::RefCell;

// 23.5
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: u32,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
  pub fn new(name: String, age: u32, height: f32) -> Self {
    Self { name, age, height, visit_count: 0, last_blood_pressure: None }
  }

  pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
    self.visit_count += 1;
    let prev_height = self.height;
    self.height = measurements.height;
    let mut bp_change = None;
    let (b1, b2) = measurements.blood_pressure;

    if let Some(bp) = self.last_blood_pressure {
      bp_change = Some((b1 as i32 - bp.0 as i32, b2 as i32 - bp.1 as i32));
    }

    self.last_blood_pressure = Some((b1, b2));

    HealthReport { 
      patient_name: &self.name,
      visit_count: self.visit_count,
      height_change: measurements.height - prev_height,
      blood_pressure_change: bp_change,
    }
  }
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.visit_count, 0);
    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });
    assert_eq!(report.patient_name, "Bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);
    assert!((report.height_change - 0.9).abs() < 0.00001);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    assert_eq!(report.height_change, 0.0);
}

// 24.1
fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
  return if p1.0 < p2.0 { p1 } else { p2 };
}

// 24.2
fn cab_distance(p1: &Point, p2: &Point) -> i32 {
  (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn find_nearest<'a>(points: &'a [Point], query: &Point) -> Option<&'a Point> {
  // find_nearest<'a, 'q>(points: &'a [Point], query: &'q Point) -> Option<&'q Point> wont compile for lying on return lifetime
  let mut nearest = None;
  for p in points {
    if let Some((_, nearest_dist)) = nearest {
      let dist = cab_distance(p, query);
      if dist < nearest_dist {
        nearest = Some((p, dist));
      }
    } else {
      nearest = Some((p, cab_distance(p, query)));
    };
  }
  nearest.map(|(p, _)| p)
}

// 24.3
#[derive(Debug)]
enum HighlightColor {
  Pink,
  Yellow,
}

#[derive(Debug)]
struct Highlight<'document> {
  slice: &'document str,
  color: HighlightColor,
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

  // 23.4.1
  let cell = Cell::new(5);
  cell.set(123);
  println!("cell: {}", cell.get());

  // 23.4.2
  let re_cell = RefCell::new(6);
  {
    let mut cell_ref = re_cell.borrow_mut();
    *cell_ref = 130;
    // runtime error for double borrow
    // let other = re_cell.borrow();
    // println!("o: {}", *other);
  }
  println!("refcell: {re_cell:?}");
  println!();

  // 23.5
  let bob = User::new(String::from("Bob"), 32, 155.2);
  println!("I'm {} and my age is {}", bob.name, bob.age);

  // 24.1
  println!();
  let pp1 = Point(23,2);
  let pp2 = Point(4, 2);
  let pp3 = left_most(&pp1, &pp2);
  println!("pp3: {pp3:?}");

  // 24.2
  let points = &[Point(1, 0), Point(1, 0), Point(-1, 0), Point(0, -1)];
  let nearest = {
    let _query = Point(0, 2);
    find_nearest(points, &Point(0, 2))
  };
  println!("nearest: {:?}", nearest);

  // 24.3
  let doc: String = "The quick brown fox jumps over the lazy dog.".into();
  let noun = Highlight { slice: &doc[16..19], color: HighlightColor::Yellow };
  let verb = Highlight { slice: &doc[20..25], color: HighlightColor::Pink };
  // drop(doc);
  println!("{noun:?}");
  println!("{verb:?}");
}