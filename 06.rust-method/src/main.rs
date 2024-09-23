fn main() {
    let x = add_one(10);
    println!("The value of x is {}", x);
    let y = is_even_or_odd(10);
    println!("The value of y is {}", y);

    let z = { x + 1 };
    println!("The value of z is {}", z);
}

/**
 * 数字加1
*/
fn add_one(num: i32) -> i32 {
    num + 1
}

/**
 * 判断一个数字说奇数还是偶数
 */
fn is_even_or_odd(num: i32) -> u8 {
    if num % 2 == 0 {
        return 0;
    } else {
        return 1;
    }
}
