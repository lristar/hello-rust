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

enum Coin {
    Penny(u8),
    Nickel(u8),
    Dime(u8),
    Quarter(u8),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // 类似switch
        Coin::Penny(coin) => {
            println!("Penny");
            coin
        }
        Coin::Nickel(coin) => {
            println!("Nickel");
            coin
        }
        Coin::Dime(coin) => {
            println!("Dime");
            coin
        }
        Coin::Quarter(coin) => {
            println!("Quarter");
            coin
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::Some(i) => Option::Some(i + 1),
        Option::None => Option::None, //必须要捕获None
    }
}

fn get_one() -> i8 {
    return 1;
}

fn get_two() -> i8 {
    return 2;
}

fn get_three() -> i8 {
    return 3;
}

fn get_other() -> i8 {
    return 4;
}

fn select_fn(x: i8) -> i8 {
    match x {
        1 => get_one(),
        2 => get_one(),
        3 => get_three(),
        _ => get_other(), // 实现其他的意思，类似default
    }
}

//fn iflet() {
//    // 以下有两种表达方式，作用一样
//
//    // 1.
//    let mut count = 0;
//    match coin {
//        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
//        _ => count += 1,
//    }
//
//    // 2. if let 方式 ,if let 相对比match简洁一点在单条件的情况
//    let mut count = 0;
//    if let Coin::Quarter(state) = coin {
//        println!("State quarter from {:?}!", state);
//    } else {
//        count += 1;
//    }
//}

#[test]
pub fn test_enum() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(6);
    println!("four is{:#?}", x = four);
    println!("{}", six.get_num());
    let op = Option::Some("12334");
    dbg!(op);
    let aa = crate::five_struct::struct_test::create_user();
}

#[test]
fn test_match() {
    let a = value_in_cents(Coin::Penny(8));
    println!("value: ,{}", a);
    let none = plus_one(Option::None);
    println!("None is :{:#?}", none);
    let five = Option::Some(5);
    println!("six: {:#?}", plus_one(five));

    let num = select_fn(3);
    println!("num is {}", num);
}
