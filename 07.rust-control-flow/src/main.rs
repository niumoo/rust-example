fn main() {
    //  if else
    let num = 9;
    if num < 20 {
        println!("number < 20,number is :{}", num);
    } else {
        println!("number > 20,number is :{}", num);
    }

    // if else if
    if num % 10 == 0 {
        println!("数字可以被 10 整除，数字是:{}", num);
    } else if num % 4 == 0 {
        println!("数字可以被 4 整除，数字是:{}", num);
    } else if num % 3 == 0 {
        println!("数字可以被 3 整除，数字是:{}", num);
    } else {
        println!("数字不能被 10 、4、3 整除；数字是:{}", num);
    }

    // if 一行,每个分支的返回值类型必须是相同的
    let num2 = if true { 10 } else { 9 };
    println!("the value of num2 is :{}", num2);

    // 循环 loop
    let mut count = 0;
    loop {
        count = count + 1;
        println!("loop....");
        if count == 2 {
            println!("loop 循环 3 次结束。。。");
            break;
        }
    }
    // 循环 loop 可以有返回值
    let result = loop {
        count = count + 1;
        if count == 5 {
            break count * 2;
        }
    };
    println!("the loop result is :{}", result);

    // while
    let mut count = 3;
    while count != 0 {
        println!("{}", count);
        count = count - 1;
    }
    println!("倒计时结束...");

    println!("---------for--------");

    // for 循环, in array 和 in array.iter 的区别，iter 后 temp 为元素的引用
    let array = [1, 2, 3, 4, 5, 6];
    // for temp in array {
    // println!("temp is :{}", temp);
    // }
    for temp in array.iter() {
        println!("temp is :{}", temp);
    }
    println!("---------for Range------");
    // for 的 range
    for temp in (1.. 3) {
        println
        !("temp {}", temp);
    }
    // for 的 range 反转
    for temp in (1.. 3).rev() {
        println
        !("rev temp {}", temp);
    }
}
