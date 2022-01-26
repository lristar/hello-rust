//这里我们想要告诉 Rust
//关于参数中的引用和返回值之间的限制是他们都必须拥有相同的生命周期，在每个引用中都加上了 'a 那样
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //而只需要知道有某个可以被 'a 替代的作用域将会满足这个签名
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn test_lifetime() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());  //这样是不行的，因为result的作用域是x,y中较小作用域的一个，也就是y,所以result出不了{}，编译通不过
    // }
    // println!("The longest string is {}", result);
    // 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
    // 所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来。
    let s: &'static str = "I have a static lifetime.";
}
