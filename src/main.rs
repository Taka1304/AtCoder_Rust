mod goldbach_conjecture;
use std::time::Instant;
use proconio::input;

fn main() {
  println!("1: Strong Goldbach Conjecture");
  println!("2: Weak Goldbach Conjecture");
  println!("3: Extended Goldbach Conjecture");

  input! {
    which_conjecture: u64,
  }

  let start = Instant::now();
  goldbach_conjecture::main(which_conjecture);
  let duration = start.elapsed();
  println!("Time: {:?}", duration);
}
