use std::io;
use std::io::{ stdout, BufWriter };
use std::cmp::Ordering;
use rand::Rng;
use ferris_says::say;

fn main() {
    // 生成一个随机数
    let rand_number = rand::thread_rng().gen_range(1..=100);
    // println!("秘钥数字是:{rand_number}");
  
    println!("Rust 猜数字游戏,请输入一个数字(1-100)：");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("数字读取失败");
        // let guess: u32 = guess.trim().parse().expect("输入的数字有误");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入正确的数字:");
                continue;
            }
        };
        // println!("你输入的数字是:{}", guess);

        // 比较
        match guess.cmp(&rand_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                let stdout = stdout();
                let message = String::from("恭喜你，猜对了！！！");
                let width = message.chars().count();
                let mut write = BufWriter::new(stdout.lock());
                say(message.as_bytes(), width, &mut write).unwrap();
                break;
            }
        }
    }

    // let _apples = 5 ; // 不可变
    // apples = 1;

    // let mut bananas; // 可变
    // bananas = 10;
    // println!("{}",bananas);
}