mod cgrandom;
#[cfg(debug_assertions)]
mod debug_funcs;
#[cfg(test)]
mod tests;
mod time_lib;

use std::env;

use chrono::{DateTime, Local, Offset, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use iana_time_zone::get_timezone;
use hex::{FromHex, ToHex};

use cgrandom::generators::cgcsprng1::CgCsPrng1;
use cgrandom::generators::generator::{RngBase, RngSkippable};
use cgrandom::generators::mt19937::{Mt19937_32, Mt19937_64};
use cgrandom::generators::non_random::{CounterGenerator8, FourGenerator8};
#[cfg(debug_assertions)]
use debug_funcs::print_type_of;
use time_lib::{advanced_sleep, FixedPrec};
use time_lib::FixedPrec::{FPInfinite, FPNumber};

fn main() {
  let cmd_line_args: Vec<String> = env::args().skip(1).collect();
  
  if cmd_line_args.len() == 0 {
    println!("Coolguy284's basic rust experimentation program.");
    println!();
    println!("USAGE:");
    println!("    basic-rust-test <SUBCOMMAND>");
    println!();
    println!("SUBCOMMANDS (PRIMARY):");
    println!("    current_time");
    println!("    rng_simple");
    println!("    sleep");
    println!();
    println!("SUBCOMMANDS (DEBUGGING):");
    println!("    fixed_prec_parse_test");
    println!();
    println!("DISCUSSION:");
    println!("    Just a collection of random commands to do random things in rust.");
  } else {
    let arg_0 = cmd_line_args[0].as_str();
    match arg_0 {
      "current_time" => {
        let now_utc = Utc::now();
        
        let now_local: DateTime<Local> = DateTime::from(now_utc);
        
        // i don't even know bro https://stackoverflow.com/questions/59603665/how-do-you-find-the-local-timezone-offset-in-rust/59603899#59603899
        let local_pre_fixed_offset = Local.timestamp_opt(0, 0).single().unwrap();
        let local_fixed_offset = local_pre_fixed_offset.offset();
        let local_offset_secs = local_fixed_offset.fix().local_minus_utc();
        
        let local_offset_secs_is_positive = local_offset_secs >= 0;
        let local_offset_secs_absolute = local_offset_secs.abs();
        
        let local_tz_str = get_timezone().unwrap();
        let local_tz: Tz = local_tz_str.parse().unwrap();
        let local_utc_offset = local_tz.offset_from_utc_date(&now_utc.date_naive());
        let local_tz_abbreviation = local_utc_offset.abbreviation();
        
        #[cfg(debug_assertions)]
        {
          // TODO somewhen: reconcile these 2 inexplicably different types
          println!("Types of the 2 inexplicably different offset variables:");
          print_type_of(&local_fixed_offset);
          print_type_of(&local_utc_offset);
          println!();
        }
        
        println!("Current time, written in many ways:");
        println!();
        println!("Human readable:");
        println!("  Current Local time is {} {}", now_local.format("%A, %B %d, %Y, %I:%M:%S.%9f %p %z"), local_tz_abbreviation);
        println!("  Current UTC time is   {}", now_utc.format("%A, %B %d, %Y, %I:%M:%S.%9f %p %z %Z"));
        println!();
        println!("Technical:");
        println!("  Current Local time is {}", now_local.format("%Y-%m-%dT%H:%M:%S.%9f"));
        println!("  Current UTC time is   {}", now_utc.format("%Y-%m-%dT%H:%M:%S.%9fZ"));
        println!("  Local time offset is  {}{}:{:0>2} ({}, {})", if local_offset_secs_is_positive { "" } else { "-" }, local_offset_secs_absolute / 3600, local_offset_secs_absolute / 60 % 60, local_tz_abbreviation, local_tz_str);
      },
      "rng_simple" => {
        if cmd_line_args.len() == 1 {
          println!("Coolguy284's basic rust experimentation program, random number module.");
          println!();
          println!("USAGE:");
          println!("    basic-rust-test rng_simple <OPTIONS>");
          println!();
          println!("OPTIONS:");
          println!("    --rng=<NAME>       The name of the RNG to use.");
          println!("        Valid RNGs:");
          println!("          Non random:    fourgenerator, countergenerator");
          println!("          PRNGs:         mt19937_32, mt19937_64");
          println!("          CSPRNGs:       cgcsprng1 (untested)");
          println!("    --seed-hex=<SEED>  The seed for the RNG, in hex.");
          println!("    --skip=<COUNT>     The number of initial outputs of the RNG to skip, in decimal.");
          println!("    --count=<COUNT>    The number of outputs of the RNG to display, in decimal.");
          println!();
          println!("DISCUSSION:");
          println!("    A command to print the output of a given RNG with a given seed. This should result in the same outputs every time on every platform, as long as a deterministic RNG is chosen.");
        } else {
          let (_subcommand_args, subcommand_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
          
          let rng_name = subcommand_argv.get("rng").and_then(|v| v.last()).expect("--rng not specified");
          let rng_seed_hex_string = subcommand_argv.get("seed-hex").and_then(|v| v.last()).expect("--seed-hex not specified");
          let rng_skip_count_str_option = subcommand_argv.get("skip").and_then(|v| v.last());
          let rng_skip_count;
          match rng_skip_count_str_option {
            Some(x) => {
              rng_skip_count = x.parse::<u64>().unwrap();
            },
            None => {
              rng_skip_count = 0;
            },
          }
          let rng_count_str_option = subcommand_argv.get("count").and_then(|v| v.last());
          let rng_count;
          match rng_count_str_option {
            Some(x) => {
              rng_count = x.parse::<u64>().unwrap();
            },
            None => {
              rng_count = 10;
            },
          }
          
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
              
              let rng_seed = u8::from_str_radix(rng_seed_hex_string, 16).expect("--seed-hex not a valid hex value");
              
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
              
              let rng_seed = u32::from_str_radix(rng_seed_hex_string, 16).expect("--seed-hex not a valid hex value");
              
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
              
              let rng_seed = u64::from_str_radix(rng_seed_hex_string, 16).expect("--seed-hex not a valid hex value");
              
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
              
              let rng_seed = <[u8; 64]>::from_hex(rng_seed_hex_string).expect("--seed-hex is invalid hex");
              
              println!("Seed: {}", rng_seed.encode_hex::<String>());
              println!();
              
              let rng_modifier_hex_string_option = subcommand_argv.get("modifier-hex").and_then(|v| v.last());
              
              let rng_modifier;
              match rng_modifier_hex_string_option {
                Some(x) => {
                  if x.len() != 128 {
                    panic!("--modifier-hex length of {} is invalid, must be 128 chars", x.len());
                  }
                  
                  rng_modifier = <[u8; 64]>::from_hex(x).expect("--modifier-hex is invalid hex");
                },
                None => {
                  rng_modifier = [0u8; 64];
                },
              }
              
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
      },
      "sleep" => {
        if cmd_line_args.len() == 1 {
          println!("Coolguy284's basic rust experimentation program, sleep module.");
          println!();
          println!("USAGE:");
          println!("    basic-rust-test sleep [SECONDS] [OPTIONS]");
          println!();
          println!("OPTIONS:");
          println!("    [SECONDS]                     Time in seconds to sleep as an integer or a decimal.");
          println!("    --time-seconds=<SECONDS>      Time in seconds to sleep as an integer or a decimal.");
          println!("    --time-milliseconds=<MILLIS>  Time in milliseconds to sleep as an integer or a decimal.");
          println!("    --time-microseconds=<MICROS>  Time in microseconds to sleep as an integer or a decimal.");
          println!("    --time-nanoseconds=<NANOS>    Time in nanoseconds to sleep as an integer.");
          println!();
          println!("DISCUSSION:");
          println!("    A command to sleep a certain length of time. If a normal argument is given the first argument has precedence, if not the first one from the list above has precedence.");
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
          
          println!("Coolguy284's basic rust experimentation program, sleep module.");
          println!();
          println!("USAGE:");
          println!("    basic-rust-test sleep [SECONDS] [OPTIONS]");
          println!();
          println!("OPTIONS:");
          println!("    [SECONDS]                     Time in seconds to sleep as an integer or a decimal.");
          println!("    --time-seconds=<SECONDS>      Time in seconds to sleep as an integer or a decimal.");
          println!("    --time-milliseconds=<MILLIS>  Time in milliseconds to sleep as an integer or a decimal.");
          println!("    --time-microseconds=<MICROS>  Time in microseconds to sleep as an integer or a decimal.");
          println!("    --time-nanoseconds=<NANOS>    Time in nanoseconds to sleep as an integer.");
          println!();
          println!("DISCUSSION:");
          println!("    A command to sleep a certain length of time. If a normal argument is given the first argument has precedence, if not the first one from the list above has precedence.");
        }
      },
      "fixed_prec_parse_test" => {
        let (_subcommand_args, subcommand_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
        
        if cmd_line_args.len() == 1 {
          println!("Coolguy284's basic rust experimentation program, FixedPrec parse test module.");
          println!();
          println!("USAGE:");
          println!("    basic-rust-test fixed_prec_parse_test <OPTIONS>");
          println!();
          println!("OPTIONS:");
          println!("    --num-str=<VALUE>              The value to convert into a FixedPrec object and then back into a string.");
          println!("    --fractional-digits=<INTEGER>  The number of digits after the decimal point the FixedPrec object should have.");
          println!();
          println!("DISCUSSION:");
          println!("    A debug command to convert a number into a FixedPrec object and then back into a string, for testing.");
        } else {
          let num_str = subcommand_argv.get("num-str").and_then(|v| v.last()).expect("--num-str not present");
          
          let fractional_digits_option = subcommand_argv.get("fractional-digits").and_then(|v| v.last());
          
          let fractional_digits;
          match fractional_digits_option {
            Some(x) => {
              fractional_digits = x.parse::<u8>().expect("--fractional-digits invalid");
            },
            None => {
              fractional_digits = 9;
            },
          }
          
          println!("Input numerical string: \"{}\"", num_str);
          println!();
          
          let fixed_prec_object = FixedPrec::from_str(&num_str, fractional_digits);
          
          println!("Object:");
          match fixed_prec_object {
            FPNumber { negative, integer_part, fractional_part, fractional_digits } => {
              println!("FixedPrec::FPNumber {{");
              println!("  negative: {},", negative);
              println!("  integer_part: {},", integer_part);
              println!("  fractional_part: {},", fractional_part);
              println!("  fractional_digits: {},", fractional_digits);
              println!("}}");
            },
            FPInfinite { negative, fractional_digits } => {
              println!("FixedPrec::FPInfinite {{");
              println!("  negative: {},", negative);
              println!("  fractional_digits: {},", fractional_digits);
              println!("}}");
            },
          }
          println!();
          
          let fixed_prec_str = fixed_prec_object.to_string();
          
          println!("Converted string: \"{}\"", fixed_prec_str);
        }
      },
      _ => {
        println!("Invalid command \"basic-rust-test {}\". For list of commands, run \"basic-rust-test\" with no arguments.", arg_0);
      },
    }
  }
}
