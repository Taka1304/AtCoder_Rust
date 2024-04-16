mod goldbach_conjecture;

use proconio::input;
use std::time::Instant;

fn main() {
  println!("1: Strong Goldbach Conjecture");
  println!("2: Weak Goldbach Conjecture");
  println!("3: Extended Goldbach Conjecture");

  input! {
    which_conjecture: u64,
  }
  // 実行時間を計測
  let start = Instant::now();

  let limit: u64 = 30;
  match which_conjecture {
    1 => {
      println!("Testing the Strong Goldbach Conjecture...");
      for n in (4..=limit).step_by(2) {
        if !goldbach_conjecture::strong_goldbach(n) {
          println!("Failed at {}", n);
          continue;
        }
      }
    }
    2 => {
      println!("Testing the Weak Goldbach Conjecture...");
      for n in (9..=limit).step_by(2) {
        if !goldbach_conjecture::weak_goldbach(n) {
          println!("Failed at {}", n);
          continue;
        }
      }
    }
    3 => {
      println!("Testing the Extended Goldbach Conjecture...");
      for n in 8..=limit {
        if !goldbach_conjecture::extended_goldbach(n) {
          println!("Failed at {}", n);
          continue;
        }
      }
    }
    _ => println!("Invalid input!"),
  }

  let duration = start.elapsed();
  println!("Time: {:?}", duration);
}
