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

  /// Returns an iterator over prime factors (p, multiplicity).
  pub fn prime_factors(&self, n: u32) -> impl Iterator<Item = (u32, u32)> {
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
}

#[cfg(test)]
mod tests {
  use itertools::Itertools;

  use super::PrimeFactorSieve;

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
}
