use chio_rust::{get_music_instruments, Product}; // check lib.rs

fn product_list(products: &Vec<Product>) {
  for product in products {
    println!("name: {}, price: {}", product.name, product.price);
  }
}

fn main() {
  let instruments = get_music_instruments();
  product_list(&instruments);
}
