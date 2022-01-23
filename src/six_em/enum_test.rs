#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(i32),
}
// 可以给同个enum枚举类定义相同的方法
impl IpAddr {
    fn get_num(&self) -> i8 {
        return 1;
    }
}

enum Message {
    Quit,                       // 没有关联任何数据
    Move { x: i32, y: i32 },    //包含一个匿名结构体{}
    Write(String),              // 包含单独一个 String
    ChangeColor(i32, i32, i32), //包含三个 i32
}
// 泛型枚举
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

pub fn test_enum() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(6);
    println!("four is{:#?}", x = four);
    println!("{}", six.get_num());
    let op = Option::Some("12334");
    dbg!(op);
    let aa = crate::five_struct::struct_test::create_user();
}
