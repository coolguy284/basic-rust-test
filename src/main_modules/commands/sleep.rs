use crate::libs::time_fixed_prec::{advanced_sleep, FixedPrec};
use crate::libs::time_fixed_prec::FixedPrec::{FPInfinite, FPNumber};
use crate::main_modules::large_prints::large_print_sleep;

pub fn command_sleep(cmd_line_args: Vec<String>) {
  if cmd_line_args.len() == 1 {
    large_print_sleep();
  } else {
    let (subcommand_args, subcommand_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
    
    if subcommand_args.len() >= 1 {
      let sleep_seconds = FixedPrec::from_str(&subcommand_args[0], 9);
      println!("Sleeping for {} seconds...", sleep_seconds.to_string());
      advanced_sleep(sleep_seconds);
      return;
    }
    
    let sleep_seconds_option = subcommand_argv.get("time-seconds").and_then(|v| v.last());
    match sleep_seconds_option {
      Some(x) => {
        let sleep_seconds = FixedPrec::from_str(x, 9);
        println!("Sleeping for {} seconds...", sleep_seconds.to_string());
        advanced_sleep(sleep_seconds);
        return;
      },
      None => {},
    }
    
    let sleep_milliseconds_option = subcommand_argv.get("time-milliseconds").and_then(|v| v.last());
    match sleep_milliseconds_option {
      Some(x) => {
        let sleep_milliseconds = FixedPrec::from_str(x, 6);
        let sleep_seconds = match sleep_milliseconds {
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
        };
        println!("Sleeping for {} seconds...", sleep_seconds.to_string());
        advanced_sleep(sleep_seconds);
        return;
      },
      None => {},
    }
    
    let sleep_microseconds_option = subcommand_argv.get("time-microseconds").and_then(|v| v.last());
    match sleep_microseconds_option {
      Some(x) => {
        let sleep_microseconds = FixedPrec::from_str(x, 3);
        let sleep_seconds = match sleep_microseconds {
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
        };
        println!("Sleeping for {} seconds...", sleep_seconds.to_string());
        advanced_sleep(sleep_seconds);
        return;
      },
      None => {},
    }
    
    let sleep_nanoseconds_option = subcommand_argv.get("time-nanoseconds").and_then(|v| v.last());
    match sleep_nanoseconds_option {
      Some(x) => {
        let sleep_nanoseconds = FixedPrec::from_str(x, 0);
        let sleep_seconds = match sleep_nanoseconds {
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
        };
        println!("Sleeping for {} seconds...", sleep_seconds.to_string());
        advanced_sleep(sleep_seconds);
        return;
      },
      None => {},
    }
    
    large_print_sleep();
  }
}
