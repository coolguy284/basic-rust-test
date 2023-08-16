use std::thread::sleep;
use std::time::Duration;

use substring::Substring;

use crate::libs::unwrap_macros::unwrap_result_else_return_error;

use FixedPrec::{FPInfinite, FPNumber};
use FixedPrecStrParseError::{DecimalParseError, HighPrecError};

#[derive(Debug, PartialEq)]
pub enum FixedPrec {
  FPNumber {
    negative: bool,
    integer_part: u128,
    fractional_part: u32,
    fractional_digits: u8,
  },
  FPInfinite {
    negative: bool,
    fractional_digits: u8,
  }
}

#[derive(Debug)]
pub enum FixedPrecStrParseError {
  HighPrecError(String),
  DecimalParseError(String),
}

impl FixedPrec {
  pub fn from_str(str_val: &str, fractional_digits: u8) -> Result<FixedPrec, FixedPrecStrParseError> {
    if fractional_digits > 9 {
      return Err(HighPrecError(format!("fractional_digits of {} cannot fit unto u32 type", fractional_digits)));
    }
    
    if str_val.len() == 0 {
      Ok(FPNumber {
        negative: false,
        integer_part: 0u128,
        fractional_part: 0u32,
        fractional_digits,
      })
    } else {
      let negative;
      let str_val_nosign;
      if str_val.substring(0, 1) == "-" {
        negative = true;
        str_val_nosign = str_val.substring(1, str_val.len());
      } else {
        negative = false;
        str_val_nosign = str_val;
      }
      
      let str_val_nosign_lowercase = str_val_nosign.to_lowercase();
      
      if str_val_nosign_lowercase == "infinite" ||
        str_val_nosign_lowercase == "infinity" ||
        str_val_nosign_lowercase == "forever" ||
        str_val_nosign_lowercase == "eternity" {
        Ok(FPInfinite {
          negative,
          fractional_digits,
        })
      } else if str_val_nosign.contains(".") {
        let str_val_nosign_split: Vec<_> = str_val_nosign.split('.').collect();
        
        if str_val_nosign_split.len() > 2 {
          return Err(DecimalParseError(format!("String \"{}\" not a valid decimal number (too many decimal points).", str_val_nosign)));
        }
        
        let str_num_digits = str_val_nosign_split[1].len() as u8;
        
        if str_num_digits == 0 {
          Ok(FPNumber { 
            negative,
            integer_part: unwrap_result_else_return_error!(str_val_nosign_split[0].parse::<u128>(), DecimalParseError(format!("Integer part of string \"{}\" invalid.", str_val_nosign))),
            fractional_part: 0,
            fractional_digits,
          })
        } else if str_num_digits > fractional_digits {
          if fractional_digits == 0 {
            Ok(FPNumber {
              negative,
              integer_part: unwrap_result_else_return_error!(str_val_nosign_split[0].parse::<u128>(), DecimalParseError(format!("Integer part of string \"{}\" invalid.", str_val_nosign))),
              fractional_part: 0u32,
              fractional_digits,
            })
          } else {
            let str_val_nosign_split_1_trimmed = str_val_nosign_split[1].substring(0, fractional_digits.into());
            
            Ok(FPNumber {
              negative,
              integer_part: unwrap_result_else_return_error!(str_val_nosign_split[0].parse::<u128>(), DecimalParseError(format!("Integer part of string \"{}\" invalid.", str_val_nosign))),
              fractional_part: unwrap_result_else_return_error!(str_val_nosign_split_1_trimmed.parse::<u32>(), DecimalParseError(format!("Fractional part of string \"{}\" invalid.", str_val_nosign))),
              fractional_digits,
            })
          }
        } else {
          Ok(FPNumber {
            negative,
            integer_part: unwrap_result_else_return_error!(str_val_nosign_split[0].parse::<u128>(), DecimalParseError(format!("Integer part of string \"{}\" invalid.", str_val_nosign))),
            fractional_part: unwrap_result_else_return_error!(str_val_nosign_split[1].parse::<u32>(), DecimalParseError(format!("Fractional part of string \"{}\" invalid.", str_val_nosign))) * 10u32.pow((fractional_digits - str_num_digits).into()),
            fractional_digits,
          })
        }
      } else {
        Ok(FPNumber {
          negative,
          integer_part: unwrap_result_else_return_error!(str_val_nosign.parse::<u128>(), DecimalParseError(format!("Integer part of string \"{}\" invalid.", str_val_nosign))),
          fractional_part: 0u32,
          fractional_digits,
        })
      }
    }
  }
  
  pub fn to_string(&self) -> String {
    match self {
      FPNumber { negative, integer_part, fractional_part, fractional_digits } => {
        if *fractional_digits == 0 {
          format!("{}{}", if *negative { "-" } else { "" }, integer_part)
        } else {
          format!("{}{}.{:0>3$}", if *negative { "-" } else { "" }, integer_part, fractional_part, *fractional_digits as usize)
        }
      },
      FPInfinite { negative, .. } => {
        format!("{}Infinity", if *negative { "-" } else { "" })
      }
    }
  }
}

pub fn advanced_sleep(duration: FixedPrec) {
  match duration {
    FPNumber { negative: true, .. } |
    FPInfinite { negative: true, .. } |
    FPNumber { integer_part: 0, fractional_part: 0, .. } => {
      return;
    },
    FPNumber { integer_part, fractional_part, fractional_digits, .. } => {
      if fractional_digits != 9 {
        unimplemented!("FixedPrec fractional_digits not 9");
      }
      
      if integer_part <= u64::MAX.into() {
        sleep(Duration::new(integer_part.try_into().expect("u128 should always be within u64 range in this code"), fractional_part));
      } else {
        let mut remaining_seconds = integer_part;
        
        while remaining_seconds > u64::MAX.into() {
          sleep(Duration::new(u64::MAX, 0));
          remaining_seconds -= u64::MAX as u128;
        }
        
        sleep(Duration::new(remaining_seconds.try_into().expect("u128 should always be within u64 range in this code"), fractional_part));
      }
    },
    FPInfinite { .. } => {
      loop {
        sleep(Duration::new(u64::MAX, 0));
      }
    },
  }
}
