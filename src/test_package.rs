pub fn test() -> i32 {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
    }
    return x;
}

pub fn testif() -> bool {
    let a = 5;
    return if a > 10 {
        println!("big");
        true
    } else {
        println!("min");
        false
    };
}

pub fn testwhile() -> i32 {
    let mut result = 0;
    for i in 0..100 {
        result = i + result;
    }
    return result;
}

pub fn testloop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 跳出label为counting_up的loop;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

pub fn test_for_rev() {
    for number in (1..4).rev() {
        //rev是指从4到1,倒过来(不包含4)
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
