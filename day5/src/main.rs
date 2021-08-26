// 定义结构体
trait doubt {
    // 定义方法dou带参数返回值self
    fn dou(&self) -> Self;
}
// 实现结构体
impl doubt for i32 {
    fn dou(&self) -> Self {
        *self * 2
    }
}
// 主函数
fn main() {
    // 变量名x的类型的值是调用dou
    let x: i32 = 10.dou();
    print!("{}", x)
}
