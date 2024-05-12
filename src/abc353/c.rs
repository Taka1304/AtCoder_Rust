// https://atcoder.jp/contests/abc353/tasks/abc353_c
// C - Sigma Problem

// memo
// 各項が N−1 回加算されることから、これは (N−1) * a.sum()で求められる。 -> O(n)
// あとは 足して超える組み合わせの個数を求め、100_000_000 と掛けて引けばよい。

use proconio::input;

pub fn main() {
  input! {
    n: usize,
    mut a: [usize; n]
  }

  a.sort_unstable();

  let mut count = 0;
  let mut r = n - 1;
  let mut l = 0;
  while r > l {
    if a[l] + a[r] >= 100_000_000 {
      count += r - l;
      r -= 1
    } else {
      l += 1
    }
  }
  let ans = a.into_iter().sum::<usize>() * (n - 1) - 100_000_000 * count;
  println!("{}", ans)
}
