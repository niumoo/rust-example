//  常量
const PI_3: f64 = 3.1415;

fn main() {
    println!("Hello, world!");

    // 变量
    // mut 可以让变量可变
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("The const value is :{}", PI_3);

    // 隐藏 Shadowing
    let y = 1;
    let y = y + 1;
    println!("The value of y is {}", y);

    // shadow 和 mut 的区别 - shadow 可以改变类型
    let str = "abc";
    let str = str.len();
    println!("The value of str is {}", str);

    main2();
    main3();
}


fn main2() {
    // 类型转换成指定类型
    let number: i32 = "111".trim().parse().expect("not a number");
    println!("convert to number: {}", number);

    // 整数类型（代码规范，以 _ 开头）
    let _num0: i8 = 1;
    let _num1: i16 = 1;
    let _num2: i32 = 1;
    let _num3: i64 = 1;
    let _num4: i128 = 1;
    let _num5: isize = 1;
    // 整数的其他标识
    let _num6: i32 = 123_456; // 10 进制
    let _num7: i32 = 0x123; // 十六进制
    let _num8: i32 = 0o123; // 8 进制
    let _num9: i32 = 0b111_000; // 二进制
    let _num10: u8 = b'A'; //Byte字符
                           // 整数溢出, build 报错
                           // let _num11:u8 = 11;

    //  浮点类型
    let _fnum = 2.0;
    let _fnum: f32 = 3.14;
    let _fnum: f64 = 3.14;

    // 操作 = - x /
    let _res1 = 1 + 1;
    let _res2 = 12 * 2;
    let _res3 = 10.3 / 2.3;
    let _res4 = 10 % 3;

    // 布尔类型 true /false ,占用一个字节
    let _t = true;
    let _f: bool = false;

    // 字符类型，4个字节
    let _a = 'A';
    let _b = '你';
    let _c = '😯';
    println!("字符表情:{}", _c);
}

fn main3(){

}