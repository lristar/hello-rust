pub fn test_string_heap() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("{}", s); // 将打印 `hello, world!
}

pub fn test_string_copy() {
    // s copy给s2后s就失效了
    //    let mut s = String::from("hello");
    //    let s2 = s;
    //    println!("s2{}", s2);

    // 用clone就是深度复制，完全复制
    let s = String::from("hello");
    // *****注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    // 如果使用方法这个s，这个s的作用域就移动到use_string了，后面的s.clone()就失效了。
    //    use_string(s);
    // 如果使用参数的引用，允许你使用s的数值而不是所有权，引用不能修改数值，但如果你使用了&mut String，则可以修改。
    use_string(&s);
    let s2 = s.clone();
    println!("s{},s2{}", s, s2);
}
fn use_string(s: &String) {
    println!("s:{}", s);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    // 如果是创建单例，则是必须要交出所有权，如果不交出所有权则s就会在这个方法的作用域内失效，return出去的引用则是无用
    // return &s;  -- 这个是错误的
    return s;
}

pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
