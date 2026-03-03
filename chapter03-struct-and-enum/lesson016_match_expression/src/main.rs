/// # Lesson 016 - match 表达式
///
/// match 是 Rust 中最强大的控制流结构之一，可以对值进行模式匹配并执行对应代码。
///
/// ## 学习目标
/// - 掌握 match 的基本用法
/// - 理解穷尽匹配（exhaustive matching）
/// - 掌握 _ 通配符的使用
/// - 学会使用匹配守卫（guard）
/// - 掌握 @ 绑定
/// - 学会解构元组、结构体、枚举
/// - 理解 match 作为表达式返回值
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson016_match_expression
/// ```

// =============================================================
// Lesson 016: match 表达式 - Match Expression
// =============================================================

// 为后续演示定义的枚举和结构体
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter 携带州信息
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    NewYork,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Command {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    ChangeColor(u8, u8, u8),
}

#[derive(Debug)]
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

fn main() {
    println!("=== Lesson 016: match 表达式 ===\n");

    // ---------------------------------------------------------
    // 1. match 基本用法
    // ---------------------------------------------------------
    println!("--- 1. match 基本用法 ---");

    // match 将一个值与一系列模式进行比较
    // 执行第一个匹配到的模式对应的代码
    fn coin_value(coin: &Coin) -> u32 {
        match coin {
            Coin::Penny => {
                // 代码块可以包含多条语句
                println!("  幸运便士！");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // 可以绑定枚举变体中的数据
                println!("  这是来自 {:?} 州的 25 美分硬币", state);
                25
            }
        }
    }

    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::California),
    ];

    for coin in &coins {
        let value = coin_value(coin);
        println!("  {:?} = {} 美分", coin, value);
    }

    // ---------------------------------------------------------
    // 2. match 作为表达式返回值
    // ---------------------------------------------------------
    println!("\n--- 2. match 作为表达式 ---");

    // match 是一个表达式，可以返回值
    // 所有分支必须返回相同的类型
    let number = 13;
    let description = match number {
        1 => "一",
        2 => "二",
        3 => "三",
        13 => "十三（幸运数字）",
        _ => "其他数字",
    };
    println!("{} 是 {}", number, description);

    // 用 match 表达式赋值
    let is_even = match number % 2 {
        0 => true,
        _ => false,
    };
    println!("{} 是偶数? {}", number, is_even);

    // ---------------------------------------------------------
    // 3. 穷尽匹配
    // ---------------------------------------------------------
    println!("\n--- 3. 穷尽匹配 ---");

    // Rust 要求 match 必须覆盖所有可能的值（穷尽匹配）
    // 对于枚举，必须列出所有变体（或使用通配符）
    // 这是 Rust 安全性的重要保证

    let direction = "north";
    let chinese = match direction {
        "north" => "北",
        "south" => "南",
        "east" => "东",
        "west" => "西",
        _ => "未知方向", // 必须有这一行，因为 &str 有无数种可能值
    };
    println!("方向: {} = {}", direction, chinese);

    // 对于布尔值：两个分支就够了
    let flag = true;
    let text = match flag {
        true => "是",
        false => "否",
    };
    println!("标志: {}", text);

    // ---------------------------------------------------------
    // 4. _ 通配符
    // ---------------------------------------------------------
    println!("\n--- 4. _ 通配符 ---");

    // _ 匹配所有值，但不绑定变量
    // 必须放在最后，因为 match 从上到下匹配

    let number = 7;
    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        _ => println!("大于三的数: {}", number), // 匹配所有其他值
    }

    // 使用 _ 忽略不需要的变体
    let msg = Command::Echo(String::from("hello"));
    match &msg {
        Command::Quit => println!("退出"),
        Command::Echo(text) => println!("回显: {}", text),
        _ => println!("其他命令: {:?}", msg), // 忽略 Move 和 ChangeColor
    }

    // 使用 .. 忽略结构体中的部分字段
    let point = Point { x: 10, y: 20 };
    match point {
        Point { x, .. } => println!("x 坐标: {} (忽略 y)", x),
    }

    // ---------------------------------------------------------
    // 5. 匹配多个值和范围
    // ---------------------------------------------------------
    println!("\n--- 5. 匹配多个值和范围 ---");

    // 使用 | 匹配多个值
    let number = 3;
    let kind = match number {
        1 | 3 | 5 | 7 | 9 => "奇数",
        2 | 4 | 6 | 8 | 0 => "偶数",
        _ => "其他",
    };
    println!("{} 是 {}", number, kind);

    // 使用 ..= 匹配范围（包含两端）
    let score = 85;
    let grade = match score {
        90..=100 => "A（优秀）",
        80..=89 => "B（良好）",
        70..=79 => "C（中等）",
        60..=69 => "D（及格）",
        0..=59 => "F（不及格）",
        _ => "无效分数",
    };
    println!("分数 {} => 等级 {}", score, grade);

    // 字符范围
    let ch = 'K';
    let kind = match ch {
        'a'..='z' => "小写字母",
        'A'..='Z' => "大写字母",
        '0'..='9' => "数字",
        _ => "其他字符",
    };
    println!("字符 '{}' 是 {}", ch, kind);

    // ---------------------------------------------------------
    // 6. 匹配守卫（Match Guard）
    // ---------------------------------------------------------
    println!("\n--- 6. 匹配守卫 ---");

    // 匹配守卫是 match 分支后面的额外 if 条件
    // 格式：模式 if 条件 => 表达式
    let num = Some(4);
    match num {
        Some(x) if x < 0 => println!("负数: {}", x),
        Some(x) if x == 0 => println!("零"),
        Some(x) if x % 2 == 0 => println!("正偶数: {}", x),
        Some(x) => println!("正奇数: {}", x),
        None => println!("无值"),
    }

    // 匹配守卫的另一个例子：温度转换
    let temp = Temperature::Celsius(36.6);
    let description = match temp {
        Temperature::Celsius(t) if t < 0.0 => format!("{:.1}°C - 零下，非常冷！", t),
        Temperature::Celsius(t) if t < 20.0 => format!("{:.1}°C - 有点冷", t),
        Temperature::Celsius(t) if t < 35.0 => format!("{:.1}°C - 舒适", t),
        Temperature::Celsius(t) => format!("{:.1}°C - 很热！", t),
        Temperature::Fahrenheit(t) if t < 32.0 => format!("{:.1}°F - 零下，非常冷！", t),
        Temperature::Fahrenheit(t) if t < 68.0 => format!("{:.1}°F - 有点冷", t),
        Temperature::Fahrenheit(t) if t < 95.0 => format!("{:.1}°F - 舒适", t),
        Temperature::Fahrenheit(t) => format!("{:.1}°F - 很热！", t),
    };
    println!("温度: {}", description);

    // 守卫与 | 结合使用
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("匹配!"),      // 守卫应用于 4|5|6 整体
        _ => println!("不匹配: x={}, y={}", x, y), // 因为 y 是 false
    }

    // ---------------------------------------------------------
    // 7. @ 绑定
    // ---------------------------------------------------------
    println!("\n--- 7. @ 绑定 ---");

    // @ 允许你在测试值是否匹配模式的同时，将该值绑定到变量
    // 格式：变量名 @ 模式

    let age: u32 = 25; // 使用 u32（无符号），年龄不会为负数
    match age {
        n @ 0..=12 => println!("儿童，年龄: {}", n),
        n @ 13..=17 => println!("青少年，年龄: {}", n),
        n @ 18..=64 => println!("成年人，年龄: {}", n),
        n @ 65.. => println!("老年人，年龄: {}", n),
        // 使用 u32 后，0..=12, 13..=17, 18..=64, 65.. 覆盖了所有可能值
    }

    // @ 绑定与枚举结合
    #[derive(Debug)]
    enum Greeting {
        Hello(String),
    }

    let greeting = Greeting::Hello(String::from("世界"));
    match greeting {
        Greeting::Hello(ref name @ _) if name.len() > 3 => {
            println!("长问候: Hello, {}!", name);
        }
        Greeting::Hello(name) => {
            println!("短问候: Hello, {}!", name);
        }
    }

    // @ 绑定实际应用：HTTP 状态码
    let status_code = 404;
    let status = match status_code {
        code @ 200..=299 => format!("成功 ({})", code),
        code @ 300..=399 => format!("重定向 ({})", code),
        code @ 400..=499 => format!("客户端错误 ({})", code),
        code @ 500..=599 => format!("服务器错误 ({})", code),
        code => format!("未知状态码 ({})", code),
    };
    println!("HTTP 状态: {}", status);

    // ---------------------------------------------------------
    // 8. 解构元组
    // ---------------------------------------------------------
    println!("\n--- 8. 解构元组 ---");

    let point = (3, -5);
    match point {
        (0, 0) => println!("原点"),
        (x, 0) => println!("x 轴上: x={}", x),
        (0, y) => println!("y 轴上: y={}", y),
        (x, y) if x > 0 && y > 0 => println!("第一象限: ({}, {})", x, y),
        (x, y) if x < 0 && y > 0 => println!("第二象限: ({}, {})", x, y),
        (x, y) if x < 0 && y < 0 => println!("第三象限: ({}, {})", x, y),
        (x, y) => println!("第四象限: ({}, {})", x, y),
    }

    // 嵌套元组解构
    let data = ((1, 2), (3, 4));
    match data {
        ((a, b), (c, d)) => {
            println!("嵌套元组: ({}, {}), ({}, {})", a, b, c, d);
        }
    }

    // ---------------------------------------------------------
    // 9. 解构结构体
    // ---------------------------------------------------------
    println!("\n--- 9. 解构结构体 ---");

    let point = Point { x: 5, y: 0 };

    // 解构结构体字段
    match point {
        Point { x, y: 0 } => println!("在 x 轴上: x={}", x),
        Point { x: 0, y } => println!("在 y 轴上: y={}", y),
        Point { x, y } => println!("普通点: ({}, {})", x, y),
    }

    // 使用简写解构
    let Point { x, y } = Point { x: 10, y: 20 };
    println!("解构赋值: x={}, y={}", x, y);

    // 忽略部分字段
    let point = Point { x: 3, y: 7 };
    let Point { x, .. } = point;
    println!("只取 x: {}", x);

    // ---------------------------------------------------------
    // 10. 解构枚举
    // ---------------------------------------------------------
    println!("\n--- 10. 解构枚举 ---");

    let commands = vec![
        Command::Quit,
        Command::Echo(String::from("你好")),
        Command::Move { x: 10, y: -5 },
        Command::ChangeColor(255, 128, 0),
    ];

    for cmd in &commands {
        match cmd {
            Command::Quit => {
                println!("命令: 退出");
            }
            Command::Echo(message) => {
                println!("命令: 回显 \"{}\"", message);
            }
            Command::Move { x, y } => {
                println!("命令: 移动到 ({}, {})", x, y);
            }
            Command::ChangeColor(r, g, b) => {
                println!("命令: 改变颜色 RGB({}, {}, {})", r, g, b);
            }
        }
    }

    // ---------------------------------------------------------
    // 11. 嵌套解构
    // ---------------------------------------------------------
    println!("\n--- 11. 嵌套解构 ---");

    #[derive(Debug)]
    enum Action {
        Click { x: i32, y: i32, button: Button },
        KeyPress(char),
        Scroll(i32),
    }

    #[derive(Debug)]
    enum Button {
        Left,
        Right,
        Middle,
    }

    let events = vec![
        Action::Click {
            x: 100,
            y: 200,
            button: Button::Left,
        },
        Action::Click {
            x: 50,
            y: 75,
            button: Button::Right,
        },
        Action::KeyPress('a'),
        Action::Scroll(-3),
    ];

    for event in &events {
        match event {
            // 嵌套解构：同时解构外层枚举和内层枚举
            Action::Click {
                x,
                y,
                button: Button::Left,
            } => {
                println!("左键点击: ({}, {})", x, y);
            }
            Action::Click {
                x,
                y,
                button: Button::Right,
            } => {
                println!("右键点击: ({}, {})", x, y);
            }
            Action::Click { button, .. } => {
                println!("{:?}键点击", button);
            }
            Action::KeyPress(ch) => {
                println!("按键: '{}'", ch);
            }
            Action::Scroll(amount) if *amount > 0 => {
                println!("向上滚动 {} 行", amount);
            }
            Action::Scroll(amount) => {
                println!("向下滚动 {} 行", -amount);
            }
        }
    }

    // ---------------------------------------------------------
    // 12. 综合示例：简易计算器
    // ---------------------------------------------------------
    println!("\n--- 12. 综合示例：简易计算器 ---");

    #[derive(Debug)]
    enum CalcOp {
        Add(f64, f64),
        Subtract(f64, f64),
        Multiply(f64, f64),
        Divide(f64, f64),
    }

    fn calculate(op: &CalcOp) -> Result<f64, String> {
        match op {
            CalcOp::Add(a, b) => Ok(a + b),
            CalcOp::Subtract(a, b) => Ok(a - b),
            CalcOp::Multiply(a, b) => Ok(a * b),
            CalcOp::Divide(_, b) if *b == 0.0 => Err(String::from("除数不能为零")),
            CalcOp::Divide(a, b) => Ok(a / b),
        }
    }

    let operations = vec![
        CalcOp::Add(10.0, 5.0),
        CalcOp::Subtract(10.0, 5.0),
        CalcOp::Multiply(10.0, 5.0),
        CalcOp::Divide(10.0, 5.0),
        CalcOp::Divide(10.0, 0.0),
    ];

    for op in &operations {
        let symbol = match op {
            CalcOp::Add(..) => "+",
            CalcOp::Subtract(..) => "-",
            CalcOp::Multiply(..) => "*",
            CalcOp::Divide(..) => "/",
        };

        match calculate(op) {
            Ok(result) => println!("  {:?} {} => {:.2}", op, symbol, result),
            Err(msg) => println!("  {:?} {} => 错误: {}", op, symbol, msg),
        }
    }

    // ---------------------------------------------------------
    // 13. match 与 ref/ref mut
    // ---------------------------------------------------------
    println!("\n--- 13. match 中的 ref ---");

    // 在 match 中，可以用 ref 来创建引用而不是移动值
    let robot_name = Some(String::from("Bors"));

    // 使用 ref 避免移动
    match robot_name {
        Some(ref name) => println!("机器人名字: {}", name),
        None => println!("没有名字"),
    }
    // robot_name 仍然可用，因为我们只借用了
    println!("robot_name 仍然可用: {:?}", robot_name);

    // 使用 ref mut 修改
    let mut data = Some(42);
    match data {
        Some(ref mut value) => {
            *value += 10;
            println!("修改后: {}", value);
        }
        None => println!("无值"),
    }
    println!("data = {:?}", data);

    // ---------------------------------------------------------
    // 14. 小技巧与最佳实践
    // ---------------------------------------------------------
    println!("\n--- 14. 小技巧 ---");

    // matches! 宏 - 简洁地检查是否匹配某个模式
    let value = 42;
    let is_answer = matches!(value, 42);
    println!("42 是答案? {}", is_answer);

    let letter = 'R';
    let is_alpha = matches!(letter, 'a'..='z' | 'A'..='Z');
    println!("'R' 是字母? {}", is_alpha);

    let opt = Some(3);
    let is_some_positive = matches!(opt, Some(x) if x > 0);
    println!("是正数的 Some? {}", is_some_positive);

    // match 的每个分支都是一个新的作用域
    let result = match Some(String::from("hello")) {
        Some(s) => {
            let upper = s.to_uppercase();
            format!("处理结果: {}", upper)
        } // s 和 upper 在这里被销毁
        None => String::from("无"),
    };
    println!("{}", result);

    println!("\n🎉 恭喜！你已经完成了 match 表达式的学习！");
}
