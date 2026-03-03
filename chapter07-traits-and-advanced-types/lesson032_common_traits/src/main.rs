#![allow(dead_code)]

/// # Lesson 032 - 常用 Trait
///
/// Rust 标准库提供了许多重要的 trait，理解它们是编写地道 Rust 代码的关键。
///
/// ## 学习目标
/// - 掌握 Display 和 Debug trait 的实现与区别
/// - 理解 Clone 和 Copy trait 的语义
/// - 了解 PartialEq/Eq 和 PartialOrd/Ord trait
/// - 掌握 Hash、Default、From/Into 等常用 trait
/// - 区分手动实现和 #[derive] 自动派生
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson032_common_traits
/// ```

// =============================================================
// Lesson 032: 常用 Trait - 标准库中的核心 Trait
// =============================================================

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

// ---------------------------------------------------------
// 1. Display 与 Debug
// ---------------------------------------------------------
// Display: 面向用户的格式化输出，使用 {} 占位符
// Debug:   面向开发者的调试输出，使用 {:?} 占位符

/// 手动实现 Display 和 Debug
struct Color {
    red: u8,
    green: u8,
    blue: u8,
    name: String,
}

/// 手动实现 Display —— 面向用户的输出
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(#{:02X}{:02X}{:02X})", self.name, self.red, self.green, self.blue)
    }
}

/// 手动实现 Debug —— 面向开发者的调试输出
impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Color")
            .field("name", &self.name)
            .field("red", &self.red)
            .field("green", &self.green)
            .field("blue", &self.blue)
            .finish()
    }
}

/// 使用 #[derive(Debug)] 自动派生 Debug trait
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ---------------------------------------------------------
// 2. Clone 与 Copy
// ---------------------------------------------------------
// Clone: 显式深拷贝，通过调用 .clone() 方法
// Copy:  隐式按位拷贝（栈上拷贝），赋值时自动复制而非移动
//
// Copy 是 Clone 的子 trait：实现 Copy 必须先实现 Clone
// Copy 只适用于简单类型（不包含堆数据的类型）

/// 实现了 Copy 的简单类型（通过 derive）
#[derive(Debug, Clone, Copy)]
struct Temperature {
    celsius: f64,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.celsius)
    }
}

/// 只实现 Clone 不实现 Copy 的类型（包含 String，不能 Copy）
#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u8,
    grade: f64,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({}岁, 成绩: {:.1})", self.name, self.age, self.grade)
    }
}

// ---------------------------------------------------------
// 3. PartialEq / Eq
// ---------------------------------------------------------
// PartialEq: 部分等价关系（允许存在不等于自身的值，如 f64::NAN）
// Eq:        完全等价关系（保证自反性：a == a 总是 true）
//
// 大部分类型实现 Eq，浮点数只实现 PartialEq（因为 NAN != NAN）

/// 使用 derive 自动实现 PartialEq 和 Eq
#[derive(Debug, PartialEq, Eq)]
struct UserId(u64);

/// 手动实现 PartialEq —— 自定义比较逻辑
#[derive(Debug)]
struct Person {
    name: String,
    id: u64,
}

/// 按 id 比较两个 Person 是否相等（忽略 name）
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// id 是整数，满足自反性，所以可以实现 Eq
impl Eq for Person {}

// ---------------------------------------------------------
// 4. PartialOrd / Ord
// ---------------------------------------------------------
// PartialOrd: 部分排序（可能存在无法比较的值对）
// Ord:        全排序（任意两个值都可以比较）

/// 使用 derive 自动实现所有比较相关 trait
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Score {
    value: u32,
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}分", self.value)
    }
}

/// 手动实现 PartialOrd
#[derive(Debug, PartialEq)]
struct Priority {
    level: u8,
    name: String,
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // 只按 level 比较
        self.level.partial_cmp(&other.level)
    }
}

// ---------------------------------------------------------
// 5. Hash
// ---------------------------------------------------------
// Hash trait 使类型可以用于 HashMap 的键或 HashSet 的元素。
// 规则：如果 a == b，则 hash(a) == hash(b)

#[derive(Debug, Eq)]
struct Employee {
    id: u64,
    name: String,
    department: String,
}

/// 按 id 判断相等
impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Hash 必须与 PartialEq 一致：只按 id 计算 hash
impl Hash for Employee {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

// ---------------------------------------------------------
// 6. Default
// ---------------------------------------------------------
// Default 提供类型的默认值。

/// 使用 derive 自动实现 Default
#[derive(Debug, Default)]
struct Config {
    width: u32,
    height: u32,
    title: String,
    fullscreen: bool,
}

/// 手动实现 Default —— 提供有意义的默认值
#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_seconds: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: String::from("127.0.0.1"),
            port: 8080,
            max_connections: 100,
            timeout_seconds: 30,
        }
    }
}

// ---------------------------------------------------------
// 7. From / Into
// ---------------------------------------------------------
// From<T>: 从类型 T 转换为 Self
// Into<T>: 把 Self 转换为类型 T
//
// 实现了 From<T> 后，Into<T> 会自动获得（blanket implementation）

#[derive(Debug)]
struct Celsius(f64);

#[derive(Debug)]
struct Fahrenheit(f64);

/// 从华氏度转换为摄氏度
impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

/// 从摄氏度转换为华氏度
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

/// 从 &str 创建 Person
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 简单解析 "name:id" 格式
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 2 {
            Person {
                name: parts[0].to_string(),
                id: parts[1].parse().unwrap_or(0),
            }
        } else {
            Person {
                name: s.to_string(),
                id: 0,
            }
        }
    }
}

fn main() {
    println!("=== Lesson 032: 常用 Trait ===\n");

    // ---------------------------------------------------------
    // 演示 1: Display 与 Debug
    // ---------------------------------------------------------
    println!("--- 1. Display 与 Debug ---");

    let red = Color {
        red: 255,
        green: 0,
        blue: 0,
        name: String::from("红色"),
    };

    // Display: 面向用户
    println!("Display: {}", red);
    // Debug: 面向开发者
    println!("Debug:   {:?}", red);
    // Debug 美化输出
    println!("Debug(美化): {:#?}", red);

    let point = Point { x: 1.5, y: 2.7 };
    println!("Display: {}", point);
    println!("Debug:   {:?}", point);

    // ---------------------------------------------------------
    // 演示 2: Clone 与 Copy
    // ---------------------------------------------------------
    println!("\n--- 2. Clone 与 Copy ---");

    // Copy 类型：赋值时自动复制
    let temp1 = Temperature { celsius: 36.5 };
    let temp2 = temp1; // Copy，temp1 仍然可用
    println!("temp1 = {}, temp2 = {}", temp1, temp2);

    // 非 Copy 类型：赋值时移动
    let student1 = Student {
        name: String::from("小明"),
        age: 18,
        grade: 92.5,
    };
    let student2 = student1.clone(); // 必须显式 clone
    // let student3 = student1; // 如果这样做，student1 就被移动了
    println!("student1 = {}", student1);
    println!("student2（克隆）= {}", student2);

    // ---------------------------------------------------------
    // 演示 3: PartialEq / Eq
    // ---------------------------------------------------------
    println!("\n--- 3. PartialEq / Eq ---");

    let id1 = UserId(1001);
    let id2 = UserId(1001);
    let id3 = UserId(1002);
    println!("id1 == id2: {}", id1 == id2); // true
    println!("id1 == id3: {}", id1 == id3); // false

    // 自定义相等逻辑：只比较 id
    let person1 = Person {
        name: String::from("张三"),
        id: 42,
    };
    let person2 = Person {
        name: String::from("张三（别名）"),
        id: 42,
    };
    let person3 = Person {
        name: String::from("李四"),
        id: 43,
    };
    println!("person1 == person2: {} (同 id，不同 name)", person1 == person2);
    println!("person1 == person3: {} (不同 id)", person1 == person3);

    // 浮点数的特殊性
    println!("\n浮点数的特殊性:");
    // 使用变量绑定来演示 NaN 比较，避免编译器警告
    let nan_value: f64 = f64::NAN;
    let nan_value2: f64 = f64::NAN;
    println!("NaN == NaN: {}", nan_value == nan_value2); // false!
    println!("NaN != NaN: {}", nan_value != nan_value2); // true!
    println!("NaN.is_nan(): {}", nan_value.is_nan());    // true

    // ---------------------------------------------------------
    // 演示 4: PartialOrd / Ord
    // ---------------------------------------------------------
    println!("\n--- 4. PartialOrd / Ord ---");

    let score1 = Score { value: 85 };
    let score2 = Score { value: 92 };
    let score3 = Score { value: 78 };
    println!("{} > {}: {}", score1, score2, score1 > score2);
    println!("{} < {}: {}", score1, score2, score1 < score2);

    // 实现了 Ord 就可以排序
    let mut scores = vec![score1.clone(), score2.clone(), score3.clone()];
    scores.sort();
    println!(
        "排序后: {:?}",
        scores.iter().map(|s| s.value).collect::<Vec<_>>()
    );

    let p1 = Priority {
        level: 3,
        name: "高".to_string(),
    };
    let p2 = Priority {
        level: 1,
        name: "低".to_string(),
    };
    println!("优先级 {} > {}: {}", p1.name, p2.name, p1 > p2);

    // ---------------------------------------------------------
    // 演示 5: Hash
    // ---------------------------------------------------------
    println!("\n--- 5. Hash ---");

    let mut department_map: HashMap<Employee, String> = HashMap::new();

    let emp1 = Employee {
        id: 1,
        name: "Alice".to_string(),
        department: "工程".to_string(),
    };
    let emp2 = Employee {
        id: 2,
        name: "Bob".to_string(),
        department: "设计".to_string(),
    };

    department_map.insert(emp1, "一层".to_string());
    department_map.insert(emp2, "二层".to_string());

    // 用相同 id 的 Employee 查询
    let query = Employee {
        id: 1,
        name: "Alice（查询用）".to_string(), // name 不同但 id 相同
        department: String::new(),
    };
    if let Some(floor) = department_map.get(&query) {
        println!("员工 id=1 在: {}", floor);
    }

    // ---------------------------------------------------------
    // 演示 6: Default
    // ---------------------------------------------------------
    println!("\n--- 6. Default ---");

    // derive 的 Default: 所有字段使用各自的默认值
    let config = Config::default();
    println!("默认配置: {:?}", config);
    // 注意：u32 默认 0，String 默认 ""，bool 默认 false

    // 手动实现的 Default: 有意义的默认值
    let server = ServerConfig::default();
    println!("服务器配置: {:?}", server);

    // 使用 struct update 语法部分覆盖默认值
    let custom_server = ServerConfig {
        port: 3000,
        max_connections: 500,
        ..ServerConfig::default()
    };
    println!("自定义服务器: {:?}", custom_server);

    // ---------------------------------------------------------
    // 演示 7: From / Into
    // ---------------------------------------------------------
    println!("\n--- 7. From / Into ---");

    // From: 显式转换
    let boiling = Fahrenheit(212.0);
    let celsius = Celsius::from(boiling);
    println!("212°F = {:?}", celsius);

    // Into: 由 From 自动获得
    let body_temp = Celsius(37.0);
    let fahrenheit: Fahrenheit = body_temp.into();
    println!("37°C = {:?}", fahrenheit);

    // 从字符串创建 Person
    let person: Person = Person::from("王五:100");
    println!("从字符串创建: {:?}", person);

    // Into 的隐式调用（常用于函数参数）
    let person2: Person = "赵六:200".into();
    println!("使用 into(): {:?}", person2);

    // 标准库中的 From 例子
    let s = String::from("hello"); // &str -> String
    let num: i64 = i64::from(42i32); // i32 -> i64（无损转换）
    println!("String::from: {}", s);
    println!("i64::from(42i32): {}", num);

    // ---------------------------------------------------------
    // 总结: #[derive] vs 手动实现
    // ---------------------------------------------------------
    println!("\n--- 总结: #[derive] vs 手动实现 ---");
    println!("✅ #[derive] 适合:");
    println!("   - 标准的逐字段比较、克隆、哈希");
    println!("   - Debug 的默认格式化输出");
    println!("   - 简单类型的 Default（全部默认值）");
    println!();
    println!("✅ 手动实现适合:");
    println!("   - 自定义 Display 格式");
    println!("   - 按特定字段比较（如只按 id 比较 PartialEq）");
    println!("   - 自定义默认值（Default）");
    println!("   - 自定义 Hash（必须与 PartialEq 一致）");
    println!("   - 复杂的 From 转换逻辑");

    println!("\n🎉 恭喜！你已经完成了常用 Trait 的学习！");
}
