pub fn main() {
    let sum = add(19, 33);
    println!("The sum is: {:?}", sum);

    is_gt_50(sum);

    assert_eq!(sum, 52);
}

// u8 = 0..255
fn add(num1: u8, num2: u8) -> u8 {
    num1 + num2
}

// is greater than 50 ?
fn is_gt_50(num: u8) {
    if num > 50 {
        println!("The sum is: {} an it's greater than 50", num)
    } else {
        println!("The sum is: {} and it's less than 50", num)
    }
}
