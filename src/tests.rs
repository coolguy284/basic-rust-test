use crypto::digest::Digest;
use crypto::sha3::Sha3;
use hex::FromHex;

use crate::cgrandom::engine::cgcsprng1::CgCsPrng1;
use crate::cgrandom::engine::engine::RngEngine;
use crate::cgrandom::engine::mt19937::{Mt19937_32, Mt19937_64};

// the 10000th output of mt19937_32 seeded with 5489 must be 4123659995
#[test]
fn test_mt19937_32() {
  let mut rng = Mt19937_32::new();
  
  rng.seed(5489u32);
  
  for _ in 1usize..=9999usize {
    rng.generate();
  }
  
  let result = rng.generate();
  
  assert_eq!(result, 4123659995u32);
}

// the 10000th output of mt19937_64 seeded with 5489 must be 9981545732273789042
#[test]
fn test_mt19937_64() {
  let mut rng = Mt19937_64::new();
  
  rng.seed(5489u64);
  
  for _ in 1usize..=9999usize {
    rng.generate();
  }
  
  let result = rng.generate();
  
  assert_eq!(result, 9981545732273789042u64);
}

// the 10000th output of cgcsprng1 seeded with fe4464b847255caa418f92061e23f8bac922e979a2c323b4bec1b4b5f171faecec3390232216fb90ffbc4f20cc75902f91a52d4d7fae0068b7e2498ae01db135 and with modifier 9ff1fd69c060fc029f6b30566f2fb763d939f519187a38a528caed3d60e60c9ef9f1b51f1dd0c7ee4e9265f38919e09f38511f9f21f0779ddb290084005b7f3a must be ee40bb060265620041ceef6f69d7f1c5f968d936e8750e249fc0b50c3744c356658dcc877c25450d27f73971e2468396f63d10ba62dd56f8cd9d1060fe5b9618
// (it is sha3-512 hash of hex "fe4464b847255caa418f92061e23f8bac922e979a2c323b4bec1b4b5f171faecec3390232216fb90ffbc4f20cc75902f91a52d4d7fae0068b7e2498ae01db1359ff1fd69c060fc029f6b30566f2fb763d939f519187a38a528caed3d60e60c9ef9f1b51f1dd0c7ee4e9265f38919e09f38511f9f21f0779ddb290084005b7f3a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000270f", all 3 concatenated together)
#[test]
fn test_cgcsprng1() {
  let rng_seed = <[u8; 64]>::from_hex("fe4464b847255caa418f92061e23f8bac922e979a2c323b4bec1b4b5f171faecec3390232216fb90ffbc4f20cc75902f91a52d4d7fae0068b7e2498ae01db135").expect("this test has invalid hex seed somehow");
  let rng_modifier = <[u8; 64]>::from_hex("9ff1fd69c060fc029f6b30566f2fb763d939f519187a38a528caed3d60e60c9ef9f1b51f1dd0c7ee4e9265f38919e09f38511f9f21f0779ddb290084005b7f3a").expect("this test has invalid hex modifier somehow");
  
  let mut rng = CgCsPrng1::new();
  
  rng.seed(rng_seed);
  rng.set_modifier(rng_modifier);
  
  rng.skip(9999);
  
  let result = rng.generate();
  
  assert_eq!(result, <[u8; 64]>::from_hex("ee40bb060265620041ceef6f69d7f1c5f968d936e8750e249fc0b50c3744c356658dcc877c25450d27f73971e2468396f63d10ba62dd56f8cd9d1060fe5b9618").expect("this test has invalid hex result somehow"));
}

// sha3-512 hash of "" should be "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26"
#[test]
fn test_sha3_emptyhash() {
  let hash_input = [0u8; 0];
  
  let mut hasher = Sha3::sha3_512();
  
  hasher.input(&hash_input[..]);
  
  assert_eq!(hasher.result_str(), "a69f73cca23a9ac5c8b567dc185a756e97c982164fe25859e0d1dcc1475c80a615b2123af1f5f94c11e3e9402c3ac558f500199d95b6d3e301758586281dcd26");
}

// sha3-512 hash of "e" should be "6ebb8a73bfd0459bd575b9dbef6dcb970bb11182591f5ecd7c8c0d771b3269b715fcb84005d542ff74306565a46b3b893f64ca41b8519457ae137f6429dfbb1e"
#[test]
fn test_sha3_ehash_stringinput() {
  let mut hasher = Sha3::sha3_512();
  
  hasher.input_str("e");
  
  assert_eq!(hasher.result_str(), "6ebb8a73bfd0459bd575b9dbef6dcb970bb11182591f5ecd7c8c0d771b3269b715fcb84005d542ff74306565a46b3b893f64ca41b8519457ae137f6429dfbb1e");
}

// sha3-512 hash of hex "65" (hex for "e") should be "6ebb8a73bfd0459bd575b9dbef6dcb970bb11182591f5ecd7c8c0d771b3269b715fcb84005d542ff74306565a46b3b893f64ca41b8519457ae137f6429dfbb1e"
#[test]
fn test_sha3_ehash() {
  let hash_input = <[u8; 1]>::from_hex("65").expect("this test has invalid hex hash input somehow");
  
  let mut hasher = Sha3::sha3_512();
  
  hasher.input(&hash_input[..]);
  
  assert_eq!(hasher.result_str(), "6ebb8a73bfd0459bd575b9dbef6dcb970bb11182591f5ecd7c8c0d771b3269b715fcb84005d542ff74306565a46b3b893f64ca41b8519457ae137f6429dfbb1e");
}

// sha3-512 hash of hex "fe4464b847255caa418f92061e23f8bac922e979a2c323b4bec1b4b5f171faecec3390232216fb90ffbc4f20cc75902f91a52d4d7fae0068b7e2498ae01db1359ff1fd69c060fc029f6b30566f2fb763d939f519187a38a528caed3d60e60c9ef9f1b51f1dd0c7ee4e9265f38919e09f38511f9f21f0779ddb290084005b7f3a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000270f" should be "ee40bb060265620041ceef6f69d7f1c5f968d936e8750e249fc0b50c3744c356658dcc877c25450d27f73971e2468396f63d10ba62dd56f8cd9d1060fe5b9618"
#[test]
fn test_sha3_certainhash() {
  let hash_input = <[u8; 192]>::from_hex("fe4464b847255caa418f92061e23f8bac922e979a2c323b4bec1b4b5f171faecec3390232216fb90ffbc4f20cc75902f91a52d4d7fae0068b7e2498ae01db1359ff1fd69c060fc029f6b30566f2fb763d939f519187a38a528caed3d60e60c9ef9f1b51f1dd0c7ee4e9265f38919e09f38511f9f21f0779ddb290084005b7f3a0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000270f").expect("this test has invalid hex hash input somehow");
  
  let mut hasher = Sha3::sha3_512();
  
  hasher.input(&hash_input[..]);
  
  assert_eq!(hasher.result_str(), "ee40bb060265620041ceef6f69d7f1c5f968d936e8750e249fc0b50c3744c356658dcc877c25450d27f73971e2468396f63d10ba62dd56f8cd9d1060fe5b9618");
}
