use std::thread;
use std::time;
use std::time::Duration;
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
}

fn generate_workout1(intensity: u32, random_number: u32) {
    // 提前执行好所需要的程序后续直接返回数值
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}


#[test]
fn test_closure() {
    // let example_closure = |x| x;

    // // 闭包前面定义了字符串后面默认是字符串则输入5就会panic
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    let plus_one = |x| {
        let mut result: i32 = x;
        result += result;
        result += 5;
        result
    };
    let plus_two = |x, y| -> i32 {
        let a = x;
        let b = y;
        a + b
    };
    assert_eq!(plus_one(4), 13);
    assert_eq!(plus_two(2, 3), 5);

    // 调用上边定义的匿名函数
    generate_workout1(20, 5)
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    
    let v2 = c.value(2);
    let v1 = c.value(1);
    
    assert_eq!(v2, 2);
}