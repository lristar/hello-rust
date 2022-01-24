use std::collections::HashMap;
#[derive(Debug)]
struct HM {
    pub h: HashMap<i32, String>,
}

impl HM {
    fn test_get_value(&self, s: i32) -> String {
        let a = self.h.get(&s).unwrap().clone();
        return a;
    }

    // Todo 怎么解决这个所有权问题
    fn test_del_value(mut self, s: i32) -> HM {
        self.h.remove(&s);
        return self;
    }
}

fn test_add_value() -> HM {
    let mut hm = HashMap::new();
    // 插值 ,如果key一样的，值不一样会更新值
    hm.insert(13 as i32, "13".to_string());
    hm.insert(12 as i32, "12".to_string());
    // 如果存在key,则不插入
    hm.entry(12 as i32).or_insert("15".to_string());
    return HM { h: hm };
}
fn test_del_value() {}

// 不过为了更高的安全性值得付出一些性能的代价。如果性能监测显示此哈希函数非常慢，以致于你无法接受，
// 你可以指定一个不同的 hasher 来切换为其它函数。hasher 是一个实现了 BuildHasher trait 的类型
#[test]
fn test_hash_map() {
    let hm = test_add_value();
    let _aaa = hm.test_get_value(13);
    // 内部如果修改了结构体需要重新给予所有权，要不然所有权进去了基本在方法内就消失了。
    let hh = hm.test_del_value(13);
    println!("hm:{:#?}", hh);
}
