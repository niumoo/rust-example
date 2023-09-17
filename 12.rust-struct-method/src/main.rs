fn main() {
    println!("Hello, world!");
    let mut re = Rectangle {
        width: 12,
        height: 22,
    };
    re.update_width(10);
    println!("{} x {} = {}", re.width, re.height, re.area());
}

// 结构体中的方法，
// 方法与函数是不同的是因为它们在结构体的上下文中被定义
// 并且它们第一个参数总是 self，它代表调用该方法的结构体实例。
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    // &self 实际上是 self: &Self 的缩写。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn update_width(&mut self, w: u32) {
        self.width = w;
    }
}
