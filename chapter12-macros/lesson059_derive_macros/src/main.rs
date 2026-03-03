// ============================================================
// Lesson 059: 派生宏 (Derive Macros) — 概念讲解
// ============================================================
// 本课讲解 #[derive] 属性的工作原理和常用 derive 宏的使用。
//
// #[derive] 是 Rust 中最常用的宏形式之一，它能自动为类型
// 生成 trait 的实现代码，避免大量重复的样板代码。
//
// 自定义 derive 宏需要创建 proc-macro crate（过程宏 crate），
// 本课侧重于使用标准库内置的 derive 宏，并在注释中说明原理。
// ============================================================

use std::fmt;

// ============================================================
// 第一部分：#[derive] 的工作原理
// ============================================================
//
// #[derive] 是一种过程宏 (procedural macro) 的语法糖。
//
// 当编译器看到 #[derive(Debug)] 时，它会：
// 1. 解析被标注的类型（结构体/枚举）的 AST（抽象语法树）
// 2. 将 AST 传递给 Debug derive 宏的实现
// 3. derive 宏生成 impl Debug for YourType { ... } 的代码
// 4. 生成的代码被插入到原始代码旁边
//
// 本质上，#[derive(SomeTrait)] 等价于手动编写：
// impl SomeTrait for YourType {
//     // ... 自动生成的实现
// }
//
// 编译流程：
// 源代码 → 解析AST → 宏展开(包括derive) → 类型检查 → ... → 机器码
//
// derive 宏的限制：
// - 只能用于 struct、enum、union
// - 只能添加新的 impl 块，不能修改原始类型定义
// - 内置 derive 要求所有字段都实现了对应的 trait

// ============================================================
// 第二部分：常用 derive 宏 — Debug
// ============================================================

// Debug — 用于 {:?} 格式化输出
// 这是最常用的 derive，几乎所有类型都应该派生它
#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    grades: Vec<f64>,
}

// Debug 对枚举同样有效
#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended { reason: String },
}

// ============================================================
// 第三部分：Clone 和 Copy
// ============================================================

// Clone — 提供 .clone() 方法进行深拷贝
// Copy — 标记类型为"可复制"，赋值时自动复制而非移动
//
// Copy 要求：
// - 所有字段都必须实现 Copy
// - 必须同时实现 Clone（Copy 是 Clone 的子 trait）
// - 不能包含堆分配的数据（如 String, Vec, Box）

// 可以同时 derive Clone 和 Copy（适用于简单值类型）
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// 只能 derive Clone，不能 Copy（因为包含 String）
#[derive(Debug, Clone)]
struct Person {
    name: String, // String 没有实现 Copy
    age: u32,
}

// ============================================================
// 第四部分：PartialEq, Eq, PartialOrd, Ord
// ============================================================

// PartialEq — 支持 == 和 != 比较
// Eq — 表示等价关系是自反的（a == a 恒为 true）
//       浮点数因为 NaN != NaN 所以只有 PartialEq
//
// PartialOrd — 支持 <, <=, >, >= 比较
// Ord — 全序关系（任意两个值都可比较）
//       浮点数因为 NaN 无法比较所以只有 PartialOrd

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Score {
    points: u32,
    bonus: u32,
}

// 包含浮点数的类型只能 derive PartialEq 和 PartialOrd
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Temperature {
    celsius: f64,
}

// 枚举的比较 — 按变体声明顺序排序
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Priority {
    Low,    // 最小
    Medium, // 中间
    High,   // 最大（最后声明）
}

// ============================================================
// 第五部分：Hash
// ============================================================

// Hash — 使类型可以用作 HashMap 的键
// 要求同时实现 Eq（所以浮点数字段不能用 Hash）

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct UserId {
    id: u64,
    domain: String,
}

// ============================================================
// 第六部分：Default
// ============================================================

// Default — 提供默认值
// 基本类型的默认值：
// - 整数: 0
// - 浮点数: 0.0
// - bool: false
// - char: '\0'
// - String: "" (空字符串)
// - Vec<T>: vec![] (空向量)
// - Option<T>: None

#[derive(Debug, Default)]
struct Config {
    host: String,       // 默认 ""
    port: u16,          // 默认 0
    debug: bool,        // 默认 false
    max_connections: u32, // 默认 0
    tags: Vec<String>,  // 默认 vec![]
}

// 枚举的 Default 需要指定默认变体
#[derive(Debug, Default)]
enum LogLevel {
    Trace,
    Debug,
    #[default] // 指定 Info 为默认值
    Info,
    Warn,
    Error,
}

// ============================================================
// 第七部分：手动实现 vs Derive
// ============================================================

// 有时候 derive 生成的实现不符合需求，需要手动实现

// 示例1：自定义 Debug 格式
struct Password(String);

// 手动实现 Debug，隐藏密码内容
impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Password(****)")
    }
}

// 示例2：自定义 PartialEq — 忽略某些字段
struct User {
    id: u64,
    name: String,
    _cache: Option<String>, // 缓存字段，不参与比较
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        // 只比较 id 和 name，忽略 _cache
        self.id == other.id && self.name == other.name
    }
}
impl Eq for User {}

// 示例3：自定义 Display（无法用 derive）
// Display 不支持 derive，必须手动实现
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

// 示例4：自定义 Clone — 深拷贝时做额外处理
#[derive(Debug)]
struct Connection {
    url: String,
    pool_size: u32,
    _connection_id: u64, // 每个连接应该有独立的 ID
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        static NEXT_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1000);
        Connection {
            url: self.url.clone(),
            pool_size: self.pool_size,
            // 克隆时生成新的连接 ID
            _connection_id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
        }
    }
}

// ============================================================
// 第八部分：Derive 的组合使用和最佳实践
// ============================================================

// 推荐的 derive 组合模式
// 1. 数据传输对象 (DTO): Debug, Clone, PartialEq
// 2. 值类型: Debug, Clone, Copy, PartialEq, Eq, Hash
// 3. 配置类型: Debug, Clone, Default
// 4. 需要排序的类型: + PartialOrd, Ord
// 5. 用作 HashMap 键: + Eq, Hash

// 一个完整的示例——游戏角色
#[derive(Debug, Clone, PartialEq)]
struct Character {
    name: String,
    class: CharacterClass,
    stats: Stats,
    level: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Healer,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Stats {
    health: f64,
    mana: f64,
    attack: f64,
    defense: f64,
}

// ============================================================
// 第九部分：如何创建自定义 derive 宏（概念说明）
// ============================================================
//
// 创建自定义 derive 宏需要以下步骤：
//
// 1. 创建一个新的库 crate，在 Cargo.toml 中设置：
//    ```toml
//    [lib]
//    proc-macro = true
//
//    [dependencies]
//    syn = "2"       # 解析 Rust 代码为 AST
//    quote = "1"     # 将 AST 转回 Rust 代码
//    proc-macro2 = "1"  # proc_macro 的更方便的封装
//    ```
//
// 2. 在 lib.rs 中定义 derive 宏：
//    ```rust
//    use proc_macro::TokenStream;
//    use quote::quote;
//    use syn::{parse_macro_input, DeriveInput};
//
//    #[proc_macro_derive(MyTrait)]
//    pub fn my_trait_derive(input: TokenStream) -> TokenStream {
//        // 解析输入
//        let input = parse_macro_input!(input as DeriveInput);
//        let name = &input.ident; // 获取类型名
//
//        // 生成实现代码
//        let expanded = quote! {
//            impl MyTrait for #name {
//                fn my_method(&self) -> String {
//                    format!("I am a {}", stringify!(#name))
//                }
//            }
//        };
//
//        TokenStream::from(expanded)
//    }
//    ```
//
// 3. 在使用方 crate 中添加依赖并使用：
//    ```rust
//    use my_derive_crate::MyTrait;
//
//    #[derive(MyTrait)]
//    struct Foo;
//
//    fn main() {
//        let foo = Foo;
//        println!("{}", foo.my_method()); // "I am a Foo"
//    }
//    ```
//
// 注意：proc-macro crate 有特殊限制：
// - 只能导出过程宏，不能导出其他项
// - 只能在编译期运行
// - 不能与定义宏的 crate 在同一个 crate 中使用 derive

fn main() {
    println!("=== Lesson 059: 派生宏 (Derive Macros) ===\n");

    // ---- 第二部分：Debug ----
    println!("--- Debug derive ---");
    let student = Student {
        name: "张三".to_string(),
        age: 20,
        grades: vec![95.0, 87.5, 92.0],
    };
    println!("Student (Debug): {:?}", student);
    println!("Student (pretty): {:#?}", student);

    let status = Status::Suspended {
        reason: "违规操作".to_string(),
    };
    println!("Status: {:?}", status);
    println!();

    // ---- 第三部分：Clone 和 Copy ----
    println!("--- Clone 和 Copy ---");

    // Copy 类型：赋值后原值仍可使用
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1; // Copy，不是移动
    println!("p1 = {:?} (Copy 后仍然可用)", p1);
    println!("p2 = {:?}", p2);

    // Clone 类型：需要显式调用 .clone()
    let person1 = Person {
        name: "李四".to_string(),
        age: 25,
    };
    let person2 = person1.clone(); // 显式 clone
    println!("person1 = {:?}", person1);
    println!("person2 = {:?} (clone 的副本)", person2);

    // 如果不 clone，赋值会移动所有权
    let person3 = person1; // 移动，person1 不再可用
    println!("person3 = {:?} (从 person1 移动而来)", person3);
    // println!("{:?}", person1); // 编译错误！person1 已移动
    println!();

    // ---- 第四部分：比较 trait ----
    println!("--- PartialEq, Eq, PartialOrd, Ord ---");

    let s1 = Score {
        points: 100,
        bonus: 10,
    };
    let s2 = Score {
        points: 100,
        bonus: 10,
    };
    let s3 = Score {
        points: 90,
        bonus: 20,
    };

    println!("s1 == s2: {}", s1 == s2);
    println!("s1 == s3: {}", s1 == s3);
    println!("s1 > s3: {}", s1 > s3); // 先比较 points，再比较 bonus

    // 枚举排序
    let mut priorities = vec![Priority::High, Priority::Low, Priority::Medium, Priority::Low];
    priorities.sort();
    println!("排序后的优先级: {:?}", priorities);

    // 温度比较（PartialOrd）
    let t1 = Temperature { celsius: 36.5 };
    let t2 = Temperature { celsius: 38.0 };
    println!("{}°C < {}°C: {}", t1.celsius, t2.celsius, t1 < t2);
    println!();

    // ---- 第五部分：Hash ----
    println!("--- Hash ---");

    let mut user_scores: HashMap<UserId, u32> = HashMap::new();
    let user1 = UserId {
        id: 1,
        domain: "example.com".to_string(),
    };
    let user2 = UserId {
        id: 2,
        domain: "example.com".to_string(),
    };

    user_scores.insert(user1.clone(), 100);
    user_scores.insert(user2.clone(), 200);

    println!("用户分数: {:?}", user_scores);
    println!("user1 的分数: {:?}", user_scores.get(&user1));
    println!();

    // ---- 第六部分：Default ----
    println!("--- Default ---");

    let config = Config::default();
    println!("默认配置: {:#?}", config);

    // 结合 struct update 语法使用 Default
    let custom_config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
        ..Config::default() // 其余字段使用默认值
    };
    println!("自定义配置: {:#?}", custom_config);

    let log_level = LogLevel::default();
    println!("默认日志级别: {:?}", log_level);
    println!();

    // ---- 第七部分：手动实现 vs Derive ----
    println!("--- 手动实现 vs Derive ---");

    // 自定义 Debug — 隐藏密码
    let pwd = Password("super_secret_123".to_string());
    println!("密码: {:?}", pwd); // 输出 Password(****) 而非真实密码

    // 自定义 PartialEq — 忽略缓存字段
    let u1 = User {
        id: 1,
        name: "Alice".to_string(),
        _cache: Some("cached_data".to_string()),
    };
    let u2 = User {
        id: 1,
        name: "Alice".to_string(),
        _cache: None, // 缓存不同
    };
    println!("u1 == u2 (忽略缓存字段): {}", u1 == u2);

    // 自定义 Display
    let color = Color {
        r: 255,
        g: 128,
        b: 0,
    };
    println!("Color Display: {}", color);  // #FF8000
    println!("Color Debug: {:?}", color);   // Color { r: 255, g: 128, b: 0 }

    // 自定义 Clone — 克隆时生成新 ID
    let conn1 = Connection {
        url: "postgres://localhost".to_string(),
        pool_size: 10,
        _connection_id: 1,
    };
    let conn2 = conn1.clone();
    println!("原始连接: {:?}", conn1);
    println!("克隆连接: {:?} (新的 connection_id)", conn2);
    println!();

    // ---- 第八部分：组合使用 ----
    println!("--- Derive 组合使用 ---");

    let warrior = Character {
        name: "亚瑟".to_string(),
        class: CharacterClass::Warrior,
        stats: Stats {
            health: 100.0,
            mana: 30.0,
            attack: 80.0,
            defense: 70.0,
        },
        level: 10,
    };

    let mage = Character {
        name: "梅林".to_string(),
        class: CharacterClass::Mage,
        stats: Stats {
            health: 60.0,
            mana: 100.0,
            attack: 90.0,
            defense: 30.0,
        },
        level: 12,
    };

    println!("战士: {:#?}", warrior);
    println!("法师: {:?}", mage);
    println!("同一个职业? {}", warrior.class == mage.class);

    // Stats 的 Default
    let default_stats = Stats::default();
    println!("默认属性: {:?}", default_stats);

    // CharacterClass 作为 HashMap 的键
    let mut class_count: HashMap<CharacterClass, u32> = HashMap::new();
    class_count.insert(CharacterClass::Warrior, 3);
    class_count.insert(CharacterClass::Mage, 5);
    class_count.insert(CharacterClass::Rogue, 2);
    class_count.insert(CharacterClass::Healer, 4);
    println!("各职业人数: {:?}", class_count);

    // ---- 何时 derive vs 手动实现 总结 ----
    println!("\n--- 何时使用 derive vs 手动实现 ---");
    println!("使用 derive 的情况：");
    println!("  • 标准行为足够：Debug 格式化显示所有字段");
    println!("  • 简单值类型：Copy, Clone, PartialEq 等");
    println!("  • 快速原型开发：先 derive，后续按需手动实现");
    println!();
    println!("手动实现的情况：");
    println!("  • 需要隐藏敏感信息（如 Debug 中隐藏密码）");
    println!("  • 比较时要忽略某些字段（如缓存、ID）");
    println!("  • 需要自定义格式（Display 必须手动实现）");
    println!("  • Clone 时需要额外逻辑（如生成新 ID）");
    println!("  • 性能优化（手动实现可能更高效）");

    println!("\n=== Lesson 059 完成 ===");
}
