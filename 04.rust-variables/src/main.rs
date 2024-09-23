fn main() {
    println!("Hello, world!");
    var();
    var_const();
    var_shadowing();
    var_type();
    var_num_calc();
    var_tuple();
    var_array();
}

fn var() {
    let x = 5;
    println!("The value of x is {}", x);
    // cannot mutate immutable variable `x`
    // x = 6;
    // 在 Rust 中，变量默认是不可变的。如果想要变量可变，需要使用 mut
    let mut y = 10;
    y = 20;
    println!("The value of y is {}", y);
}
/**
 * 常量的定义
 */
fn var_const() {
    const PI: f64 = 3.1415926;
    println!("The value of pi is {}", PI);
}

/**
 * 变量的隐藏，或者叫变量的遮蔽,变量名可以重复定义使用，且类型可以不同，被遮蔽的变量自动回收。
 * 问题：变量隐藏和 mut 变量的区别
 */
fn var_shadowing() {
    let str: &str = "hello rust";
    let str = str.len();
    println!("The value of str is {}", str);
}

// 变量的数据类型
fn var_type() {
    // 类型转换成指定类型
    let number: i32 = "111".trim().parse().expect("not a number");
    println!("convert to number: {}", number);

    // 整数类型（代码规范，以 _ 开头）
    let num0: i8 = 1;
    let num1: i16 = 1;
    let num2: i32 = 1;
    let num3: i64 = 1;
    let num4: i128 = 1;
    let num5: isize = 1;
    // 整数的其他标识
    let num6: i32 = 123_456; // 10 进制
    let num7: i32 = 0x123; // 十六进制
    let num8: i32 = 0o123; // 8 进制
    let num9: i32 = 0b111_000; // 二进制
    let num10: u8 = b'A'; //Byte字符
                          // 整数溢出, build 报错
                          // let num11:u8 = 11;

    //  浮点类型
    let fnum = 2.0;
    let fnum: f32 = 3.14;
    let fnum: f64 = 3.14;

    // 操作 = - x /
    let res1 = 1 + 1;
    let res2 = 12 * 2;
    let res3 = 10.3 / 2.3;
    let res4 = 10 % 3;

    // 布尔类型 true /false ,占用一个字节
    let t = true;
    let f: bool = false;

    // 字符类型，4个字节
    let a = 'A';
    let b = '你';
    let c = '😯';
    println!("字符表情:{}", c);
}

/**
 * 数字的计算
 */
fn var_num_calc() {
    let sum = 10 + 10;
    let diff = 99.5 - 22.2;
    let product = 10 * 20;
    let quotient = 100 / 5;
    let remainder = 10 % 3;
    println!("The value of remainder is {}", remainder)
}

/**
 * 复合数据类型：元组
 */
fn var_tuple() {
    let tuple: (i32, f64) = (100, 3.14);
    println!("The value of tuple 1 is {}", tuple.0);
    let (x, y) = tuple;
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

/**
 * 复合数据类型：数组
 */
fn var_array() {
    let array = [1, 2, 3, 4, 5, 6, 7];
    println!("array value 0 :{}", array[0]);

    // 定义数组同时指定类型和大小
    let array2: [&str; 2] = ["one", "two"];
    // ^^^^^^^^^ index out of bounds: the length is 2 but the index is 3
    // println!("array2 value 0 :{}",array2[3]);
    println!("array2 value 0 :{}",array2[0]);
}
