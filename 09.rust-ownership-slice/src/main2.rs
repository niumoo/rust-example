fn main() {
    let s = String::from("12345");
    let _len = first_wornd(&s);

    // 通过  &s[0.._len] 可以生成字符串的切片
    let res = &s[0.._len];
    println!("res : {}", res);

    let mut s = String::from("123 45");
    let _len = first_wornd(&s);
    let res = &s[0.._len];
    println!("res : {}", res);

    // 切片的内容会随着原始内容的变化而变化

    s.clear();
    // 下面的输出会报错，因为 res 使用了 0..len 获取 s内容切片。
    // 而 s.clear 清空了 字符串内容。
    // println!("res : {}",res);

    // 字符串 slice 切片,引用了字符串中的一部分
    let s = String::from("123 456");
    let s1 = &s[0..3]; // s1 的长度等于 3-0 = 3
    let s2 = &s[4..7];
    // slice 可以省略以0位置开始和到最后末尾的数字
    let s3 = &s[..3];
    let s4 = &s[4..];
    // slice 如果是全部截取，前后都可以省略
    let s5 = &s[..];

    println!("s1:{},s2:{},s3:{},s4:{},s5:{}", s1, s2, s3, s4, s5);

    // slice 对于汉字这种非 ASCII 字符的，会有问题，以后学习
    // let s = String::from("学习 rust");
    // let s1 = &s[..2];
    // println!("s1:{}", s1);

    let s = String::from("12345");
    let s1 = first_wornd_slice(&s);
    println!("s1:{}", s1);
    let s = String::from("123 45");
    let s1 = first_wornd_slice(&s);
    println!("s1:{}", s1);

    // 字符串直接赋值，类型就是 Slice &str，因 slice 不可变，所以直接赋值的字符串它不可变
    let s = "Hello world";
    println!("{s}");
}

/**
 * 这里有一个编程小习题：编写一个函数，该函数接收一个用空格分隔单词的字符串，
 * 并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，
 * 则整个字符串就是一个单词，所以应该返回整个字符串。
 */
fn first_wornd(s: &String) -> usize {
    let mut index = 0;
    for ele in s.chars() {
        if ele == ' ' {
            break;
        } else {
            index = index + 1;
        }
    }
    index
}

// slice 类型的字符串记作：&str
fn first_wornd_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item ==b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
