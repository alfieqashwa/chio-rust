fn fizzbuzz() {
  for num in 1..=33 {
    if num % (3 * 5) == 0 {
      println!("fizzbuzz")
    } else if num % 5 == 0 {
      println!("buzz")
    } else if num % 3 == 0 {
      println!("fizz")
    } else {
      println!("{}", num)
    }
  }
}

fn main() {
  fizzbuzz();
}

#[test]

fn test_if_in_a_let_statement() {
  let condition = true;
  let number = if condition { 5 } else { 6 };
  assert_eq!(number, 5);

  // must have the same type
  //     if not, the panic will occur.
  //
  //     e.g:
  //     let number: u8 = if condition { "five" } else { "six" };
}
