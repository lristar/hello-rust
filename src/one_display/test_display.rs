use std::fmt::{self, write};

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

struct List(Vec<i32>);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// 类似地对 `Point2D` 实现 `Display`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义格式，使得仅显示 `x` 和 `y` 的值。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = &self.0;
        write!(f, "[")?;
        for (k, v) in v.iter().enumerate() {
            if k != 0 {
                write!(f, ",")?;
            }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

#[test]
fn test_display() {
    let Stu = Structure(4);
    println!("fmt : {}", Stu);
    let p = Point2D { x: 2.2, y: 7.7 };
    println!("Debug: {:?}", p);
}

#[test]
fn test_display_list() {
    let list = List(vec![1, 2, 3, 4, 5]);
    println!("list is {}", list);
}
