use hex::{FromHex, ToHex};

use crate::libs::cgrandom::generators::cgcsprng1::CgCsPrng1;
use crate::libs::cgrandom::generators::generator::{RngBase, RngSkippable};
use crate::libs::cgrandom::generators::mt19937::{Mt19937_32, Mt19937_64};
use crate::libs::cgrandom::generators::non_random::{CounterGenerator8, FourGenerator8};
use crate::libs::unwrap_macros::{unwrap_option_else_return_error, unwrap_result_else_return_error};
use crate::main_modules::large_prints::large_print_rng_simple;
use crate::main_modules::errors::CommandError;
use crate::main_modules::errors::CommandError::{ArgInvalidHexError, ArgInvalidNumberError, ArgNotSpecifiedError};

pub fn command_rng_simple(cmd_line_args: Vec<String>) -> Result<(), CommandError> {
  if cmd_line_args.len() == 1 {
    large_print_rng_simple();
  } else {
    let (_subcommand_args, subcommand_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
    
    let rng_name = unwrap_option_else_return_error!(subcommand_argv.get("rng").and_then(|v| v.last()), ArgNotSpecifiedError("--rng not specified".to_string()));
    let rng_seed_hex_string = unwrap_option_else_return_error!(subcommand_argv.get("seed-hex").and_then(|v| v.last()), ArgNotSpecifiedError("--seed-hex not specified".to_string()));
    let rng_skip_count_str_option = subcommand_argv.get("skip").and_then(|v| v.last());
    let rng_skip_count = match rng_skip_count_str_option {
      Some(x) => unwrap_result_else_return_error!(x.parse::<u64>(), ArgInvalidNumberError("--skip not a valid number".to_string())),
      None => 0,
    };
    let rng_count_str_option = subcommand_argv.get("count").and_then(|v| v.last());
    let rng_count = match rng_count_str_option {
      Some(x) => unwrap_result_else_return_error!(x.parse::<u64>(), ArgInvalidNumberError("--count not a valid number".to_string())),
      None => 10,
    };
    
    match rng_name.as_str() {
      "fourgenerator" => {
        println!("Random Number Generator: {}", rng_name);
        println!();
        
        let mut rng = FourGenerator8::new();
        
        // output some outputs
        println!("Outputs ({}):", rng_count);
        for _ in 0..rng_count {
          let result = rng.generate();
          println!("{}", result);
        }
      },
      "countergenerator" => {
        println!("Random Number Generator: {}", rng_name);
        println!();
        
        let rng_seed = unwrap_result_else_return_error!(u8::from_str_radix(rng_seed_hex_string, 16), ArgInvalidHexError("--seed-hex not a valid hex value".to_string()));
        
        println!("Seed: {}", rng_seed);
        println!();
        
        let mut rng = CounterGenerator8::new();
        
        rng.seed(rng_seed);
        
        // skip some outputs
        if rng_skip_count > 0 {
          println!("Skipping {} outputs", rng_skip_count);
          println!();
          
          rng.skip((rng_skip_count % 256u64) as u8);
        }
        
        // output some outputs
        println!("Outputs ({}):", rng_count);
        for _ in 0..rng_count {
          let result = rng.generate();
          println!("{}", result);
        }
      },
      "mt19937_32" => {
        println!("Random Number Generator: {}", rng_name);
        println!();
        
        let rng_seed = unwrap_result_else_return_error!(u32::from_str_radix(rng_seed_hex_string, 16), ArgInvalidHexError("--seed-hex not a valid hex value".to_string()));
        
        println!("Seed: {}", rng_seed);
        println!();
        
        let mut rng = Mt19937_32::new();
        
        rng.seed(rng_seed);
        
        // skip some outputs
        if rng_skip_count > 0 {
          println!("Skipping {} outputs", rng_skip_count);
          println!();
          for _ in 0..rng_skip_count {
            rng.generate();
          }
        }
        
        // output some outputs
        println!("Outputs ({}):", rng_count);
        for _ in 0..rng_count {
          let result = rng.generate();
          println!("{}", result);
        }
      },
      "mt19937_64" => {
        println!("Random Number Generator: {}", rng_name);
        println!();
        
        let rng_seed = unwrap_result_else_return_error!(u64::from_str_radix(rng_seed_hex_string, 16), ArgInvalidHexError("--seed-hex not a valid hex value".to_string()));
        
        println!("Seed: {}", rng_seed);
        println!();
        
        let mut rng = Mt19937_64::new();
        
        rng.seed(rng_seed);
        
        // skip some outputs
        if rng_skip_count > 0 {
          println!("Skipping {} outputs", rng_skip_count);
          println!();
          for _ in 0..rng_skip_count {
            rng.generate();
          }
        }
        
        // output some outputs
        println!("Outputs ({}):", rng_count);
        for _ in 0..rng_count {
          let result = rng.generate();
          println!("{}", result);
        }
      },
      "cgcsprng1" => {
        println!("Random Number Generator: {}", rng_name);
        println!();
        
        if rng_seed_hex_string.len() != 128 {
          panic!("--seed-hex length of {} is invalid, must be 128 chars", rng_seed_hex_string.len());
        }
        
        let rng_seed = unwrap_result_else_return_error!(<[u8; 64]>::from_hex(rng_seed_hex_string), ArgInvalidHexError("--seed-hex is invalid hex".to_string()));
        
        println!("Seed: {}", rng_seed.encode_hex::<String>());
        println!();
        
        let rng_modifier_hex_string_option = subcommand_argv.get("modifier-hex").and_then(|v| v.last());
        
        let rng_modifier = match rng_modifier_hex_string_option {
          Some(x) => {
            if x.len() != 128 {
              panic!("--modifier-hex length of {} is invalid, must be 128 chars", x.len());
            }
            
            unwrap_result_else_return_error!(<[u8; 64]>::from_hex(x), ArgInvalidHexError("--modifier-hex is invalid hex".to_string()))
          },
          None => [0u8; 64],
        };
        
        println!("Modifier: {}", rng_modifier.encode_hex::<String>());
        println!();
        
        let mut rng = CgCsPrng1::new();
        
        rng.seed(rng_seed);
        rng.set_modifier(rng_modifier);
        
        // skip some outputs
        if rng_skip_count > 0 {
          println!("Skipping {} outputs", rng_skip_count);
          println!();
          rng.skip(rng_skip_count as u128);
        }
        
        // output some outputs
        println!("Outputs ({}):", rng_count);
        for _ in 0..rng_count {
          let result = rng.generate();
          println!("{}", result.encode_hex::<String>());
        }
      },
      _ => {
        println!("Invalid value \"{}\" for \"basic-rust-test rng_basic\" argument \"--rng\". For list of arguments, run \"basic-rust-test rng_basic\" with no arguments.", rng_name);
      },
    }
  }
  
  Ok(())
}
