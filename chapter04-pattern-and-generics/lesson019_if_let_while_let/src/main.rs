/// # Lesson 019 - if let 与 while let
///
/// if let 和 while let 是 match 的简洁替代语法。
///
/// ## 学习目标
/// - 掌握 if let 简化单分支 match
/// - 掌握 while let 循环处理模式
/// - 学会 let-else 语法提前返回
/// - 学会使用 matches! 宏
/// - 理解嵌套 if let 的用法
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson019_if_let_while_let
/// ```

// =============================================================
// Lesson 019: if let 与 while let - 优雅的模式匹配语法糖
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. if let 简化 match
    // ---------------------------------------------------------
    println!("--- if let 简化 match ---");

    // 场景：我们只关心 Option 是 Some 的情况
    let config_value: Option<i32> = Some(42);

    // 使用 match（有些冗长）
    match config_value {
        Some(val) => println!("[match] 配置值: {}", val),
        None => {} // 我们不关心 None，但必须写这个分支
    }

    // 使用 if let（更简洁！）
    if let Some(val) = config_value {
        println!("[if let] 配置值: {}", val);
    }

    // if let 与 else 结合
    let config_value: Option<i32> = None;
    if let Some(val) = config_value {
        println!("找到配置值: {}", val);
    } else {
        println!("没有配置值，使用默认值");
    }

    // if let 也可以解构枚举
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String), // 25美分硬币可能有州名
    }

    let coin = Coin::Quarter("Alaska".to_string());

    // 只关心 Quarter 的情况
    if let Coin::Quarter(state) = &coin {
        println!("25美分硬币，来自: {}", state);
    } else {
        println!("不是25美分硬币");
    }

    // if let 解构结构体
    struct User {
        name: String,
        age: u32,
        email: Option<String>,
    }

    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    };

    if let User {
        email: Some(ref email),
        ref name,
        ..
    } = user
    {
        println!("{} 的邮箱: {}", name, email);
    }

    // ---------------------------------------------------------
    // 2. if let 链 (if let else if let)
    // ---------------------------------------------------------
    println!("\n--- if let 链 ---");

    enum Status {
        Active(String),
        Inactive,
        Pending { reason: String },
    }

    let status = Status::Pending {
        reason: "等待审核".to_string(),
    };

    // if let 可以像 if-else if 一样链接
    if let Status::Active(name) = &status {
        println!("活跃用户: {}", name);
    } else if let Status::Pending { reason } = &status {
        println!("待处理状态，原因: {}", reason);
    } else {
        println!("不活跃");
    }

    // 混合使用 if let 和普通 if
    let temperature: Option<f64> = Some(38.5);
    let is_summer = true;

    if let Some(temp) = temperature {
        if temp > 37.0 && is_summer {
            println!("夏天高温: {:.1}°C，注意防暑！", temp);
        } else if temp > 37.0 {
            println!("温度偏高: {:.1}°C", temp);
        } else {
            println!("温度正常: {:.1}°C", temp);
        }
    } else {
        println!("温度数据缺失");
    }

    // ---------------------------------------------------------
    // 3. while let 循环
    // ---------------------------------------------------------
    println!("\n--- while let 循环 ---");

    // while let 会持续循环，直到模式不再匹配
    // 经典用例：从栈中弹出元素
    let mut stack = vec![1, 2, 3, 4, 5];
    print!("从栈弹出: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();
    println!("栈已清空，长度: {}", stack.len());

    // while let 处理迭代器
    let data = vec!["hello", "world", "rust"];
    let mut iter = data.iter();
    println!("\n迭代字符串:");
    while let Some(word) = iter.next() {
        println!("  -> {}", word.to_uppercase());
    }

    // while let 解构复杂类型
    let mut events: Vec<(i32, &str)> = vec![
        (1, "登录"),
        (2, "浏览"),
        (3, "下单"),
        (4, "支付"),
    ];
    events.reverse(); // 反转以使用 pop

    println!("\n处理事件:");
    while let Some((id, action)) = events.pop() {
        println!("  事件 #{}: {}", id, action);
    }

    // while let 与 Option 链
    fn next_even(start: &mut i32) -> Option<i32> {
        *start += 1;
        if *start > 10 {
            None
        } else if *start % 2 == 0 {
            Some(*start)
        } else {
            *start += 1;
            if *start > 10 {
                None
            } else {
                Some(*start)
            }
        }
    }

    let mut counter = 0;
    print!("偶数序列: ");
    while let Some(even) = next_even(&mut counter) {
        print!("{} ", even);
    }
    println!();

    // ---------------------------------------------------------
    // 4. let-else 语法 (Rust 1.65+)
    // ---------------------------------------------------------
    println!("\n--- let-else 语法 ---");

    // let-else 让我们在模式不匹配时提前返回/中断
    // 语法: let PATTERN = EXPR else { DIVERGE };
    // else 块必须发散（return、break、continue、panic! 等）

    // 封装在函数中演示 let-else 与 return
    fn process_config(config: Option<&str>) -> String {
        // 如果 config 是 None，直接 return
        let Some(value) = config else {
            return "使用默认配置".to_string();
        };
        // 到这里 value 一定是有效的，可以直接使用
        format!("使用配置: {}", value)
    }

    println!("{}", process_config(Some("production")));
    println!("{}", process_config(None));

    // let-else 解构更复杂的模式
    fn parse_command(input: &str) -> String {
        let Some((cmd, args)) = input.split_once(' ') else {
            return format!("命令 '{}' 没有参数", input);
        };
        format!("命令: '{}', 参数: '{}'", cmd, args)
    }

    println!("{}", parse_command("hello world"));
    println!("{}", parse_command("quit"));

    // let-else 在循环中使用 continue
    let values: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    print!("过滤后的值: ");
    for val in &values {
        let Some(num) = val else {
            continue; // 跳过 None 值
        };
        print!("{} ", num);
    }
    println!();

    // let-else 解构结构体
    #[derive(Debug)]
    struct ApiResponse {
        status: u16,
        data: Option<String>,
    }

    fn handle_response(resp: ApiResponse) {
        let ApiResponse {
            status: 200,
            data: Some(body),
        } = resp
        else {
            println!("请求失败或无数据");
            return;
        };
        println!("成功获取数据: {}", body);
    }

    handle_response(ApiResponse {
        status: 200,
        data: Some("用户列表".to_string()),
    });
    handle_response(ApiResponse {
        status: 404,
        data: None,
    });

    // ---------------------------------------------------------
    // 5. matches! 宏
    // ---------------------------------------------------------
    println!("\n--- matches! 宏 ---");

    // matches! 宏用于检查一个值是否匹配某个模式，返回 bool
    // 相当于 match 的简化版本

    // 基本用法
    let x = 42;
    let is_answer = matches!(x, 42);
    println!("{} 是终极答案吗? {}", x, is_answer);

    // 与范围模式结合
    let score = 85;
    let is_passing = matches!(score, 60..=100);
    println!("分数 {} 及格了吗? {}", score, is_passing);

    // 与或模式结合
    let direction = 'N';
    let is_cardinal = matches!(direction, 'N' | 'S' | 'E' | 'W');
    println!("'{}' 是基本方向吗? {}", direction, is_cardinal);

    // 与枚举一起使用
    let status = Status::Active("Bob".to_string());
    let is_active = matches!(status, Status::Active(_));
    println!("用户活跃吗? {}", is_active);

    // matches! 带守卫条件
    let val = Some(42);
    let is_large_some = matches!(val, Some(x) if x > 10);
    println!("是大于10的Some吗? {}", is_large_some);

    // 在 filter 中使用 matches!
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let special: Vec<_> = values
        .iter()
        .filter(|&&x| matches!(x, 1 | 3 | 5 | 7 | 9))
        .collect();
    println!("奇数: {:?}", special);

    // 字符分类
    let chars = vec!['a', '1', 'B', ' ', '你', '!'];
    for ch in &chars {
        let category = if matches!(ch, 'a'..='z') {
            "小写字母"
        } else if matches!(ch, 'A'..='Z') {
            "大写字母"
        } else if matches!(ch, '0'..='9') {
            "数字"
        } else {
            "其他"
        };
        println!("'{}' -> {}", ch, category);
    }

    // ---------------------------------------------------------
    // 6. 嵌套 if let
    // ---------------------------------------------------------
    println!("\n--- 嵌套 if let ---");

    // 嵌套的 Option
    let nested: Option<Option<i32>> = Some(Some(42));

    // 使用嵌套 if let 解构
    if let Some(inner) = nested {
        if let Some(value) = inner {
            println!("嵌套值: {}", value);
        }
    }

    // 也可以直接模式匹配嵌套结构
    match nested {
        Some(Some(val)) => println!("[match] 嵌套值: {}", val),
        Some(None) => println!("外层 Some，内层 None"),
        None => println!("外层就是 None"),
    }

    // 实际应用：嵌套的 JSON 式结构
    #[derive(Debug)]
    enum JsonValue {
        Null,
        Number(f64),
        Str(String),
        Array(Vec<JsonValue>),
        Object(Vec<(String, JsonValue)>),
    }

    let data = JsonValue::Object(vec![
        ("name".to_string(), JsonValue::Str("Alice".to_string())),
        ("age".to_string(), JsonValue::Number(30.0)),
        (
            "hobbies".to_string(),
            JsonValue::Array(vec![
                JsonValue::Str("reading".to_string()),
                JsonValue::Str("coding".to_string()),
            ]),
        ),
    ]);

    // 使用嵌套模式匹配提取信息
    if let JsonValue::Object(fields) = &data {
        for (key, value) in fields {
            match value {
                JsonValue::Str(s) => println!("{}: \"{}\"", key, s),
                JsonValue::Number(n) => println!("{}: {}", key, n),
                JsonValue::Array(items) => {
                    print!("{}: [", key);
                    for (i, item) in items.iter().enumerate() {
                        if let JsonValue::Str(s) = item {
                            if i > 0 {
                                print!(", ");
                            }
                            print!("\"{}\"", s);
                        }
                    }
                    println!("]");
                }
                _ => println!("{}: (其他)", key),
            }
        }
    }

    // ---------------------------------------------------------
    // 7. 综合示例：命令行解析器
    // ---------------------------------------------------------
    println!("\n--- 综合示例: 简易命令处理器 ---");

    #[derive(Debug)]
    enum Command {
        Get { key: String },
        Set { key: String, value: String },
        Delete { key: String },
        List,
        Quit,
    }

    fn parse_input(input: &str) -> Option<Command> {
        let input = input.trim();
        // 使用 let-else 提取命令部分
        if input.is_empty() {
            return None;
        }

        let parts: Vec<&str> = input.splitn(3, ' ').collect();

        match parts.as_slice() {
            ["get", key] => Some(Command::Get {
                key: key.to_string(),
            }),
            ["set", key, value] => Some(Command::Set {
                key: key.to_string(),
                value: value.to_string(),
            }),
            ["del", key] => Some(Command::Delete {
                key: key.to_string(),
            }),
            ["list"] => Some(Command::List),
            ["quit"] | ["exit"] => Some(Command::Quit),
            _ => None,
        }
    }

    // 模拟处理命令
    let inputs = vec![
        "set name Alice",
        "get name",
        "list",
        "del name",
        "invalid command here",
        "quit",
    ];

    for input in inputs {
        print!("输入: {:>25} => ", format!("\"{}\"", input));

        if let Some(cmd) = parse_input(input) {
            match cmd {
                Command::Get { key } => println!("获取键 '{}'", key),
                Command::Set { key, value } => {
                    println!("设置 '{}' = '{}'", key, value)
                }
                Command::Delete { key } => println!("删除键 '{}'", key),
                Command::List => println!("列出所有键"),
                Command::Quit => println!("退出程序"),
            }
        } else {
            println!("无法解析的命令");
        }
    }

    // ---------------------------------------------------------
    // 8. 总结
    // ---------------------------------------------------------
    println!("\n--- 总结 ---");
    println!("📌 if let: 单分支 match 的简洁替代");
    println!("📌 while let: 持续匹配模式直到失败");
    println!("📌 let-else: 匹配失败时提前发散 (return/break/continue)");
    println!("📌 matches!: 返回 bool 的模式匹配宏");
    println!("📌 嵌套 if let: 处理多层嵌套的 Option 等类型");
    println!("📌 选择建议:");
    println!("     只关心一个分支 → if let");
    println!("     循环解构 → while let");
    println!("     提前返回 → let-else");
    println!("     只需要 bool → matches!");
    println!("     多个分支 → match");

    println!("\n🎉 恭喜！你已经掌握了 if let、while let 及相关语法糖！");
}
