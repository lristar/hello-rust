use std::fmt::format;

// 以下是不用接口trait 的模式，每个结构体都得定义个ipml
// pub struct NewsArticle {
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// 利用trait定义接口
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub trait Display {
    fn show(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn show(&self) -> String {
        "hello".to_string()
    }
}

pub fn notify<T: Summary>(item: T) {
    // 前面的<T:Summary>这里指设置Summary类型为T ,也就是说后面用T就代表用Summary
    println!("Breaking news! {}", item.summarize());
}

// 如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，
// 那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现

pub fn notify1<T: Summary + Display>(item: T) {
    println!("{}", item.show())
}
#[test]
fn test_trait() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("2 new Tweet:{}", tweet.summarize_author());
    println!("start 接口的多态");
    // notify(tweet);
    notify1(tweet);
}
