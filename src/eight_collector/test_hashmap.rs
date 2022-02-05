use std::collections::HashMap;
#[derive(Debug)]
struct HM {
    h: HashMap<i32, String>,
}

impl HM {
    fn test_new() -> HM {
        let mut hm = HashMap::new();
        HM { h: (hm) }
    }
    fn test_get_value(&self, s: i32) -> String {
        let default = String::from("");
        let a = self.h.get(&s).unwrap_or(&default);
        return a.to_string();
    }

    // Todo 怎么解决这个所有权问题
    fn test_del_value(&mut self, s: i32) {
        self.h.remove(&s);
    }
    fn test_add_value(&mut self, k: i32, v: String) {
        self.h.insert(k, v);
    }
    fn test_get_all(&self) {
        for kv in self.h.iter() {
            println!("kv : {:?}", kv);
        }
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
    let mut hm = HM::test_new();
    let _aaa = hm.test_get_value(13);
    if _aaa == "".to_string() {
        println!("_aaa is{}", _aaa);
    }
    // 内部如果修改了结构体需要重新给予所有权，要不然所有权进去了基本在方法内就消失了。
    hm.test_add_value(15, "15".to_string());
    hm.test_del_value(15);
    hm.test_add_value(16, "16".to_string());
    hm.test_get_all();
    println!("hm:{:#?}", hm);
}
