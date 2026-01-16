fn fullname(firstname: &str, lastname: &str) -> String {
  format!("{} {}", firstname, lastname) // format!() will create String
}

fn main() {
  let cello_firstname = String::from("Cello");
  let cello_lastname = "El Zhafran".to_string();

  let cello_fullname = fullname(&cello_firstname, &cello_lastname);

  assert_eq!(cello_fullname.to_uppercase(), "CELLO EL ZHAFRAN");

  let cello = "cello";

  println!("{:?}", cello.chars());
  println!("{:?}", cello.as_bytes());
}
