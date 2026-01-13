use chio_rust::{get_music_instruments, Product}; // check lib.rs

fn total(products: &Vec<Product>) -> f64 {
  let mut total = 0.00;
  for product in products {
    total += product.price
  }
  total
}

fn main() {
  let instruments = get_music_instruments();
  total(&instruments);
}
