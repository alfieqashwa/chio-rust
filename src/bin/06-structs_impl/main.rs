struct Product {
  name: String,
  price: f64,
}

impl Product {
  // type: Vec<Product>
  fn list_products(products: &Vec<Product>) {
    for product in products {
      println!("Name: {}, Price: {}", product.name, product.price);
    }
  }
  // type: [Product]
  fn total_price(products: &[Product]) -> f64 {
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
  fn name(&self) {
    println!(
      "Name of product is: {}, and the length is: {}",
      self.name,
      self.name.len()
    )
  }
}

fn main() {
  let products = vec![
    Product {
      name: "coffee".to_owned(),
      price: 300.00,
    },
    Product {
      name: "iced lemon".to_owned(),
      price: 500.00,
    },
    Product {
      name: "mineral water".to_owned(),
      price: 200.00,
    },
  ];

  let product = Product {
    name: "milk shake".to_owned(),
    price: 450.00,
  };

  println!("========== List of Products: ==========");
  Product::list_products(&products);

  let total_price = Product::total_price(&products);

  println!("========== Get Total Price: ==========");
  println!("Total price is: {}", total_price);

  println!("========== Impl Product Name: ==========");
  product.name();

  // Get product with minimum price
  let min_product = products
    .iter()
    .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    .unwrap(); // unwrap is used here for simplicity, but in production code, you should handle
               // the case where there are no products.

  // Get product with maximum price
  let max_product = products
    .iter()
    .max_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    .unwrap();

  println!("========== Min & Max Price Product ==========");

  println!(
    "Min Price Product: {}, Price: Rp{}",
    min_product.name, min_product.price
  );
  println!(
    "Max Price Product: {}, Price: Rp{}",
    max_product.name, max_product.price
  );
}
