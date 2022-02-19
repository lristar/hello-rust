use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// mutex
#[test]
fn test_mutex() {
    let m = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    let mut sum = 0;
    for _ in 0..10 {
        let m1 = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut num = m1.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        println!("111");
        handle.join().unwrap();
    }
    println!("some is {}", *m.lock().unwrap());
}
