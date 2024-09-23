fn main() {
    //  if else
    rust_if();
    rust_loop();
    rust_while();
    rust_for();
}

fn rust_if() {
    let x = 10;
    if x == 1 {
        println!("x == 1");
    } else {
        println!("x != 1");
    }

    if x % 2 == 0 {
        println!("x % 2 == 0");
    } else if x % 2 == 1 {
        println!("x % 2 == 1");
    }

    let y = if x % 2 == 0 { "even" } else { "odd" };
    println!("The value of y is {}", y);
}

fn rust_loop() {
    // loop 3
    let mut x = 1;
    loop {
        x = x + 1;
        if x == 3 {
            break;
        }
    }
    println!("the loop result, x:{}", x);

    let mut y: i32 = 1;
    y = loop {
        y = y + 1;
        if y == 2 {
            break y * 2;
        }
    };
    println!("the loop result, y:{}", y);
}

fn rust_while() {
    println!("---------while--------");
    // 3 秒倒计时
    let mut second = 3;
    while second != 0 {
        println!("{second}");
        second = second - 1;
    }
    println!("liftoff!");
    // 遍历数组
    let array = [1, 2, 3, 4, 5];
    let mut index = 0;
    let size = array.len();
    while index < size {
        println!("The value of array,index:{},value:{}", index, array[index]);
        index += 1;
    }
}

fn rust_for() {
    println!("---------for--------");

    // for 循环, in array 和 in array.iter 的区别，iter 后 temp 为元素的引用
    let array = [1, 2, 3, 4, 5, 6];

    // for 遍历
    for temp in array {
        println!("for temp in array ,value:{}", temp);
    }

    //  for range 遍历
    for index in 0..6 {
        println!("index in (0..6), index:{},value:{}", index, array[index]);
    }

    //  for range 反转
    for index in (0..6).rev() {
        println!(
            "index in (0..6).rev(), index:{},value:{}",
            index, array[index]
        );
    }
}
