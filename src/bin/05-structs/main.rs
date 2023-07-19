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

#[cfg(test)]
mod test {
    // use crate::{products::Product, total_price};
    use super::*;

    #[test]
    fn test_total_price() {
        let products = vec![
            Product {
                name: String::from("juice"),
                price: 700.00,
            },
            Product {
                name: String::from("beer"),
                price: 400.00,
            },
        ];

        let result = total_price::run(&products);
        assert_eq!(result, 1100.00);
    }
}
