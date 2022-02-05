// 这里用智能指针Box 加dyn做兼容
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

#[test]
fn test_top_package(){
    let a =returns_closure();
    let b= a(16);
    println!("b is {}",b);
}