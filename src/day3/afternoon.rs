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

// 24.4
// message PhoneNumber {
//   optional string number = 1;
//   optional string type = 2;
// }

// message Person {
//   optional string name = 1;
//   optional int32 id = 2;
//   repeated PhoneNumber phones = 3;
// }

/// A wire type as seen on the wire.
enum WireType {
  /// The Varint WireType indicates the value is a single VARINT.
  Varint,
  // The I64 WireType indicates that the value is precisely 8 bytes in
  // little-endian order containing a 64-bit signed integer or double type.
  //I64,  -- not needed for this exercise
  /// The Len WireType indicates that the value is a length represented as a
  /// VARINT followed by exactly that number of bytes.
  Len,
  // The I32 WireType indicates that the value is precisely 4 bytes in
  // little-endian order containing a 32-bit signed integer or float type.
  //I32,  -- not needed for this exercise
}

#[derive(Debug)]
/// A field's value, typed based on the wire type.
enum FieldValue<'a> {
  Varint(u64),
  //I64(i64),  -- not needed for this exercise
  Len(&'a [u8]),
  //I32(i32),  -- not needed for this exercise
}

#[derive(Debug)]
/// A field, containing the field number and its value.
struct Field<'a> {
  field_num: u64,
  value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
  fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
  fn from(value: u64) -> Self {
      match value {
          0 => WireType::Varint,
          //1 => WireType::I64,  -- not needed for this exercise
          2 => WireType::Len,
          //5 => WireType::I32,  -- not needed for this exercise
          _ => panic!("Invalid wire type: {value}"),
      }
  }
}

impl<'a> FieldValue<'a> {
  fn as_str(&self) -> &'a str {
      let FieldValue::Len(data) = self else {
          panic!("Expected string to be a `Len` field");
      };
      std::str::from_utf8(data).expect("Invalid string")
  }

  fn as_bytes(&self) -> &'a [u8] {
      let FieldValue::Len(data) = self else {
          panic!("Expected bytes to be a `Len` field");
      };
      data
  }

  fn as_u64(&self) -> u64 {
      let FieldValue::Varint(value) = self else {
          panic!("Expected `u64` to be a `Varint` field");
      };
      *value
  }
}

/// Parse a VARINT, returning the parsed value and the remaining bytes.
fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
  for i in 0..7 {
      let Some(b) = data.get(i) else {
          panic!("Not enough bytes for varint");
      };
      if b & 0x80 == 0 {
          // This is the last byte of the VARINT, so convert it to
          // a u64 and return it.
          let mut value = 0u64;
          for b in data[..=i].iter().rev() {
              value = (value << 7) | (b & 0x7f) as u64;
          }
          return (value, &data[i + 1..]);
      }
  }

  // More than 7 bytes is invalid.
  panic!("Too many bytes for varint");
}

/// Convert a tag into a field number and a WireType.
fn unpack_tag(tag: u64) -> (u64, WireType) {
  let field_num = tag >> 3;
  let wire_type = WireType::from(tag & 0x7);
  (field_num, wire_type)
}


/// Parse a field, returning the remaining bytes
fn parse_field(data: &[u8]) -> (Field, &[u8]) {
  let (tag, remainder) = parse_varint(data);
  let (field_num, wire_type) = unpack_tag(tag);
  let (fieldvalue, remainder) = match wire_type {
    WireType::Varint => {
      let (value, remainder) = parse_varint(remainder);
      (FieldValue::Varint(value), remainder)
    }
    WireType::Len => {
      let (len, remainder) = parse_varint(remainder);
      let len: usize = len.try_into().expect("len is not a valid usize.");
      if remainder.len() < len {
        panic!("got EOF");
      }
      let (value, remainder) = remainder.split_at(len);
      (FieldValue::Len(value), remainder)
    }
  };
  (Field { field_num: field_num, value: fieldvalue }, remainder)
}

/// Parse a message in the given data, calling `T::add_field` for each field in
/// the message.
///
/// The entire input is consumed.
fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> T {
  let mut result = T::default();
  while !data.is_empty() {
      let parsed = parse_field(data);
      result.add_field(parsed.0);
      data = parsed.1;
  }
  result
}

#[derive(PartialEq)]
#[derive(Debug, Default)]
struct PhoneNumber<'a> {
  number: &'a str,
  type_: &'a str,
}

#[derive(PartialEq)]
#[derive(Debug, Default)]
struct Person<'a> {
  name: &'a str,
  id: u64,
  phone: Vec<PhoneNumber<'a>>,
}

// TODO: Implement ProtoMessage for Person and PhoneNumber.
impl<'a> ProtoMessage<'a> for Person<'a> {
  fn add_field(&mut self, field: Field<'a>) {
    match field.field_num {
      1 => self.name = field.value.as_str(),
      2 => self.id = field.value.as_u64(),
      3 => self.phone.push(parse_message(field.value.as_bytes())),
      _ => {},
    }
  }
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
  fn add_field(&mut self, field: Field<'a>) {
    match field.field_num {
      1 => self.number = field.value.as_str(),
      2 => self.type_ = field.value.as_str(),
      _ => {},
    }
  }
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
  println!();

  // 24.4
  let person_id: Person = parse_message(&[0x10, 0x2a]);
  assert_eq!(person_id, Person { name: "", id: 42, phone: vec![] });

  let person_name: Person = parse_message(&[
      0x0a, 0x0e, 0x62, 0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x20,
      0x6e, 0x61, 0x6d, 0x65,
  ]);
  assert_eq!(person_name, Person { name: "beautiful name", id: 0, phone: vec![] });

  let person_name_id: Person =
      parse_message(&[0x0a, 0x04, 0x45, 0x76, 0x61, 0x6e, 0x10, 0x16]);
  assert_eq!(person_name_id, Person { name: "Evan", id: 22, phone: vec![] });

  let phone: Person = parse_message(&[
      0x0a, 0x00, 0x10, 0x00, 0x1a, 0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x33,
      0x34, 0x2d, 0x37, 0x37, 0x37, 0x2d, 0x39, 0x30, 0x39, 0x30, 0x12, 0x04,
      0x68, 0x6f, 0x6d, 0x65,
  ]);
  assert_eq!(
      phone,
      Person {
          name: "",
          id: 0,
          phone: vec![PhoneNumber { number: "+1234-777-9090", type_: "home" },],
      }
  );

  // Put that all together into a single parse.
  let person: Person = parse_message(&[
      0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a,
      0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35,
      0x2d, 0x31, 0x32, 0x31, 0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a,
      0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d, 0x38, 0x36, 0x37,
      0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
      0x65,
  ]);
  assert_eq!(
      person,
      Person {
          name: "maxwell",
          id: 42,
          phone: vec![
              PhoneNumber { number: "+1202-555-1212", type_: "home" },
              PhoneNumber { number: "+1800-867-5308", type_: "mobile" },
          ]
      }
  );
}