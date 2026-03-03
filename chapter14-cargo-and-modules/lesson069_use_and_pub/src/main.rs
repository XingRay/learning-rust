// ============================================================
// Lesson 069: use 与可见性 (use and Visibility)
// ============================================================
// use 关键字用于将路径引入当前作用域，简化代码中的长路径。
// 本课讲解 use 的各种用法和细粒度可见性控制。
// ============================================================

// ============================================================
// 1. use 导入路径
// ============================================================

mod geometry {
    pub mod shapes {
        pub struct Circle {
            pub radius: f64,
        }

        impl Circle {
            pub fn new(radius: f64) -> Self {
                Circle { radius }
            }

            pub fn area(&self) -> f64 {
                std::f64::consts::PI * self.radius * self.radius
            }

            pub fn circumference(&self) -> f64 {
                2.0 * std::f64::consts::PI * self.radius
            }
        }

        pub struct Rectangle {
            pub width: f64,
            pub height: f64,
        }

        impl Rectangle {
            pub fn new(width: f64, height: f64) -> Self {
                Rectangle { width, height }
            }

            pub fn area(&self) -> f64 {
                self.width * self.height
            }

            pub fn perimeter(&self) -> f64 {
                2.0 * (self.width + self.height)
            }
        }

        pub fn describe(name: &str, area: f64) -> String {
            format!("{}: 面积 = {:.2}", name, area)
        }
    }

    pub mod utils {
        pub fn degrees_to_radians(degrees: f64) -> f64 {
            degrees * std::f64::consts::PI / 180.0
        }

        pub fn radians_to_degrees(radians: f64) -> f64 {
            radians * 180.0 / std::f64::consts::PI
        }
    }
}

// ============================================================
// 2. use as 别名
// ============================================================

mod io_simulation {
    pub mod file_reader {
        pub struct Reader {
            pub name: String,
        }

        impl Reader {
            pub fn new(name: &str) -> Self {
                Reader {
                    name: name.to_string(),
                }
            }

            pub fn read(&self) -> String {
                format!("从文件 [{}] 读取数据", self.name)
            }
        }
    }

    pub mod network_reader {
        pub struct Reader {
            pub url: String,
        }

        impl Reader {
            pub fn new(url: &str) -> Self {
                Reader {
                    url: url.to_string(),
                }
            }

            pub fn read(&self) -> String {
                format!("从网络 [{}] 读取数据", self.url)
            }
        }
    }
}

// ============================================================
// 3. pub use 重导出 (re-export)
// ============================================================
// pub use 可以将内部模块的项重新导出，让外部用户使用更短的路径

mod api {
    // 内部实现模块（用户不需要知道这些内部结构）
    mod internal {
        pub mod database {
            pub struct Connection {
                pub db_name: String,
            }

            impl Connection {
                pub fn new(name: &str) -> Self {
                    Connection {
                        db_name: name.to_string(),
                    }
                }

                pub fn query(&self, sql: &str) -> String {
                    format!("[{}] 执行查询: {}", self.db_name, sql)
                }
            }
        }

        pub mod cache {
            pub struct Cache {
                items: Vec<(String, String)>,
            }

            impl Cache {
                pub fn new() -> Self {
                    Cache { items: Vec::new() }
                }

                pub fn set(&mut self, key: &str, value: &str) {
                    self.items
                        .retain(|(k, _)| k != key);
                    self.items.push((key.to_string(), value.to_string()));
                }

                pub fn get(&self, key: &str) -> Option<&str> {
                    self.items
                        .iter()
                        .find(|(k, _)| k == key)
                        .map(|(_, v)| v.as_str())
                }

                pub fn len(&self) -> usize {
                    self.items.len()
                }
            }
        }
    }

    // 使用 pub use 重导出，用户可以直接用 api::Connection 而不是 api::internal::database::Connection
    pub use internal::cache::Cache;
    pub use internal::database::Connection;

    // 也可以重导出为不同名称
    pub use internal::database::Connection as DbConn;
}

// ============================================================
// 4. pub(crate) / pub(super) 细粒度可见性
// ============================================================

mod visibility_demo {
    // pub(crate): 在整个 crate 内可见，但不会暴露给外部 crate
    pub(crate) fn crate_level_function() -> &'static str {
        "我在整个 crate 内可见"
    }

    pub mod inner {
        // pub(super): 只对父模块可见
        pub(super) fn parent_only() -> &'static str {
            "我只对父模块 visibility_demo 可见"
        }

        // pub(crate): 对整个 crate 可见
        pub(crate) fn crate_wide() -> &'static str {
            "我在整个 crate 内可见 (来自 inner)"
        }

        // pub(in crate::visibility_demo): 只对指定路径的模块可见
        pub(in crate::visibility_demo) fn specific_path() -> &'static str {
            "我只对 visibility_demo 模块可见"
        }

        // 普通 pub：完全公有
        pub fn fully_public() -> &'static str {
            "我完全公有"
        }

        // 私有（默认）：只在 inner 模块内可见
        fn _private_only() -> &'static str {
            "我是私有的"
        }
    }

    // 在 visibility_demo 中可以访问所有这些
    pub fn demonstrate() -> Vec<String> {
        vec![
            format!("crate_level: {}", crate_level_function()),
            format!("parent_only: {}", inner::parent_only()),
            format!("crate_wide: {}", inner::crate_wide()),
            format!("specific_path: {}", inner::specific_path()),
            format!("fully_public: {}", inner::fully_public()),
            // inner::_private_only()  // 编译错误！私有函数
        ]
    }
}

// ============================================================
// 5. 嵌套 use 和 glob 导入
// ============================================================

mod collections_demo {
    pub mod list {
        pub struct SimpleList<T> {
            items: Vec<T>,
        }

        impl<T: std::fmt::Display> SimpleList<T> {
            pub fn new() -> Self {
                SimpleList { items: Vec::new() }
            }

            pub fn add(&mut self, item: T) {
                self.items.push(item);
            }

            pub fn display(&self) -> String {
                self.items
                    .iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
    }

    pub mod map {
        use std::collections::HashMap;

        pub struct SimpleMap {
            data: HashMap<String, String>,
        }

        impl SimpleMap {
            pub fn new() -> Self {
                SimpleMap {
                    data: HashMap::new(),
                }
            }

            pub fn insert(&mut self, key: &str, value: &str) {
                self.data.insert(key.to_string(), value.to_string());
            }

            pub fn get(&self, key: &str) -> Option<&String> {
                self.data.get(key)
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }
        }
    }

    pub mod set {
        use std::collections::HashSet;

        pub struct SimpleSet {
            data: HashSet<String>,
        }

        impl SimpleSet {
            pub fn new() -> Self {
                SimpleSet {
                    data: HashSet::new(),
                }
            }

            pub fn insert(&mut self, item: &str) -> bool {
                self.data.insert(item.to_string())
            }

            pub fn contains(&self, item: &str) -> bool {
                self.data.contains(item)
            }

            pub fn len(&self) -> usize {
                self.data.len()
            }
        }
    }
}

fn main() {
    println!("=== Lesson 069: use 与可见性 ===\n");

    // ---- 1. use 导入路径 ----
    println!("--- 1. use 导入路径 ---");

    // 不使用 use 的长路径写法：
    let c1 = geometry::shapes::Circle::new(5.0);
    println!("圆(不用use): 面积 = {:.2}", c1.area());

    // 使用 use 简化路径
    use geometry::shapes::Circle;
    use geometry::shapes::Rectangle;

    let c2 = Circle::new(3.0);
    println!("圆(用use):   面积 = {:.2}, 周长 = {:.2}", c2.area(), c2.circumference());

    let r1 = Rectangle::new(4.0, 6.0);
    println!("矩形:        面积 = {:.2}, 周长 = {:.2}", r1.area(), r1.perimeter());

    // 也可以 use 到模块级别（惯用做法：函数 use 到父模块，类型 use 到具体类型）
    use geometry::shapes;
    println!("{}", shapes::describe("圆(r=3)", c2.area()));
    println!("{}", shapes::describe("矩形(4x6)", r1.area()));

    use geometry::utils;
    println!("90° = {:.4} rad", utils::degrees_to_radians(90.0));
    println!("π rad = {:.1}°", utils::radians_to_degrees(std::f64::consts::PI));
    println!();

    // ---- 2. use as 别名 ----
    println!("--- 2. use as 别名 ---");

    // 当两个模块有同名类型时，使用 as 起别名
    use io_simulation::file_reader::Reader as FileReader;
    use io_simulation::network_reader::Reader as NetworkReader;

    let fr = FileReader::new("data.txt");
    let nr = NetworkReader::new("https://example.com/api");
    println!("{}", fr.read());
    println!("{}", nr.read());
    println!();

    // ---- 3. 嵌套 use ----
    println!("--- 3. 嵌套 use ---");

    // 嵌套 use 语法：合并共同前缀
    // 等价于：
    //   use collections_demo::list;
    //   use collections_demo::map;
    //   use collections_demo::set;
    use collections_demo::{list, map, set};

    let mut sl = list::SimpleList::new();
    sl.add(1);
    sl.add(2);
    sl.add(3);
    println!("SimpleList: [{}]", sl.display());

    let mut sm = map::SimpleMap::new();
    sm.insert("name", "Rust");
    sm.insert("version", "2021");
    println!("SimpleMap: name={}, 共{}项",
        sm.get("name").unwrap(),
        sm.len()
    );

    let mut ss = set::SimpleSet::new();
    ss.insert("apple");
    ss.insert("banana");
    ss.insert("apple"); // 重复插入
    println!("SimpleSet: 包含apple={}, 共{}项", ss.contains("apple"), ss.len());

    // 嵌套 use 的更多形式：
    // use std::io::{self, Read, Write};    // self 表示 std::io 本身
    // use std::collections::{HashMap, HashSet, BTreeMap};
    println!();

    // 标准库嵌套 use 示例
    use std::collections::{BTreeMap, BinaryHeap};
    let mut btree = BTreeMap::new();
    btree.insert("c", 3);
    btree.insert("a", 1);
    btree.insert("b", 2);
    // BTreeMap 自动按 key 排序
    println!("BTreeMap (有序): {:?}", btree);

    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    println!("BinaryHeap max: {:?}", heap.peek());
    println!();

    // ---- 4. glob 导入 ----
    println!("--- 4. glob 导入 (use xxx::*) ---");

    // glob 导入会将模块中所有公有项引入当前作用域
    // 通常不推荐在生产代码中使用（可能导致命名冲突、可读性差）
    // 常见于：测试代码 (use super::*) 和 prelude 模式
    //
    // 示例（注释说明）：
    // use geometry::shapes::*;  // 导入 Circle, Rectangle, describe 等所有公有项
    //
    // Prelude 模式：
    // 有些库提供 prelude 模块，方便用户一次性导入常用项：
    // use my_library::prelude::*;

    println!("glob 导入 (use xxx::*) 会导入模块所有公有项");
    println!("适用场景: 测试代码中 use super::*");
    println!("适用场景: prelude 模块 use xxx::prelude::*");
    println!("生产代码中不推荐使用，容易导致命名冲突");
    println!();

    // ---- 5. pub use 重导出 ----
    println!("--- 5. pub use 重导出 ---");

    // 通过 pub use，外部用户可以直接使用 api::Connection
    // 而不需要知道 api::internal::database::Connection
    let conn = api::Connection::new("users_db");
    println!("{}", conn.query("SELECT * FROM users"));

    // 也可以用别名导出的 DbConn
    let conn2 = api::DbConn::new("orders_db");
    println!("{}", conn2.query("SELECT * FROM orders"));

    // 使用重导出的 Cache
    let mut cache = api::Cache::new();
    cache.set("user:1", "Alice");
    cache.set("user:2", "Bob");
    println!("缓存 user:1 = {:?}", cache.get("user:1"));
    println!("缓存大小 = {}", cache.len());
    println!();

    // ---- 6. pub(crate) / pub(super) 细粒度可见性 ----
    println!("--- 6. 细粒度可见性 ---");

    // pub(crate) 函数在 crate 内任何位置都可以调用
    println!("{}", visibility_demo::crate_level_function());

    // pub(crate) 函数来自 inner 模块，在 crate 内也可以调用
    println!("{}", visibility_demo::inner::crate_wide());

    // pub 完全公有的函数
    println!("{}", visibility_demo::inner::fully_public());

    // pub(super) 只对父模块可见，main 不是父模块，所以不能直接调用：
    // visibility_demo::inner::parent_only();  // 编译错误！

    // pub(in crate::visibility_demo) 只对指定模块可见：
    // visibility_demo::inner::specific_path();  // 编译错误！

    // 但可以通过 visibility_demo 的公有函数间接访问
    println!("\n通过 demonstrate() 间接访问:");
    for line in visibility_demo::demonstrate() {
        println!("  {}", line);
    }
    println!();

    // ---- 7. 可见性总结 ----
    println!("--- 7. 可见性级别总结 ---");
    println!("┌─────────────────────────┬────────────────────────────┐");
    println!("│ 可见性修饰符             │ 可见范围                    │");
    println!("├─────────────────────────┼────────────────────────────┤");
    println!("│ (默认，无修饰符)          │ 当前模块及其子模块           │");
    println!("│ pub                     │ 完全公有                    │");
    println!("│ pub(crate)              │ 当前 crate 内               │");
    println!("│ pub(super)              │ 父模块                     │");
    println!("│ pub(in path)            │ 指定的祖先模块               │");
    println!("└─────────────────────────┴────────────────────────────┘");
    println!();

    // ---- 总结 ----
    println!("=== 总结 ===");
    println!("1. use 简化路径，惯例：类型 use 到具体项，函数 use 到父模块");
    println!("2. use as 为同名项起别名，避免冲突");
    println!("3. 嵌套 use 合并共同前缀：use std::{{io, fs}}");
    println!("4. glob 导入 use xxx::* 适合测试和 prelude");
    println!("5. pub use 重导出，简化外部使用路径");
    println!("6. pub(crate)/pub(super)/pub(in path) 细粒度控制可见性");
}
