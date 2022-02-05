trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
      println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
      println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
      println!("*waving arms furiously*");
  }
}

#[test]
fn test_implement() {
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  // person.fly();
}

// 
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}

struct Counter{
}
impl Iterator for Counter {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
     None
  }
}

#[test]
fn test_Type(){
  let c = Counter{};
  c.next()
}