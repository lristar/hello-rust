enum List {
  Cons(i32, Box<List>),
  Nil,
}

#[test]
fn test_con_list(){
    let list = List::Cons(1,Box::new(List::Cons(2,Box::new(List::Cons(3,Box::new(List::Nil))))));
}

#[test]
fn test_box(){
    let b = Box::new(5);
    dbg!(b);
}
