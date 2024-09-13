use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("猜数字游戏！");
    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("神秘数字是:{}",secret_number);

    loop {
        println!("请输入一个数字：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取数据失败");

        // shadow ，重名变量隐藏老的变量，复用变量名。
        // let guess:u32 = guess.trim().parse().expect("你输入的不是数字");
        let guess:u32 = match guess.trim().parse() {
            Result::Ok(num)=>num,
            Result::Err(_) => continue,
        };
        println!("你猜测的数字：{}",guess);

        match guess.cmp(&secret_number) {
            Ordering::Less =>print!("太小了！"),
            Ordering::Greater => println!("太大了！" ),
            Ordering::Equal => {
                println!("你赢了！");
                break;
            },
        }
    }
}
