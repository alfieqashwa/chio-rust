use super::Product;

pub fn run(products: &Vec<Product>) -> f64 {
    let mut total = 0.00;
    for product in products {
        total += product.price
    }
    total
}
