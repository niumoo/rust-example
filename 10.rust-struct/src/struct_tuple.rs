// 元组结构体
// 通过定义元组的形式也可以创建结构体，但是这种结构体没有字段名，只有具体的字段类型。
struct Rgb(i8, i8, i8);
struct Point(i32, i32, i32);

fn main() {
    let rgb = Rgb(10, 10, 10);
    let mut point = Point(20, 20, 20);
    point.1 = 30;
    println!("rgb:{},point:{}", rgb.0, point.1);
}
