// https://atcoder.jp/contests/abc190/tasks/abc190_c
// C - Bowls and Dishes

use proconio::input;

pub fn main() {
  input! {
    n: usize,
    m: usize,
    ab: [(usize, usize); m],
    k: usize,
    cd: [(usize, usize); k],
  }

  let mut max = 0;
  for i in 0..1 << k {
    let mut dishes = vec![false; n];
    for j in 0..k {
      if i >> j & 1 == 0 {
        dishes[cd[j].0 - 1] = true;
      } else {
        dishes[cd[j].1 - 1] = true;
      }
    }
    let mut count = 0;
    for &(a, b) in &ab {
      if dishes[a - 1] && dishes[b - 1] {
        count += 1;
      }
    }
    max = max.max(count);
  }
  println!("{}", max);
}
