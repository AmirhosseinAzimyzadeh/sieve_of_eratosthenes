use std::{fmt, time::Duration};

pub struct UserInput {
  pub start: u32,
  /// end is included
  pub end: u32,
  pub delay: Duration,
}

pub enum SOEErrors {
  NotEnoughNumbers,
  InvalidRange,
  NotANumber,
}

impl fmt::Display for SOEErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
          SOEErrors::NotANumber => write!(f, "Not a number !"),
          SOEErrors::InvalidRange => write!(f, "Invalid range !"),
          SOEErrors::NotEnoughNumbers => write!(f, "Not enough arguments !"),
        }
    }
}