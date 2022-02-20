pub fn calculate(s: String) -> i32 {
    let mut process: Vec<String> = vec![];
    let mut ops: Vec<String> = vec![];
    for a in s.chars() {
        match priority_of(&a.to_string()) {
            1 | 2 => {
                if ops.len() == 0 {
                    ops.push(a.to_string());
                } else {
                    let front = ops.pop().unwrap();
                    let curLevel = priority_of(&a.to_string());
                    let frontLevel = priority_of(&front.to_string());
                    if frontLevel >= curLevel {
                        // 取出数字数组塞到结果集中
                        let a1 = process.pop().unwrap();
                        let a2 = process.pop().unwrap();
                        let op = ops.pop().unwrap();
                        let rs = cal(a1, a2, op);
                        process.push(rs.to_string())
                    } else {
                        ops.push(front);
                        ops.push(a.to_string());
                    }
                }
            }
            0 => {
                process.push(a.to_string());
            }
            _ => (),
        }
    }
    let result = process.pop().unwrap().parse::<i32>().unwrap();
    return result;
}

pub fn tailPro() {}

pub fn priority_of(s: &String) -> i32 {
    match s.as_str() {
        "*" | "/" => 2,
        "+" | "-" => 1,
        _ => 0,
    }
}

pub fn cal(a: String, b: String, op: String) -> i32 {
    let mut result: Vec<i32> = vec![];
    let a1 = a.parse::<i32>().unwrap();
    let a2 = b.parse::<i32>().unwrap();
    match op.as_str() {
        "+" => result.push(a2 + a1),
        "-" => result.push(a2 - a1),
        "*" => result.push(a2 * a1),
        "/" => result.push(a2 / a1),
        _ => (),
    }
    return result.pop().unwrap_or(0);
}

use std::{ascii::AsciiExt, u8, vec};

enum Parentheses {
    Left(u8),
    Right(u8),
}

enum Brackets {
    Left(u8),
    Right(u8),
}

enum CurlyBrackets {
    Left(u8),
    Right(u8),
}
#[test]
fn leetcode_test() {
    let token: Vec<String> = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];
    // let num = calculate("1+2*3-4".to_string());
    // let num = eval_rpn(token);
    let num = calculate("1+2*3+4".to_string());
    eprintln!("num = {:?}", num);
    // let sum = String::from("1+2*3/4");
    // calculate(sum);
}
