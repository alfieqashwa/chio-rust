/*
Compound types can group multiple values into one type.
Rust has two primitive compound types: tuples and arrays.
*/

pub fn run() {
  println!("=== COMPOUND TYPES ===");
  tuple_example();
  println!("===================");
  array_example();
}

/*
=== The Tuple Type ===
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
*/

fn tuple_example() {
  let tuple: (char, u8, bool, f32) = ('a', 1, true, 19.31);
  println!("tuple is {:?}", tuple);

  // destructuring
  let cars = ("BMW", "Audi", "Mercedes");
  let (bmw, audi, mercy) = cars;

  assert_eq!(bmw, "BMW");
  println!("first car is {}", bmw);

  assert_eq!(audi, "Audi");
  println!("second car is {}", audi);

  assert_eq!(mercy, "Mercedes");
  println!("third car is {}", mercy);

  let s = ("cello world", String::from("hello world"));
  println!(
    "string literal example: {}\nstring object example: {}",
    s.0, s.1
  );
}

/*
=== The Array Type ===
Unlike a tuple, every element of an array must have the same type.
Unlike arrays in some other languages, arrays in Rust have a fixed length.

Arrays are useful when you want your data allocated on the stack rather than the heap
or when you want to ensure you always have a fixed number of elements.
*/

fn array_example() {
  let months = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];
  println!("months: {:?}", months);
  println!("===================");
  println!("months in prettier output: {:#?}", months);
  println!("===================");

  let nums: [i32; 5] = [1, 2, 3, 4, 5];
  println!("nums: {:?}", nums);

  let a = [10; 5];
  println!("{:#?}", a);

  let vals = [6, 7, 8, 9, 10];
  println!("Vals: {:?}", vals);

  let eight = vals[2];
  assert_eq!(8, eight);

  // reference to a slice
  let six_to_nine = &vals[0..4];
  assert_eq!(six_to_nine, &[6, 7, 8, 9]);

  let six_til_nine = &vals[0..=4];
  assert_eq!(six_til_nine, &[6, 7, 8, 9, 10]);

  println!(
    "eight: {},\nsix_to_nine: {:?}\nsix_til_nine: {:?}",
    eight, six_to_nine, six_til_nine
  );

  let numbers = [1, 2, 3];
  for i in numbers.iter().rev() {
    println!("COUNTDOWN {:?}", i);
  }
  println!("HAPPY HOLIDAYS!");
}
