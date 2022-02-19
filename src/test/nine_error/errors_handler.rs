use std::num::ParseIntError;

type AliasResult<T> = Result<T, ParseIntError>;

fn double_first(vec: Vec<&str>) -> AliasResult<Option<i32>> {
    let v = vec.first().map(|first| first.parse::<i32>().map(|f| f * 2));
    v.map_or(Ok(None), |f| f.map(Some))
}

#[test]
fn test_map_or() {
    let numbers = vec!["42", "93", "18"];
    let strings = vec!["tofu", "93", "18"];
    println!("The first doubled is {:?}", double_first(numbers));
    println!("The first doubled is {:?}", double_first(strings));
}
