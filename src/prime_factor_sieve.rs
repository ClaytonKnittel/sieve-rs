use either::Either;

pub struct PrimeFactorSieve {
  smallest_prime_factors: Vec<u32>,
}

impl PrimeFactorSieve {
  pub fn new(n: u32) -> Self {
    let n = n as usize;
    let mut v = vec![0; n + 1];
    for i in 2..=n {
      if v[i] != 0 {
        continue;
      }

      for j in (i..=n).step_by(i) {
        if v[j] == 0 {
          v[j] = i as u32;
        }
      }
    }

    Self { smallest_prime_factors: v }
  }

  pub fn is_prime(&self, n: u32) -> bool {
    debug_assert!(n >= 2);
    self.smallest_prime_factors[n as usize] == n
  }

  /// Returns an iterator over all primes.
  pub fn primes(&self) -> impl Iterator<Item = u32> {
    self
      .smallest_prime_factors
      .iter()
      .enumerate()
      .skip(2)
      .filter_map(|(p, &spf)| (spf == p as u32).then_some(p as u32))
  }

  /// Returns an iterator over prime factors (p, multiplicity).
  pub fn prime_factors(&self, n: u32) -> impl Iterator<Item = (u32, u32)> + Clone {
    let mut n = n as usize;
    debug_assert_ne!(n, 0);
    debug_assert!(n <= self.smallest_prime_factors.len());

    std::iter::from_fn(move || {
      (n != 1).then(|| {
        let p = self.smallest_prime_factors[n];
        let mut count = 1;
        n /= p as usize;
        while self.smallest_prime_factors[n] == p {
          n /= p as usize;
          count += 1;
        }

        (p, count)
      })
    })
  }

  fn factors_generator<'a>(
    &'a self,
    multiplier: u32,
    mut prime_factors: impl Iterator<Item = (u32, u32)> + Clone + 'a,
  ) -> impl Iterator<Item = u32> {
    match prime_factors.next() {
      Some((p, m)) => Either::Left(
        std::iter::successors(Some(1), move |n| Some(n * p))
          .take(m as usize + 1)
          .flat_map(move |p| {
            Box::new(self.factors_generator(p * multiplier, prime_factors.clone()))
              as Box<dyn Iterator<Item = u32>>
          }),
      ),
      None => Either::Right(std::iter::once(multiplier)),
    }
  }

  pub fn factors(&self, n: u32) -> impl Iterator<Item = u32> {
    self.factors_generator(1, self.prime_factors(n))
  }
}

#[cfg(test)]
mod tests {
  use googletest::{assert_that, prelude::unordered_elements_are};
  use itertools::Itertools;

  use super::PrimeFactorSieve;

  #[test]
  fn test_primes() {
    let sieve = PrimeFactorSieve::new(100);
    assert_eq!(
      sieve.primes().collect_vec(),
      vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97
      ]
    );
  }

  #[test]
  fn test_is_prime() {
    let sieve = PrimeFactorSieve::new(10);
    assert!(sieve.is_prime(2));
    assert!(sieve.is_prime(3));
    assert!(!sieve.is_prime(4));
    assert!(sieve.is_prime(5));
    assert!(!sieve.is_prime(6));
    assert!(sieve.is_prime(7));
    assert!(!sieve.is_prime(8));
    assert!(!sieve.is_prime(9));
    assert!(!sieve.is_prime(10));
  }

  #[test]
  fn test_2() {
    let sieve = PrimeFactorSieve::new(2);
    assert_eq!(sieve.prime_factors(2).collect_vec(), vec![(2, 1)]);
  }

  #[test]
  fn test_10() {
    let sieve = PrimeFactorSieve::new(10);
    assert_eq!(sieve.prime_factors(2).collect_vec(), vec![(2, 1)]);
    assert_eq!(sieve.prime_factors(3).collect_vec(), vec![(3, 1)]);
    assert_eq!(sieve.prime_factors(4).collect_vec(), vec![(2, 2)]);
    assert_eq!(sieve.prime_factors(5).collect_vec(), vec![(5, 1)]);
    assert_eq!(sieve.prime_factors(6).collect_vec(), vec![(2, 1), (3, 1)]);
    assert_eq!(sieve.prime_factors(7).collect_vec(), vec![(7, 1)]);
    assert_eq!(sieve.prime_factors(8).collect_vec(), vec![(2, 3)]);
    assert_eq!(sieve.prime_factors(9).collect_vec(), vec![(3, 2)]);
    assert_eq!(sieve.prime_factors(10).collect_vec(), vec![(2, 1), (5, 1)]);
  }

  #[test]
  fn test_factors() {
    let sieve = PrimeFactorSieve::new(30);

    assert_that!(
      sieve.factors(10).collect_vec(),
      unordered_elements_are![&1, &2, &5, &10]
    );
    assert_that!(
      sieve.factors(13).collect_vec(),
      unordered_elements_are![&1, &13]
    );
    assert_that!(
      sieve.factors(24).collect_vec(),
      unordered_elements_are![&1, &2, &3, &4, &6, &8, &12, &24]
    );
  }
}
