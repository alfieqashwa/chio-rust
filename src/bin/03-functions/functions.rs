pub fn run() {
    let (num1, num2) = (33, 19);

    let sum1 = add(num1, num2);
    println!("The sum of {} and {} is: {:?}", num1, num2, sum1);

    is_gt_50(sum1);

    let sum2 = minus(num1, num2);
    println!("The sum of {} minus {} is: {:?}", num1, num2, sum2);

    is_gt_50(sum2);

    assert_eq!(sum1, 52);
    assert_eq!(sum2, 14);
}

// u8 = 0..255
fn add(num1: u8, num2: u8) -> u8 {
    num1 + num2
}
fn minus(num1: u8, num2: u8) -> u8 {
    match num1 >= num2 {
        true => num1 - num2,
        false => num2 - num1,
    }
}

// is greater than 50 ?
fn is_gt_50(num: u8) {
    if num > 50 {
        println!("The sum is: {} an it's greater than 50", num)
    } else {
        println!("The sum is: {} and it's less than 50", num)
    }
}
