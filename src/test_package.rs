use std::io;
pub fn test() -> i32{
    let x = 5;
    let x =x+1 ;
    {
        let x=x *2;
    }
    return x
    }

pub fn testif() ->bool{
    let mut a = 5;
    return if a > 10 {
        println!("big");
        true
    } else {
        println!("min");
        false
    }
}

pub fn testwhile()->i32{
     let mut result=0;
    for i in 0..100 {
        result = i + result;
    }
    return result;
}


