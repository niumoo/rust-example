pub mod main2;
pub mod slice_array;

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();
    // s 清空后，word 依旧有值。
    println!("s:{} , word:{}", s, word);

    slice();

    let word = first_word_slice(&s);
    println!("world: {}", word);

    println!("------------------------------");

    let word = first_word_str_slice(&s);
    println!("world: {}", word);

    let word = first_word_str_slice(&s[0..3]);
    println!("world: {}", word);

    let word = first_word_str_slice(&s[..]);
    println!("world: {}", word);

    let word = first_word_str_slice(&s);
    println!("world: {}", word);

    //  针对数组进行 slice
    let a = [1, 2, 3, 4, 5, 6];
    let b = &a[2..];
    assert_eq!(b, &[3, 4, 5, 6]);
}

/**
 * 有一个编程小习题：编写一个函数，该函数接收一个用空格分隔单词的字符串，
 * 并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，
 * 则整个字符串就是一个单词，所以应该返回整个字符串。
 * 下面的这种写法，在 S 回收后，依旧保存了返回的值。
 */
fn first_word(s: &String) -> String {
    let space = s.contains(" ");
    if space {
        for ele in s.split(" ") {
            return ele.to_string();
        }
    }
    s.to_string()
}

// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一类引用，所以它没有所有权。
// 符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字
// 符串 slice，则程序将会因错误而退出。出于介绍字符串 slice 的目的，
fn slice() {
    println!("------------------------------");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    // s.clear(); error ,- immutable borrow occurs here
    println!(" {} {} ！", hello, world);
    // 范围的另一种写法
    let hello2 = &s[..5];
    let world2 = &s[6..];
    let s2 = &s[..];
    println!(" {} {} ！{}", hello2, world2, s2);
}

//  使用 slice 重新实现上面的需求
fn first_word_slice(s: &String) -> &str {
    println!("------------------------------");
    let bytes = s.as_bytes();
    for ele in 0..bytes.len() {
        if bytes[ele] == b' ' {
            return &s[0..ele];
        }
    }
    return &s[..];
}

// 使用字符串 slice 作为接受参数类型来重新实现上面的需求
fn first_word_str_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for ele in 0..bytes.len() {
        if bytes[ele] == b' ' {
            return &s[0..ele];
        }
    }
    return s;
}
