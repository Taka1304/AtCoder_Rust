// https://atcoder.jp/contests/abc349/tasks/abc349_a
// A - Zero Sum Game

use proconio::input;

pub fn main() {
  input! {
    n: usize,
    arr: [isize; n - 1]
  }
  let s: isize = arr.iter().sum();

  println!("{}", s * -1)
}
