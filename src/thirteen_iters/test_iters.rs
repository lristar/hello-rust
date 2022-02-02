#[test]
fn test_iter1() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // let v1_iter = v1.iter();
    for v in v1 {
        println!("iter is {}", v);
    }
}

// 过滤器
#[test]
fn test_iter_filter() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let result: Vec<_> = v1.iter().filter(|&x| x % 3 == 0).collect();
    dbg!(result);
}

// 取反
#[test]
fn test_iter_rev() {
    let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let v2 = v1.clone();
    let v3 = v2.iter().rev();
    for ele in v3 {
        dbg!(ele);
    }
}

#[test]
fn test_iters2() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

// 测试zip合并压缩
#[test]
fn test_iter_zip() {
    let vec1 = vec![3, 5, 7];
    let vec2 = vec![2, 4, 6];
    let vec3:Vec<_>= vec1.iter().zip(vec2.iter()).map(|x| x.0 +x.1).collect();
    dbg!(vec3);
}


// 自定义迭代器
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
      Counter { count: 0 }
  }
}

impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
      self.count += 1;

      if self.count < 6 {
          Some(self.count)
      } else {
          None
      }
  }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
