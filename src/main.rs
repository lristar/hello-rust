pub mod test_package;
pub mod types_test;
fn main() {
   let mut x = test_package::test();
    println!("test{}",x);
    let is_true = test_package::testif();
    println!("a >10 ? {}", is_true);
    let result = test_package::testwhile();
    println!("result is {}",result);
    types_test::test_tuple()
}
