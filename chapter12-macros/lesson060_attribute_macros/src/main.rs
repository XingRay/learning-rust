// ============================================================
// Lesson 060: 属性宏 (Attribute Macros) — 概念讲解
// ============================================================
// 本课讲解 Rust 中常用的属性(attributes)及其用法。
// 属性是附加在代码项上的元数据，可以影响编译行为。
//
// 属性的两种形式：
// - 外部属性：#[attr] — 应用于紧随其后的项
// - 内部属性：#![attr] — 应用于包含它的项（通常在文件顶部）
//
// 属性的分类：
// 1. 内置属性 — 编译器直接识别
// 2. derive 属性 — 用于自动实现 trait
// 3. 自定义属性宏 — 通过过程宏定义（需要 proc-macro crate）
// ============================================================

// ============================================================
// 内部属性示例（文件级别）
// ============================================================
// #![allow(unused)] 允许整个文件有未使用的变量/函数
#![allow(dead_code)] // 允许未使用的函数和结构体

// ============================================================
// 第一部分：常用属性 — #[allow] / #[warn] / #[deny]
// ============================================================
// 这些属性控制编译器的 lint（代码检查）行为：
// - #[allow(lint)] — 关闭指定的 lint 检查
// - #[warn(lint)] — 将 lint 降为警告
// - #[deny(lint)] — 将 lint 升为编译错误
// - #[forbid(lint)] — 类似 deny，但不能被内部代码覆盖

#[allow(unused_variables)] // 允许此函数内的未使用变量
fn demo_allow() {
    let x = 42; // 不会产生 "unused variable" 警告
    let y = "hello"; // 同上
    println!("demo_allow: 允许未使用的变量");
}

// 可以在表达式级别使用 allow
fn demo_allow_expression() {
    #[allow(unused_assignments)]
    let mut x = 0;
    x = 42;
    println!("demo_allow_expression: x = {}", x);
}

// 常见的 lint 名称：
// - unused_variables: 未使用的变量
// - unused_imports: 未使用的导入
// - unused_mut: 不需要 mut 的变量声明为 mut
// - dead_code: 未使用的函数/结构体
// - unreachable_code: 不可达的代码
// - non_snake_case: 不是 snake_case 的变量/函数名
// - non_camel_case_types: 不是 CamelCase 的类型名
// - clippy::xxx: clippy 相关的 lint

// ============================================================
// 第二部分：#[derive] — 派生宏属性
// ============================================================
// #[derive] 在前一课已详细讲解，这里展示它作为属性的用法

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// 派生多个 trait
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

// ============================================================
// 第三部分：#[cfg] — 条件编译
// ============================================================
//
// #[cfg] 是 Rust 最强大的属性之一，用于条件编译。
// 如果条件不满足，被标注的代码完全不会被编译。
//
// 常用条件：
// - cfg(target_os = "windows/linux/macos")
// - cfg(target_arch = "x86_64/aarch64")
// - cfg(feature = "feature_name")
// - cfg(test) — 在 cargo test 时为 true
// - cfg(debug_assertions) — 在 debug 模式下为 true
// - cfg(target_family = "unix/windows")
// - cfg(target_endian = "little/big")
// - cfg(target_pointer_width = "32/64")
//
// 逻辑组合：
// - cfg(all(condition1, condition2)) — AND
// - cfg(any(condition1, condition2)) — OR
// - cfg(not(condition)) — NOT

// 根据操作系统编译不同的函数
#[cfg(target_os = "windows")]
fn get_os_name() -> &'static str {
    "Windows"
}

#[cfg(target_os = "linux")]
fn get_os_name() -> &'static str {
    "Linux"
}

#[cfg(target_os = "macos")]
fn get_os_name() -> &'static str {
    "macOS"
}

// 兜底：其他操作系统
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn get_os_name() -> &'static str {
    "Unknown OS"
}

// 根据架构编译
#[cfg(target_arch = "x86_64")]
fn get_arch() -> &'static str {
    "x86_64"
}

#[cfg(target_arch = "aarch64")]
fn get_arch() -> &'static str {
    "aarch64 (ARM64)"
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn get_arch() -> &'static str {
    "其他架构"
}

// 使用 cfg! 宏（注意：这是宏，不是属性）
// cfg! 宏在运行时返回 bool，但条件在编译期求值
fn check_platform_features() {
    println!("  是否为 debug 模式: {}", cfg!(debug_assertions));
    println!("  是否为 64 位: {}", cfg!(target_pointer_width = "64"));
    println!("  是否为 Unix 系系统: {}", cfg!(target_family = "unix"));
    println!("  是否为 Windows: {}", cfg!(target_family = "windows"));
    println!("  是否为小端序: {}", cfg!(target_endian = "little"));
}

// 条件编译 — 仅在测试时包含的模块
#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}

// debug_assertions — 只在 debug 模式下执行的代码
fn expensive_check(value: i32) -> i32 {
    // 这段代码只在 debug 模式下执行
    #[cfg(debug_assertions)]
    {
        if value < 0 {
            eprintln!("警告：值为负数: {}", value);
        }
    }
    value.abs()
}

// all() 和 any() 的组合使用
#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
fn platform_specific() -> &'static str {
    "Windows x86_64 专用功能"
}

#[cfg(not(all(target_os = "windows", target_arch = "x86_64")))]
fn platform_specific() -> &'static str {
    "非 Windows x86_64 平台"
}

// ============================================================
// 第四部分：cfg_attr — 条件属性
// ============================================================
//
// cfg_attr 根据条件决定是否应用某个属性
// 语法：#[cfg_attr(condition, attribute)]
// 如果 condition 为 true，则应用 attribute

// 在 test 模式下 derive 额外的 trait
#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))] // 只在测试时需要比较
struct TestConfig {
    name: String,
    value: i32,
}

// 在 debug 模式下添加额外的 derive
#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))] // release 模式不需要 Debug
struct OptimizedStruct {
    data: Vec<u8>,
}

// ============================================================
// 第五部分：#[test] 和 #[bench] — 测试属性
// ============================================================
//
// #[test] 标记一个函数为测试函数，只在 cargo test 时编译和运行
// #[bench] 标记基准测试函数（需要 nightly Rust）
//
// 相关属性：
// #[should_panic] — 测试应该 panic 才算通过
// #[ignore] — 默认跳过此测试，可用 cargo test -- --ignored 运行

// 测试模块（概念展示）
// 注意：这些函数在 cargo test 时才会运行
#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_point_creation() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    #[should_panic(expected = "除以零")]
    fn test_divide_by_zero() {
        fn divide(a: i32, b: i32) -> i32 {
            if b == 0 {
                panic!("除以零");
            }
            a / b
        }
        divide(10, 0);
    }

    #[test]
    #[ignore] // 默认跳过
    fn test_slow_operation() {
        // 耗时很长的测试
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true);
    }
}

// ============================================================
// 第六部分：其他常用内置属性
// ============================================================

// #[inline] — 建议编译器内联函数
#[inline]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// #[inline(always)] — 强制内联
#[inline(always)]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// #[inline(never)] — 禁止内联
#[inline(never)]
fn complex_operation(x: i32) -> i32 {
    x * x + 2 * x + 1
}

// #[must_use] — 如果返回值未使用，发出警告
#[must_use = "这个结果不应被忽略"]
fn important_calculation(x: i32) -> i32 {
    x * 2
}

// #[deprecated] — 标记为已弃用
#[deprecated(since = "2.0.0", note = "请使用 new_function() 代替")]
fn old_function() -> i32 {
    42
}

fn new_function() -> i32 {
    42
}

// #[repr] — 控制类型的内存布局
#[repr(C)] // 使用 C 语言的布局规则
struct CCompatible {
    x: i32,
    y: i32,
    z: f64,
}

#[repr(u8)] // 枚举的判别值使用 u8
enum SmallEnum {
    A = 0,
    B = 1,
    C = 2,
}

// #[repr(transparent)] — 单字段结构体使用与内部类型相同的布局
#[repr(transparent)]
struct Wrapper(i32);

// #[non_exhaustive] — 防止外部代码穷举匹配
// 在库中使用，允许未来添加新变体而不破坏兼容性
#[non_exhaustive]
#[derive(Debug)]
enum ApiError {
    NotFound,
    Unauthorized,
    InternalError,
    // 未来可以添加新变体，不会破坏使用者的代码
}

// ============================================================
// 第七部分：自定义属性宏概念说明
// ============================================================
//
// 自定义属性宏是过程宏的一种，允许你定义自己的属性语法。
// 与 derive 宏不同，属性宏可以修改被标注的项本身。
//
// 创建自定义属性宏的步骤：
//
// 1. 创建 proc-macro crate:
//    ```toml
//    [lib]
//    proc-macro = true
//    ```
//
// 2. 实现属性宏:
//    ```rust
//    use proc_macro::TokenStream;
//
//    #[proc_macro_attribute]
//    pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
//        // attr: 属性的参数，如 #[my_attribute(param)] 中的 param
//        // item: 被标注的项（函数、结构体等）
//        // 返回: 修改后的（或新生成的）代码
//
//        // 可以：
//        // - 修改原始函数/结构体
//        // - 添加额外的 impl 块
//        // - 用新代码替换原始代码
//        // - 添加包装逻辑
//
//        item // 简单返回原始代码不做修改
//    }
//    ```
//
// 3. 使用自定义属性:
//    ```rust
//    #[my_attribute(some_param)]
//    fn my_function() {
//        // ...
//    }
//    ```
//
// 常见应用场景：
// - Web 框架的路由宏: #[get("/api/users")]
// - 序列化控制: #[serde(rename = "user_name")]
// - 日志/追踪: #[instrument]
// - 测试框架: #[tokio::test]
// - 条件编译增强: 自定义 cfg 逻辑

// ============================================================
// 第八部分：实用属性模式
// ============================================================

// 使用 #[allow] 的常见模式
mod api_module {
    #![allow(non_snake_case)] // 模块级别：允许非 snake_case 命名

    #[derive(Debug)]
    pub struct UserResponse {
        pub userId: u64,       // API 返回的字段名可能不是 snake_case
        pub userName: String,
        pub isActive: bool,
    }
}

// 条件编译用于平台特定代码
mod platform {
    pub fn get_line_ending() -> &'static str {
        if cfg!(target_os = "windows") {
            "\r\n"
        } else {
            "\n"
        }
    }

    pub fn get_path_separator() -> char {
        if cfg!(target_os = "windows") {
            '\\'
        } else {
            '/'
        }
    }
}

// 组合使用多个属性
#[derive(Debug, Clone)]
#[must_use = "配置对象不应被丢弃"]
struct AppConfig {
    name: String,
    version: String,
    debug: bool,
}

impl AppConfig {
    fn new(name: &str, version: &str) -> Self {
        AppConfig {
            name: name.to_string(),
            version: version.to_string(),
            debug: cfg!(debug_assertions),
        }
    }
}

fn main() {
    println!("=== Lesson 060: 属性宏 (Attribute Macros) ===\n");

    // ---- 第一部分：#[allow] ----
    println!("--- 第一部分：#[allow] / #[warn] / #[deny] ---");
    demo_allow();
    demo_allow_expression();
    println!("说明: #[allow] 抑制编译器警告，#[deny] 将警告升为错误");
    println!();

    // ---- 第二部分：#[derive] ----
    println!("--- 第二部分：#[derive] 回顾 ---");
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point: {:?}", p);
    let px = Pixel::default();
    println!("Pixel::default(): {:?}", px);
    println!();

    // ---- 第三部分：#[cfg] ----
    println!("--- 第三部分：#[cfg] 条件编译 ---");
    println!("当前操作系统: {}", get_os_name());
    println!("当前架构: {}", get_arch());
    println!("平台特定功能: {}", platform_specific());

    println!("\n平台特性检查:");
    check_platform_features();

    // cfg! 宏 vs #[cfg] 属性的区别
    println!("\ncfg! 宏 vs #[cfg] 属性:");
    println!("  cfg! 宏: 在表达式位置使用，返回 bool");
    println!("  #[cfg] 属性: 在项声明上使用，决定是否编译");

    // debug_assertions 示例
    println!("\ndebug 模式检查:");
    let result = expensive_check(-5);
    println!("expensive_check(-5) = {} (debug模式下会有警告)", result);
    println!();

    // ---- 第四部分：cfg_attr ----
    println!("--- 第四部分：cfg_attr 条件属性 ---");
    let tc = TestConfig {
        name: "test".to_string(),
        value: 42,
    };
    println!("TestConfig: {:?}", tc);
    println!("说明: cfg_attr 根据条件决定是否应用属性");
    println!("例如: #[cfg_attr(test, derive(PartialEq))] 只在测试时添加 PartialEq");
    println!();

    // ---- 第五部分：#[test] ----
    println!("--- 第五部分：#[test] 测试属性 ---");
    println!("测试属性说明:");
    println!("  #[test]           — 标记测试函数");
    println!("  #[should_panic]   — 测试应该 panic");
    println!("  #[ignore]         — 默认跳过，--ignored 运行");
    println!("  运行: cargo test");
    println!("  运行被忽略的: cargo test -- --ignored");
    println!("  运行所有的: cargo test -- --include-ignored");
    println!();

    // ---- 第六部分：其他常用属性 ----
    println!("--- 第六部分：其他常用内置属性 ---");

    // #[inline] 示例
    println!("#[inline] 示例:");
    println!("  add(3, 4) = {} (#[inline])", add(3, 4));
    println!("  multiply(3, 4) = {} (#[inline(always)])", multiply(3, 4));
    println!(
        "  complex_operation(5) = {} (#[inline(never)])",
        complex_operation(5)
    );

    // #[must_use] 示例
    println!("\n#[must_use] 示例:");
    let result = important_calculation(21);
    println!("  important_calculation(21) = {} (必须使用返回值)", result);
    // important_calculation(21); // 如果不使用返回值，编译器会警告

    // #[deprecated] 示例
    println!("\n#[deprecated] 示例:");
    #[allow(deprecated)]
    let old_result = old_function();
    let new_result = new_function();
    println!("  old_function() = {} (已弃用)", old_result);
    println!("  new_function() = {} (推荐使用)", new_result);

    // #[repr] 示例
    println!("\n#[repr] 示例:");
    println!("  CCompatible 使用 C 布局，大小 = {} 字节", std::mem::size_of::<CCompatible>());
    println!("  SmallEnum 使用 u8 判别值，大小 = {} 字节", std::mem::size_of::<SmallEnum>());
    println!("  Wrapper(i32) transparent 布局，大小 = {} 字节", std::mem::size_of::<Wrapper>());
    println!("  普通 i32 大小 = {} 字节", std::mem::size_of::<i32>());

    // #[non_exhaustive]
    println!("\n#[non_exhaustive] 示例:");
    let err = ApiError::NotFound;
    // 注意：#[non_exhaustive] 主要影响**外部** crate 的匹配。
    // 在定义 non_exhaustive 枚举的同一 crate 内部，编译器知道所有变体，
    // 所以 _ 分支会被标记为 unreachable。
    // 但在外部 crate 中使用时，_ 分支是**必须**的，否则编译失败。
    #[allow(unreachable_patterns)]
    match err {
        ApiError::NotFound => println!("  错误: 未找到"),
        ApiError::Unauthorized => println!("  错误: 未授权"),
        ApiError::InternalError => println!("  错误: 内部错误"),
        _ => println!("  错误: 未知错误 (外部 crate 必须有此分支)"),
    }
    println!();

    // ---- 第七部分：自定义属性宏概念 ----
    println!("--- 第七部分：自定义属性宏概念 ---");
    println!("自定义属性宏的常见应用:");
    println!("  • Web 路由:     #[get(\"/api/users\")]");
    println!("  • 序列化控制:   #[serde(rename = \"userName\")]");
    println!("  • 异步测试:     #[tokio::test]");
    println!("  • 追踪/日志:    #[tracing::instrument]");
    println!("  • 这些都需要 proc-macro crate 来实现");
    println!();

    // ---- 第八部分：实用模式 ----
    println!("--- 第八部分：实用属性模式 ---");

    // 模块级 allow
    let user = api_module::UserResponse {
        userId: 1,
        userName: "张三".to_string(),
        isActive: true,
    };
    println!("API 响应 (非 snake_case 字段): {:?}", user);

    // 平台特定工具
    println!("行尾符: {:?}", platform::get_line_ending());
    println!("路径分隔符: {:?}", platform::get_path_separator());

    // AppConfig 使用多个属性
    let config = AppConfig::new("MyApp", "1.0.0");
    println!("应用配置: {:?}", config);

    // ---- 属性总结 ----
    println!("\n--- 常用属性速查表 ---");
    println!("编译控制:");
    println!("  #[cfg(condition)]          — 条件编译");
    println!("  #[cfg_attr(cond, attr)]    — 条件属性");
    println!("  #[allow(lint)]             — 允许 lint");
    println!("  #[deny(lint)]              — 拒绝 lint");
    println!();
    println!("代码生成:");
    println!("  #[derive(Trait)]           — 自动实现 trait");
    println!("  #[repr(C/u8/transparent)]  — 控制内存布局");
    println!();
    println!("提示与约束:");
    println!("  #[must_use]                — 返回值必须使用");
    println!("  #[deprecated]              — 标记弃用");
    println!("  #[non_exhaustive]          — 不可穷举匹配");
    println!("  #[inline]                  — 内联提示");
    println!();
    println!("测试:");
    println!("  #[test]                    — 标记测试函数");
    println!("  #[should_panic]            — 应该 panic");
    println!("  #[ignore]                  — 跳过测试");

    println!("\n=== Lesson 060 完成 ===");
}
