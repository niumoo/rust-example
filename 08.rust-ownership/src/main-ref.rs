/**
 * 引用与借用
 */
fn main() {
    let s1 = String::from("hello");
    // 使用 &s1 传入 s1 的引用
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello rust");
    let len2 = calculate_length2(&mut s2);
    println!("The length of '{}' is {}.", s1, len2);
}

//  通过 & 传入一个变量的应用，这不会获取所有权
fn calculate_length(s: &String) -> usize {
    // s.push_str(" world");
    // 注意，这里的引用不可变。`s` is a `&` reference, so the data it refers to cannot
    s.len()
    
}

//  通过 & 传入一个变量的应用，这不会获取所有权
//  但是需要更改变量内容，需要使用 mut 标记
fn calculate_length2(s: &mut String) -> usize {
    s.push_str(" world");
    s.len()
}
