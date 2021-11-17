use std::{thread, time::Duration};

use crate::data::{ UserInput, SOEErrors };

pub fn user_input_validator(input: &str) -> Result<UserInput, SOEErrors> {
  println!("User Input is {}", input);
  return Result::Ok(
    UserInput{
      start: 12,
      end: 3,
      delay: Some(Duration::from_millis(200)),
    });
}