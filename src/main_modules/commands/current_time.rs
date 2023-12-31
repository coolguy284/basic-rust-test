use chrono::{DateTime, Local, Offset, TimeZone, Utc};
use chrono_tz::{OffsetName, Tz};
use iana_time_zone::get_timezone;

#[cfg(debug_assertions)]
use crate::libs::debug_funcs::print_type_of;
use crate::libs::unwrap_macros::{unwrap_option_else_return_error, unwrap_result_else_return_error};

use CommandCurrentTimeError::{TimezoneRetrievalError, TimezoneNotFoundError};

pub enum CommandCurrentTimeError {
  TimezoneRetrievalError(String),
  TimezoneNotFoundError(String),
}

pub fn command_current_time() -> Result<(), CommandCurrentTimeError> {
  let now_utc = Utc::now();
  
  let now_local: DateTime<Local> = DateTime::from(now_utc);
  
  // i don't even know bro https://stackoverflow.com/questions/59603665/how-do-you-find-the-local-timezone-offset-in-rust/59603899#59603899
  let local_pre_fixed_offset = unwrap_option_else_return_error!(Local.timestamp_opt(0, 0).single(), TimezoneRetrievalError("Error getting timezone, first part".to_string()));
  let local_fixed_offset = local_pre_fixed_offset.offset();
  let local_offset_secs = local_fixed_offset.fix().local_minus_utc();
  
  let local_offset_secs_is_positive = local_offset_secs >= 0;
  let local_offset_secs_absolute = local_offset_secs.abs();
  
  let local_tz_str = unwrap_result_else_return_error!(get_timezone(), TimezoneRetrievalError("Error getting timezone, second part".to_string()));
  let local_tz: Tz = unwrap_result_else_return_error!(local_tz_str.parse(), TimezoneNotFoundError("Timezone not in list of timezones".to_string()));
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
  
  Ok(())
}
