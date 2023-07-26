use hex::FromHex;

use crate::cgrandom::engine::engine::RngEngine;
use crate::cgrandom::engine::cgcsprng1::CgCsPrng1;

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
