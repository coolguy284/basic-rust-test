use std::env;
use chrono::{Utc, Local, TimeZone, DateTime, Offset};

fn main() {
  let cmd_line_args: Vec<String> = env::args().collect();
  
  if cmd_line_args.len() == 1 {
    println!("Coolguy284's basic rust experimentation program.");
    println!();
    println!("USAGE:");
    println!("    basic-rust-test <SUBCOMMAND>");
    println!();
    println!("SUBCOMMANDS:");
    println!("    current_time");
    println!();
    println!("DISCUSSION:");
    println!("    Just a collection of random commands to do random things in rust.");
  } else {
    let arg_1 = cmd_line_args[1].as_str();
    match arg_1 {
      "current_time" => {
        let now_utc = Utc::now();
        
        let now_local: DateTime<Local> = DateTime::from(now_utc);
        
        // i don't even know bro https://stackoverflow.com/questions/59603665/how-do-you-find-the-local-timezone-offset-in-rust/59603899#59603899
        let local_offset = Local.timestamp_opt(0, 0).single().unwrap().offset().fix().local_minus_utc();
        
        let local_offset_is_positive = local_offset >= 0;
        let local_offset_absolute = local_offset.abs();
        
        let local_tz_str = iana_time_zone::get_timezone().unwrap();
        
        println!("Current time, written in many ways:");
        println!();
        println!("Human readable:");
        println!("  Current Local time is {} {}", now_local.format("%A, %B %d, %Y, %I:%M:%S.%9f %p %z"), local_tz_str);
        println!("  Current UTC time is   {}", now_utc.format("%A, %B %d, %Y, %I:%M:%S.%9f %p %z %Z"));
        println!();
        println!("Technical:");
        println!("  Current Local time is {}", now_local.format("%Y-%m-%dT%H:%M:%S.%9f"));
        println!("  Current UTC time is   {}", now_utc.format("%Y-%m-%dT%H:%M:%S.%9fZ"));
        println!("  Local time offset is  {}{}:{:0>2} ({})", if local_offset_is_positive { "" } else { "-" }, local_offset_absolute / 3600, local_offset_absolute / 60 % 60, local_tz_str);
      },
      _ => {
        println!("Invalid argument {}, for list of arguments run \"basic-rust-test\" with no arguments", arg_1);
      },
    }
  }
}
