fn main() {
    // 枚举类
    enum Book {
        // 枚举的类型
        Papery(u32),
        Electronic(String),
    }
    // 定义变量book
    let book = Book::Electronic(String::from("url"));
    if let Book::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}
