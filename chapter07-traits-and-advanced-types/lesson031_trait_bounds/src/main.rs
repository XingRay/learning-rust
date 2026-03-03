#![allow(dead_code)]

/// # Lesson 031 - Trait 约束（Trait Bounds）
///
/// Trait 约束让我们能精确控制泛型参数必须具备哪些能力。
///
/// ## 学习目标
/// - 掌握泛型的 trait bound 语法（T: Display）
/// - 了解多重约束（T: Display + Clone）
/// - 学会使用 where 子句简化复杂约束
/// - 理解有条件的方法实现（blanket implementations）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson031_trait_bounds
/// ```

// =============================================================
// Lesson 031: Trait 约束 - 精确控制泛型行为
// =============================================================

use std::fmt::{self, Display};

// ---------------------------------------------------------
// 1. 基本 Trait Bound 语法
// ---------------------------------------------------------
// `impl Trait` 语法实际上是 trait bound 的语法糖。
// 两种写法是等价的：
//   fn foo(item: &impl Summary)        ← 语法糖
//   fn foo<T: Summary>(item: &T)       ← trait bound 完整写法

trait Summary {
    fn summarize(&self) -> String;
}

trait Category {
    fn category(&self) -> &str;
}

#[derive(Clone)]
struct Article {
    title: String,
    content: String,
    section: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("《{}》", self.title)
    }
}

impl Category for Article {
    fn category(&self) -> &str {
        &self.section
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.section, self.title)
    }
}

/// 使用 trait bound 语法（完整写法）
/// T 必须实现 Summary trait
fn notify_bound<T: Summary>(item: &T) {
    println!("📢 通知: {}", item.summarize());
}

/// 使用 impl Trait 语法（语法糖写法）
/// 与上面的函数功能等价
fn notify_sugar(item: &impl Summary) {
    println!("📢 通知: {}", item.summarize());
}

/// 当两个参数需要是相同类型时，必须使用 trait bound 语法
/// 以下确保 a 和 b 是同一类型 T
fn compare_summaries<T: Summary>(a: &T, b: &T) {
    println!("比较: {} vs {}", a.summarize(), b.summarize());
}

// 如果用 impl Trait，a 和 b 可以是不同类型：
// fn compare_summaries(a: &impl Summary, b: &impl Summary)
// 这里 a 可以是 Article，b 可以是 Tweet

// ---------------------------------------------------------
// 2. 多重 Trait Bound（使用 + 号）
// ---------------------------------------------------------
// 可以要求类型同时实现多个 trait

/// 要求 T 同时实现 Summary 和 Display
fn display_summary<T: Summary + Display>(item: &T) {
    println!("显示: {}", item);            // 使用 Display
    println!("摘要: {}", item.summarize()); // 使用 Summary
}

/// 要求 T 同时实现 Summary、Category 和 Display
fn full_info<T: Summary + Category + Display>(item: &T) {
    println!("完整信息: {} [分类: {}]", item, item.category());
}

// ---------------------------------------------------------
// 3. where 子句
// ---------------------------------------------------------
// 当 trait bound 变得复杂时，函数签名会变得难以阅读。
// where 子句让我们把约束移到函数签名之后。

/// 不使用 where 子句（难以阅读）
fn _complex_function_without_where<T: Display + Clone + Summary, U: Display + Category>(
    _t: &T,
    _u: &U,
) {
    // ...
}

/// 使用 where 子句（清晰易读）
fn complex_function<T, U>(t: &T, u: &U)
where
    T: Display + Clone + Summary,
    U: Display + Category,
{
    println!("T 的显示: {}", t);
    println!("T 的摘要: {}", t.summarize());
    let _t_clone = t.clone();
    println!("U 的显示: {}", u);
    println!("U 的分类: {}", u.category());
}

// ---------------------------------------------------------
// 4. 使用 trait bound 有条件地实现方法
// ---------------------------------------------------------

/// 一个泛型容器
struct Container<T> {
    value: T,
    label: String,
}

impl<T> Container<T> {
    /// 所有 Container<T> 都有的方法，不需要任何 trait bound
    fn new(value: T, label: &str) -> Self {
        Container {
            value,
            label: label.to_string(),
        }
    }

    /// 获取标签
    fn label(&self) -> &str {
        &self.label
    }
}

/// 只有当 T 实现了 Display 时，才能调用 show 方法
impl<T: Display> Container<T> {
    fn show(&self) {
        println!("[{}]: {}", self.label, self.value);
    }
}

/// 只有当 T 同时实现 Display 和 PartialOrd 时，才能调用 is_greater_than
impl<T: Display + PartialOrd> Container<T> {
    fn is_greater_than(&self, other: &T) -> bool {
        self.value > *other
    }

    fn compare_with(&self, other: &T) {
        if self.value > *other {
            println!("{} 大于 {}", self.value, other);
        } else if self.value < *other {
            println!("{} 小于 {}", self.value, other);
        } else {
            println!("{} 等于 {}", self.value, other);
        }
    }
}

/// 只有当 T 实现了 Clone 时，才能调用 duplicate
impl<T: Clone> Container<T> {
    fn duplicate(&self) -> Container<T> {
        Container {
            value: self.value.clone(),
            label: format!("{}_copy", self.label),
        }
    }
}

// ---------------------------------------------------------
// 5. Blanket Implementation（毯子实现/全面实现）
// ---------------------------------------------------------
// 为所有满足某些 trait bound 的类型统一实现 trait。
// 标准库大量使用这种模式。

/// 自定义一个 Describable trait
trait Describable {
    fn describe(&self) -> String;
}

/// 为所有实现了 Display 的类型自动实现 Describable
/// 这就是 blanket implementation
impl<T: Display> Describable for T {
    fn describe(&self) -> String {
        format!("值为「{}」的对象", self)
    }
}

// 标准库中的经典例子：
// impl<T: Display> ToString for T { ... }
// 任何实现了 Display 的类型，都自动获得 to_string() 方法！

// ---------------------------------------------------------
// 6. 函数返回值的 trait bound
// ---------------------------------------------------------

/// 返回实现了指定 trait 的类型
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 使用 Clone + PartialOrd 返回值的拷贝
fn largest_cloned<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in &list[1..] {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}

fn main() {
    println!("=== Lesson 031: Trait 约束 ===\n");

    // ---------------------------------------------------------
    // 演示 1: 基本 trait bound
    // ---------------------------------------------------------
    println!("--- 1. 基本 Trait Bound ---");

    let article1 = Article {
        title: String::from("Rust 异步编程指南"),
        content: String::from("异步编程是现代系统开发的重要组成部分..."),
        section: String::from("技术"),
    };

    let article2 = Article {
        title: String::from("Rust 并发模型详解"),
        content: String::from("Rust 的所有权系统天然支持安全并发..."),
        section: String::from("技术"),
    };

    // 两种语法等价
    notify_bound(&article1);
    notify_sugar(&article1);

    // 使用 trait bound 确保相同类型
    compare_summaries(&article1, &article2);

    // ---------------------------------------------------------
    // 演示 2: 多重 trait bound
    // ---------------------------------------------------------
    println!("\n--- 2. 多重 Trait Bound ---");

    display_summary(&article1);
    println!();
    full_info(&article1);

    // ---------------------------------------------------------
    // 演示 3: where 子句
    // ---------------------------------------------------------
    println!("\n--- 3. Where 子句 ---");

    complex_function(&article1, &article2);

    // ---------------------------------------------------------
    // 演示 4: 有条件的方法实现
    // ---------------------------------------------------------
    println!("\n--- 4. 有条件的方法实现 ---");

    let num_container = Container::new(42, "数字");
    let str_container = Container::new("Hello Rust", "字符串");

    // 所有 Container 都有 label() 方法
    println!("标签: {}", num_container.label());
    println!("标签: {}", str_container.label());

    // 因为 i32 实现了 Display，所以可以调用 show
    num_container.show();
    str_container.show();

    // 因为 i32 实现了 Display + PartialOrd，可以调用 compare_with
    num_container.compare_with(&50);
    num_container.compare_with(&30);
    println!("42 > 50? {}", num_container.is_greater_than(&50));

    // 因为 i32 实现了 Clone，可以调用 duplicate
    let num_copy = num_container.duplicate();
    num_copy.show();

    // ---------------------------------------------------------
    // 演示 5: Blanket Implementation
    // ---------------------------------------------------------
    println!("\n--- 5. Blanket Implementation ---");

    // 因为 i32 实现了 Display，它自动获得了 Describable
    let num = 42;
    println!("{}", num.describe());

    // 因为 &str 实现了 Display，它也自动获得了 Describable
    let text = "Rust";
    println!("{}", text.describe());

    // 因为 f64 实现了 Display，它也自动获得了 Describable
    let pi = 3.14159;
    println!("{}", pi.describe());

    // 因为 Article 实现了 Display，它也自动获得了 Describable
    println!("{}", article1.describe());

    // 标准库中的例子：to_string() 来自 blanket implementation
    println!("i32 转字符串: {}", 123.to_string());

    // ---------------------------------------------------------
    // 演示 6: 带 trait bound 的泛型函数
    // ---------------------------------------------------------
    println!("\n--- 6. 带 Trait Bound 的泛型函数 ---");

    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大数字: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("最大字符: {}", largest(&chars));

    // 使用 largest_cloned 获取拷贝
    let result = largest_cloned(&numbers);
    println!("最大数字（拷贝）: {}", result);

    let words = vec![
        String::from("苹果"),
        String::from("香蕉"),
        String::from("橙子"),
    ];
    let largest_word = largest_cloned(&words);
    println!("最大字符串（拷贝）: {}", largest_word);

    println!("\n🎉 恭喜！你已经完成了 Trait 约束的学习！");
}
