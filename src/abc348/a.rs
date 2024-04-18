// https://atcoder.jp/contests/abc348/tasks/abc348_a
// A - Penalty Kick

use proconio::input;

pub fn main() {
  input! {
    n: usize
  }

  for i in 1..n + 1 {
    print!("{}", if i % 3 == 0 { "x" } else { "o" });
  }
}
