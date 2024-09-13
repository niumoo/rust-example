// 结构体的定义和实例化
fn main() {
    // 结构体的实例化
    let user1 = User {
        active: true,
        username: String::from("wdbyte.com"),
        email: String::from("xxx@126.com"),
        sign_in_count: 123,
    };

    println!("{}", user1.username);

    let name = String::from("niulang");
    let email = String::from("xxx@qq.com");
    let user1 = build_user(name, email);
    println!("{}", user1.username);

    let name = String::from("debug.group");
    let email = String::from("xxx@qq.com");
    let user1 = build_user2(name, email);
    println!("{}", user1.username);

    let user2 = copy_user(user1, String::from("程序猿阿朗"));
    println!("username:{}, email:{}", user2.username, user2.email);
    // 下面的输出会报错，因为 user1中的 String 的所有权已经移交到 User2中，虽然User2只重新赋值了 username
    // println!("username:{}", user1.username);
}

// 结构体定义
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 写一个结构体实例化函数
fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 123,
    }
}

// 结构体参数名与字段名都完全相同，我们可以使用 字段初始化简写语法
fn build_user2(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 123,
    }
}

// 复制一个实例中的所有属性，但是修改其中的 username
// 结构更新语法就像带有 = 的赋值，因为它移动了数据，
fn copy_user(user: User, username: String) -> User {
    User {
        username: username,
        ..user
    }
}


// 利用结构体编写一个计算长方形面积的程序
