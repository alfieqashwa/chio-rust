#[derive(Debug)]
pub struct Product {
  pub name: String,
  pub price: f64,
}

impl Product {
  // type: Vec<Product>
  pub fn list_products(products: &Vec<Product>) {
    for product in products {
      println!("Name: {}, Price: {}", product.name, product.price);
    }
  }
  // type: [Product]
  pub fn total_price(products: &[Product]) -> f64 {
    // approach 1: using a for loop

    // let mut total = 0.00;
    // for product in products {
    //     total += product.price;
    // }
    // return total;

    // approach 2: using iter and map
    // products.iter().map(|product| product.price).sum()

    // approach 3: using fold
    products
      .iter()
      .fold(0.0, |total, product| total + product.price)
  }
  // use &self
  pub fn name(&self) {
    println!(
      "Name of product is: {}, and the length is: {}",
      self.name,
      self.name.len()
    )
  }
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
