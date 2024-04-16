use primes::{Sieve, PrimeSet};

pub fn main(which_conjecture: u64) {
  let limit: u64 = 30;

  match which_conjecture {
    1 => {
      println!("Testing the Strong Goldbach Conjecture...");
      for n in (4..=limit).step_by(2) {
        if !strong_goldbach(n) {
          println!("Failed at {}", n);
          continue;
        }
      }
    }
    2 => {
      println!("Testing the Weak Goldbach Conjecture...");
      for n in (9..=limit).step_by(2) {
        if !weak_goldbach(n) {
          println!("Failed at {}", n);
          continue;
        }
      }
    }
    3 => {
      println!("Testing the Extended Goldbach Conjecture...");
      for n in 8..=limit {
        if !extended_goldbach(n) {
          println!("Failed at {}", n);
          continue;
        }
      }
    }
    _ => println!("Invalid input!"),
  }
}

fn strong_goldbach(n: u64) -> bool {
  let mut primes = Sieve::new();
  let vec = primes.iter().take((n).try_into().unwrap()).collect::<Vec<_>>();

  let mut result: Vec<(u64, u64)> = vec![];

  for i in 0..vec.len() {
    let p = vec[i];
    if p > n / 2 {
      break;
    }
    if p <= n && primes.is_prime(n - p) {
      result.push((p, n - p));
      continue;
    }
  }
  println!("{}: {:?}", n, result);
  result.len() > 0
}

fn weak_goldbach(n: u64) -> bool {
  let mut primes = Sieve::new();
  let vec = primes.iter().take((n).try_into().unwrap()).collect::<Vec<_>>();

  let mut result: Vec<(u64, u64, u64)> = vec![];

  for i in 0..vec.len() {
    let p = vec[i];
    if p > n / 2 {
      break;
    }
    for j in i..vec.len() {
      let q = vec[j];
      if p + q <= n && primes.is_prime(n - p - q) {
        result.push((p, q, n - p - q));
        break;
      }
    }
  }
  println!("{}: {:?}", n, result);
  result.len() > 0
}

#[derive(Debug)]
#[allow(dead_code)]
enum GoldbachTuple {
  Triple(u64, u64, u64),
  Quadruple(u64, u64, u64, u64),
}

fn extended_goldbach(n: u64) -> bool {
  let mut primes = Sieve::new();
  let vec = primes.iter().take((n).try_into().unwrap()).collect::<Vec<_>>();

  let mut result: Vec<GoldbachTuple> = vec![];

  for i in 0..vec.len() {
    let p = vec[i];
    if p > n / 2 {
      break;
    }
    for j in i..vec.len() {
      let q = vec[j];
      if p + q > n || q > n - p - q {
        break;
      } else if primes.is_prime(n - p - q) {
        result.push(GoldbachTuple::Triple(p, q, n - p - q));
      }
      for k in j..vec.len() {
        let r = vec[k];
        if p + q + r > n || n - p - q - r < r {
          break;
        } else if primes.is_prime(n - p - q - r) {
          result.push(GoldbachTuple::Quadruple(p, q, r, n - p - q - r));
          break;
        }
      }
    }
  }
  println!("{}: {:?}", n, result);
  result.len() > 0
}
