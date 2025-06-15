use super::Product;

pub fn run(products: &Vec<Product>) {
  for product in products {
    println!("name: {}, price: {}", product.name, product.price);
  }
}
