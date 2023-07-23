mod cgrandom;
#[cfg(debug_assertions)]
mod debug_funcs;

use std::env;

use chrono::{DateTime, Local, Offset, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use iana_time_zone::get_timezone;

use cgrandom::engine::engine::RngEngine;
use cgrandom::engine::mt19937::{Mt19937_32, Mt19937_64};
#[cfg(debug_assertions)]
use debug_funcs::print_type_of;

fn main() {
  let cmd_line_args: Vec<String> = env::args().skip(1).collect();
  
  if cmd_line_args.len() == 0 {
    println!("Coolguy284's basic rust experimentation program.");
    println!();
    println!("USAGE:");
    println!("    basic-rust-test <SUBCOMMAND>");
    println!();
    println!("SUBCOMMANDS:");
    println!("    current_time");
    println!("    rng_simple");
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
          println!("        Valid RNGs: mt19937_32, mt19937_64");
          println!("    --seed-hex=<SEED>  The seed for the RNG, in hex.");
          println!("    --skip=<COUNT>     The number of initial outputs of the RNG to skip, in decimal.");
          println!("    --count=<COUNT>    The number of outputs of the RNG to display, in decimal.");
          println!();
          println!("DISCUSSION:");
          println!("    A command to print the output of a given RNG with a given seed. This should result in the same outputs every time on every platform, as long as a deterministic RNG is chosen.");
        } else {
          let (_rng_args, rng_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
          
          let rng_name = rng_argv.get("rng").and_then(|v| v.last()).expect("--rng not specified");
          let rng_seed_hex_string = rng_argv.get("seed-hex").and_then(|v| v.last()).expect("--seed-hex not specified");
          let rng_skip_count_str_option = rng_argv.get("skip").and_then(|v| v.last());
          let rng_skip_count;
          match rng_skip_count_str_option {
            Some(x) => {
              rng_skip_count = x.parse::<u64>().unwrap();
            },
            None => {
              rng_skip_count = 0;
            },
          }
          let rng_count_str_option = rng_argv.get("count").and_then(|v| v.last());
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
            "mt19937_32" => {
              println!("Random Number Generator: {}", rng_name);
              println!();
              
              let rng_seed = u32::from_str_radix(rng_seed_hex_string, 16).expect("--seed-hex not a valid hex value");
              
              println!("Seed: {}", rng_seed);
              println!();
              
              let rng = Mt19937_32::new();
              
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
              
              let rng = Mt19937_64::new();
              
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
            _ => {
              println!("Invalid value \"{}\" for \"basic-rust-test rng_basic\" argument \"--rng\". For list of arguments, run \"basic-rust-test rng_basic\" with no arguments.", rng_name);
            },
          }
        }
      },
      _ => {
        println!("Invalid argument \"{}\" for \"basic-rust-test\". For list of arguments, run \"basic-rust-test\" with no arguments", arg_0);
      },
    }
  }
}
