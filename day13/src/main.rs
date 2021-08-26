fn main() {
    // 结构体颜色（三个参数）
    struct Color(u8, u8, u8);
    // 结构体点（坐标）
    struct Point(f64, f64);
    // 定义black 的颜色
    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
    println!("Hello, world!");
}
