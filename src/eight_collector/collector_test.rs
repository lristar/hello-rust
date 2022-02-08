#[test]
fn test_vector() {
    //第一种创建方式
    let mut v: Vec<i32> = Vec::new();
    //第二种创建方式
    let  v1 = vec![1, 2, 3];
    v.push(5);
    v.push(6); // push 推数据进去

    println!("v :2{:#?}", v.get(1)); //拿数据出来 不会删除拿出来的数据
    println!("v :2{:#?}", v.get(1));
    // vector 只能储存相同类型的值。这是很不方便的；绝对会有需要储存一系列不同类型的值的用例。
    // 幸运的是，枚举的成员都被定义为相同的枚举类型，
    // 所以当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

#[test]
fn test_string() {
    // 两种方式创建字符串
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    // 以下是字符串的方法
    //    println!("answer:{:#?}", answer);
}
