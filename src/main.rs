extern crate rand;
pub mod test_package;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub mod types_test;

fn main() {
    let mut x = test_package::test();
    println!("test{}", x);
    let is_true = test_package::testif();
    println!("a >10 ? {}", is_true);
    let result = test_package::testwhile();
    println!("result is {}", result);
    test()
}

// 测试loop循环，match的匹配实现
fn test() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
