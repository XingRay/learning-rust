/// # Lesson 014 - 枚举基础
///
/// 枚举（enum）允许你定义一个类型，它的值只能是几个确定的变体之一。
///
/// ## 学习目标
/// - 理解枚举的定义与基本用法
/// - 掌握无数据变体、元组变体、结构体变体
/// - 学会为枚举实现方法（impl）
/// - 理解用枚举表示多种类型
/// - 了解枚举的内存大小
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson014_enum_basics
/// ```

// =============================================================
// Lesson 014: 枚举基础 - Enum Basics
// =============================================================

// ---------------------------------------------------------
// 1. 基本枚举定义 - 无数据变体
// ---------------------------------------------------------
// 最简单的枚举：每个变体只是一个名字，没有附加数据
// 类似于 C/Java 中的枚举
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ---------------------------------------------------------
// 2. 带数据的枚举变体
// ---------------------------------------------------------
// Rust 枚举的强大之处：每个变体可以携带不同类型和数量的数据

#[derive(Debug)]
enum IpAddr {
    // 元组变体（Tuple Variant）- 像元组一样携带数据
    V4(u8, u8, u8, u8), // IPv4 地址由 4 个字节组成
    V6(String),          // IPv6 地址用字符串表示
}

// ---------------------------------------------------------
// 3. 混合不同类型的变体
// ---------------------------------------------------------
// 一个枚举中可以混合不同风格的变体
#[derive(Debug)]
enum Message {
    // 无数据变体（Unit Variant）- 类似于单元结构体
    Quit,

    // 结构体变体（Struct Variant）- 有命名字段
    Move { x: i32, y: i32 },

    // 元组变体（Tuple Variant）- 有位置数据
    Write(String),

    // 多字段元组变体
    ChangeColor(u8, u8, u8),
}

// ---------------------------------------------------------
// 4. 为枚举实现方法
// ---------------------------------------------------------
// 枚举也可以有 impl 块，就像结构体一样
impl Message {
    /// 处理消息
    fn process(&self) {
        // 使用 match 来处理不同的变体
        match self {
            Message::Quit => {
                println!("  处理: 收到退出消息");
            }
            Message::Move { x, y } => {
                println!("  处理: 移动到坐标 ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("  处理: 写入文本 \"{}\"", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("  处理: 改变颜色为 RGB({}, {}, {})", r, g, b);
            }
        }
    }

    /// 判断是否为退出消息
    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }

    /// 获取消息的描述
    fn description(&self) -> String {
        match self {
            Message::Quit => String::from("退出"),
            Message::Move { x, y } => format!("移动({}, {})", x, y),
            Message::Write(text) => format!("写入({})", text),
            Message::ChangeColor(r, g, b) => format!("变色({},{},{})", r, g, b),
        }
    }
}

// ---------------------------------------------------------
// 5. 用枚举表示多种类型
// ---------------------------------------------------------
// 枚举的一个重要用途：在需要统一类型的地方表示多种不同的值
// 例如：配置项可能是字符串、数字或布尔值

#[derive(Debug)]
enum ConfigValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    List(Vec<String>),
}

impl ConfigValue {
    /// 尝试获取文本值
    fn as_text(&self) -> Option<&str> {
        match self {
            ConfigValue::Text(s) => Some(s),
            _ => None,
        }
    }

    /// 尝试获取数字值
    fn as_number(&self) -> Option<f64> {
        match self {
            ConfigValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// 尝试获取布尔值
    fn as_bool(&self) -> Option<bool> {
        match self {
            ConfigValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

// ---------------------------------------------------------
// 6. 类 C 枚举（可指定判别值）
// ---------------------------------------------------------
// 可以给枚举变体指定整数值，类似 C 语言的枚举
#[derive(Debug)]
#[allow(dead_code)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalError = 500,
}

// ---------------------------------------------------------
// 7. 递归枚举（需要使用 Box）
// ---------------------------------------------------------
// 枚举可以引用自身，但必须通过 Box<T> 进行间接引用
// 因为编译器需要在编译时知道类型的大小
#[derive(Debug)]
enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
}

impl Expr {
    /// 计算表达式的值
    fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(left, right) => left.eval() + right.eval(),
            Expr::Multiply(left, right) => left.eval() * right.eval(),
        }
    }
}

// ---------------------------------------------------------
// 8. 实际场景：形状计算
// ---------------------------------------------------------
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "圆形",
            Shape::Rectangle { .. } => "矩形",
            Shape::Triangle { .. } => "三角形",
        }
    }
}

fn main() {
    println!("=== Lesson 014: 枚举基础 ===\n");

    // ---------------------------------------------------------
    // 1. 基本枚举的使用
    // ---------------------------------------------------------
    println!("--- 1. 基本枚举 ---");

    // 使用 :: 语法访问枚举变体
    let dir = Direction::North;
    println!("方向: {:?}", dir);

    // 枚举在条件判断中的使用
    if dir == Direction::North {
        println!("正在向北移动");
    }

    // 遍历方向（手动列举）
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for d in &directions {
        let chinese = match d {
            Direction::North => "北",
            Direction::South => "南",
            Direction::East => "东",
            Direction::West => "西",
        };
        println!("  {:?} => {}", d, chinese);
    }

    // ---------------------------------------------------------
    // 2. 带数据的枚举
    // ---------------------------------------------------------
    println!("\n--- 2. 带数据的枚举 ---");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("家庭地址: {:?}", home);
    println!("回环地址: {:?}", loopback);

    // 使用 match 解构枚举中的数据
    match &home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 地址: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 地址: {}", addr);
        }
    }

    // ---------------------------------------------------------
    // 3. 混合变体枚举
    // ---------------------------------------------------------
    println!("\n--- 3. 混合变体枚举 ---");

    // 创建不同变体的消息
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
    ];

    // 枚举的优势：不同变体可以存在同一个 Vec 中（因为它们是同一类型）
    for msg in &messages {
        println!("消息 {:?}:", msg);
        msg.process();
    }

    // ---------------------------------------------------------
    // 4. 枚举方法
    // ---------------------------------------------------------
    println!("\n--- 4. 枚举方法 ---");

    let msg = Message::Write(String::from("Rust 真棒"));
    println!("是退出消息? {}", msg.is_quit());
    println!("描述: {}", msg.description());

    let quit_msg = Message::Quit;
    println!("是退出消息? {}", quit_msg.is_quit());
    println!("描述: {}", quit_msg.description());

    // ---------------------------------------------------------
    // 5. 枚举表示多种类型
    // ---------------------------------------------------------
    println!("\n--- 5. 枚举表示多种类型 ---");

    // 使用枚举在同一个 Vec 中存储不同类型的值
    let config: Vec<(&str, ConfigValue)> = vec![
        ("host", ConfigValue::Text(String::from("localhost"))),
        ("port", ConfigValue::Number(8080.0)),
        ("debug", ConfigValue::Boolean(true)),
        (
            "features",
            ConfigValue::List(vec![
                String::from("auth"),
                String::from("logging"),
            ]),
        ),
    ];

    for (key, value) in &config {
        println!("  {} = {:?}", key, value);
    }

    // 使用辅助方法获取特定类型的值
    let host_value = &config[0].1;
    if let Some(host) = host_value.as_text() {
        println!("主机: {}", host);
    }

    let port_value = &config[1].1;
    if let Some(port) = port_value.as_number() {
        println!("端口: {}", port as u16);
    }

    // ---------------------------------------------------------
    // 6. 类 C 枚举
    // ---------------------------------------------------------
    println!("\n--- 6. 类 C 枚举（带判别值） ---");

    // 枚举变体可以转换为整数
    println!("200 OK: {}", HttpStatus::Ok as i32);
    println!("404 Not Found: {}", HttpStatus::NotFound as i32);
    println!("500 Internal Error: {}", HttpStatus::InternalError as i32);

    // ---------------------------------------------------------
    // 7. 递归枚举
    // ---------------------------------------------------------
    println!("\n--- 7. 递归枚举 ---");

    // 表达式: (2 + 3) * 4
    let expr = Expr::Multiply(
        Box::new(Expr::Add(
            Box::new(Expr::Number(2.0)),
            Box::new(Expr::Number(3.0)),
        )),
        Box::new(Expr::Number(4.0)),
    );

    println!("表达式: {:?}", expr);
    println!("结果: (2 + 3) * 4 = {}", expr.eval());

    // 表达式: 1 + 2 + 3
    let expr2 = Expr::Add(
        Box::new(Expr::Add(
            Box::new(Expr::Number(1.0)),
            Box::new(Expr::Number(2.0)),
        )),
        Box::new(Expr::Number(3.0)),
    );
    println!("结果: 1 + 2 + 3 = {}", expr2.eval());

    // ---------------------------------------------------------
    // 8. 枚举的内存大小
    // ---------------------------------------------------------
    println!("\n--- 8. 枚举的内存大小 ---");

    // 枚举的大小 = 判别标签的大小 + 最大变体的大小
    // Rust 会进行内存对齐优化
    println!(
        "Direction 大小: {} 字节（4 个无数据变体）",
        std::mem::size_of::<Direction>()
    );
    println!(
        "IpAddr 大小: {} 字节（最大变体 V6 含 String）",
        std::mem::size_of::<IpAddr>()
    );
    println!(
        "Message 大小: {} 字节（最大变体含 String）",
        std::mem::size_of::<Message>()
    );
    println!(
        "HttpStatus 大小: {} 字节（类 C 枚举）",
        std::mem::size_of::<HttpStatus>()
    );

    // 所有变体共享同一块内存空间，大小由最大的变体决定
    // 这意味着即使某个变体很小，它仍然占用最大变体的空间
    println!("\n每个变体的数据大小（不含标签）:");
    println!(
        "  () 单元类型: {} 字节",
        std::mem::size_of::<()>()
    );
    println!(
        "  (i32, i32) 结构体变体数据: {} 字节",
        std::mem::size_of::<(i32, i32)>()
    );
    println!(
        "  String: {} 字节",
        std::mem::size_of::<String>()
    );
    println!(
        "  (u8, u8, u8): {} 字节",
        std::mem::size_of::<(u8, u8, u8)>()
    );

    // ---------------------------------------------------------
    // 9. 综合示例：形状计算
    // ---------------------------------------------------------
    println!("\n--- 9. 综合示例：形状计算 ---");

    let shapes: Vec<Shape> = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 3.0,
        },
        Shape::Triangle {
            base: 8.0,
            height: 6.0,
        },
    ];

    let mut total_area = 0.0;
    for shape in &shapes {
        let area = shape.area();
        println!("  {}: 面积 = {:.2}", shape.name(), area);
        total_area += area;
    }
    println!("  总面积: {:.2}", total_area);

    // ---------------------------------------------------------
    // 10. 枚举 vs 结构体的选择
    // ---------------------------------------------------------
    println!("\n--- 10. 何时使用枚举 vs 结构体 ---");
    println!("使用结构体：当数据总是有相同的字段时");
    println!("  例如：User 总是有 name, email, age");
    println!("使用枚举：当数据可能是几种不同形态之一时");
    println!("  例如：Shape 可以是 Circle 或 Rectangle 或 Triangle");
    println!("  例如：Message 可以是 Quit 或 Move 或 Write");
    println!("  例如：Result<T, E> 可以是 Ok(T) 或 Err(E)");

    println!("\n🎉 恭喜！你已经完成了枚举基础的学习！");
}
