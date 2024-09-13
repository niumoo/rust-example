fn main() {
    let mut s = String::from("Hello");
    s.push_str(" world");
    print_string(s);
    // println!("s value is :{}",s);
    // 运行报错                  ^ value borrowed here after move

    let s2 = String::from("Rust");
    let _s3 = s2;
    // println!("s2 value:{}",s2);
        // ^^ value borrowed here after move

    println!("s3 value:{}",_s3);

    let s4 = String::from("hello");
    let s5 = s4.clone();

    println!("s4 = {}, s5 = {}", s4, s5);

    //  基本类型自动拷贝
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn print_string(str: String) {
    println!("{}", str);
}

