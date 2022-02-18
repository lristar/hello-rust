use std::fmt::{self, write, Error};
#[derive(Debug)]
struct Stack<T> {
    node: Vec<Box<T>>,
}

trait Common<T> {
    fn is_empty(&self) -> bool;
    fn length(&self) -> i32;
    fn pop(&mut self);
    fn push(&mut self, v: T);
}

impl<T> Stack<T> {
    fn new(b: T) -> Stack<T> {
        return Stack {
            node: vec![Box::new(b)],
        };
    }
}

// impl<T> fmt::Display for Stack<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let m: Result<_, Error> = self.node.iter().map(|x|&x*1);
//         match m {
//             Ok(v) => write!(f, "{}", m.),
//             Err(e) => write!(f, "{}", e),
//         }
//     }
// }

impl<T> Common<T> for Stack<T> {
    fn pop(&mut self) {
        self.node.pop();
    }

    fn length(&self) -> i32 {
        return self.node.len() as i32;
    }

    fn push(&mut self, v: T) {
        self.node.push(Box::new(v));
    }

    fn is_empty(&self) -> bool {
        self.node.is_empty()
    }
}

#[test]
fn test_stack() {
    let mut s = Stack::new(10);
    s.push(12);
    s.push(18);
    s.push(19);
    s.pop();
    println!("value is {:#?}", s);
}
