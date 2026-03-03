// ============================================================
// Lesson 057: 声明式宏 (Declarative Macros)
// ============================================================
// 本课介绍 Rust 中 macro_rules! 声明式宏的基本语法和用法。
// 声明式宏也称为"按示例宏"(macros by example)，是 Rust 中最常用的宏类型。
//
// 核心概念：
// - 宏是一种**元编程**工具，在编译期展开为代码
// - 宏可以接受可变数量的参数（而函数不能）
// - 宏在编译期展开，不会产生运行时开销
// ============================================================

// ============================================================
// 第一部分：macro_rules! 基本语法
// ============================================================

// 最简单的宏：不接受任何参数
macro_rules! say_hello {
    // `()` 表示宏不接受任何参数
    () => {
        println!("你好，世界！这是一个宏！");
    };
}

// 接受一个参数的宏
// `$name:expr` 表示捕获一个表达式(expression)，绑定到 `$name`
macro_rules! greet {
    ($name:expr) => {
        println!("你好，{}！欢迎学习 Rust 宏！", $name);
    };
}

// 多个参数的宏
macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// ============================================================
// 第二部分：模式匹配 — 片段说明符(Fragment Specifiers)
// ============================================================
//
// macro_rules! 中可以使用以下片段说明符：
//
// $x:expr   - 表达式 (expression): 1+2, foo(), vec![1,2,3]
// $x:ident  - 标识符 (identifier): 变量名、函数名、类型名等
// $x:ty     - 类型 (type): i32, String, Vec<u8>
// $x:block  - 代码块 (block): { ... }
// $x:pat    - 模式 (pattern): Some(x), (a, b), _
// $x:stmt   - 语句 (statement): let x = 1;
// $x:item   - 项 (item): fn, struct, impl 等
// $x:path   - 路径 (path): std::collections::HashMap
// $x:tt     - 单个 token tree: 任何单个标记或 (...)/{...}/[...] 包围的标记组
// $x:meta   - 元数据 (meta): 用于属性内部，如 derive(Debug)
// $x:literal- 字面量: 42, "hello", true
// $x:lifetime - 生命周期: 'a, 'static
// $x:vis    - 可见性修饰符: pub, pub(crate), (空)

// 演示不同的片段说明符
macro_rules! demo_expr {
    ($e:expr) => {
        println!("表达式的值 = {:?}", $e);
    };
}

macro_rules! demo_ident {
    ($name:ident) => {
        // ident 可以用来创建变量名、函数名等
        let $name = 42;
        println!("变量 {} = {}", stringify!($name), $name);
    };
}

macro_rules! demo_ty {
    ($t:ty) => {
        // ty 可以用来指定类型
        let _value: $t = Default::default();
        println!("类型 {} 的默认值 = {:?}", stringify!($t), _value);
    };
}

macro_rules! demo_block {
    ($b:block) => {
        println!("代码块的结果 = {:?}", $b);
    };
}

macro_rules! demo_pat {
    ($p:pat, $val:expr) => {
        match $val {
            $p => println!("模式 {} 匹配成功！", stringify!($p)),
            _ => println!("模式 {} 未匹配", stringify!($p)),
        }
    };
}

macro_rules! demo_stmt {
    ($s:stmt) => {
        $s
        // stmt 捕获的是一条语句
    };
}

macro_rules! demo_path {
    ($p:path) => {
        // path 可以是模块路径
        println!("路径: {}", stringify!($p));
    };
}

macro_rules! demo_literal {
    ($l:literal) => {
        println!("字面量: {}", $l);
    };
}

// 多分支模式匹配 — 宏可以有多个匹配分支，类似 match
macro_rules! multi_branch {
    // 分支1：没有参数
    () => {
        println!("无参数调用");
    };
    // 分支2：一个表达式
    ($x:expr) => {
        println!("一个参数: {:?}", $x);
    };
    // 分支3：两个表达式
    ($x:expr, $y:expr) => {
        println!("两个参数: {:?} 和 {:?}", $x, $y);
    };
}

// ============================================================
// 第三部分：重复 (Repetition) — $(...),*
// ============================================================
//
// 重复语法：$( ... )sep rep
// - sep 是可选的分隔符（如逗号 ,）
// - rep 是重复运算符：
//   * — 零次或多次
//   + — 一次或多次
//   ? — 零次或一次
//
// 在展开侧，对应的 $(...),* 会展开对应次数

// 自定义 vec!-like 宏
macro_rules! my_vec {
    // 空 vec
    () => {
        Vec::new()
    };
    // 从元素列表创建：my_vec![1, 2, 3]
    // $( $elem:expr ),+ 匹配一个或多个用逗号分隔的表达式
    // $(,)? 允许末尾可选的逗号（trailing comma）
    ( $( $elem:expr ),+ $(,)? ) => {
        {
            let mut v = Vec::new();
            $( v.push($elem); )+  // 对每个 $elem 展开一次 push
            v
        }
    };
    // 用重复值创建：my_vec![0; 5] => [0, 0, 0, 0, 0]
    ( $elem:expr ; $count:expr ) => {
        vec![$elem; $count]
    };
}

// 自定义 hashmap! 宏 — 演示 key => value 语法
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

// ============================================================
// 第四部分：自定义 println!-like 宏
// ============================================================

// 带前缀的打印宏 — 模拟日志功能
macro_rules! log_info {
    // 使用 $($arg:tt)* 来捕获任意 token tree，
    // 这是转发给 format!/println! 等宏的惯用方式
    ($($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
}

macro_rules! log_warn {
    ($($arg:tt)*) => {
        println!("[WARN] {}", format!($($arg)*));
    };
}

macro_rules! log_error {
    ($($arg:tt)*) => {
        println!("[ERROR] {}", format!($($arg)*));
    };
}

// 更复杂的日志宏 — 带日志级别参数
macro_rules! log {
    // 匹配 log!(INFO, "message", args...)
    (INFO, $($arg:tt)*) => {
        println!("[INFO]  {}", format!($($arg)*));
    };
    (WARN, $($arg:tt)*) => {
        println!("[WARN]  {}", format!($($arg)*));
    };
    (ERROR, $($arg:tt)*) => {
        println!("[ERROR] {}", format!($($arg)*));
    };
}

// ============================================================
// 第五部分：实用宏示例
// ============================================================

// 创建结构体和 impl 的宏
macro_rules! make_struct {
    ($name:ident { $( $field:ident : $ty:ty ),* $(,)? }) => {
        #[derive(Debug)]
        struct $name {
            $( $field: $ty, )*
        }

        impl $name {
            fn new($( $field: $ty ),*) -> Self {
                Self {
                    $( $field, )*
                }
            }
        }
    };
}

// 使用宏创建结构体
make_struct!(Point { x: f64, y: f64 });
make_struct!(Color { r: u8, g: u8, b: u8 });

// 测试生成器宏
macro_rules! generate_tests {
    ($($name:ident: $input:expr => $expected:expr),* $(,)?) => {
        $(
            fn $name() -> bool {
                let result = $input;
                let pass = result == $expected;
                if pass {
                    println!("  ✓ {} 通过: {:?} == {:?}", stringify!($name), result, $expected);
                } else {
                    println!("  ✗ {} 失败: {:?} != {:?}", stringify!($name), result, $expected);
                }
                pass
            }
        )*

        fn run_all_generated_tests() {
            println!("运行生成的测试:");
            let mut passed = 0;
            let mut total = 0;
            $(
                total += 1;
                if $name() { passed += 1; }
            )*
            println!("  结果: {}/{} 测试通过\n", passed, total);
        }
    };
}

// 使用宏生成测试函数
generate_tests! {
    test_add: 2 + 3 => 5,
    test_mul: 4 * 5 => 20,
    test_str: "hello".to_uppercase() => "HELLO",
    test_vec_len: vec![1, 2, 3].len() => 3,
}

// ============================================================
// 第六部分：宏展开顺序
// ============================================================
//
// Rust 宏的展开顺序是"由外向内"(outside-in):
// 1. 编译器首先展开最外层的宏
// 2. 然后检查展开后的代码，继续展开内部的宏
// 3. 重复此过程直到所有宏都被展开
//
// 例如: println!("{}", my_vec![1, 2, 3].len())
// 1. 先展开 println! — 但它内部的参数需要先求值
// 2. 实际上编译器会先展开 my_vec!，因为它是参数的一部分
//
// 重要规则：
// - 宏必须在使用之前定义（与函数不同）
// - 在同一作用域中，宏的定义顺序很重要
// - 宏可以调用其他宏（包括自身 — 递归宏）

// 演示宏的嵌套调用
macro_rules! double {
    ($e:expr) => {
        $e * 2
    };
}

macro_rules! quadruple {
    ($e:expr) => {
        // 宏可以嵌套调用其他宏
        double!(double!($e))
    };
}

fn main() {
    println!("=== Lesson 057: 声明式宏 (Declarative Macros) ===\n");

    // ---- 第一部分：基本语法 ----
    println!("--- 第一部分：macro_rules! 基本语法 ---");
    say_hello!();
    greet!("张三");
    let sum = add!(10, 20);
    println!("add!(10, 20) = {}", sum);
    println!();

    // ---- 第二部分：片段说明符 ----
    println!("--- 第二部分：片段说明符演示 ---");

    // expr: 表达式
    demo_expr!(1 + 2 + 3);
    demo_expr!("hello".to_string());
    demo_expr!(vec![1, 2, 3]);

    // ident: 标识符
    demo_ident!(my_var);
    demo_ident!(another_var);

    // ty: 类型
    demo_ty!(i32);
    demo_ty!(String);
    demo_ty!(bool);

    // block: 代码块
    demo_block!({ 10 + 20 });
    demo_block!({
        let x = 5;
        let y = 10;
        x * y
    });

    // pat: 模式
    demo_pat!(Some(42), Some(42));
    demo_pat!(Some(42), Some(99));
    demo_pat!(1..=10, 5);

    // stmt: 语句
    demo_stmt!(let z = 100);
    println!("demo_stmt 创建的变量 z = {}", z);

    // path: 路径
    demo_path!(std::collections::HashMap);
    demo_path!(std::io::Result);

    // literal: 字面量
    demo_literal!(42);
    demo_literal!("hello");
    demo_literal!(true);

    println!();

    // ---- 多分支宏 ----
    println!("--- 多分支模式匹配 ---");
    multi_branch!();
    multi_branch!(42);
    multi_branch!("hello", "world");
    println!();

    // ---- 第三部分：重复 ----
    println!("--- 第三部分：重复 ($(...),*) ---");

    // my_vec! 宏
    let v1: Vec<i32> = my_vec![];
    let v2 = my_vec![1, 2, 3, 4, 5];
    let v3 = my_vec![0; 3];
    let v4 = my_vec!["hello", "world",]; // 支持尾逗号
    println!("my_vec![] = {:?}", v1);
    println!("my_vec![1,2,3,4,5] = {:?}", v2);
    println!("my_vec![0; 3] = {:?}", v3);
    println!("my_vec![\"hello\", \"world\",] = {:?}", v4);

    // hashmap! 宏
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Charlie" => 92,
    };
    println!("hashmap! 创建的 HashMap: {:?}", scores);
    println!();

    // ---- 第四部分：自定义 println!-like 宏 ----
    println!("--- 第四部分：自定义日志宏 ---");
    log_info!("服务器启动，端口: {}", 8080);
    log_warn!("内存使用率: {}%", 85);
    log_error!("连接数据库失败: {}", "timeout");

    log!(INFO, "用户 {} 登录成功", "admin");
    log!(WARN, "磁盘空间不足，剩余 {} GB", 5);
    log!(ERROR, "文件 {} 不存在", "/etc/config.toml");
    println!();

    // ---- 第五部分：实用宏 ----
    println!("--- 第五部分：实用宏示例 ---");

    // 使用 make_struct! 宏生成的结构体
    let p = Point::new(3.0, 4.0);
    println!("Point: {:?}", p);

    let c = Color::new(255, 128, 0);
    println!("Color: {:?}", c);

    // 使用 generate_tests! 宏生成的测试
    run_all_generated_tests();

    // ---- 第六部分：宏展开顺序 ----
    println!("--- 第六部分：宏展开顺序 ---");
    println!("double!(5) = {}", double!(5));
    println!("quadruple!(5) = {}", quadruple!(5));

    // 宏的嵌套使用
    let v = my_vec![double!(1), double!(2), double!(3)];
    println!("my_vec![double!(1), double!(2), double!(3)] = {:?}", v);

    // stringify! 宏 — 将代码转为字符串（编译期）
    println!("stringify!(1 + 2) = \"{}\"", stringify!(1 + 2));
    println!("stringify!(my_vec![1,2,3]) = \"{}\"", stringify!(my_vec![1, 2, 3]));

    // concat! 宏 — 编译期字符串拼接
    let s = concat!("Hello", ", ", "World", "!");
    println!("concat!(\"Hello\", \", \", \"World\", \"!\") = \"{}\"", s);

    // file! 和 line! 宏 — 获取当前文件名和行号
    println!("当前文件: {}", file!());
    println!("当前行号: {}", line!());

    // 宏调用可以使用不同的括号
    // 以下三种写法等价：
    let _a = my_vec!(1, 2, 3);   // 圆括号
    let _b = my_vec![1, 2, 3];   // 方括号
    let _c = my_vec!{1, 2, 3};   // 花括号
    println!("宏调用支持 ()、[] 和 {{}} 三种括号");

    println!("\n=== Lesson 057 完成 ===");
}
