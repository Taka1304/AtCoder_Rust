// https://atcoder.jp/contests/abc349/tasks/abc349_b
// B - Commencement

use proconio::input;

pub fn main() {
  input! {
    s: String
  }

  let mut ans = "Yes";
  let mut map = std::collections::HashMap::new();

  for c in s.chars() {
    *map.entry(c).or_insert(0) += 1;
  }

  for i in 1..=s.len() {
    let count = map
      .values()
      .filter(|&&v| v == i.try_into().unwrap())
      .count();
    if count != 0 && count != 2 {
      ans = "No";
      break;
    }
  }

  println!("{}", ans)
}
