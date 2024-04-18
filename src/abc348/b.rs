// https://atcoder.jp/contests/abc348/tasks/abc348_b
// B - Farthest Point

use proconio::input;

pub fn main() {
  input! {
  n: usize,
  xy: [(isize, isize); n],
  }

  for i in 0..n {
    let (m, _v) = xy
      .iter()
      .enumerate()
      .fold((0, isize::MIN), |(i_a, a), (i_b, &(x, y))| {
        let d = (x - xy[i].0).pow(2) + (y - xy[i].1).pow(2);
        if d > a {
          (i_b, d)
        } else {
          (i_a, a)
        }
      });
    println!("{}", m + 1);
  }
}
