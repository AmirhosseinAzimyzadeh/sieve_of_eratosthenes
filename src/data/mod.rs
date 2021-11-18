use std::{
  fmt,
  time::Duration,
  io:: { stdout }
};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

pub struct UserInput {
  pub start: u32,
  /// end is included
  pub end: u32,
  pub delay: Duration,
}

pub struct Number {
  pub is_prime: Option<bool>,
  pub value: u32,
}

impl Number {
  pub fn print(&self) {
    match self.is_prime {
      None => {

      execute!(
        stdout(),
        SetForegroundColor(Color::White),
        Print(format!("{}", self.value)),
        ResetColor
      ).unwrap();

      } Some(is_prime) => {
        if is_prime {

        execute!(
          stdout(),
          SetForegroundColor(Color::Green),
          Print(format!("{}", self.value)),
          ResetColor
        ).unwrap();

        } else {
          
        execute!(
          stdout(),
          SetForegroundColor(Color::Red),
          Print(format!("{}", self.value)),
          ResetColor
        ).unwrap();

        }
      }
    }
  }
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