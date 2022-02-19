#![allow(dead_code)]
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);
#[derive(Debug)]
struct Chopped(Food);
#[derive(Debug)]
struct Cooked(Food);

// fn peer(food: Option<Food>) -> Option<Peeled> {
//     match food {
//         Some(food) => Some(Peeled(food)),
//         None => None,
//     }
// }
// 以下用map转化
fn peer(food: Option<Food>) -> Option<Peeled> {
    food.map(|f| Peeled(f))
}

// fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
//     match peeled {
//         Some(Peeled(Food)) => Some(Chopped(Food)),
//         None => None,
//     }
// }
// 以下用map转化
fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    peeled.map(|Peeled(f)| Chopped(f))
}

// fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
//     match chopped {
//         Some(Chopped(Food)) => Some(Cooked(Food)),
//         None => None,
//     }
// }
// 以下用map转化
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(f)| Cooked(f))
}

// 整合以上所有操作
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food:Option<Cooked>) {
    match food {
        Some(f) => {println!("is delision")},
        None => {println!("is bad ,not eat")},
    }
    
}

#[test]
fn test_cookie() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;
    let cooked_apple = cook(chop(peer(apple)));
    let cooked_carrot = cook(chop(peer(carrot)));
    let cookie_potato = process(potato);

    eat(cooked_apple);
    eat(cookie_potato);
}
