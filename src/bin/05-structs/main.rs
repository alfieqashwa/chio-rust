/*
  This example is using:
  - struct,
  - vector,
  - functions,
  - for_in loop,
  - borrowed checker,
  - mut,
  - pub,
  - mod,
  - use,
  - super

  cli:
  - cr -q --bin 05-structs
*/

mod list_products;
mod products;
mod total_price;

use products::Product;

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
            name: String::from("mineral water"),
            price: 200.00,
        },
    ];

    println!("========== List of Products ==========");
    list_products::run(&products);

    let total_price = total_price::run(&products);

    println!("========== Get Total Price ==========");
    println!("Total Price: {}", total_price);
}
