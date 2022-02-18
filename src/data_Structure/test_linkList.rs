#[derive(Debug)]
enum LLink<T> {
    Node(T, Box<LLink<T>>),
    Nil,
}

impl<T> LLink<T> {
    fn new() -> LLink<T> {
        LLink::Nil
    }
    fn add(self, v: T) -> LLink<T> {
        LLink::Node(v, Box::new(self))
    }
    fn remove(self) -> LLink<T> {
        match self {
            LLink::Nil => LLink::Nil,
            LLink::Node(v, next) => *(next),
        }
    }
}

#[test]
fn test_LLink() {
    let a = LLink::new() as LLink<i32>;
    let mut a1 = a.add(13);
    a1 = a1.add(14);
    a1 = a1.add(15);
    a1 = a1.add(16);
    a1 =a1.remove();
    println!("a1 is {:#?}", a1);
}
