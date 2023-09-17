// 引入标准输入输出库
use std::io;
use rand::Rng;

fn main() {
    println!("猜数字游戏!");    
    let rand_number = rand::thread_rng().gen_range(0, 10);
    println!("神秘数字是:{}",rand_number);
    println!("-------------------");

    loop {
        println!("请输入一个数字：");
        // let 创建变量，默认不可变
        // let name = 1;
        // name = 2;
        // let mut 创建变量，变量是可变的
        // ::new 是关联函数，类似于 java 的静态方法
        let mut guess = String::new();
        // & 说明是一个引用，mut 说明是可变的
        io::stdin().read_line(&mut guess).expect("无法读取信息");

        // Rust 中变量可以重新命名绑定其他值
        // let guess:u32 = guess.trim().parse().expect("请输入一个数字！");
        // 不要报错，自己处理异常
        let guess:u32 =match guess.trim().parse(){
           Ok(num) => num,
           Err(_)=>{
            print!("你输入的不是数字，请重新输入：");
            continue;
           },
        }; 

        match guess.cmp(&rand_number) {
            std::cmp::Ordering::Less => println!("太小了"),
            std::cmp::Ordering::Greater => println!("太大了"),
            std::cmp::Ordering::Equal =>{
                println!("你赢了");
                break;
            },
        }
        // println!("你猜测的数字是:{},{}", guess,"!!");
    }
}

