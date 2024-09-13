use std::net::Ipv4Addr;

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 定义枚举类型时可以直接定义附加的数据体结构，这样通过一个枚举就可以包含类型和具体的值
#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 枚举类型也可以有方法
impl IpAddr2 {
    fn print(&self){
        print!("hello emun");
    }
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?},{:?}", four, six);
    dbg!(four);
    dbg!(six);

    let home_ip = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home_ipv6 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    dbg!(&home_ip);
    dbg!(&home_ipv6);

    let four = IpAddr2::V4(127, 0, 0, 1);
    let six = IpAddr2::V6(String::from("127.0.0.1"));
    dbg!(&four);
    dbg!(&six);
    six.print();
    test_option();

    // 官方定义的 IP V4 类
    let ipv4 = Ipv4Addr::new(111, 111, 111, 111);
    print!("{}",ipv4)


    
}


fn test_option(){
    let mut some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("some number is null:{}",some_number.is_none());
    println!("some char is not null:{}",some_number.is_some());


    // let x = some_number.take();
    // println!("x value:{}",x);
}