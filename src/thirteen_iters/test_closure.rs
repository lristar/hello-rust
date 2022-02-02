#[test]
fn test_closure() {
    // let example_closure = |x| x;

    // // 闭包前面定义了字符串后面默认是字符串则输入5就会panic
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    let plus_one = |x| {
        let mut result: i32 = x;
        result += result;
        result += 5;
        result
    };
    let plus_two = |x, y| -> i32 {
        let a = x;
        let b = y;
        a + b
    };
    assert_eq!(plus_one(4), 13);
    assert_eq!(plus_two(2, 3), 5);
}
