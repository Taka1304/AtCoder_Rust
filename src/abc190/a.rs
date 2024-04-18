// https://atcoder.jp/contests/abc190/tasks/abc190_a
// A - Very Very Primitive Game

use proconio::input;

pub fn main() {
  input! {
    a: u8,
    b: u8,
    c: u8,
  };
  
  if a + c > b {
    println!("Takahashi")
  } else {
    println!("Aoki")
  }
}
