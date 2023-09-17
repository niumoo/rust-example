fn main() {
    println!("Hello, world!");
    let mut re = Rectangle {
        width: 12,
        height: 22,
    };
    re.update_width(10);
    println!("{} x {} = {}", re.width, re.height, re.area());

    let re2 = Rectangle {
        width: 8,
        height: 5,
    };
    let re3 = Rectangle {
        width: 80,
        height: 50,
    };
    println!("{},{}", re.can_hold(&re2), re.can_hold(&re3));
    let re4 = Rectangle::square(10);
    println!("{} x {} = {}", re4.width, re4.height, re4.area());
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

/**所有在 impl 块中定义的函数被称为 关联函数（associated functions），因为它们与 impl 后面命名的类型相关。我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。我们已经使用了一个这样的函数：在 String 类型上定义的 String::from 函数。
 * 
 */
impl Rectangle {
    // 传入另一个参数
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 没有self 参数的关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
