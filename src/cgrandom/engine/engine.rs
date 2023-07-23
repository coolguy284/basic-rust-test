pub trait RngEngine {
  type RngIntType;
  
  fn new() -> Self;
  
  fn seed(&mut self, seed_num: Self::RngIntType);
  
  fn generate(&mut self) -> Self::RngIntType;
}
