enum List {
    Cons(u32,Box<List>),
    Nil,
}

impl List{
    fn new() -> List{
        List::Nil
    }
    fn prepend(self ,elem: u32) ->List{
        List::Cons(elem,Box::new(self))
    }

    fn len(&self) -> u32{
        match *self {
            List::Cons(_,ref tail) => {
                tail.len()+1
            },
            List::Nil => 0
        }
    }

    fn stringify(&self) ->String{
        match *self{
            List::Cons(v,ref u) =>{
                format!("{},{}",v,u.stringify())
            },
            List::Nil => format!("Nil"),
        }
    }
}

#[test]
fn test_linkList(){
    let  l =List::new();
    let p1 =  l.prepend(10);
    let p2 = p1.prepend(12);
    println!("list is {}",p2.stringify())
}

struct Number {
    value:i32,
}

impl From<i32> for Number{
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[test]
fn test_typesChange(){
    // string exchange i32
    let s =String::from("1");
    let i:i32 = s.parse().unwrap();
    println!("i is {}",i);
    // i32 exchange string
    let  s2:String = i.to_string();
    println!("s2 is {}",s2);
    // i32 exchange f32
    let  f1 =i as f32;
    let f2 = f1 + 1.1;
    println!("f1 is {}",f2);

    
}

