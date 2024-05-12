use proconio::*;

#[fastout]
pub fn main() {
  input! {
    mut l: usize, r: usize
  }
  let mut intervals = Vec::new();

  while l < r {
    let mut p = 1;
    while l + p <= r && l & (p - 1) == 0 {
      p <<= 1;
    }
    p >>= 1;

    if l + p <= r {
      intervals.push((l, l + p));
      l += p;
    } else {
      intervals.push((l, r));
      break;
    }
  }

  println!("{}", intervals.len());
  for (start, end) in intervals {
    println!("{} {}", start, end);
  }
}
