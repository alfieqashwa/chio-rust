pub struct Product {
  pub name: String,
  pub price: f64,
}

pub fn get_music_instruments() -> Vec<Product> {
  let guitar = Product {
    name: String::from("Guitar"),
    price: 134.66,
  };
  let piano = Product {
    name: String::from("Piano"),
    price: 454.90,
  };

  let drum = Product {
    name: String::from("Drum"),
    price: 888.88,
  };

  vec![guitar, piano, drum]
}
