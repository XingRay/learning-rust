/// # Lesson 002 - 变量与可变性
///
/// 本课学习 Rust 中变量的核心概念。
///
/// ## 学习目标
/// - 理解 `let` 绑定与不可变性
/// - 掌握 `mut` 可变变量
/// - 了解 `const` 常量与 `static` 静态变量
/// - 理解 Shadowing（变量遮蔽）
/// - 掌握变量作用域与类型推断
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson002_variables
/// ```

// =============================================================
// Lesson 002: 变量与可变性
// =============================================================

// const 常量：必须标注类型，在编译期求值，全局有效
// 命名惯例：全大写 + 下划线分隔
const MAX_SCORE: u32 = 100;
const PI: f64 = 3.141_592_653_589_793;

// static 静态变量：在整个程序运行期间存在，有固定的内存地址
// 与 const 不同，static 有唯一的内存位置，可以取引用
static GREETING: &str = "欢迎学习 Rust！";

// static mut 可变静态变量（不推荐，Rust 2024 起进一步限制）
// 推荐使用 AtomicI32 等原子类型替代
use std::sync::atomic::{AtomicI32, Ordering};
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    println!("=== Lesson 002: 变量与可变性 ===\n");

    // ---------------------------------------------------------
    // 1. let 绑定 —— 不可变变量
    // ---------------------------------------------------------
    println!("--- 1. let 绑定（不可变变量） ---");

    // 在 Rust 中，变量默认是不可变的（immutable）
    let x = 5;
    println!("x 的值是: {}", x);

    // 下面这行如果取消注释会导致编译错误：
    // x = 10; // ❌ 错误: cannot assign twice to immutable variable

    // Rust 默认不可变的好处：
    // - 防止意外修改数据
    // - 使代码更容易推理
    // - 有助于并发安全

    let name = "Rust";
    let version = 2021;
    println!("{} edition: {}", name, version);

    // ---------------------------------------------------------
    // 2. mut 可变变量
    // ---------------------------------------------------------
    println!("\n--- 2. mut 可变变量 ---");

    // 使用 `mut` 关键字声明可变变量
    let mut score = 0;
    println!("初始分数: {}", score);

    score = 50;
    println!("加分后: {}", score);

    score += 30;
    println!("再加分: {}", score);

    // 可变变量可以多次修改，但类型不能变
    let mut message = String::from("Hello");
    println!("原始消息: {}", message);

    message.push_str(", World!");
    println!("修改后的消息: {}", message);

    // 注意：mut 只允许修改值，不允许改变类型
    // message = 42; // ❌ 错误: 类型不匹配

    // ---------------------------------------------------------
    // 3. const 常量
    // ---------------------------------------------------------
    println!("\n--- 3. const 常量 ---");

    // const 常量在编译时确定，不能使用运行时计算的值
    // 必须显式标注类型
    println!("最高分: {}", MAX_SCORE);
    println!("圆周率: {}", PI);

    // const 也可以在函数内定义
    const SECONDS_IN_HOUR: u32 = 60 * 60;
    const SECONDS_IN_DAY: u32 = SECONDS_IN_HOUR * 24;
    println!("一小时有 {} 秒", SECONDS_IN_HOUR);
    println!("一天有 {} 秒", SECONDS_IN_DAY);

    // const 与 let 的区别：
    // 1. const 必须标注类型，let 可以推断
    // 2. const 的值必须是编译期常量表达式
    // 3. const 不能使用 mut
    // 4. const 可以在任何作用域定义，包括全局

    // ---------------------------------------------------------
    // 4. static 静态变量
    // ---------------------------------------------------------
    println!("\n--- 4. static 静态变量 ---");

    println!("{}", GREETING);

    // static 与 const 的区别：
    // 1. static 有固定的内存地址，整个程序生命周期都存在
    // 2. static 可以是 mut 的（但需要 unsafe 块）
    // 3. const 在编译时会被内联替换，没有固定地址

    // 获取 static 的引用
    let greeting_ref: &str = GREETING;
    println!("通过引用访问: {}", greeting_ref);

    // static mut 已经不推荐使用（Rust 2024 起会更严格）
    // 推荐使用原子类型（AtomicI32 等）替代
    COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("原子静态计数器: {}", COUNTER.load(Ordering::SeqCst));
    // 提示: 在实际项目中，应该使用 AtomicI32 等原子类型替代 static mut

    // ---------------------------------------------------------
    // 5. Shadowing（变量遮蔽）
    // ---------------------------------------------------------
    println!("\n--- 5. Shadowing（变量遮蔽） ---");

    // Shadowing: 可以用 let 重新声明同名变量，新变量会"遮蔽"旧变量
    let y = 5;
    println!("y 的初始值: {}", y);

    let y = y + 1; // 新的 y 遮蔽了旧的 y
    println!("第一次遮蔽后: {}", y); // 6

    let y = y * 2; // 再次遮蔽
    println!("第二次遮蔽后: {}", y); // 12

    // Shadowing 最大的特点：可以改变类型！
    let spaces = "   "; // &str 类型
    println!("spaces 字符串: '{}'", spaces);

    let spaces = spaces.len(); // usize 类型，遮蔽了之前的 &str
    println!("spaces 长度: {}", spaces);

    // 如果用 mut 就不行：
    // let mut spaces = "   ";
    // spaces = spaces.len(); // ❌ 类型不匹配

    // Shadowing 在类型转换时很有用
    let input = "42";
    let input: i32 = input.parse().expect("解析失败");
    println!("解析后的数字: {}", input);

    // Shadowing 在条件处理中也很有用
    let config_value = "enabled";
    let config_value = config_value == "enabled"; // 从 &str 变为 bool
    println!("配置是否启用: {}", config_value);

    // ---------------------------------------------------------
    // 6. 变量作用域
    // ---------------------------------------------------------
    println!("\n--- 6. 变量作用域 ---");

    let outer = "我是外部变量";
    println!("{}", outer);

    {
        // 内部作用域
        let inner = "我是内部变量";
        println!("{}", inner);
        println!("内部可以访问外部: {}", outer);

        // 在内部作用域中遮蔽外部变量
        let outer = "我遮蔽了外部变量";
        println!("{}", outer);
    } // inner 在此离开作用域

    // println!("{}", inner); // ❌ 错误: inner 不在作用域中
    println!("外部变量未受影响: {}", outer); // 原始的 outer 仍然有效

    // 作用域示例：利用作用域限制变量的生命周期
    let result = {
        let a = 10;
        let b = 20;
        a + b // 注意：没有分号，这是一个表达式，其值成为块的返回值
    };
    println!("块表达式的结果: {}", result);

    // ---------------------------------------------------------
    // 7. 类型推断
    // ---------------------------------------------------------
    println!("\n--- 7. 类型推断 ---");

    // Rust 有强大的类型推断系统，很多时候不需要显式标注类型

    // 整数默认推断为 i32
    let num = 42;
    println!("num (i32): {}", num);

    // 浮点数默认推断为 f64
    let pi = 3.14;
    println!("pi (f64): {}", pi);

    // 通过后缀指定类型
    let small_num = 42u8; // u8 类型
    let big_num = 100_000i64; // i64 类型，下划线提高可读性
    let float_num = 3.14f32; // f32 类型
    println!("u8: {}, i64: {}, f32: {}", small_num, big_num, float_num);

    // 显式类型标注
    let explicit: i64 = 100;
    println!("显式标注 i64: {}", explicit);

    // 根据使用上下文推断类型
    let mut numbers = Vec::new(); // 此时类型未确定
    numbers.push(1); // 推断为 Vec<i32>
    numbers.push(2);
    numbers.push(3);
    println!("推断出的 Vec<i32>: {:?}", numbers);

    // 使用 turbofish 语法 ::<> 指定泛型类型
    let parsed = "42".parse::<i32>().unwrap();
    println!("turbofish 解析: {}", parsed);

    // ---------------------------------------------------------
    // 8. 解构绑定（补充）
    // ---------------------------------------------------------
    println!("\n--- 8. 解构绑定 ---");

    // 元组解构
    let (a, b, c) = (1, 2.0, "three");
    println!("a = {}, b = {}, c = {}", a, b, c);

    // 部分解构，用 _ 忽略不需要的值
    let (first, _, third) = (10, 20, 30);
    println!("first = {}, third = {}", first, third);

    // 嵌套解构
    let ((x1, y1), (x2, y2)) = ((0, 0), (5, 10));
    println!("点1: ({}, {}), 点2: ({}, {})", x1, y1, x2, y2);

    // ---------------------------------------------------------
    // 9. 小结
    // ---------------------------------------------------------
    println!("\n--- 小结 ---");
    println!("✅ let 创建不可变绑定，mut 使其可变");
    println!("✅ const 是编译期常量，必须标注类型");
    println!("✅ static 是有固定地址的全局变量");
    println!("✅ Shadowing 允许重新声明同名变量，甚至改变类型");
    println!("✅ 变量有作用域，离开作用域后自动销毁");
    println!("✅ Rust 有强大的类型推断，但也支持显式标注");

    println!("\n🎉 恭喜！你已经完成了第二课！");
}
