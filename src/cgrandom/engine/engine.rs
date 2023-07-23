pub trait RngEngine {
  type RngIntType;
  
  fn new() -> Self;
  
  fn seed(&self, seed_num: Self::RngIntType);
  
  fn generate(&self) -> Self::RngIntType;
}
