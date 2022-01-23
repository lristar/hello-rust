//pub fn length_of_longest_substring(s: String) -> i32 {
//    use std::cmp::max;
//    let mut last: [i32; 128] = [-1; 128]; //定义长度为128的数组，初始值为-1 //ascii编码表最大是127，取128长度的数组
//    let mut left = -1;
//    let mut ans = 0;
//    for (i, v) in s.chars().enumerate() {
//        left = max(left, last[v as usize]);
//        last[v as usize] = (i as i32);
//        ans = max(ans, ((i as i32) - left) as i32);
//    }
//    return ans;
//}
pub fn length_of_longest_substring(s: String) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;
    let mut last: [i32; 128] = [-1; 128]; //定义长度为128的数组，初始值为-1 //ascii编码表最大是127，取128长度的数组
    let mut left = -1;
    let mut ans = 0;
    for (i, v) in s.chars().enumerate() {
        left = max(left, last[v as usize]);
        last[v as usize] = (i as i32);
        ans = max(ans, ((i as i32) - left) as i32);
    }
    return ans;
}

#[test]
fn test_length_of_longest_substring() {
    let len = length_of_longest_substring(String::from("adfadb"));
    println!("len is :{}", len);
}
