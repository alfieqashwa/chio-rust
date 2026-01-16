fn fullname(firstname: &str, lastname: &str) -> String {
  format!("{} {}", firstname, lastname) // format!() will create String
}

fn main() {
  let cello_firstname = String::from("Cello");
  let cello_lastname = "El Zhafran".to_string();

  let cello_fullname = fullname(&cello_firstname, &cello_lastname);

  assert_eq!(cello_fullname.to_uppercase(), "CELLO EL ZHAFRAN");

  let cello = "cello";

  // Fold way
  // let total_bytes: u32 = cello
  //   .as_bytes()
  //   .iter()
  //   .fold(0, |acc, &byte| acc + byte as u32);

  let total_bytes: u32 = cello.as_bytes().iter().map(|&b| b as u32).sum();

  assert_eq!(total_bytes, 527);
  assert_eq!(cello.as_bytes(), [99, 101, 108, 108, 111]);

  println!("{:?}", cello.chars());
}
