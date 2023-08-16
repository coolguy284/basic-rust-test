use crate::libs::time_fixed_prec::FixedPrec;
use crate::libs::time_fixed_prec::FixedPrec::{FPInfinite, FPNumber};
use crate::libs::time_fixed_prec::FixedPrecStrParseError::HighPrecError;


#[test]
fn test_fixed_prec_precision_overflow() {
  assert!(matches!(
    FixedPrec::from_str("", 10),
    Err(HighPrecError(x)) if x == "fractional_digits of 10 cannot fit unto u32 type".to_string()
  ));
}


#[test]
fn test_fixed_prec_empty_string() {
  assert_eq!(
    FixedPrec::from_str("", 9).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 0u128,
      fractional_part: 0u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_infinite_positive() {
  assert_eq!(
    FixedPrec::from_str("Forever", 9).unwrap(),
    FPInfinite {
      negative: false,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_infinite_negative() {
  assert_eq!(
    FixedPrec::from_str("-Forever", 9).unwrap(),
    FPInfinite {
      negative: true,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_decimal_positive() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.576", 9).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 576000000u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_decimal_negative() {
  assert_eq!(
    FixedPrec::from_str("-352641836249782037739009.576", 9).unwrap(),
    FPNumber {
      negative: true,
      integer_part: 352641836249782037739009u128,
      fractional_part: 576000000u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_decimal_max_prec() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.576128223", 9).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 576128223u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_decimal_over_precision() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.576128223121", 9).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 576128223u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_integer() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009", 9).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 0u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_integer_decimal_point() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.", 9).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 0u32,
      fractional_digits: 9u8,
    }
  );
}

#[test]
fn test_fixed_prec_integer_type_given_integer() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009", 0).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 0u32,
      fractional_digits: 0u8,
    }
  );
}

#[test]
fn test_fixed_prec_integer_type_given_decimal() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.576", 0).unwrap(),
    FPNumber {
      negative: false,
      integer_part: 352641836249782037739009u128,
      fractional_part: 0u32,
      fractional_digits: 0u8,
    }
  );
}


#[test]
fn test_fixed_prec_to_string_empty_string() {
  assert_eq!(
    FixedPrec::from_str("", 9).unwrap().to_string(),
    "0.000000000"
  );
}

#[test]
fn test_fixed_prec_to_string_infinite_positive() {
  assert_eq!(
    FixedPrec::from_str("Forever", 9).unwrap().to_string(),
    "Infinity"
  );
}

#[test]
fn test_fixed_prec_to_string_infinite_negative() {
  assert_eq!(
    FixedPrec::from_str("-Forever", 9).unwrap().to_string(),
    "-Infinity"
  );
}

#[test]
fn test_fixed_prec_to_string_decimal_positive() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.576", 9).unwrap().to_string(),
    "352641836249782037739009.576000000"
  );
}

#[test]
fn test_fixed_prec_to_string_decimal_negative() {
  assert_eq!(
    FixedPrec::from_str("-352641836249782037739009.576", 9).unwrap().to_string(),
    "-352641836249782037739009.576000000"
  );
}

#[test]
fn test_fixed_prec_to_string_decimal_max_precision() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009.576128223", 9).unwrap().to_string(),
    "352641836249782037739009.576128223"
  );
}

#[test]
fn test_fixed_prec_to_string_integer() {
  assert_eq!(
    FixedPrec::from_str("352641836249782037739009", 9).unwrap().to_string(),
    "352641836249782037739009.000000000"
  );
}
