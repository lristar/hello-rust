use std::{f32::consts::E, num::ParseIntError};
// 自定义别名
type AliasResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    let second_number = match second_number_str.parse::<i32>() {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    return Ok(first_number * second_number);
}

// 使用？代替
fn multiply2(first_number_str: &str, second_number_str: &str) -> AliasResult<i32> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    return Ok(first_number * second_number);
}

fn printf(result: AliasResult<i32>) {
    match result {
        Ok(s) => {
            println!("result is {}", s)
        }
        Err(_) => {
            println!("no result")
        }
    }
}

#[test]
fn test_front_result() {
    printf(multiply2("1", "2"))
}
