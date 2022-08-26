use std::io;
mod validator;
mod data;
mod runner;
use runner::runner;
fn main() {

  guide_and_greeting();

  let mut user_input = String::new();
  io::stdin()
    .read_line(&mut user_input)
    .expect("read line faild");
  match validator::user_input_validator(&user_input) {
      Ok(result) => { runner(result); },
      Err(err) => { eprintln!("{}", err) },
  };
}

fn guide_and_greeting() {
  println!("Sieve of Eratosthenes");
  println!("Enter the max and delay: (separated with '-') [e.g 100 - 100]");
}