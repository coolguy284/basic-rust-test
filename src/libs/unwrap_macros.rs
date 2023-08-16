#[macro_export]
macro_rules! unwrap_option_else_return_error {
  ($main_expr:expr, $error_expr:expr) => {
    match $main_expr {
      Some(x) => x,
      None => return Err($error_expr),
    }
  };
}

#[macro_export]
macro_rules! unwrap_result_else_return_error {
  ($main_expr:expr, $error_expr:expr) => {
    match $main_expr {
      Ok(x) => x,
      Err(_) => return Err($error_expr),
    }
  };
}

pub use unwrap_option_else_return_error;
pub use unwrap_result_else_return_error;
