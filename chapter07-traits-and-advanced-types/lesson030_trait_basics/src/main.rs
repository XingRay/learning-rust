/// # Lesson 030 - Trait 基础
///
/// Trait 是 Rust 中定义共享行为的方式，类似于其他语言中的接口（interface）。
///
/// ## 学习目标
/// - 理解 trait 的定义语法
/// - 学会为类型实现 trait（impl Trait for Type）
/// - 掌握默认方法实现
/// - 了解 trait 作为参数（impl Trait 语法）
/// - 了解 trait 作为返回值
/// - 理解孤儿规则（Orphan Rule）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson030_trait_basics
/// ```

// =============================================================
// Lesson 030: Trait 基础 - 定义共享行为
// =============================================================

use std::fmt;

// ---------------------------------------------------------
// 1. 定义 Trait
// ---------------------------------------------------------
// trait 就像一份"合同"，规定了类型必须提供哪些行为。
// 使用 `trait` 关键字定义。

/// 定义一个 `Summary` trait，任何实现它的类型都能生成摘要
trait Summary {
    // 必须实现的方法（没有函数体）
    fn summarize(&self) -> String;

    // 带默认实现的方法（有函数体）
    // 实现者可以选择覆盖它，也可以直接使用默认实现
    fn preview(&self) -> String {
        format!("(阅读更多: {}...)", self.summarize())
    }
}

/// 定义一个 `Printable` trait
trait Printable {
    fn print(&self);
}

// ---------------------------------------------------------
// 2. 为类型实现 Trait
// ---------------------------------------------------------
// 使用 `impl TraitName for TypeName` 语法

/// 新闻文章结构体
struct NewsArticle {
    title: String,
    author: String,
    content: String,
}

/// 为 NewsArticle 实现 Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("《{}》- 作者: {}", self.title, self.author)
    }

    // 覆盖默认的 preview 方法
    fn preview(&self) -> String {
        // 取内容前 20 个字符作为预览
        let preview_text: String = self.content.chars().take(20).collect();
        format!("{}...", preview_text)
    }
}

impl Printable for NewsArticle {
    fn print(&self) {
        println!("[新闻] {} (作者: {})", self.title, self.author);
    }
}

/// 推文结构体
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

/// 为 Tweet 实现 Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
    // 注意：这里没有覆盖 preview()，所以会使用默认实现
}

impl Printable for Tweet {
    fn print(&self) {
        let status = if self.retweet {
            "转推"
        } else if self.reply {
            "回复"
        } else {
            "原创"
        };
        println!("[推文-{}] @{}: {}", status, self.username, self.content);
    }
}

// ---------------------------------------------------------
// 3. Trait 作为参数（impl Trait 语法）
// ---------------------------------------------------------
// 可以用 `impl Trait` 作为参数类型，接受任何实现了该 trait 的类型。
// 这是 trait bound 的语法糖。

/// 接受任何实现了 Summary 的类型，并打印通知
fn notify(item: &impl Summary) {
    println!("📢 速报: {}", item.summarize());
}

/// 参数可以同时要求多个 trait（使用 + 号）
fn notify_and_print(item: &(impl Summary + Printable)) {
    item.print();
    println!("   摘要: {}", item.summarize());
}

// ---------------------------------------------------------
// 4. Trait 作为返回值
// ---------------------------------------------------------
// 可以用 `impl Trait` 作为返回类型。
// 注意：当返回 impl Trait 时，函数体中只能返回同一种具体类型。

/// 返回一个实现了 Summary 的类型
fn create_default_tweet() -> impl Summary {
    Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust 是一门注重安全和性能的语言！"),
        reply: false,
        retweet: false,
    }
}

// 注意：以下写法是不允许的，因为返回了不同的具体类型
// fn create_summary(is_tweet: bool) -> impl Summary {
//     if is_tweet {
//         Tweet { ... }       // 类型 A
//     } else {
//         NewsArticle { ... } // 类型 B  ← 编译错误！
//     }
// }

// ---------------------------------------------------------
// 5. 为类型实现 Display trait（标准库中的 trait）
// ---------------------------------------------------------
// 我们可以为自己的类型实现标准库中的 trait

struct Point {
    x: f64,
    y: f64,
}

/// 为 Point 实现 fmt::Display trait，使其能用 {} 格式化
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// ---------------------------------------------------------
// 6. 孤儿规则（Orphan Rule / Coherence）
// ---------------------------------------------------------
// 孤儿规则: 要为类型实现 trait，trait 或类型至少有一个是在当前 crate 中定义的。
//
// ✅ 可以：为自定义类型 Point 实现标准库的 Display（类型是我们的）
// ✅ 可以：为标准库类型 Vec<T> 实现自定义的 Summary（trait 是我们的）
// ❌ 不可以：为标准库类型 Vec<T> 实现标准库的 Display（都不是我们的）
//
// 这个规则确保了不会有两个 crate 对同一类型的同一 trait 给出不同实现，
// 从而避免冲突。

/// 为 Vec<String> 实现我们自定义的 Summary trait（trait 是我们的，合法）
/// 注意：需要用 newtype 包装来避免直接在外部类型上实现
struct StringCollection(Vec<String>);

impl Summary for StringCollection {
    fn summarize(&self) -> String {
        let items: Vec<&str> = self.0.iter().map(|s| s.as_str()).take(3).collect();
        format!("集合包含 {} 项: [{}...]", self.0.len(), items.join(", "))
    }
}

fn main() {
    println!("=== Lesson 030: Trait 基础 ===\n");

    // ---------------------------------------------------------
    // 演示 1: 基本 trait 实现
    // ---------------------------------------------------------
    println!("--- 1. 基本 trait 实现 ---");

    let article = NewsArticle {
        title: String::from("Rust 2025 路线图发布"),
        author: String::from("Rust 团队"),
        content: String::from("Rust 编程语言团队今天发布了2025年的发展路线图，重点关注异步生态和编译速度优化..."),
    };

    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("刚学了 Rust 的 trait，太强大了！"),
        reply: false,
        retweet: false,
    };

    // 调用 trait 方法
    println!("文章摘要: {}", article.summarize());
    println!("推文摘要: {}", tweet.summarize());

    // ---------------------------------------------------------
    // 演示 2: 默认方法实现
    // ---------------------------------------------------------
    println!("\n--- 2. 默认方法实现 ---");

    // NewsArticle 覆盖了 preview()，使用自定义实现
    println!("文章预览: {}", article.preview());

    // Tweet 没有覆盖 preview()，使用默认实现
    println!("推文预览: {}", tweet.preview());

    // ---------------------------------------------------------
    // 演示 3: trait 作为参数
    // ---------------------------------------------------------
    println!("\n--- 3. Trait 作为参数 ---");

    // notify 函数可以接受任何实现了 Summary 的类型
    notify(&article);
    notify(&tweet);

    // notify_and_print 要求同时实现 Summary 和 Printable
    println!();
    notify_and_print(&article);
    println!();
    notify_and_print(&tweet);

    // ---------------------------------------------------------
    // 演示 4: trait 作为返回值
    // ---------------------------------------------------------
    println!("\n--- 4. Trait 作为返回值 ---");

    let default_tweet = create_default_tweet();
    println!("默认推文: {}", default_tweet.summarize());
    println!("默认推文预览: {}", default_tweet.preview());

    // ---------------------------------------------------------
    // 演示 5: 为类型实现标准库 trait
    // ---------------------------------------------------------
    println!("\n--- 5. 实现标准库 Display trait ---");

    let point = Point { x: 3.14, y: 2.72 };
    // 因为实现了 Display，可以直接用 {} 格式化
    println!("点的坐标: {}", point);
    // 实现了 Display 后，ToString trait 也自动可用了
    let point_str = point.to_string();
    println!("转为字符串: {}", point_str);

    // ---------------------------------------------------------
    // 演示 6: 孤儿规则相关示例
    // ---------------------------------------------------------
    println!("\n--- 6. 孤儿规则 ---");

    let collection = StringCollection(vec![
        "苹果".to_string(),
        "香蕉".to_string(),
        "橙子".to_string(),
        "葡萄".to_string(),
        "西瓜".to_string(),
    ]);
    println!("{}", collection.summarize());

    println!("\n✅ 孤儿规则总结:");
    println!("   - trait 或类型至少有一个定义在当前 crate 中");
    println!("   - 不能为外部类型实现外部 trait");
    println!("   - 可以用 newtype 模式绕过此限制（见 lesson035）");

    println!("\n🎉 恭喜！你已经完成了 Trait 基础的学习！");
}
