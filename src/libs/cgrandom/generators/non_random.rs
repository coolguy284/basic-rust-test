use std::num::Wrapping;

use crate::libs::cgrandom::generators::generator::{RngBase, RngSkippable};

pub struct FourGenerator8;

impl RngBase for FourGenerator8 {
  type RngOutputType = u8;
  
  fn new() -> FourGenerator8 {
    FourGenerator8
  }
  
  fn seed(&mut self, _seed_val: Self::RngOutputType) {}
  
  fn generate(&mut self) -> Self::RngOutputType {
    4u8
  }
}

pub struct CounterGenerator8 {
  pub cg_index: Wrapping<u8>,
}

impl RngBase for CounterGenerator8 {
  type RngOutputType = u8;
  
  fn new() -> CounterGenerator8 {
    CounterGenerator8 {
      cg_index: Wrapping(0u8),
    }
  }
  
  fn seed(&mut self, seed_val: Self::RngOutputType) {
    self.cg_index = Wrapping(seed_val);
  }
  
  fn generate(&mut self) -> Self::RngOutputType {
    let return_val = self.cg_index.0;
    
    self.cg_index += 1u8;
    
    return_val
  }
}

impl RngSkippable for CounterGenerator8 {
  type RngSkipType = Self::RngOutputType;
  
  fn skip(&mut self, count: Self::RngSkipType) {
    self.cg_index += count;
  }
}
