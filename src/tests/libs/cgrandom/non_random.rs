use crate::libs::cgrandom::generators::generator::{RngBase, RngSkippable};
use crate::libs::cgrandom::generators::non_random::{FourGenerator8, CounterGenerator8};

// the 1st output of fourgenerator_8 must be 4
#[test]
fn test_fourgenerator_8() {
  let mut rng = FourGenerator8::new();
  
  let result = rng.generate();
  
  assert_eq!(result, 4u8);
}

// the 10000th output of countergenerator_8 seeded with 54 must be 69
#[test]
fn test_countergenerator_8() {
  let mut rng = CounterGenerator8::new();
  
  rng.seed(54u8);
  
  rng.skip((9999u16 % 256u16) as u8);
  
  let result = rng.generate();
  
  assert_eq!(result, 69u8);
}
