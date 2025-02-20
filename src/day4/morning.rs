// 26.2
struct SliceIterator<'s, T> {
  slice: &'s [T],
  i: usize,
}

impl<'s, T> Iterator for SliceIterator<'s, T> {
  type Item = &'s T;
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

  // 26.3
  println!();
  let result: i32 = (1..=10)
    .filter(|x| x%2 == 0)
    .map(|x| x*x)
    .sum();
  println!("sum of squared even nums up to 10: {result}");

  // 26.4
  println!();
  let primes = vec![2, 3, 5, 7];
  // let prime_squares = primes.into_iter().map(|elem| elem * elem).collect::<Vec<_>>();
  let prime_squares: Vec<_> = primes.into_iter().map(|elem| elem * elem).collect();
  println!("prime squares: {prime_squares:?}");
}