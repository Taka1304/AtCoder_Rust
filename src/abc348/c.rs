// https://atcoder.jp/contests/abc348/tasks/abc348_c
// C - Colorful Beans

use proconio::input;
use std::collections::HashMap;

pub fn main() {
  input! {
    n: usize, ac: [(usize, usize); n]
  }

  let mut map = HashMap::new();
  for (a, c) in ac {
    map
      .entry(c)
      .and_modify(|e| {
        if *e > a {
          *e = a
        }
      })
      .or_insert(a);
  }

  println!("{}", map.values().max().unwrap());
}
