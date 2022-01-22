pub use rand::Rng;

pub use std::cmp::Ordering;
pub use std::io;

#[test]
#[should_panic(expected = "assertion failed")]
pub fn test_tuple() {
    let tup = (1, 2, 3);
    let (x, y, z) = tup;
    println!("{x},{y},{z}", x = x, y = y, z = z);
    assert_eq!(2, x)
}

#[test]
pub fn test_array() {
    let arr = [1, 2, 3, 4, 5];
    for i in arr {
        println!("{i}", i = i);
    }
}

#[test]
pub fn test_rand() {
    let num = rand::thread_rng().gen_range(1, 101);
    println!("num{x}", x = num);
}

#[test]
pub fn test_ordering() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}
