// 定义结构体
trait doubt {
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
    let x: i32 = 10.dou();
    print!("{}", x)
}
