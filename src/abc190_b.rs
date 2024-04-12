// https://atcoder.jp/contests/abc190/tasks/abc190_b
// B - Magic 3

use proconio::input;

pub fn main() {
  input! {
    n: u8,
    s: u32,
    d: u32,
    spells: [[u32; 2]; n],
  }

  for spell in spells.iter() {
    if spell[0] < s && spell[1] > d {
      println!("Yes");
      return;
    }
  }
  println!("No");
}