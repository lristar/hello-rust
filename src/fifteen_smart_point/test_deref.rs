#[test]
fn test_deref_first() {
  let x = 5;
  let y = &x;

  assert_eq!(5, x);
  // assert_eq!(5, y);
  // 必须使用解引用来跟踪数据的类型
  assert_eq!(5, *y);
}
