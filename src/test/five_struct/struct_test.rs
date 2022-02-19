#[derive(Debug)] // 加上这个可以用println输出
pub struct User {
    pub username: String, //这里做了一个pub 则，在main方法就可以使用user1.username
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn set_username(mut self, username: String) -> User {
        self.username = username;
        return self;
    }
}
// 定义接口
trait IUser {
    fn get_user(&self) -> String;
}
// 指向 User类实现IUser接口
impl IUser for User {
    fn get_user(&self) -> String {
        return String::from("hahaah");
    }
}

pub fn create_user() -> User {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 这里必须保证user1是mut ，方法set_username内有用self,所以可以用user1.set_username
    user1 = user1.set_username(String::from("lzy"));
    let user2 = User {
        username: user1.username,
        email: user1.email,
        sign_in_count: 0,
        active: false,
    };
    // 方法中使用了&self 用于对象引用
    let i = user2.get_user();
    println!("user say:{}", i);
    // dbg!() 返回user2的所有权同时在终端打印数据
    return dbg!(user2);
}
