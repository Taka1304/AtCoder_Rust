// https://atcoder.jp/contests/abc349/tasks/abc349_c
// C - Airport Code

use proconio::input;

pub fn main() {
  input! {
    s: String,
    t: String
  }
  let mut ans = "No";
  let mut l = 0;
  let is_x = t.chars().nth(2).unwrap() == 'X';
  for c in s.chars() {
    if t.chars().nth(l).unwrap().eq_ignore_ascii_case(&c) {
      l += 1;
    }
    if l == 3 || (l == 2 && is_x) {
      ans = "Yes";
      break;
    }
  }
  println!("{}", ans)
}
