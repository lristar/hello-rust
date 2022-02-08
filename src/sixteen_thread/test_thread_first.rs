use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn test_thread_first() {
    let th = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..10 {
            thread::sleep(Duration::from_millis(1));
            sum += i;
            println!("sum is {}", sum);
        }
    });
    th.join().unwrap(); // 等待线程跑完后再执行以下步骤

    for i in 1..10 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// move闭包的使用例子
#[test]
fn test_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // 所有权已经被拿走了，所以这里的v已经无用了
    // println!("v is {}",v);
    handle.join().unwrap();
}

#[test]
fn test_channel() {
    let (tx, rx) = mpsc::channel();
    let t1 = thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // 等线程跑完再执行下一步
    t1.join().unwrap();

    for received in rx {
        println!("Got: {}", received);
    }
}

#[test]
fn test_channel_two() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

   let t1 =  thread::spawn(move || {
      let val =vec![
        String::from("hi"),
        String::from("lzy"),
        String::from("the"),
        String::from("go"),
      ];

      for v in val {
          tx1.send(v).unwrap();
      }
    });
    t1.join().unwrap();
   let t2 =  thread::spawn(move || {
      let val =vec![
        String::from("hello"),
        String::from("lzy"),
        String::from("where"),
      ];

      for v in val {
          tx.send(v).unwrap();
      }
    });

   for r in rx {
       println!("receive is {}",r)
   }
}
