use crypto::digest::Digest;
use crypto::sha3::Sha3;

use crate::cgrandom::engine::engine::{RngEngine, RngSkippable};

pub struct CgCsPrng1 {
  pub seed: [u8; 64],
  pub modifier: [u8; 64],
  pub counter: [u128; 4],
}

impl RngEngine for CgCsPrng1 {
  type RngOutputType = [u8; 64];
  
  fn new() -> CgCsPrng1 {
    CgCsPrng1 {
      seed: [0u8; 64],
      modifier: [0u8; 64],
      counter: [0u128; 4],
    }
  }
  
  fn seed(&mut self, seed_val: Self::RngOutputType) {
    self.seed.clone_from_slice(&seed_val[..]);
  }
  
  fn generate(&mut self) -> Self::RngOutputType {
    let mut hasher = Sha3::sha3_512();
    
    let mut hash_input = [0u8; 192];
    
    hash_input[..64].copy_from_slice(&self.seed[..]);
    hash_input[64..128].copy_from_slice(&self.modifier[..]);
    CgCsPrng1::copy_u128_to_u8(self.counter[0], &mut hash_input[128..144]);
    CgCsPrng1::copy_u128_to_u8(self.counter[1], &mut hash_input[144..160]);
    CgCsPrng1::copy_u128_to_u8(self.counter[2], &mut hash_input[160..176]);
    CgCsPrng1::copy_u128_to_u8(self.counter[3], &mut hash_input[176..]);
    
    hasher.input(&hash_input[..]);
    
    let mut hash_result = [0u8; 64];
    
    hasher.result(&mut hash_result[..]);
    
    self.increase_counter(1u128);
    
    hash_result
  }
}

impl RngSkippable for CgCsPrng1 {
  type RngSkipType = u128;
  
  fn skip(&mut self, count: Self::RngSkipType) {
    self.increase_counter(count);
  }
}

impl CgCsPrng1 {
  fn copy_u128_to_u8(input: u128, output: &mut [u8]) {
    if output.len() != 16 {
      panic!("copy_u128_to_u8: output length must be 16");
    }
    output[0] = (input >> 15 * 8) as u8;
    output[1] = (input >> 14 * 8 & 0xffu128) as u8;
    output[2] = (input >> 13 * 8 & 0xffu128) as u8;
    output[3] = (input >> 12 * 8 & 0xffu128) as u8;
    output[4] = (input >> 11 * 8 & 0xffu128) as u8;
    output[5] = (input >> 10 * 8 & 0xffu128) as u8;
    output[6] = (input >> 9 * 8 & 0xffu128) as u8;
    output[7] = (input >> 8 * 8 & 0xffu128) as u8;
    output[8] = (input >> 7 * 8 & 0xffu128) as u8;
    output[9] = (input >> 6 * 8 & 0xffu128) as u8;
    output[10] = (input >> 5 * 8 & 0xffu128) as u8;
    output[11] = (input >> 4 * 8 & 0xffu128) as u8;
    output[12] = (input >> 3 * 8 & 0xffu128) as u8;
    output[13] = (input >> 2 * 8 & 0xffu128) as u8;
    output[14] = (input >> 8 & 0xffu128) as u8;
    output[15] = (input & 0xffu128) as u8;
  }
  
  pub fn set_modifier(&mut self, modifier: <CgCsPrng1 as RngEngine>::RngOutputType) {
    self.modifier.clone_from_slice(&modifier[..]);
  }
  
  pub fn increase_counter(&mut self, count: u128) {
    let (pos3_res, pos3_carry) = self.counter[3].overflowing_add(count);
    self.counter[3] = pos3_res;
    if !pos3_carry { return; }
    
    let (pos2_res, pos2_carry) = self.counter[2].overflowing_add(1u128);
    self.counter[2] = pos2_res;
    if !pos2_carry { return; }
    
    let (pos1_res, pos1_carry) = self.counter[1].overflowing_add(1u128);
    self.counter[1] = pos1_res;
    if !pos1_carry { return; }
    
    let (pos0_res, _pos0_carry) = self.counter[0].overflowing_add(1u128);
    self.counter[0] = pos0_res;
  }
  
  #[allow(dead_code)]
  pub fn decrease_counter(&mut self, count: u128) {
    let (pos3_res, pos3_carry) = self.counter[3].overflowing_sub(count);
    self.counter[3] = pos3_res;
    if !pos3_carry { return; }
    
    let (pos2_res, pos2_carry) = self.counter[2].overflowing_sub(1u128);
    self.counter[2] = pos2_res;
    if !pos2_carry { return; }
    
    let (pos1_res, pos1_carry) = self.counter[1].overflowing_sub(1u128);
    self.counter[1] = pos1_res;
    if !pos1_carry { return; }
    
    let (pos0_res, _pos0_carry) = self.counter[0].overflowing_sub(1u128);
    self.counter[0] = pos0_res;
  }
}
