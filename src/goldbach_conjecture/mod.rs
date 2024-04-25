// 課題でやったゴールドバッハ予想問題

use itertools::Itertools;
use primes::{PrimeSet, Sieve};

pub fn strong_goldbach(n: u64) -> bool {
  let mut primes = Sieve::new();
  let vec = primes.iter().take_while(|&p| p <= n).collect::<Vec<_>>();

  let result = vec
    .iter()
    .combinations_with_replacement(2)
    .filter(|comb| comb.iter().map(|&&p| p).sum::<u64>() == n)
    .collect::<Vec<_>>();

  println!("{}: {:?}", n, result);
  !result.is_empty()
}

pub fn weak_goldbach(n: u64) -> bool {
  let mut primes = Sieve::new();
  let vec = primes.iter().take_while(|&p| p <= n).collect::<Vec<_>>();

  let result = vec
    .iter()
    .combinations_with_replacement(3)
    .filter(|comb| comb.iter().map(|&&p| p).sum::<u64>() == n)
    .collect::<Vec<_>>();

  println!("{}: {:?}", n, result);
  !result.is_empty()
}

pub fn extended_goldbach(n: u64) -> bool {
  let mut primes = Sieve::new();
  let vec = primes.iter().take_while(|&p| p <= n).collect::<Vec<_>>();

  let result_triples = vec
    .iter()
    .combinations_with_replacement(3)
    .filter(|comb| comb.iter().map(|&&p| p).sum::<u64>() == n)
    .collect::<Vec<_>>();

  let result_quadruples = vec
    .iter()
    .combinations_with_replacement(4)
    .filter(|comb| comb.iter().map(|&&p| p).sum::<u64>() == n)
    .collect::<Vec<_>>();

  println!("{}: {:?} {:?}", n, result_triples, result_quadruples);
  !result_triples.is_empty() || !result_quadruples.is_empty()
}

// main.rs
//
// use proconio::input;
// use std::time::Instant;

// mod goldbach_conjecture;

// fn main() {
//   println!("1: Strong Goldbach Conjecture, 4..=limit");
//   println!("2: Weak Goldbach Conjecture, 7..=limit");
//   println!("3: Extended Goldbach Conjecture, 7..=limit");
//   println!("4: Strong Goldbach Conjecture, limit");
//   println!("5: Weak Goldbach Conjecture, limit");
//   println!("6: Extended Goldbach Conjecture, limit");

//   input! {
//     which_conjecture: u64,
//     limit: u64
//   }
//   // 実行時間を計測
//   let start = Instant::now();

//   match which_conjecture {
//     1 => {
//       println!("Testing the Strong Goldbach Conjecture...");
//       for n in (4..=limit).step_by(2) {
//         if !goldbach_conjecture::strong_goldbach(n) {
//           println!("Failed at {}", n);
//           continue;
//         }
//       }
//     }
//     2 => {
//       println!("Testing the Weak Goldbach Conjecture...");
//       for n in (7..=limit).step_by(2) {
//         if !goldbach_conjecture::weak_goldbach(n) {
//           println!("Failed at {}", n);
//           continue;
//         }
//       }
//     }
//     3 => {
//       println!("Testing the Extended Goldbach Conjecture...");
//       for n in 7..=limit {
//         if !goldbach_conjecture::extended_goldbach(n) {
//           println!("Failed at {}", n);
//           continue;
//         }
//       }
//     }
//     4 => {
//       println!("Testing the Strong Goldbach Conjecture...");
//       if !goldbach_conjecture::strong_goldbach(limit) {
//         println!("Failed at {}", limit);
//       }
//     }
//     5 => {
//       println!("Testing the Weak Goldbach Conjecture...");
//       if !goldbach_conjecture::weak_goldbach(limit) {
//         println!("Failed at {}", limit);
//       }
//     }
//     6 => {
//       println!("Testing the Extended Goldbach Conjecture...");
//       if !goldbach_conjecture::extended_goldbach(limit) {
//         println!("Failed at {}", limit);
//       }
//     }
//     _ => println!("Invalid input!"),
//   }

//   let duration = start.elapsed();
//   println!("Time: {:?}", duration);
// }