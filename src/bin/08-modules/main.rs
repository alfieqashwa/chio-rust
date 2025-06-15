mod products;
fn main() {
  let lenovo = products::Computer {
    name: "lenovo".to_string(),
    price: 16_000_000,
  };

  let dell = products::Computer {
    name: String::from("Dell"),
    price: 32_000_000,
  };

  println!("{:#?}", lenovo);
  println!("{:#?}", dell);
}
