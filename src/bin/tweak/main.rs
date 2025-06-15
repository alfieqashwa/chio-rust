fn main() {
  println!("Chio universe!");

  fizzbuzz();

  println!("====== fizzbuzz ======");
  match_fizzbuzz();
}

fn fizzbuzz() {
  for num in 1..=30 {
    if num % (5 * 3) == 0 {
      println!("fizzbuzz")
    } else if num % 3 == 0 {
      println!("fizz")
    } else if num % 5 == 0 {
      println!("buzz")
    } else {
      println!("{}", num)
    }
  }
}

fn match_fizzbuzz() {
  for num in 1..=30 {
    match (num % 3, num % 5) {
      (0, 0) => println!("fizzbuzz"),
      (0, _) => println!("fizz"),
      (_, 0) => println!("buzz"),
      _ => println!("{num}"),
    }
  }
}
