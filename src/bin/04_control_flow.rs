fn main() {
  println!("=== CONTROL FLOW ===\n");
  println!("\n=== IF ELSE ===\n");
  fizzbuzz();
  println!("\n=== MATCH ===\n");
  fizzbuzz_match();
}

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

fn fizzbuzz_match() {
  for num in 1..=33 {
    match (num % 3, num % 5) {
      (0, 0) => println!("fizzbuzz"),
      (0, _) => println!("fizz"),
      (_, 0) => println!("buzz"),
      _ => println!("{:}", num),
    }
  }
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
