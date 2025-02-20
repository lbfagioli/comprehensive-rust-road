// 26.2
struct SliceIterator<'s> {
  slice: &'s [i32],
  i: usize,
}

impl<'s> Iterator for SliceIterator<'s> {
  type Item = &'s i32;
  fn next(&mut self) -> Option<Self::Item> {
    if self.i == self.slice.len() {
      None
    } else {
      let next = &self.slice[self.i];
      self.i += 1;
      Some(next)
    }
  }
}

pub fn run() {
  println!("so 'day4' starts..");

  // 26.2
  let slice = &[2, 4, 5, 3];
  let iter = SliceIterator { slice: slice, i: 0 };
  for elem in iter {
    println!("{elem}");
  }
}