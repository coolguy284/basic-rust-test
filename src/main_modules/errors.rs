use crate::libs::time_fixed_prec::FixedPrecStrParseError;
use crate::libs::time_fixed_prec::FixedPrecStrParseError::{DecimalParseError, HighPrecError};
use crate::main_modules::commands::current_time::CommandCurrentTimeError;
use crate::main_modules::commands::current_time::CommandCurrentTimeError::{TimezoneRetrievalError, TimezoneNotFoundError};

use CommandError::ArgInvalidError;

#[derive(Debug)]
pub enum CommandError {
  ArgInvalidError(String),
  ArgInvalidHexError(String),
  ArgInvalidNumberError(String),
  ArgNotSpecifiedError(String),
  InvalidCommandError(String),
  TimezoneRetrievalError(String),
  TimezoneNotFoundError(String),
}

impl From<FixedPrecStrParseError> for CommandError {
  fn from(err: FixedPrecStrParseError) -> Self {
    match err {
      DecimalParseError(x) | HighPrecError(x) => ArgInvalidError(x)
    }
  }
}

impl From<CommandCurrentTimeError> for CommandError {
  fn from(err: CommandCurrentTimeError) -> Self {
    match err {
      TimezoneRetrievalError(x) => CommandError::TimezoneRetrievalError(x),
      TimezoneNotFoundError(x) => CommandError::TimezoneNotFoundError(x),
    }
  }
}
