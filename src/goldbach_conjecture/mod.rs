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

// use proconio::input;
// use std::time::Instant;

// fn main() {
//   println!("1: Strong Goldbach Conjecture");
//   println!("2: Weak Goldbach Conjecture");
//   println!("3: Extended Goldbach Conjecture");

//   input! {
//     which_conjecture: u64,
//   }
//   // 実行時間を計測
//   let start = Instant::now();

//   let limit: u64 = 50;
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
//     _ => println!("Invalid input!"),
//   }

//   let duration = start.elapsed();
//   println!("Time: {:?}", duration);
// }
