// https://atcoder.jp/contests/abc190/tasks/abc190_b
// B - Magic 3

use proconio::input;

pub fn main() {
  input! {
    n: u8,
    s: u32,
    d: u32,
    spells: [(u32, u32); n],
  }

  for &(x, y) in &spells {
    if x < s && y > d {
      println!("Yes");
      return;
    }
  }
  println!("No");
}