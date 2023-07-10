mod if_else;
mod matches;

pub fn run() {
    println!("===== Example using IF ELSE control-flow =====");
    if_else::fizzbuzz();

    println!("===== Example using MATCH control-flow =====");
    matches::fizzbuzz();
}
