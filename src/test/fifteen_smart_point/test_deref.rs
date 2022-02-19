#[test]
fn test_deref_first() {
  let x = 5;
  // let y = &x;
  let ref y = x;

  assert_eq!(5, x);
  // assert_eq!(5, y);
  // 必须使用解引用来跟踪数据的类型
  assert_eq!(5, *y);
}


// box can exchange the data  from the  heap   to  stack 
#[test]
fn test_box_first(){
  let x = String::from("hello");
  let y = Box::new(x);
  println!("y is {}",*y);

  println!("y is {}",*y);
  println!("y is {}",*y);
}
