fn main() {
    if_else_fizzbuzz();
    match_fizzbuzz();
}

fn if_else_fizzbuzz() {
    for num in 1..=100 {
        if num % (3 * 5) == 0 {
            println!("fizzbuzz")
        } else if num % 5 == 0 {
            println!("buzz")
        } else if num % 3 == 0 {
            println!("fizz")
        } else {
            println!("{}", num)
        }
    }
}

fn match_fizzbuzz() {
    for num in 1..=100 {
        match (num % 3, num % 5) {
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", num),
        }
    }
}
