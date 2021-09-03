// 定义trait函数，trait可以是很多功能
trait doubt {
    // 定义方法dou带参数返回值self
    fn dou(&self) -> Self;
}
// 实现trait
impl doubt for i32 {
    // 实现dout的方法dou以及功能
    fn dou(&self) -> Self {
        // 值
        *self * 2
    }
}
// 主函数
fn main() {
    // 变量名x的类型的值是调用dou
    let x: i32 = 10.dou();
    print!("{}", x)
}
