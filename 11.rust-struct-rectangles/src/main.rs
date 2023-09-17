// 使用结构体求一个长方形的面积
fn main() {
    let w = 10;
    let h = 20;
    let area = area(w, h);
    println!("w:{},h:{},area:{}", w, h, area);

    let rectangle_w_and_h = (11, 22);
    println!("w:{},h:{},area:{}", w, h, area_tupe(rectangle_w_and_h));

    let rectangle = Rectangle {
        width: 8,
        height: 9,
    };
    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);
    // 使用 dbg 宏可以输出行号和内容，dbg 默认会获得所有权，所以使用 & 来传递引用
    dbg!(&rectangle);
    dbg!(30 * 20);
    let area = area_struct(&rectangle);
    println!("w:{},h:{},area:{}", rectangle.width, rectangle.height, area);
}

// 基础写法
fn area(w: u32, h: u32) -> u32 {
    w * h
}

// 使用元组结构体重构面积计算方法
fn area_tupe(rectangle: (u32, u32)) -> u32 {
    rectangle.0 * rectangle.1
}

// 使用结构体重构上面的面积计算方法
// 添加 #[derive(debug)] 后，可以使用 {:?} {:#?} 占位符进行输出。
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
