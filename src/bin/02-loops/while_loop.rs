pub fn run() {
  println!("While loop example");
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("LIFTOFF!!!");
}
