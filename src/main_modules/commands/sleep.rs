use std::collections::HashMap;

use crate::libs::time_fixed_prec::{advanced_sleep, FixedPrec};
use crate::libs::time_fixed_prec::FixedPrec::{FPInfinite, FPNumber};
use crate::libs::time_fixed_prec::FixedPrecStrParseError;
use crate::main_modules::large_prints::large_print_sleep;

fn parse_sleep_seconds(subcommand_args: Vec<String>, subcommand_argv: HashMap<String, Vec<String>>) -> Result<Option<FixedPrec>, FixedPrecStrParseError> {
  if subcommand_args.len() >= 1 {
    return Ok(Some(FixedPrec::from_str(&subcommand_args[0], 9)?));
  }
  
  let sleep_seconds_option = subcommand_argv.get("time-seconds").and_then(|v| v.last());
  match sleep_seconds_option {
    Some(x) => return Ok(Some(FixedPrec::from_str(x, 9)?)),
    None => {},
  }
  
  let sleep_milliseconds_option = subcommand_argv.get("time-milliseconds").and_then(|v| v.last());
  match sleep_milliseconds_option {
    Some(x) => {
      let sleep_milliseconds = FixedPrec::from_str(x, 6)?;
      return Ok(Some(match sleep_milliseconds {
        FPNumber { integer_part, fractional_part, .. } => FPNumber {
          negative: false,
          integer_part: integer_part / 1000u128,
          fractional_part: (integer_part % 1000u128) as u32 * 1000000u32 + fractional_part,
          fractional_digits: 9,
        },
        FPInfinite { negative, fractional_digits } => FPInfinite {
          negative,
          fractional_digits,
        },
      }));
    },
    None => {},
  }
  
  let sleep_microseconds_option = subcommand_argv.get("time-microseconds").and_then(|v| v.last());
  match sleep_microseconds_option {
    Some(x) => {
      let sleep_microseconds = FixedPrec::from_str(x, 3)?;
      return Ok(Some(match sleep_microseconds {
        FPNumber { integer_part, fractional_part, .. } => FPNumber {
          negative: false,
          integer_part: integer_part / 1000000u128,
          fractional_part: (integer_part % 1000000u128) as u32 * 1000u32 + fractional_part,
          fractional_digits: 9,
        },
        FPInfinite { negative, fractional_digits } => FPInfinite {
          negative,
          fractional_digits,
        },
      }));
    },
    None => {},
  }
  
  let sleep_nanoseconds_option = subcommand_argv.get("time-nanoseconds").and_then(|v| v.last());
  match sleep_nanoseconds_option {
    Some(x) => {
      let sleep_nanoseconds = FixedPrec::from_str(x, 0)?;
      return Ok(Some(match sleep_nanoseconds {
        FPNumber { integer_part, .. } => FPNumber {
          negative: false,
          integer_part: integer_part / 1000000000u128,
          fractional_part: (integer_part % 1000000000u128) as u32,
          fractional_digits: 9,
        },
        FPInfinite { negative, fractional_digits } => FPInfinite {
          negative,
          fractional_digits,
        },
      }));
    },
    None => {},
  }
  
  Ok(None)
}

pub fn command_sleep(cmd_line_args: Vec<String>) -> Result<(), FixedPrecStrParseError> {
  if cmd_line_args.len() == 1 {
    large_print_sleep();
  } else {
    let (subcommand_args, subcommand_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
    
    let sleep_seconds = parse_sleep_seconds(subcommand_args, subcommand_argv)?;
    
    match sleep_seconds {
      Some(x) => {
        println!("Sleeping for {} seconds...", x.to_string());
        advanced_sleep(x);
      },
      None => large_print_sleep(),
    }
  }
  
  Ok(())
}
