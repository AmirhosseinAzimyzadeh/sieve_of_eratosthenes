use std::{time::Duration};

use crate::data::{ UserInput, SOEErrors };
const DEFAULT_DELAY: u32 = 0;

pub fn user_input_validator(input: &str) -> Result<UserInput, SOEErrors> {
  let normalized = input.trim().replace(" ", "");
  let values: Vec<&str> = normalized.split('-').collect();

  if values.len() < 2 || values.contains(&"") {
    return Err(SOEErrors::NotEnoughNumbers);
  }

  let mut numbers: Vec<u32> = Vec::with_capacity(3);

  for string in values.into_iter() {
    match string.parse() {
        Ok(converted_value) => { numbers.push(converted_value); },
        Err(_) => { return Err(SOEErrors::NotANumber); },
    };
  }

  if numbers.len() == 2 { numbers.push(DEFAULT_DELAY); }

  // if numbers[0] > numbers[1] {
  //   return Err(SOEErrors::InvalidRange);
  // }

  return Result::Ok(
    UserInput{
      start: 1,
      end: numbers[0],
      delay: Duration::from_millis(numbers[1] as u64),
    });
}