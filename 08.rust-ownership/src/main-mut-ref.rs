/**
 * 通过 mut 定义变量，&mut 传入可变引用，&mut 接收可变引用，
 * 这样可以更改变量而不会获取所有权
 * 
 * */ 
 fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    change(&mut s);
    println!("s value :{}",s);

    // 可变引用不能同时被多个地方持有
    // 这一限制以一种非常小心谨慎的方式允许可变性，防止同一时间对同一数据存在多个可变引用
    let mut _s = String::from("hello");
    // let r1 = &mut _s;
    // let r2 = &mut _s;
    // println!("{}, {}", r1, r2);

    // 虽然可变引用不能同时被多个地方持有，但是如果一个持有生命周期已经结束，
    // 那么是可以被新变量持有的。
    let mut _s = String::from("hello rust");
    {
        let _r1 = &mut _s;
    }
    let _r2 = &mut _s;
    println!("{}", _r2);

    // 也不能在拥有不可变引用的同时拥有可变引用。
    // 不然可能会导致一个不可变引用突然因为可变引用的存在，而突然内容被更改。这无法控制。
    // 但是多个不可变引用是可以的，这样都是读取没有问题。
    // 所以下面这样会报错
    let mut _s = String::from("hello rust");
    let _r0 = &_s;
    let _r1 = &_s;
    // let _r2 = &mut _s;
    // println!("r1 {}, r2 {}", _r1, _r2);
    // r1 --- immutable borrow occurs here
    // r2 --- mutable borrow occurs here

    // 但是记住，一个变量的作用域持续到它最后一次被使用的地方。
    // 所以，只要没有在某一刻，不可变引用可可变引用同时存在就没有问题。
    let mut _s = String::from("hello rust");
    let _r0 = &_s;
    let _r1 = &_s;
    let _r2 = &mut _s;
    println!("_r2: {}",_r2);

    // 垂悬引用，如果一个变量的内存已经释放，但是引用还在，这样一个错误的引用会有问题。
    // 但是 Rust 会在编译时就检查这种情况，规避这种问题。
    let _s = dangle();
    
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 这个函数返回了一个引用，只是是一个引用，离开这个函数后所有权将不复存在，这有问题。
// ^ expected named lifetime parameter
fn dangle()-> String{ 
    let s = String::from("hello dangle");
    // &s 这样报错
    s
}