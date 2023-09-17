fn main() {
    //  元组 tupe,
    let tup1: (i32, f32) = (3, 3.14);
    println!("0 is :{}, 1 is :{}", tup1.0, tup1.1);

    // 使用模式匹配来解构元组
    let (_x, _y) = tup1;
    println!("0 is :{}, 1 is :{}", _x, _y);

    // 读取赋值
    let _x2 = tup1.0;
    let _y2 = tup1.1;
    println!("0 is :{}, 1 is :{}", _x2, _y2);

    // 数组
    let array1: [i32; 4] = [1, 2, 3, 4];
    println!(
        "array info : {} {} {} {}",
        array1[0], array1[1], array1[2], array1[3]
    );

    // 数组 - 字符数组
    let array2 = ["one", "two", "three", "four"];
    println!(
        "array info : {} {} {} {}",
        array2[0], array2[1], array2[2], array2[3]
    );
    // 数组越界
    // println!("{}",array2[5]);  index out of bounds: the length is 4 but the index is 5

    // 如果数组内容一样，可以简单定义 ,如 hello,hello,hello,hello,hello
    let array3 = ["hello"; 5];
    println!("array3 index 4 value:{}", array3[4]);
}
