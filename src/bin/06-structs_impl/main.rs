struct Product {
    name: String,
    price: f64,
}

impl Product {
    fn list_products(products: &[Product]) {
        for product in products {
            println!("name: {}, price: {}", product.name, product.price);
        }
    }
    fn total_price(products: &[Product]) -> f64 {
        let mut total = 0.00;
        for product in products {
            total += product.price;
        }
        return total;
    }
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

    println!("========== List of Products ==========");
    Product::list_products(&products);

    let total_price = Product::total_price(&products);

    println!("========== Get Total Price ==========");
    println!("Total price is: {}", total_price);

    println!("========== Impl Product Name ==========");
    product.name()
}
