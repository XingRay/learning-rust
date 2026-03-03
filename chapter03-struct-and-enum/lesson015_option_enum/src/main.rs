/// # Lesson 015 - Option 枚举
///
/// Option<T> 是 Rust 中最重要的枚举之一，用于表示一个值可能存在也可能不存在。
/// Rust 没有 null，而是用 Option<T> 来表达「有或没有」的概念。
///
/// ## 学习目标
/// - 理解 Option<T> 的定义（Some/None）
/// - 理解 Rust 为什么没有 null
/// - 掌握 unwrap/expect 的使用与风险
/// - 掌握 map/and_then/unwrap_or 等组合方法
/// - 学会用 if let 优雅地处理 Option
/// - 了解 Option 在实际场景中的使用
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson015_option_enum
/// ```

// =============================================================
// Lesson 015: Option 枚举 - Option<T>
// =============================================================

fn main() {
    println!("=== Lesson 015: Option 枚举 ===\n");

    // ---------------------------------------------------------
    // 1. Option<T> 的定义
    // ---------------------------------------------------------
    println!("--- 1. Option<T> 的定义 ---");

    // Option<T> 在标准库中的定义如下（已经被预导入，无需 use）：
    // enum Option<T> {
    //     Some(T),  // 有值
    //     None,     // 没有值
    // }

    // 创建 Some 值 - Rust 可以推断类型
    let some_number: Option<i32> = Some(42);
    let some_string: Option<&str> = Some("hello");

    // 创建 None 值 - 必须指定类型，因为编译器无法从 None 推断 T 的类型
    let no_number: Option<i32> = None;

    println!("some_number = {:?}", some_number);
    println!("some_string = {:?}", some_string);
    println!("no_number = {:?}", no_number);

    // Option<T> 和 T 是不同的类型！
    // 这意味着你不能直接把 Option<i32> 当作 i32 使用
    let x: i32 = 5;
    let y: Option<i32> = Some(10);
    // let sum = x + y; // ❌ 编译错误！不能把 i32 和 Option<i32> 相加
    let sum = x + y.unwrap_or(0); // ✅ 需要先提取值
    println!("{} + {:?} = {}", x, y, sum);

    // ---------------------------------------------------------
    // 2. 为什么没有 null？
    // ---------------------------------------------------------
    println!("\n--- 2. 为什么没有 null ---");

    // 在很多语言中，null 可以赋给任何类型的变量：
    //   String name = null;  // Java/C#
    //   let name = null;     // JavaScript
    //
    // 这会导致 NullPointerException / null reference 等运行时错误
    //
    // Rust 的做法：
    // - 普通类型 T 保证一定有值，不可能是 null
    // - 如果一个值可能不存在，必须显式使用 Option<T>
    // - 编译器强制你处理 None 的情况

    // 示例：在 Vec 中查找元素
    let numbers = vec![1, 2, 3, 4, 5];

    // find 返回 Option<&i32>，而不是可能为 null 的指针
    let found = numbers.iter().find(|&&x| x == 3);
    let not_found = numbers.iter().find(|&&x| x == 99);

    println!("查找 3: {:?}", found);       // Some(&3)
    println!("查找 99: {:?}", not_found);   // None

    // ---------------------------------------------------------
    // 3. unwrap 和 expect
    // ---------------------------------------------------------
    println!("\n--- 3. unwrap 和 expect ---");

    // unwrap(): 如果是 Some，返回内部值；如果是 None，程序 panic
    // ⚠️ 仅在你确定值存在时使用，否则程序会崩溃！
    let value = Some(42);
    println!("unwrap: {}", value.unwrap()); // 42

    // expect(): 和 unwrap 类似，但可以自定义 panic 消息
    // 比 unwrap 更好，因为出错时能知道是哪里出了问题
    let config_port = Some(8080);
    println!("expect: {}", config_port.expect("配置文件中缺少端口号"));

    // ❌ 以下代码会 panic，不要在生产代码中这样做：
    // let none_value: Option<i32> = None;
    // none_value.unwrap(); // panic: called `Option::unwrap()` on a `None` value

    // ---------------------------------------------------------
    // 4. 安全地处理 Option - 模式匹配
    // ---------------------------------------------------------
    println!("\n--- 4. 使用 match 处理 Option ---");

    fn describe_option(opt: Option<i32>) {
        match opt {
            Some(value) => println!("  值为: {}", value),
            None => println!("  没有值"),
        }
    }

    describe_option(Some(100));
    describe_option(None);

    // ---------------------------------------------------------
    // 5. if let 与 Option
    // ---------------------------------------------------------
    println!("\n--- 5. if let 与 Option ---");

    // 当你只关心 Some 的情况时，if let 比 match 更简洁
    let favorite_color: Option<&str> = Some("蓝色");

    // 使用 match
    match favorite_color {
        Some(color) => println!("match: 喜欢的颜色是 {}", color),
        None => println!("match: 没有喜欢的颜色"),
    }

    // 使用 if let - 更简洁
    if let Some(color) = favorite_color {
        println!("if let: 喜欢的颜色是 {}", color);
    } else {
        println!("if let: 没有喜欢的颜色");
    }

    // 只关心 Some 的情况，忽略 None
    let nickname: Option<&str> = None;
    if let Some(name) = nickname {
        println!("昵称: {}", name);
    }
    // 没有 else，None 的情况直接跳过

    // while let - 循环处理 Option
    println!("\n  while let 示例:");
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        // pop() 返回 Option<T>，None 时退出循环
        println!("  弹出: {}", top);
    }

    // ---------------------------------------------------------
    // 6. map - 转换 Option 内部的值
    // ---------------------------------------------------------
    println!("\n--- 6. map ---");

    // map: 如果是 Some(x)，对 x 应用函数得到 Some(f(x))
    //       如果是 None，直接返回 None
    let some_str: Option<&str> = Some("42");
    let some_len: Option<usize> = some_str.map(|s| s.len());
    println!("\"42\" 的长度: {:?}", some_len); // Some(2)

    let none_str: Option<&str> = None;
    let none_len: Option<usize> = none_str.map(|s| s.len());
    println!("None 的长度: {:?}", none_len); // None（不会 panic）

    // 链式 map
    let result = Some(3)
        .map(|x| x * 2)     // Some(6)
        .map(|x| x + 10)    // Some(16)
        .map(|x| x.to_string()); // Some("16")
    println!("链式 map: {:?}", result);

    // ---------------------------------------------------------
    // 7. and_then（flatMap）- 链式处理可能失败的操作
    // ---------------------------------------------------------
    println!("\n--- 7. and_then ---");

    // and_then: 如果是 Some(x)，对 x 应用返回 Option 的函数
    //           如果是 None，直接返回 None
    // 和 map 的区别：map 的闭包返回 T，and_then 的闭包返回 Option<T>

    fn parse_number(s: &str) -> Option<i32> {
        s.parse::<i32>().ok() // Result -> Option
    }

    fn double_if_positive(n: i32) -> Option<i32> {
        if n > 0 { Some(n * 2) } else { None }
    }

    let result = Some("42")
        .and_then(parse_number)        // Some(42)
        .and_then(double_if_positive); // Some(84)
    println!("\"42\" -> 解析 -> 正数翻倍: {:?}", result);

    let result = Some("-5")
        .and_then(parse_number)        // Some(-5)
        .and_then(double_if_positive); // None（不是正数）
    println!("\"-5\" -> 解析 -> 正数翻倍: {:?}", result);

    let result = Some("abc")
        .and_then(parse_number)        // None（解析失败）
        .and_then(double_if_positive); // None
    println!("\"abc\" -> 解析 -> 正数翻倍: {:?}", result);

    // 如果用 map 代替 and_then，会得到嵌套的 Option<Option<T>>
    let nested = Some("42").map(parse_number); // Some(Some(42))
    println!("map 会嵌套: {:?}", nested);

    // ---------------------------------------------------------
    // 8. unwrap_or / unwrap_or_else / unwrap_or_default
    // ---------------------------------------------------------
    println!("\n--- 8. unwrap_or 系列 ---");

    // unwrap_or: 提供默认值
    let port: Option<u16> = None;
    let actual_port = port.unwrap_or(8080);
    println!("端口（unwrap_or 默认值）: {}", actual_port);

    let port: Option<u16> = Some(3000);
    let actual_port = port.unwrap_or(8080);
    println!("端口（有值时）: {}", actual_port);

    // unwrap_or_else: 通过闭包延迟计算默认值
    // 当默认值的计算成本较高时使用
    let config_value: Option<String> = None;
    let value = config_value.unwrap_or_else(|| {
        // 这个闭包只在 None 时才执行
        println!("  （正在计算默认值...）");
        String::from("default_value")
    });
    println!("配置值: {}", value);

    // unwrap_or_default: 使用类型的 Default trait 提供默认值
    let no_count: Option<i32> = None;
    println!("默认计数: {}", no_count.unwrap_or_default()); // i32 默认为 0

    let no_name: Option<String> = None;
    println!(
        "默认名称: \"{}\"",
        no_name.unwrap_or_default() // String 默认为 ""
    );

    // ---------------------------------------------------------
    // 9. 其他有用的 Option 方法
    // ---------------------------------------------------------
    println!("\n--- 9. 其他有用的方法 ---");

    // is_some() / is_none() - 检查是否有值
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;
    println!("a.is_some() = {}", a.is_some()); // true
    println!("b.is_none() = {}", b.is_none()); // true

    // filter - 根据条件过滤
    let even = Some(4).filter(|x| x % 2 == 0);
    let odd = Some(3).filter(|x| x % 2 == 0);
    println!("filter 偶数 Some(4): {:?}", even); // Some(4)
    println!("filter 偶数 Some(3): {:?}", odd);  // None

    // or / or_else - 提供备选 Option
    let primary: Option<&str> = None;
    let secondary: Option<&str> = Some("备选值");
    println!("or: {:?}", primary.or(secondary)); // Some("备选值")

    // zip - 将两个 Option 组合为元组
    let x = Some(1);
    let y = Some("hello");
    let z: Option<i32> = None;
    println!("zip Some + Some: {:?}", x.zip(y)); // Some((1, "hello"))
    println!("zip Some + None: {:?}", x.zip(z)); // None

    // flatten - 解开嵌套的 Option
    let nested: Option<Option<i32>> = Some(Some(42));
    println!("flatten: {:?}", nested.flatten()); // Some(42)

    let nested: Option<Option<i32>> = Some(None);
    println!("flatten None: {:?}", nested.flatten()); // None

    // ---------------------------------------------------------
    // 10. Option 在实际场景中的使用
    // ---------------------------------------------------------
    println!("\n--- 10. 实际使用场景 ---");

    // 场景 1：查找用户
    #[derive(Debug)]
    struct User {
        name: String,
        email: Option<String>,   // 邮箱可选
        phone: Option<String>,   // 电话可选
    }

    let users = vec![
        User {
            name: String::from("Alice"),
            email: Some(String::from("alice@example.com")),
            phone: Some(String::from("13800138000")),
        },
        User {
            name: String::from("Bob"),
            email: Some(String::from("bob@example.com")),
            phone: None, // Bob 没有电话
        },
        User {
            name: String::from("Charlie"),
            email: None,  // Charlie 没有邮箱
            phone: None,  // 也没有电话
        },
    ];

    fn find_user<'a>(users: &'a [User], name: &str) -> Option<&'a User> {
        users.iter().find(|u| u.name == name)
    }

    // 查找用户并获取联系方式
    for name in &["Alice", "Bob", "Charlie", "Diana"] {
        print!("查找 {}: ", name);
        match find_user(&users, name) {
            Some(user) => {
                let contact = user
                    .email
                    .as_deref()                   // Option<&String> -> Option<&str>
                    .or(user.phone.as_deref())    // 邮箱没有就用电话
                    .unwrap_or("无联系方式");
                println!("找到！联系方式: {}", contact);
            }
            None => println!("未找到"),
        }
    }

    // 场景 2：安全的除法
    println!();
    fn safe_divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None
        } else {
            Some(a / b)
        }
    }

    let results = vec![
        (10.0, 3.0),
        (10.0, 0.0),
        (0.0, 5.0),
    ];

    for (a, b) in results {
        match safe_divide(a, b) {
            Some(result) => println!("{} / {} = {:.4}", a, b, result),
            None => println!("{} / {} = 除数不能为零！", a, b),
        }
    }

    // 场景 3：链式调用处理嵌套数据
    println!();

    #[derive(Debug)]
    struct Company {
        name: String,
        ceo: Option<String>,
    }

    #[derive(Debug)]
    struct Employee {
        name: String,
        company: Option<Company>,
    }

    let employee = Employee {
        name: String::from("张三"),
        company: Some(Company {
            name: String::from("Rust 公司"),
            ceo: Some(String::from("李四")),
        }),
    };

    // 安全地访问嵌套的 Option
    let ceo_name = employee
        .company
        .as_ref()                    // Option<&Company>
        .and_then(|c| c.ceo.as_ref()) // Option<&String>
        .map(|s| s.as_str())        // Option<&str>
        .unwrap_or("未知");

    println!("{} 公司的 CEO 是: {}", employee.name, ceo_name);

    // 没有公司的员工
    let freelancer = Employee {
        name: String::from("王五"),
        company: None,
    };

    let ceo_name = freelancer
        .company
        .as_ref()
        .and_then(|c| c.ceo.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("无（自由职业者）");

    println!("{} 的 CEO 是: {}", freelancer.name, ceo_name);

    println!("\n🎉 恭喜！你已经完成了 Option 枚举的学习！");
}
