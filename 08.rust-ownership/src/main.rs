/**
 * 所有权与函数
 * Rust 的垃圾回收采取了一个不同的策略：内存在拥有它的变量离开作用域后就被自动释放。
 * 

 **/
fn main() {
    let s = String::from("hello world");
    owner(s);
    // 调用函数后，值被移动 ^ value borrowed here after move
    // println!("s value :{}",s);

    // 基本类型会自动 copy，所以可以继续使用
    let n = 123;
    makes_copy(n);
    println!("n value:{}", n);

    let a = 5;
    let b = a;

    println!("a = {}, b = {}", a, b);


    let x = get_hello_str();
    println!("x value:{}", x);

    let y = String::from("world");
    let z = get_hello_str2(y);
    println!("z value:{}", z);   
    // 所有权已经移交到 z,所以输出 y 会报错：^^^^^^ this parameter takes ownership of the value
    // println!("y value:{}", y);

    let a = String::from("world");
    let (a2, a1_len,value) = calc_length(a);
    println!("a1 value:{},al_len value:{},value:{}", a2, a1_len,value);
}

fn owner(s1: String) {
    println!("s1 lenght:{}", s1.len());
}

fn makes_copy(n1: i32) {
    println!("n1 value :{}", n1);
}

//  返回值也会转移所有权
fn get_hello_str() -> String {
    String::from("hello")
}

//  返回值也会转移所有权
fn get_hello_str2(ss: String) -> String {
    ss
}

// 通过返回元组，来获取传入的值和返回值
fn calc_length(a2: String) -> (String, usize,usize) {
    let length = a2.len();
    (a2, length,100)
}
