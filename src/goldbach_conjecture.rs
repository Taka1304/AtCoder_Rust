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
