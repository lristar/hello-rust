pub mod test_package;

fn main() {
   let mut x = test_package::test();
    println!("test{}",x);
    let is_true = test_package::testif();
    println!("a >10 ? {}", is_true);
    let result = test_package::testwhile();
    println!("result is {}",result);
}
