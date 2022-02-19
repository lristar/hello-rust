// 这里用智能指针Box 加dyn做兼容
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}

#[test]
fn test_top_package(){
    let a =returns_closure();
    let b= a(16);
    println!("b is {}",b);
}

struct Sheep {}
struct Cow {}

trait Animal {
  fn noise(&self ) -> & 'static str ;
}

impl Animal for Sheep {
    fn noise(&self ) -> & 'static str  {
        "sheep mie mie mie"
    }
}

impl Animal for Cow {
    fn noise(&self ) -> & 'static str  {
        "nu nu nu nu"
    }
}

fn random_animal(random:i32) -> Box<dyn Animal>{
   if random >10 {
    return  Box::new(Sheep{});
   } else {
    return  Box::new(Cow{});
   }
}

#[test]
fn test_randonAnimal(){
        let num = 15;
       let animal =  random_animal(num);
       println!("animal is {}",animal.noise())
}