use crate::cgrandom::engine::engine::RngEngine;

pub struct Mt19937_32;

impl RngEngine for Mt19937_32 {
  type RngIntType = u32;
  
  fn new() -> Mt19937_32 {
    Mt19937_32
  }
  
  fn seed(&self, seed_num: Self::RngIntType) {
    
  }
  
  fn generate(&self) -> Self::RngIntType {
    4
  }
}

pub struct Mt19937_64;

impl RngEngine for Mt19937_64 {
  type RngIntType = u64;
  
  fn new() -> Mt19937_64 {
    Mt19937_64
  }
  
  fn seed(&self, seed_num: Self::RngIntType) {
    
  }
  
  fn generate(&self) -> Self::RngIntType {
    4
  }
}
