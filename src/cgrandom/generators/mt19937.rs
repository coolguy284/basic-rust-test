use std::mem::size_of;
use std::num::Wrapping;
use std::ops::{BitAnd, Shl, Sub};

use num_traits::One;

use crate::cgrandom::generators::generator::RngBase;

fn lowest_n_bits_of<T: Shl<usize> + BitAnd<<<T as Shl<usize>>::Output as Sub<T>>::Output, Output = T> + One>(num: T, n_bits: usize) -> T where <T as Shl<usize>>::Output: Sub<T> {
  if n_bits >= (size_of::<T>() * 8usize) {
    num
  } else {
    num & (T::one() << n_bits) - T::one()
  }
}

// https://en.wikipedia.org/wiki/Mersenne_Twister for constants and instance vars

const MT19937_32_W: Wrapping<u32> = Wrapping(32u32);
const MT19937_32_N: Wrapping<u32> = Wrapping(624u32);
const MT19937_32_M: Wrapping<u32> = Wrapping(397u32);
const MT19937_32_R: Wrapping<u32> = Wrapping(31u32);
const MT19937_32_A: Wrapping<u32> = Wrapping(0x9908b0dfu32);
const MT19937_32_U: Wrapping<u32> = Wrapping(11u32);
const MT19937_32_D: Wrapping<u32> = Wrapping(0xffffffffu32);
const MT19937_32_S: Wrapping<u32> = Wrapping(7u32);
const MT19937_32_B: Wrapping<u32> = Wrapping(0x9d2c5680u32);
const MT19937_32_T: Wrapping<u32> = Wrapping(15u32);
const MT19937_32_C: Wrapping<u32> = Wrapping(0xefc60000u32);
const MT19937_32_L: Wrapping<u32> = Wrapping(18u32);
const MT19937_32_F: Wrapping<u32> = Wrapping(1812433253u32);

const MT19937_32_LOWER_MASK: Wrapping<u32> = Wrapping((1u32 << MT19937_32_R.0 as usize) - 1u32);
const MT19937_32_UPPER_MASK: Wrapping<u32> = Wrapping(!MT19937_32_LOWER_MASK.0); // const version of: lowest_n_bits_of::<u32>(!MT19937_32_LOWER_MASK, MT19937_32_W as usize);

pub struct Mt19937_32 {
  pub mt_state: [Wrapping<u32>; MT19937_32_N.0 as usize],
  pub mt_index: Wrapping<u32>,
}

impl RngBase for Mt19937_32 {
  type RngOutputType = u32;
  
  fn new() -> Mt19937_32 {
    Mt19937_32 {
      mt_state: [Wrapping(0u32); MT19937_32_N.0 as usize],
      mt_index: MT19937_32_N + Wrapping(1u32),
    }
  }
  
  // https://en.wikipedia.org/wiki/Mersenne_Twister
  fn seed(&mut self, seed_val: Self::RngOutputType) {
    self.mt_index = MT19937_32_N;
    self.mt_state[0] = Wrapping(seed_val);
    for i in 1u32..MT19937_32_N.0 {
      let wrapping_i = Wrapping(i);
      self.mt_state[i as usize] = Wrapping(lowest_n_bits_of::<u32>((MT19937_32_F * (self.mt_state[(i - 1u32) as usize] ^ (self.mt_state[(i - 1u32) as usize] >> (MT19937_32_W.0 - 2u32) as usize)) + wrapping_i).0, MT19937_32_W.0 as usize));
    }
  }
  
  // https://en.wikipedia.org/wiki/Mersenne_Twister
  fn generate(&mut self) -> Self::RngOutputType {
    if self.mt_index >= MT19937_32_N {
      if self.mt_index > MT19937_32_N {
        //panic!("Generator not seeded");
        // silently seed with 5489
        self.seed(5489u32);
      }
      self.twist();
    }
    
    let mut y = self.mt_state[self.mt_index.0 as usize];
    y = y ^ ((y >> MT19937_32_U.0 as usize) & MT19937_32_D);
    y = y ^ ((y << MT19937_32_S.0 as usize) & MT19937_32_B);
    y = y ^ ((y << MT19937_32_T.0 as usize) & MT19937_32_C);
    y = y ^ (y >> MT19937_32_L.0 as usize);
    
    self.mt_index += 1u32;
    
    return lowest_n_bits_of::<u32>(y.0, MT19937_32_W.0 as usize);
  }
}

impl Mt19937_32 {
  // https://en.wikipedia.org/wiki/Mersenne_Twister
  fn twist(&mut self) {
    for i in 0u32..MT19937_32_N.0 {
      let wrapping_i = Wrapping(i);
      let x = (self.mt_state[i as usize] & MT19937_32_UPPER_MASK)
                | (self.mt_state[((wrapping_i + Wrapping(1u32)) % MT19937_32_N).0 as usize] & MT19937_32_LOWER_MASK);
      let mut x_a = x >> 1usize;
      if (x % Wrapping(2u32)) != Wrapping(0u32) {
        x_a = x_a ^ MT19937_32_A;
      }
      self.mt_state[i as usize] = self.mt_state[((wrapping_i + MT19937_32_M) % MT19937_32_N).0 as usize] ^ x_a;
    }
    self.mt_index = Wrapping(0u32);
  }
}

// https://en.wikipedia.org/wiki/Mersenne_Twister for constants and instance vars

const MT19937_64_W: Wrapping<u64> = Wrapping(64u64);
const MT19937_64_N: Wrapping<u64> = Wrapping(312u64);
const MT19937_64_M: Wrapping<u64> = Wrapping(156u64);
const MT19937_64_R: Wrapping<u64> = Wrapping(31u64);
const MT19937_64_A: Wrapping<u64> = Wrapping(0xb5026f5aa96619e9u64);
const MT19937_64_U: Wrapping<u64> = Wrapping(29u64);
const MT19937_64_D: Wrapping<u64> = Wrapping(0x5555555555555555u64);
const MT19937_64_S: Wrapping<u64> = Wrapping(17u64);
const MT19937_64_B: Wrapping<u64> = Wrapping(0x71d67fffeda60000u64);
const MT19937_64_T: Wrapping<u64> = Wrapping(37u64);
const MT19937_64_C: Wrapping<u64> = Wrapping(0xfff7eee000000000u64);
const MT19937_64_L: Wrapping<u64> = Wrapping(43u64);
const MT19937_64_F: Wrapping<u64> = Wrapping(6364136223846793005u64);

const MT19937_64_LOWER_MASK: Wrapping<u64> = Wrapping((1u64 << MT19937_64_R.0 as usize) - 1u64);
const MT19937_64_UPPER_MASK: Wrapping<u64> = Wrapping(!MT19937_64_LOWER_MASK.0); // const version of: lowest_n_bits_of::<u64>(!MT19937_64_LOWER_MASK, MT19937_64_W as usize);

pub struct Mt19937_64 {
  pub mt_state: [Wrapping<u64>; MT19937_64_N.0 as usize],
  pub mt_index: Wrapping<u64>,
}

impl RngBase for Mt19937_64 {
  type RngOutputType = u64;
  
  fn new() -> Mt19937_64 {
    Mt19937_64 {
      mt_state: [Wrapping(0u64); MT19937_64_N.0 as usize],
      mt_index: MT19937_64_N + Wrapping(1u64),
    }
  }
  
  // https://en.wikipedia.org/wiki/Mersenne_Twister
  fn seed(&mut self, seed_val: Self::RngOutputType) {
    self.mt_index = MT19937_64_N;
    self.mt_state[0] = Wrapping(seed_val);
    for i in 1u64..MT19937_64_N.0 {
      let wrapping_i = Wrapping(i);
      self.mt_state[i as usize] = Wrapping(lowest_n_bits_of::<u64>((MT19937_64_F * (self.mt_state[(i - 1u64) as usize] ^ (self.mt_state[(i - 1u64) as usize] >> (MT19937_64_W.0 - 2u64) as usize)) + wrapping_i).0, MT19937_64_W.0 as usize));
    }
  }
  
  // https://en.wikipedia.org/wiki/Mersenne_Twister
  fn generate(&mut self) -> Self::RngOutputType {
    if self.mt_index >= MT19937_64_N {
      if self.mt_index > MT19937_64_N {
        //panic!("Generator not seeded");
        // silently seed with 5489
        self.seed(5489u64);
      }
      self.twist();
    }
    
    let mut y = self.mt_state[self.mt_index.0 as usize];
    y = y ^ ((y >> MT19937_64_U.0 as usize) & MT19937_64_D);
    y = y ^ ((y << MT19937_64_S.0 as usize) & MT19937_64_B);
    y = y ^ ((y << MT19937_64_T.0 as usize) & MT19937_64_C);
    y = y ^ (y >> MT19937_64_L.0 as usize);
    
    self.mt_index += 1u64;
    
    return lowest_n_bits_of::<u64>(y.0, MT19937_64_W.0 as usize);
  }
}

impl Mt19937_64 {
  // https://en.wikipedia.org/wiki/Mersenne_Twister
  fn twist(&mut self) {
    for i in 0u64..MT19937_64_N.0 {
      let wrapping_i = Wrapping(i);
      let x = (self.mt_state[i as usize] & MT19937_64_UPPER_MASK)
                | (self.mt_state[((wrapping_i + Wrapping(1u64)) % MT19937_64_N).0 as usize] & MT19937_64_LOWER_MASK);
      let mut x_a = x >> 1usize;
      if (x % Wrapping(2u64)) != Wrapping(0u64) {
        x_a = x_a ^ MT19937_64_A;
      }
      self.mt_state[i as usize] = self.mt_state[((wrapping_i + MT19937_64_M) % MT19937_64_N).0 as usize] ^ x_a;
    }
    self.mt_index = Wrapping(0u64);
  }
}
