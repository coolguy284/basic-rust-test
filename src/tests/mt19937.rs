use crate::cgrandom::generators::generator::RngBase;
use crate::cgrandom::generators::mt19937::{Mt19937_32, Mt19937_64};

// the 10000th output of mt19937_32 seeded with 5489 must be 4123659995
#[test]
fn test_mt19937_32() {
  let mut rng = Mt19937_32::new();
  
  rng.seed(5489u32);
  
  for _ in 1usize..=9999usize {
    rng.generate();
  }
  
  let result = rng.generate();
  
  assert_eq!(result, 4123659995u32);
}

// the 10000th output of mt19937_64 seeded with 5489 must be 9981545732273789042
#[test]
fn test_mt19937_64() {
  let mut rng = Mt19937_64::new();
  
  rng.seed(5489u64);
  
  for _ in 1usize..=9999usize {
    rng.generate();
  }
  
  let result = rng.generate();
  
  assert_eq!(result, 9981545732273789042u64);
}
