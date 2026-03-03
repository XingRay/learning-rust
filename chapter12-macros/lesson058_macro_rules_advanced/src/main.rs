// ============================================================
// Lesson 058: 宏进阶 (Advanced Macro Rules)
// ============================================================
// 本课深入探讨 macro_rules! 的高级用法，包括递归宏、
// TT muncher 模式、宏卫生性、宏导出和调试技巧。
// ============================================================

// ============================================================
// 第一部分：递归宏 (Recursive Macros)
// ============================================================
// 宏可以在展开时调用自身，实现递归处理。
// 这在处理可变数量的参数时非常有用。

// 示例1：递归计算参数个数
macro_rules! count_args {
    // 基本情况：没有参数
    () => { 0usize };
    // 递归情况：一个参数 + 剩余参数
    // $head:tt 捕获第一个 token tree
    // $($tail:tt)* 捕获剩余的所有 token tree
    ($head:tt $($tail:tt)*) => {
        1usize + count_args!($($tail)*)
    };
}

// 示例2：递归求和
macro_rules! sum {
    // 基本情况：只有一个值
    ($x:expr) => { $x };
    // 递归情况：第一个值 + 剩余值的和
    ($x:expr, $($rest:expr),+) => {
        $x + sum!($($rest),+)
    };
}

// 示例3：递归求最大值
macro_rules! max {
    ($x:expr) => { $x };
    ($x:expr, $($rest:expr),+) => {
        {
            let rest_max = max!($($rest),+);
            if $x > rest_max { $x } else { rest_max }
        }
    };
}

// 示例4：递归生成嵌套元组 — 展示递归结构
macro_rules! nested_tuple {
    ($x:expr) => { ($x,) };
    ($x:expr, $($rest:expr),+) => {
        ($x, nested_tuple!($($rest),+))
    };
}

// ============================================================
// 第二部分：TT Muncher 模式简介
// ============================================================
// TT Muncher (Token Tree Muncher) 是一种高级宏模式。
// 它通过逐个"吃掉"(munch) token tree 来处理输入，
// 每次处理一个 token，然后递归处理剩余部分。
//
// TT Muncher 的基本结构：
// 1. 定义一个终止条件（通常是空输入）
// 2. 匹配并处理第一个 token tree
// 3. 将剩余 token trees 递归传递给自身
//
// 这种模式的优点是可以处理复杂的、异构的输入语法。

// TT Muncher 示例：逐个打印不同类型的"命令"
macro_rules! process_commands {
    // 终止条件：没有更多命令
    () => {};

    // 匹配 "print" 命令
    (print $msg:literal; $($rest:tt)*) => {
        println!("执行打印命令: {}", $msg);
        process_commands!($($rest)*);
    };

    // 匹配 "set" 命令
    (set $name:ident = $value:expr; $($rest:tt)*) => {
        let $name = $value;
        println!("设置变量 {} = {:?}", stringify!($name), $name);
        process_commands!($($rest)*);
    };

    // 匹配 "calc" 命令
    (calc $expr:expr; $($rest:tt)*) => {
        println!("计算结果: {} = {}", stringify!($expr), $expr);
        process_commands!($($rest)*);
    };
}

// 另一个 TT Muncher 示例：简易 DSL 构建 HTML
macro_rules! html {
    // 终止条件
    () => { String::new() };

    // 匹配标签：tag "content"; rest...
    ($tag:ident ($content:expr) $($rest:tt)*) => {
        {
            let mut s = format!("<{}>{}</{}>", stringify!($tag), $content, stringify!($tag));
            s.push_str(&html!($($rest)*));
            s
        }
    };
}

// ============================================================
// 第三部分：宏卫生 (Macro Hygiene)
// ============================================================
// Rust 的宏系统是"部分卫生"的(partially hygienic)：
//
// 1. 宏内部定义的变量不会与外部同名变量冲突
//    → 这是"卫生的"(hygienic)
// 2. 但宏可以引用调用处作用域中的变量（通过参数传入）
//
// 这与 C 语言的宏完全不同——C 宏只是简单的文本替换，
// 非常容易产生变量名冲突。

// 演示宏卫生性
macro_rules! hygiene_demo {
    () => {
        // 这个 `x` 是宏内部的，不会与外部的 `x` 冲突
        let x = "我是宏内部的 x";
        println!("{}", x);
    };
}

// 演示宏中的变量作用域
macro_rules! create_variable {
    ($name:ident, $value:expr) => {
        let $name = $value;
    };
}

// 宏卫生的边界——宏不能直接访问调用者作用域中的变量
// 必须通过参数显式传递
macro_rules! use_value {
    ($val:expr) => {
        // $val 是通过参数传入的，这是允许的
        println!("值: {}", $val);
    };
}

// ============================================================
// 第四部分：$crate — 跨 crate 引用
// ============================================================
// $crate 是一个特殊的元变量，在宏展开时会被替换为
// 定义该宏的 crate 的路径。
//
// 这解决了一个重要问题：当宏被其他 crate 使用时，
// 如何正确引用定义宏的 crate 中的其他项？
//
// 用法：$crate::some_function() 或 $crate::SomeType
//
// 在同一个 crate 中，$crate 就相当于 crate（即当前 crate 根）

// 一个辅助函数，供宏内部使用
fn _format_log_message(level: &str, message: &str) -> String {
    format!("[{}] {}", level, message)
}

// 使用 $crate 引用本 crate 的函数
// 当这个宏被导出到其他 crate 时，$crate 会正确解析
macro_rules! safe_log {
    ($level:expr, $msg:expr) => {
        // 使用 $crate 确保即使在其他 crate 中调用也能找到正确的函数
        println!("{}", $crate::_format_log_message($level, $msg));
    };
}

// ============================================================
// 第五部分：宏导出 #[macro_export]
// ============================================================
// 要让宏能被其他 crate 使用，需要使用 #[macro_export] 属性。
//
// 注意事项：
// 1. #[macro_export] 会将宏提升到 crate 根级别
//    即使宏定义在子模块中，使用时也是 use crate_name::macro_name;
// 2. 在同一个 crate 内，macro_rules! 宏遵循"定义在使用之前"的规则
// 3. 使用 #[macro_export] 的宏自动具有 pub 可见性
//
// 示例代码结构（概念说明）：
//
// ```
// // 在 my_crate/src/lib.rs 中:
// #[macro_export]
// macro_rules! my_public_macro {
//     () => { /* ... */ };
// }
//
// // 在其他 crate 中使用:
// use my_crate::my_public_macro;
// // 或者（旧式）:
// #[macro_use]
// extern crate my_crate;
// ```
//
// #[macro_use] 是旧式的导入方式，在 Rust 2018 edition 之后
// 推荐使用 use 语句来导入宏。

// 模拟导出宏（在 binary crate 中只是演示概念）
// #[macro_export]  // 在库 crate 中取消注释来导出
macro_rules! public_macro_example {
    ($($arg:tt)*) => {
        println!("这是一个可导出的宏: {}", format!($($arg)*));
    };
}

// ============================================================
// 第六部分：宏的作用域规则
// ============================================================
//
// macro_rules! 宏的作用域规则：
// 1. 宏定义必须在使用之前（文本顺序）
// 2. 在模块中定义的宏，默认只在该模块内可见
// 3. 使用 #[macro_use] 可以将子模块的宏暴露给父模块
// 4. #[macro_export] 将宏暴露给外部 crate（放在 crate 根级别）

// 模块中的宏作用域
#[macro_use] // 将模块内的宏暴露给外部
mod macros {
    // 这个宏因为 #[macro_use] 可以在模块外使用
    macro_rules! module_macro {
        () => {
            println!("我来自 macros 模块！");
        };
    }
}

// ============================================================
// 第七部分：宏调试
// ============================================================
//
// 调试宏可以使用以下方法：
//
// 1. cargo expand — 查看宏展开后的代码
//    安装: cargo install cargo-expand
//    使用: cargo expand           (展开整个 crate)
//          cargo expand main      (只展开 main 函数)
//          cargo expand --ugly    (不格式化输出)
//
// 2. 编译器内置调试工具（不稳定特性，需要 nightly）:
//    #![feature(trace_macros)]
//    trace_macros!(true);     // 开启宏追踪
//    my_macro!(args);         // 会打印匹配过程
//    trace_macros!(false);    // 关闭宏追踪
//
//    #![feature(log_syntax)]
//    log_syntax!(tokens);     // 在编译期打印 token
//
// 3. stringify! — 将宏参数转为字符串，便于查看
//    println!("{}", stringify!(some_macro_arg));
//
// 4. compile_error! — 在宏中生成编译错误信息
//    用于宏的错误处理和调试

macro_rules! assert_types_match {
    ($a:ty, $b:ty) => {
        // 使用编译期类型检查来验证两个类型相同
        const _: fn($a) -> $b = |x| x;
    };
}

// 使用 compile_error! 在宏中提供有意义的错误信息
macro_rules! config {
    (mode: debug) => {
        println!("调试模式");
    };
    (mode: release) => {
        println!("发布模式");
    };
    (mode: $other:ident) => {
        compile_error!(concat!(
            "不支持的模式: '",
            stringify!($other),
            "'，请使用 'debug' 或 'release'"
        ));
    };
}

// ============================================================
// 第八部分：高级重复模式
// ============================================================

// 嵌套重复：生成枚举及其方法
macro_rules! make_enum {
    (
        $name:ident {
            $( $variant:ident $(( $($field_ty:ty),* ))? ),* $(,)?
        }
    ) => {
        #[derive(Debug)]
        enum $name {
            $( $variant $(( $($field_ty),* ))?, )*
        }

        impl $name {
            fn variant_name(&self) -> &'static str {
                match self {
                    $( $name::$variant { .. } => stringify!($variant), )*
                }
            }
        }
    };
}

make_enum!(Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
    Point,
});

// 使用重复来实现多个 trait
macro_rules! impl_display_for_newtype {
    ( $( $name:ident ),* ) => {
        $(
            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}({:?})", stringify!($name), self.0)
                }
            }
        )*
    };
}

#[derive(Debug)]
struct Meters(f64);
#[derive(Debug)]
struct Seconds(f64);
#[derive(Debug)]
struct Kilograms(f64);

impl_display_for_newtype!(Meters, Seconds, Kilograms);

fn main() {
    println!("=== Lesson 058: 宏进阶 (Advanced Macro Rules) ===\n");

    // ---- 第一部分：递归宏 ----
    println!("--- 第一部分：递归宏 ---");

    // count_args! — 计算参数个数
    println!("count_args!() = {}", count_args!());
    println!("count_args!(a) = {}", count_args!(a));
    println!("count_args!(a b c) = {}", count_args!(a b c));
    println!("count_args!(a b c d e) = {}", count_args!(a b c d e));

    // sum! — 递归求和
    println!("sum!(1) = {}", sum!(1));
    println!("sum!(1, 2, 3) = {}", sum!(1, 2, 3));
    println!("sum!(10, 20, 30, 40) = {}", sum!(10, 20, 30, 40));

    // max! — 递归求最大值
    println!("max!(5) = {}", max!(5));
    println!("max!(3, 7, 2, 9, 1) = {}", max!(3, 7, 2, 9, 1));

    // nested_tuple! — 嵌套元组
    let t = nested_tuple!(1, 2, 3);
    println!("nested_tuple!(1, 2, 3) = {:?}", t);
    println!();

    // ---- 第二部分：TT Muncher ----
    println!("--- 第二部分：TT Muncher 模式 ---");

    process_commands! {
        print "Hello, TT Muncher!";
        set x = 42;
        calc 2 + 3 * 4;
        set y = "world";
        print "处理完毕";
    }

    // 简易 HTML DSL
    let html_output = html! {
        h1("我的标题")
        p("这是段落内容")
        span("行内文本")
    };
    println!("HTML 输出: {}", html_output);
    println!();

    // ---- 第三部分：宏卫生 ----
    println!("--- 第三部分：宏卫生 (Hygiene) ---");

    let x = "我是外部的 x";
    println!("外部 x = {}", x);
    hygiene_demo!();  // 宏内部有自己的 x，不会影响外部
    println!("调用宏后，外部 x = {} (未被改变)", x);

    // 通过 ident 参数创建变量
    create_variable!(my_value, 100);
    println!("通过宏创建的变量 my_value = {}", my_value);

    // 通过参数传递值
    let secret = 42;
    use_value!(secret);
    println!();

    // ---- 第四部分：$crate ----
    println!("--- 第四部分：$crate ---");
    safe_log!("DEBUG", "这条日志使用了 $crate 路径");
    println!(
        "说明: $crate 在宏展开时会替换为当前 crate 的路径，\n\
         确保跨 crate 使用时能正确找到定义宏的 crate 中的项。"
    );
    println!();

    // ---- 第五部分：宏导出 ----
    println!("--- 第五部分：宏导出 #[macro_export] ---");
    public_macro_example!("这个宏可以被其他 crate 使用");
    println!(
        "说明: 使用 #[macro_export] 标记的宏会被提升到 crate 根级别，\n\
         其他 crate 可以通过 use crate_name::macro_name; 来导入使用。"
    );
    println!();

    // ---- 第六部分：宏作用域 ----
    println!("--- 第六部分：宏的作用域规则 ---");
    module_macro!(); // 因为模块用了 #[macro_use] 所以可以使用
    println!(
        "说明: #[macro_use] 将子模块内的宏暴露给父模块。\n\
         宏定义必须在使用之前（文本顺序）。"
    );
    println!();

    // ---- 第七部分：宏调试 ----
    println!("--- 第七部分：宏调试技巧 ---");

    // 使用 stringify! 查看宏参数
    println!("stringify! 可以将任意 token 转为字符串:");
    println!("  stringify!(1 + 2 * 3) = \"{}\"", stringify!(1 + 2 * 3));
    println!(
        "  stringify!(Vec<String>) = \"{}\"",
        stringify!(Vec<String>)
    );

    // compile_error! 演示（注释掉以避免编译失败）
    config!(mode: debug);
    config!(mode: release);
    // config!(mode: test);  // 取消注释会触发编译错误

    // 编译期类型检查
    assert_types_match!(i32, i32); // 编译通过
    // assert_types_match!(i32, u32); // 取消注释会编译失败

    println!(
        "\ncargo expand 使用方法:\n\
         1. 安装: cargo install cargo-expand\n\
         2. 查看整个 crate 的宏展开: cargo expand\n\
         3. 查看特定函数: cargo expand main\n\
         4. 不格式化输出: cargo expand --ugly"
    );
    println!();

    // ---- 第八部分：高级重复 ----
    println!("--- 第八部分：高级重复模式 ---");

    // 使用 make_enum! 宏生成的枚举
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle(3.0, 4.0, 5.0),
        Shape::Point,
    ];
    for shape in &shapes {
        println!("Shape::{} = {:?}", shape.variant_name(), shape);
    }

    // 使用 impl_display_for_newtype! 宏实现的 Display
    println!();
    let m = Meters(100.0);
    let s = Seconds(60.0);
    let kg = Kilograms(75.5);
    println!("Display trait: {}, {}, {}", m, s, kg);

    // ============================================================
    // 宏编写最佳实践总结
    // ============================================================
    println!("\n--- 宏编写最佳实践 ---");
    println!("1. 优先使用函数，只在需要时才使用宏");
    println!("2. 保持宏简洁，复杂逻辑放到辅助函数中");
    println!("3. 使用 $($arg:tt)* 转发参数给其他宏");
    println!("4. 用 $crate:: 引用当前 crate 的项，确保可移植性");
    println!("5. 用 compile_error! 提供有意义的错误信息");
    println!("6. 用 cargo expand 验证宏展开结果");
    println!("7. 为宏编写文档注释，说明用法和示例");

    println!("\n=== Lesson 058 完成 ===");
}
