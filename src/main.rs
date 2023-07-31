mod libs;
mod main_modules;
#[cfg(test)]
mod tests;

use std::env;

use crate::main_modules::commands::current_time::command_current_time;
use crate::main_modules::commands::fixed_prec_parse_test::command_fixed_prec_parse_test;
use crate::main_modules::commands::rng_simple::command_rng_simple;
use crate::main_modules::commands::sleep::command_sleep;
use crate::main_modules::large_prints::large_print_no_command;

fn main() {
  let cmd_line_args: Vec<String> = env::args().skip(1).collect();
  
  if cmd_line_args.len() == 0 {
    large_print_no_command();
  } else {
    let arg_0 = cmd_line_args[0].as_str();
    match arg_0 {
      "current_time" => command_current_time(),
      "rng_simple" => command_rng_simple(cmd_line_args),
      "sleep" => command_sleep(cmd_line_args),
      "fixed_prec_parse_test" => command_fixed_prec_parse_test(cmd_line_args),
      _ => {
        println!("Invalid command \"basic-rust-test {}\". For list of commands, run \"basic-rust-test\" with no arguments.", arg_0);
      },
    }
  }
}
