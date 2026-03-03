/// # Lesson 018 - 高级模式匹配 (Advanced Pattern Matching)
///
/// 模式匹配是 Rust 最强大的控制流工具之一。
///
/// ## 学习目标
/// - 掌握解构结构体、枚举、元组和引用
/// - 学会嵌套模式和或模式 (|)
/// - 理解范围模式 (..=)、ref/ref mut
/// - 掌握守卫条件 (if) 和 @ 绑定
/// - 学会使用 _ 和 .. 忽略值
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson018_pattern_matching
/// ```

// =============================================================
// Lesson 018: 高级模式匹配 - Rust 的瑞士军刀
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 解构结构体
    // ---------------------------------------------------------
    println!("--- 解构结构体 ---");

    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };

    // 解构结构体到变量
    let Point { x, y } = point;
    println!("x = {}, y = {}", x, y);

    // 使用不同的变量名
    let point = Point { x: 3, y: 7 };
    let Point { x: a, y: b } = point;
    println!("a = {}, b = {}", a, b);

    // 在 match 中解构结构体，并使用字面值匹配部分字段
    let point = Point { x: 0, y: 7 };
    match point {
        Point { x: 0, y } => println!("在 y 轴上，y = {}", y),
        Point { x, y: 0 } => println!("在 x 轴上，x = {}", x),
        Point { x, y } => println!("不在任何轴上: ({}, {})", x, y),
    }

    // 嵌套结构体解构
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let rect = Rectangle {
        top_left: Point { x: 0, y: 10 },
        bottom_right: Point { x: 20, y: 0 },
    };

    let Rectangle {
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 },
    } = rect;
    println!(
        "矩形: 左上({}, {}), 右下({}, {})",
        x1, y1, x2, y2
    );

    // ---------------------------------------------------------
    // 2. 解构枚举
    // ---------------------------------------------------------
    println!("\n--- 解构枚举 ---");

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("hello".to_string()),
        Message::ChangeColor(255, 0, 128),
    ];

    for msg in &messages {
        match msg {
            Message::Quit => println!("退出消息"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("文本消息: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("改变颜色: RGB({}, {}, {})", r, g, b)
            }
        }
    }

    // 嵌套枚举解构
    #[derive(Debug)]
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    #[derive(Debug)]
    enum DrawCommand {
        SetColor(Color),
        DrawLine { start: Point, end: Point },
    }

    let cmd = DrawCommand::SetColor(Color::Hsv(120, 100, 50));
    match cmd {
        DrawCommand::SetColor(Color::Rgb(r, g, b)) => {
            println!("设置 RGB 颜色: ({}, {}, {})", r, g, b);
        }
        DrawCommand::SetColor(Color::Hsv(h, s, v)) => {
            println!("设置 HSV 颜色: ({}, {}, {})", h, s, v);
        }
        DrawCommand::DrawLine {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        } => {
            println!("画线: ({}, {}) -> ({}, {})", x1, y1, x2, y2);
        }
    }

    // ---------------------------------------------------------
    // 3. 解构元组和引用
    // ---------------------------------------------------------
    println!("\n--- 解构元组和引用 ---");

    // 解构元组
    let (a, b, c) = (1, 2.0, "三");
    println!("元组: a={}, b={}, c={}", a, b, c);

    // 嵌套元组解构
    let ((x, y), z) = ((1, 2), 3);
    println!("嵌套元组: x={}, y={}, z={}", x, y, z);

    // 解构引用
    let reference = &42;
    // 方式一：使用 & 模式
    let &value = reference;
    println!("解构引用: value = {}", value);

    // 方式二：在 match 中解构引用
    let points = vec![Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
    for &Point { x, y } in points.iter() {
        println!("迭代解构: ({}, {})", x, y);
    }

    // ---------------------------------------------------------
    // 4. 或模式 (|)
    // ---------------------------------------------------------
    println!("\n--- 或模式 | ---");

    // 使用 | 匹配多个值
    let x = 3;
    match x {
        1 | 2 => println!("{} 是一或二", x),
        3 | 4 => println!("{} 是三或四", x),
        _ => println!("{} 是其他数字", x),
    }

    // 或模式也适用于枚举
    #[derive(Debug)]
    enum Fruit {
        Apple,
        Banana,
        Cherry,
        Durian,
    }

    let fruit = Fruit::Cherry;
    match fruit {
        Fruit::Apple | Fruit::Cherry => println!("{:?} 是红色水果", fruit),
        Fruit::Banana => println!("香蕉是黄色的"),
        Fruit::Durian => println!("榴莲很臭但很好吃"),
    }

    // ---------------------------------------------------------
    // 5. 范围模式 (..=)
    // ---------------------------------------------------------
    println!("\n--- 范围模式 ..= ---");

    let score = 85;
    let grade = match score {
        90..=100 => "优秀 (A)",
        80..=89 => "良好 (B)",
        70..=79 => "中等 (C)",
        60..=69 => "及格 (D)",
        0..=59 => "不及格 (F)",
        _ => "无效分数",
    };
    println!("分数 {} 对应等级: {}", score, grade);

    // 字符范围
    let ch = 'z';
    match ch {
        'a'..='z' => println!("'{}' 是小写字母", ch),
        'A'..='Z' => println!("'{}' 是大写字母", ch),
        '0'..='9' => println!("'{}' 是数字", ch),
        _ => println!("'{}' 是其他字符", ch),
    }

    // 组合范围和或模式
    let num = 15;
    match num {
        1..=5 | 11..=15 => println!("{} 在 1-5 或 11-15 范围内", num),
        6..=10 => println!("{} 在 6-10 范围内", num),
        _ => println!("{} 在其他范围", num),
    }

    // ---------------------------------------------------------
    // 6. ref 和 ref mut
    // ---------------------------------------------------------
    println!("\n--- ref 和 ref mut ---");

    // ref 用于创建引用，而非移动值
    let robot_name = Some(String::from("Bors"));

    // 使用 ref 来借用而非移动 String
    match robot_name {
        Some(ref name) => println!("机器人名字: {}", name),
        None => println!("没有名字"),
    }
    // 因为使用了 ref，robot_name 仍然可以使用
    println!("robot_name 仍然有效: {:?}", robot_name);

    // ref mut 用于可变借用
    let mut score = Some(100);
    match score {
        Some(ref mut val) => {
            *val += 50; // 修改 Option 内部的值
            println!("修改后的分数: {}", val);
        }
        None => println!("没有分数"),
    }
    println!("score 变为: {:?}", score);

    // 注意：在现代 Rust 中，match 会自动引用（match ergonomics），
    // 所以很多情况下不需要显式写 ref：
    let name = Some(String::from("Ferris"));
    match &name {
        // 这里自动对 name 取引用，n 的类型是 &String
        Some(n) => println!("名字: {}", n),
        None => println!("无名"),
    }
    println!("name 仍有效: {:?}", name);

    // ---------------------------------------------------------
    // 7. 守卫条件 (Match Guard) if
    // ---------------------------------------------------------
    println!("\n--- 守卫条件 if ---");

    // 守卫条件可以在模式匹配后添加额外的 if 检查
    let num = Some(42);
    match num {
        Some(x) if x < 0 => println!("{} 是负数", x),
        Some(x) if x == 0 => println!("是零"),
        Some(x) if x > 0 && x <= 100 => println!("{} 在 1-100 之间", x),
        Some(x) => println!("{} 大于 100", x),
        None => println!("没有值"),
    }

    // 守卫条件与或模式组合
    // 注意：守卫条件应用于整个或模式，而不仅仅是最后一个
    let x = 4;
    let y = false;
    match x {
        // if y 应用于 4 | 5 | 6 整体
        4 | 5 | 6 if y => println!("匹配到 4/5/6 且 y 为 true"),
        4 | 5 | 6 => println!("匹配到 4/5/6 但 y 为 false"),
        _ => println!("其他"),
    }

    // 实际应用：根据温度和湿度给出建议
    let temperature = 35;
    let humidity = 80;
    match temperature {
        t if t > 35 && humidity > 70 => {
            println!("🔥 温度 {}°C，湿度 {}%：极端闷热，注意防暑！", t, humidity)
        }
        t if t > 30 => {
            println!("☀️ 温度 {}°C：炎热，注意防晒", t)
        }
        t if t > 20 => {
            println!("🌤️ 温度 {}°C：舒适", t)
        }
        t if t > 10 => {
            println!("🌥️ 温度 {}°C：凉爽，建议加件外套", t)
        }
        t => println!("❄️ 温度 {}°C：寒冷，注意保暖", t),
    }

    // ---------------------------------------------------------
    // 8. @ 绑定
    // ---------------------------------------------------------
    println!("\n--- @ 绑定 ---");

    // @ 让我们在测试模式的同时将值绑定到变量
    let age: u32 = 25;
    match age {
        // 匹配 0-17 的范围，并将匹配的值绑定到 n
        n @ 0..=17 => println!("未成年: {} 岁", n),
        n @ 18..=64 => println!("成年人: {} 岁", n),
        n @ 65.. => println!("老年人: {} 岁", n),
    }

    // @ 绑定与枚举结合
    #[derive(Debug)]
    enum Greeting {
        Hello(String),
    }

    let greeting = Greeting::Hello("Rust".to_string());
    match greeting {
        // 匹配 Hello 变体并将整个内部字符串绑定到 name
        Greeting::Hello(ref name @ _) => {
            println!("问候: Hello, {}!", name);
        }
    }

    // @ 绑定的更实用的例子
    struct Config {
        max_retries: u32,
    }

    let config = Config { max_retries: 5 };
    match config.max_retries {
        0 => println!("不重试"),
        n @ 1..=3 => println!("少量重试: {} 次", n),
        n @ 4..=10 => println!("中等重试: {} 次", n),
        n => println!("大量重试: {} 次（建议减少）", n),
    }

    // ---------------------------------------------------------
    // 9. 忽略值：_ 和 ..
    // ---------------------------------------------------------
    println!("\n--- 忽略值 _ 和 .. ---");

    // 使用 _ 忽略单个值
    let (first, _, third) = (1, 2, 3);
    println!("第一个: {}, 第三个: {}", first, third);

    // 使用 _ 前缀忽略未使用的变量（避免编译器警告）
    let _unused_variable = 42; // 不会产生"未使用变量"警告

    // 使用 .. 忽略剩余的值
    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 1, y: 2, z: 3 };

    // 只关心 x，忽略其余字段
    let Point3D { x, .. } = origin;
    println!("只关心 x = {}", x);

    // 在元组中使用 ..
    let numbers = (1, 2, 3, 4, 5, 6, 7, 8);
    let (first, .., last) = numbers;
    println!("第一个: {}, 最后一个: {}", first, last);

    // 在 match 中忽略枚举的部分数据
    let msg = Message::ChangeColor(255, 128, 0);
    match msg {
        Message::Quit => println!("退出"),
        Message::Move { .. } => println!("移动（忽略坐标）"),
        Message::Write(_) => println!("写入（忽略内容）"),
        Message::ChangeColor(r, _, _) => println!("颜色（只关心红色通道: {}）", r),
    }

    // 多个 _ 占位符
    let numbers = (2, 4, 6, 8, 10);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("奇数位: {}, {}, {}", first, third, fifth);
        }
    }

    // ---------------------------------------------------------
    // 10. 嵌套模式的综合示例
    // ---------------------------------------------------------
    println!("\n--- 嵌套模式综合示例 ---");

    #[derive(Debug)]
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
    }

    #[derive(Debug)]
    struct DrawItem {
        shape: Shape,
        color: Color,
        visible: bool,
    }

    let items = vec![
        DrawItem {
            shape: Shape::Circle { radius: 5.0 },
            color: Color::Rgb(255, 0, 0),
            visible: true,
        },
        DrawItem {
            shape: Shape::Rectangle {
                width: 10.0,
                height: 20.0,
            },
            color: Color::Hsv(240, 100, 100),
            visible: false,
        },
        DrawItem {
            shape: Shape::Triangle {
                base: 8.0,
                height: 6.0,
            },
            color: Color::Rgb(0, 255, 0),
            visible: true,
        },
    ];

    for item in &items {
        match item {
            // 不可见的项目直接跳过
            DrawItem { visible: false, .. } => {
                println!("跳过不可见的图形");
            }
            // 可见的红色圆形
            DrawItem {
                shape: Shape::Circle { radius },
                color: Color::Rgb(255, 0, 0),
                visible: true,
            } => {
                println!("🔴 红色圆形，半径: {}", radius);
            }
            // 其他可见图形
            DrawItem {
                shape,
                color,
                visible: true,
            } => {
                println!("可见图形: {:?}, 颜色: {:?}", shape, color);
            }
        }
    }

    // ---------------------------------------------------------
    // 11. 总结
    // ---------------------------------------------------------
    println!("\n--- 总结 ---");
    println!("📌 解构: 可以解构结构体、枚举、元组、引用");
    println!("📌 嵌套: 模式可以任意层次嵌套");
    println!("📌 或模式: 使用 | 匹配多个选项");
    println!("📌 范围: 使用 ..= 匹配值范围");
    println!("📌 ref/ref mut: 在模式中创建引用");
    println!("📌 守卫: 使用 if 添加额外条件");
    println!("📌 @ 绑定: 测试模式同时绑定值");
    println!("📌 忽略: _ 忽略单个值，.. 忽略剩余值");

    println!("\n🎉 恭喜！你已经掌握了 Rust 高级模式匹配！");
}
