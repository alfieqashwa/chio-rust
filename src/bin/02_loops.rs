fn main() {
  loop_example();
  while_loop_example();
  for_loop_example();
}

fn loop_example() {
  println!("=== Loop example ===\n");

  let mut counter = 0;
  let result = loop {
    counter += 1;
    println!("{counter}"); // another syntax

    if counter == 10 {
      break counter * 2;
    };
  };

  println!("\nThe result is {}", result);
}

fn while_loop_example() {
  println!("\n=== While loop example ===\n");
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("\nLIFTOFF!!!\n");
}

fn for_loop_example() {
  println!("\n=== For-in loop example ===\n");
  for element in (1..4).rev() {
    println!("{}!", element);
  }
  println!("\nLIFTOFF!!!\n");
}
