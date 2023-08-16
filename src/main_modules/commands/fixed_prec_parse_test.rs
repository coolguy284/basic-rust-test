use crate::libs::time_fixed_prec::FixedPrec;
use crate::libs::time_fixed_prec::FixedPrec::{FPInfinite, FPNumber};
use crate::libs::unwrap_macros::{unwrap_option_else_return_error, unwrap_result_else_return_error};
use crate::main_modules::large_prints::large_print_fixed_prec_parse_test;
use crate::main_modules::errors::CommandError;
use crate::main_modules::errors::CommandError::{ArgInvalidError, ArgNotSpecifiedError};

pub fn command_fixed_prec_parse_test(cmd_line_args: Vec<String>) -> Result<(), CommandError> {
  let (_subcommand_args, subcommand_argv) = argmap::parse(cmd_line_args.iter().skip(1).collect::<Vec<_>>().iter());
  
  if cmd_line_args.len() == 1 {
    large_print_fixed_prec_parse_test();
  } else {
    let num_str = unwrap_option_else_return_error!(subcommand_argv.get("num-str").and_then(|v| v.last()), ArgNotSpecifiedError("--num-str not specified".to_string()));
    
    let fractional_digits_option = subcommand_argv.get("fractional-digits").and_then(|v| v.last());
    
    let fractional_digits = match fractional_digits_option {
      Some(x) => unwrap_result_else_return_error!(x.parse::<u8>(), ArgInvalidError("--fractional-digits invalid".to_string())),
      None => 9,
    };
    
    println!("Input numerical string: \"{}\"", num_str);
    println!();
    
    let fixed_prec_object = FixedPrec::from_str(&num_str, fractional_digits)?;
    
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
  
  Ok(())
}
