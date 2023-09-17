fn main() {
    method_1();

    let x = 3.14;
    method_2(x);

    let res = method_3();
    print!("method_3 return {}", res);
    
    println!("3 add 1 ,result :{}", add_1(3))
}
// 简单函数
fn method_1() {
    println!("this is method 1");
}
// 有参数的函数
fn method_2(s: f64) {
    println!("the value is :{}", s);
}
// 函数返回值，默认最后一行为返回值,箭头制定返回类型
fn method_3() -> i32 {
    100
}

// 函数入参和返回值计算
fn add_1(x: i32) -> i32 {
    x + 1
}
