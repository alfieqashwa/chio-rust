use chio_rust::Product;

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

  let new_product = Product {
    name: "milk shake".to_owned(),
    price: 450.00,
  };

  // list of products
  println!("\n=== List of Products: ===\n");
  Product::list_products(&products);

  // get total price
  let total_price = Product::total_price(&products);
  println!("\n=== Get Total Price: ===\n");
  println!("Total price is: {}", total_price);

  // show product name
  println!("\n=== Impl Product Name: ===\n");
  new_product.name();

  // Get product with minimum price
  println!("\n=== Minimum Price Product ===\n");
  let min_product = products
    .iter()
    .min_by(|a, b| a.price.partial_cmp(&b.price).unwrap())
    .unwrap(); // unwrap is used here for simplicity, but in production code, you should handle
               // the case where there are no products.
  println!(
    "Min Price Product: {}, Price: Rp{}",
    min_product.name, min_product.price
  );

  println!("\n=== Maximum Price Product ===\n");
  // Get product with maximum price
  // Safetier in concise way
  products
    .iter()
    .max_by(|a, b| {
      a.price
        .partial_cmp(&b.price)
        .unwrap_or(std::cmp::Ordering::Equal)
    })
    .map(|p| println!("Max Price Product: {}, Price: Rp{}", p.name, p.price));

  // Safetier but not concise.
  // let max_product = products.iter().max_by(|a, b| {
  //    a.price.partial_cmp_(&b.price).unwrap(std::cmp::Ordering::Equal)
  // )});
  //
  // if let Some(p) = max_product {
  //   println!("Max Price Product: {}, Price: Rp{}", p.name, p.price);
  // } else {
  //   println!("No products found!");
  // }
}
