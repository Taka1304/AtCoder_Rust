// https://atcoder.jp/contests/abc190/tasks/abc190_d
// D - Staircase Sequences

use proconio::input;

pub fn main() {
  input! {
    n: usize,
  }

  let mut count = 0;
  let a = n << 1; // 2N
  
  let mut i = 1;
  // 約数全探索
  while i * i <= a {
    // 約数か
    if a % i == 0 {
      let j = a / i;
      // 偶奇が異なるか
      if (a ^ i) & 1 == 1 {
        count+=1
      }
      // 重複確認
      if j != i && (a ^ j) & 1 == 1 {
        count += 1
      }
    }
    i += 1
  }
  println!("{}", count << 1);
}