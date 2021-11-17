use std::time::Duration;

pub struct UserInput {
  pub start: i32,
  /// end is included
  pub end: i32,
  pub delay: Option<Duration>,
}

pub enum SOEErrors {
  IvalidRange,
  NotANumber,
}