fn main() {
  // variables are immutable by default
  let im = 4;
  println!("Variable im is {}", im);

  // add mut before the name of variable to make it mutable
  let mut x = 5;
  assert_eq!(x, 5);
  println!("The value of x is: {}", x);

  x = 6;
  assert_eq!(x, 6);
  println!("The value of x is: {}", x);

  // Constants aren’t just immutable by default—they’re always immutable.
  const MAX_POINTS: u32 = 100_000;
  assert_eq!(MAX_POINTS, 100_000);
  println!("The value of MAX_POINTS is: {}", MAX_POINTS);

  const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
  assert_eq!(THREE_HOURS_IN_SECONDS, 3 * 60 * 60);
  println!(
    "The value of THREE_HOURS_IN_SECONDS is: {}",
    THREE_HOURS_IN_SECONDS
  );

  // Shadowing
  let y = 5;
  let y = y * 2;
  assert_eq!(y, 10);
  println!("The value of y is: {}", y);

  // Shadowing can change types
  let _s = "hello"; // add '_' to ignore the unused variable warning
  let s = 3;
  println!("The value of s is: {}", s);

  /*
   *  By shadowing, you don't need to think to create new var
   *  eg. let cello_len = cello.len()
   */
  let cello = "Cello";
  let cello = cello.len(); // usize is an alias for u32
  assert_eq!(cello, 5);
  println!("The value of space is: {}", cello);

  let b = false;
  assert!(!b);
}
