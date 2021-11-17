use std::io::BufRead;

use std::io;
mod validator;
mod data;
fn main() {

  guide_and_greeting();
  let mut user_input = String::new();
  io::stdin()
    .read_line(&mut user_input)
    .expect("read line faild");
  match validator::user_input_validator(&user_input) {
      Ok(_) => {},
      Err(_) => {},
  };
}

fn guide_and_greeting() {
  println!("Sieve of Eratosthenes");
  println!("Enter the range: (separated with '-') [e.g 1 - 60]:");
}