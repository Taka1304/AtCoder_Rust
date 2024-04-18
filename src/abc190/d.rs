// https://atcoder.jp/contests/abc190/tasks/abc190_d
// D - Staircase Sequences

use proconio::input;

pub fn main() {
  input! {
    n: usize,
  }
  let mut count = 0;
  let mut i = 1;
  // 約数全探索
  while i * i <= n * 2 {
    // 約数か
    if n * 2 % i == 0 {
      let j = n * 2 / i;
      // 偶奇が異なるか
      if i % 2 == 1 || (j != i && j % 2 == 1) {
        count+=1
      }
    }
    i+=1
  }
  println!("{}", count * 2);
}