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

// 26.5
struct Grid {
  x_coords: Vec<i32>,
  y_coords: Vec<i32>,
}

impl IntoIterator for Grid {
  type Item = (i32, i32);
  type IntoIter = GridIter;
  fn into_iter(self) -> GridIter {
    GridIter { grid: self, i: 0, j: 0 }
  }
}

struct GridIter {
  grid: Grid,
  i: usize,
  j: usize,
}

impl Iterator for GridIter {
  type Item = (i32, i32);

  fn next(&mut self) -> Option<(i32, i32)> {
    if self.i == self.grid.x_coords.len() {
      self.i = 0;
      self.j += 1;
      if self.j == self.grid.y_coords.len() {
        return None;
      }
    }

    let next = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
    self.i += 1;

    next

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

  // 26.5
  let grid = Grid { x_coords: vec![1, 3, 5, 7], y_coords: vec![2, 4, 6, 8] };
  for elem in grid {
    println!("coords = {elem:?}");
  }
}