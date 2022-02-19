// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// 其语法类似于函数定义中使用泛型。
// 首先，必须在结构体名称后面的尖括号中声明泛型参数的名称。
// 接着在结构体定义中可以指定具体数据类型的位置使用泛型类型

// 当你意识到代码中定义了多个结构体或枚举，它们不一样的地方只是其中的值的类型的时候，不妨通过泛型类型来避免重复
// struct Point<T> {
//     x: T,
//     y: T,  // 因为这里的x,y是同一个类型T,所以在定义Point的时候，xy是必须同一类型
// }
// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

struct Point<T, U> {
    //因为定义的是不同的T,U 所以这俩个可以是不同类型
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// fn some_function<T, U>(t: T, u: U) -> i32  // 可以用where 减少fn的bound定义
//     where T: Display + Clone,
//           U: Clone + Debug
// {}

//我们可以使用泛型来编写不重复的代码，而 Rust 将会为每一个实例编译其特定类型的代码。这意味着在使用泛型时没有运行时开销；
//当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时极其高效的原因。

#[test]
fn main() {
    // 字段 x 和 y 的类型必须相同，因为他们都有相同的泛型类型 T
    let integer = Point {
        x: "111".to_string(),
        y: 10.4,
    };
    let float = Point { x: 1.0, y: 4.0 };
    let p = integer.mixup(float);
    println!("integer: ,{}", p.x);
}

//  fn test_genericity() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest<T>(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest<T>(&char_list);
//     println!("The largest char is {}", result);
// }
