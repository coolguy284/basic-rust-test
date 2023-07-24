pub trait RngEngine {
  type RngIntType;
  
  fn new() -> Self;
  
  fn seed(&mut self, seed_val: Self::RngIntType);
  
  fn generate(&mut self) -> Self::RngIntType;
}
