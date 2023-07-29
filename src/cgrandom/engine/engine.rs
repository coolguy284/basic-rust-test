pub trait RngEngine {
  type RngOutputType;
  
  fn new() -> Self;
  
  fn seed(&mut self, seed_val: Self::RngOutputType);
  
  fn generate(&mut self) -> Self::RngOutputType;
}
