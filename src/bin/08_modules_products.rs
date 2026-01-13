#[derive(Debug)]
#[allow(dead_code)]
pub struct Computer {
  pub name: String,
  pub price: i32,
}

fn main() {
  let computer_1 = Computer {
    name: "Macbook".to_string(),
    price: 1600,
  };

  println!("{:?}", computer_1)
}
