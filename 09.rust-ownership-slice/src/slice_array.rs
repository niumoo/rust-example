fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // println!("{}", slice);
    // 数组不能直接输出，使用 dbg! 输出
    dbg!(slice);
}
