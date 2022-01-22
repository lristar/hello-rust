extern crate rand;
pub mod struct_test;
pub mod test_package;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub mod test_heap_stack;

pub mod types_test;

const NUM: u32 = 10;

fn main() {
    test_package::test_for_rev();
    test_heap_stack::test_string_copy();
    let s = String::from("hello world");
    let one = test_heap_stack::first_word(&s);
    println!("one is :{}", one);

    println!("-------------");
    let user = struct_test::create_user();
}

// 测试loop循环，match的匹配实现
fn test() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&NUM) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
