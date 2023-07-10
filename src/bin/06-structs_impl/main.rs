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
        let mut total = 0.00;
        for product in products {
            total += product.price;
        }
        return total;
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

    let min_price = 250.00;
    let max_price = 550.00;

    let filtered_products: Vec<&Product> = products
        .iter()
        .filter(|product| product.price >= min_price && product.price <= max_price)
        .collect();

    println!("========== Filtered Product: ==========");
    for product in filtered_products {
        println!("Name: {}, Price: {}", product.name, product.price);
    }
}
