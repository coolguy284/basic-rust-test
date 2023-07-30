pub trait RngBase {
  type RngOutputType;
  
  fn new() -> Self;
  
  fn seed(&mut self, seed_val: Self::RngOutputType);
  
  fn generate(&mut self) -> Self::RngOutputType;
}

pub trait RngSkippable: RngBase {
  type RngSkipType;
  
  fn skip(&mut self, count: Self::RngSkipType);
}
