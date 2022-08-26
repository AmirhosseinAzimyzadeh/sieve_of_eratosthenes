use crate::data::{ UserInput, Number };
use std::{ thread, time };

pub fn runner(user_input: UserInput) {
  let mut numbers = create_numbers(user_input.start,user_input.end);

  for i in 0..numbers.len() {
    let (current_number, other_numbers) =
      numbers.split_one_mut(i);
    if current_number.value == 1 {
      current_number.is_prime = Some(false);
      continue;
    }
    if current_number.is_prime == None {
      current_number.is_prime = Some(true);
    }
    for number in other_numbers {
      if number.value % current_number.value == 0 {
        number.is_prime = Some(false);
      }
    }
    thread::sleep(user_input.delay);
    render(&numbers);
  }
}

fn create_numbers(start: u32, end: u32) -> Vec<Number> {
  let mut result: Vec<Number> = Vec::with_capacity((end - start) as usize);
  for i in start..end + 1 {
    result.push(Number{
      is_prime: None,
      value: i,
    })
  }
  return result;
}


fn render(numbers: &[Number]) {
  // clear terminal
  print!("{esc}c", esc = 27 as char);
  // print numbers
  let mut element_counter = 0;
  for number in numbers {
    number.print();
    print!("\t");
    element_counter += 1;
    if element_counter % 6 == 0 { println!(""); }
  }
}

// slice mut

type ImplIteratorMut<'a, Item> =
    ::std::iter::Chain<
        ::std::slice::IterMut<'a, Item>,
        ::std::slice::IterMut<'a, Item>,
    >
;
trait SplitOneMut {
  type Item;

  fn split_one_mut (
      self: &'_ mut Self,
      i: usize,
  ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>);
}

impl<T> SplitOneMut for [T] {
  type Item = T;
  
  fn split_one_mut (
      self: &'_ mut Self,
      i: usize,
  ) -> (&'_ mut Self::Item, ImplIteratorMut<'_, Self::Item>)
  {
      let (prev, current_and_end) = self.split_at_mut(i);
      let (current, end) = current_and_end.split_at_mut(1);
      (
          &mut current[0],
          prev.iter_mut().chain(end),
      )
  }
}