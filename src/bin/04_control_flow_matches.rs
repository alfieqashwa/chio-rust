fn fizzbuzz() {
  for num in 1..=33 {
    match (num % 3, num % 5) {
      (0, 0) => println!("fizzbuzz"),
      (0, _) => println!("fizz"),
      (_, 0) => println!("buzz"),
      _ => println!("{}", num),
    }
  }
}

fn main() {
  fizzbuzz();
}
