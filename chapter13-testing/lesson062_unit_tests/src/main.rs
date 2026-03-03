// ============================================================
// Lesson 062: 单元测试 (Unit Tests)
// ============================================================
// 本课学习 Rust 中的单元测试机制，包括：
// - #[cfg(test)] 条件编译模块
// - #[test] 属性标记测试函数
// - assert!、assert_eq!、assert_ne! 断言宏
// - #[should_panic] 测试 panic 行为
// - 返回 Result<(), E> 的测试函数
// - #[ignore] 忽略耗时测试
// - cargo test 命令与过滤技巧
//
// 在 Rust 中，单元测试通常与被测代码放在同一文件中，
// 写在一个被 #[cfg(test)] 标注的模块里。
// 这样测试代码只在运行 `cargo test` 时才会编译，
// 不会影响正式发布的二进制文件大小。
// ============================================================

// ---- 被测试的函数和结构体 ----

/// 将两个数相加
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 计算矩形面积
fn rectangle_area(width: f64, height: f64) -> f64 {
    if width < 0.0 || height < 0.0 {
        panic!("宽度和高度不能为负数！width={}, height={}", width, height);
    }
    width * height
}

/// 判断一个数是否为偶数
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// 在字符串前后添加括号
fn wrap_in_brackets(s: &str) -> String {
    format!("[{}]", s)
}

/// 除法运算，返回 Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 一个简单的温度转换器
struct Temperature {
    celsius: f64,
}

impl Temperature {
    fn new(celsius: f64) -> Self {
        Temperature { celsius }
    }

    /// 摄氏度转华氏度
    fn to_fahrenheit(&self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    /// 摄氏度转开尔文
    fn to_kelvin(&self) -> f64 {
        self.celsius + 273.15
    }

    /// 判断是否为沸点（标准大气压下）
    fn is_boiling(&self) -> bool {
        self.celsius >= 100.0
    }
}

/// Guess 结构体：值必须在 1-100 之间
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("猜测值必须在 1 到 100 之间，收到的值是: {}", value);
        }
        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

/// 模拟一个耗时的计算
fn slow_computation(n: u64) -> u64 {
    let mut result = 0u64;
    for i in 0..n {
        result += i;
    }
    result
}

fn main() {
    println!("=== Lesson 062: 单元测试 ===\n");

    // ---- 演示被测函数的正常使用 ----
    println!("--- 被测函数演示 ---");
    println!("add(2, 3) = {}", add(2, 3));
    println!("rectangle_area(5.0, 3.0) = {}", rectangle_area(5.0, 3.0));
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));
    println!("wrap_in_brackets(\"hello\") = {}", wrap_in_brackets("hello"));

    match divide(10.0, 3.0) {
        Ok(result) => println!("divide(10.0, 3.0) = {:.4}", result),
        Err(e) => println!("除法错误: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("divide(10.0, 0.0) = {}", result),
        Err(e) => println!("divide(10.0, 0.0) => 错误: {}", e),
    }

    let temp = Temperature::new(100.0);
    println!("\n--- 温度转换 ---");
    println!("{}°C = {}°F", temp.celsius, temp.to_fahrenheit());
    println!("{}°C = {} K", temp.celsius, temp.to_kelvin());
    println!("是否沸腾: {}", temp.is_boiling());

    let guess = Guess::new(50);
    println!("\n猜测值: {}", guess.value());

    println!("\nslow_computation(1000) = {}", slow_computation(1000));

    // ---- cargo test 使用说明 ----
    println!("\n--- cargo test 命令说明 ---");
    println!("运行所有测试:           cargo test");
    println!("运行特定测试:           cargo test test_add");
    println!("运行名字包含关键词的:   cargo test temperature");
    println!("运行被忽略的测试:       cargo test -- --ignored");
    println!("运行全部（含忽略的）:   cargo test -- --include-ignored");
    println!("显示 println! 输出:     cargo test -- --nocapture");
    println!("单线程运行测试:         cargo test -- --test-threads=1");
    println!("只列出测试不运行:       cargo test -- --list");
}

// ============================================================
// 单元测试模块
// ============================================================
// #[cfg(test)] 表示这个模块只在 `cargo test` 时编译
// 这意味着测试代码不会出现在最终的发布二进制文件中
// 测试模块可以访问父模块中的私有函数（这是单元测试的优势）
#[cfg(test)]
mod tests {
    // 使用 super::* 导入父模块的所有内容（包括私有函数）
    use super::*;

    // ---- 基本的 #[test] 和 assert! ----

    /// 最简单的测试：使用 assert! 宏
    /// assert!(expression) 当 expression 为 false 时测试失败
    #[test]
    fn test_is_even() {
        assert!(is_even(4), "4 应该是偶数");
        assert!(is_even(0), "0 应该是偶数");
        assert!(is_even(-2), "-2 应该是偶数");
        assert!(!is_even(3), "3 不应该是偶数");
        assert!(!is_even(1), "1 不应该是偶数");
    }

    // ---- assert_eq! 和 assert_ne! ----

    /// assert_eq!(left, right) 判断两个值相等
    /// 失败时会打印两个值，方便调试
    /// 注意：比较的类型需要实现 PartialEq 和 Debug trait
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-5, -3), -8);
    }

    /// assert_ne!(left, right) 判断两个值不相等
    #[test]
    fn test_add_not_equal() {
        assert_ne!(add(2, 3), 6, "2 + 3 不应该等于 6");
        assert_ne!(add(1, 1), 3);
    }

    /// 测试矩形面积计算
    #[test]
    fn test_rectangle_area() {
        assert_eq!(rectangle_area(5.0, 3.0), 15.0);
        assert_eq!(rectangle_area(0.0, 10.0), 0.0);
        assert_eq!(rectangle_area(1.0, 1.0), 1.0);
    }

    /// 测试浮点数比较
    /// 浮点数比较不能直接用 assert_eq!（精度问题），
    /// 通常使用差值小于一个很小的阈值来判断
    #[test]
    fn test_float_comparison() {
        let result: f64 = 0.1 + 0.2;
        // assert_eq!(result, 0.3); // 这会失败！浮点数精度问题
        let epsilon: f64 = 1e-10;
        assert!(
            (result - 0.3_f64).abs() < epsilon,
            "0.1 + 0.2 应该约等于 0.3，实际值: {}",
            result
        );
    }

    /// 测试字符串操作
    #[test]
    fn test_wrap_in_brackets() {
        assert_eq!(wrap_in_brackets("hello"), "[hello]");
        assert_eq!(wrap_in_brackets(""), "[]");
        assert_eq!(wrap_in_brackets("Rust"), "[Rust]");
    }

    // ---- 温度转换测试 ----

    #[test]
    fn test_temperature_to_fahrenheit() {
        let temp = Temperature::new(0.0);
        assert_eq!(temp.to_fahrenheit(), 32.0);

        let temp = Temperature::new(100.0);
        assert_eq!(temp.to_fahrenheit(), 212.0);
    }

    #[test]
    fn test_temperature_to_kelvin() {
        let temp = Temperature::new(0.0);
        assert_eq!(temp.to_kelvin(), 273.15);

        let temp = Temperature::new(-273.15);
        assert_eq!(temp.to_kelvin(), 0.0);
    }

    #[test]
    fn test_temperature_is_boiling() {
        assert!(Temperature::new(100.0).is_boiling());
        assert!(Temperature::new(150.0).is_boiling());
        assert!(!Temperature::new(99.9).is_boiling());
        assert!(!Temperature::new(0.0).is_boiling());
    }

    // ---- #[should_panic] 测试 panic ----

    /// #[should_panic] 标记期望函数会 panic
    /// 如果函数没有 panic，测试反而失败
    #[test]
    #[should_panic]
    fn test_rectangle_negative_width() {
        rectangle_area(-1.0, 5.0); // 应该 panic
    }

    /// 可以指定 expected 参数，检查 panic 信息中是否包含特定字符串
    /// 这样可以确保是预期的 panic，而不是其他意外的 panic
    #[test]
    #[should_panic(expected = "宽度和高度不能为负数")]
    fn test_rectangle_negative_height() {
        rectangle_area(5.0, -1.0); // 应该 panic 并包含特定信息
    }

    /// 测试 Guess::new 越界时的 panic
    #[test]
    #[should_panic(expected = "猜测值必须在 1 到 100 之间")]
    fn test_guess_too_large() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "猜测值必须在 1 到 100 之间")]
    fn test_guess_too_small() {
        Guess::new(0);
    }

    /// 测试 Guess 正常范围内不 panic
    #[test]
    fn test_guess_valid() {
        let g = Guess::new(50);
        assert_eq!(g.value(), 50);

        let g = Guess::new(1);
        assert_eq!(g.value(), 1);

        let g = Guess::new(100);
        assert_eq!(g.value(), 100);
    }

    // ---- 返回 Result 的测试 ----

    /// 测试函数可以返回 Result<(), E>
    /// 返回 Ok(()) 表示测试通过，返回 Err 表示测试失败
    /// 这种方式允许在测试中使用 ? 运算符，代码更简洁
    /// 注意：返回 Result 的测试不能同时使用 #[should_panic]
    #[test]
    fn test_divide_ok() -> Result<(), String> {
        let result = divide(10.0, 2.0)?;
        // 使用 ? 运算符，如果 divide 返回 Err，测试自动失败
        assert_eq!(result, 5.0);
        Ok(())
    }

    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "除数不能为零");
    }

    /// 使用 Result 测试多个操作的链式调用
    #[test]
    fn test_divide_chain() -> Result<(), String> {
        let a = divide(100.0, 4.0)?;
        let b = divide(a, 5.0)?;
        assert_eq!(b, 5.0);
        Ok(())
    }

    // ---- #[ignore] 忽略测试 ----

    /// #[ignore] 标记的测试默认不会运行
    /// 适用于耗时很长的测试，或者需要特殊环境的测试
    /// 运行被忽略的测试: cargo test -- --ignored
    /// 运行全部测试（包括被忽略的）: cargo test -- --include-ignored
    #[test]
    #[ignore = "这个测试比较耗时，仅在需要时运行"]
    fn test_slow_computation() {
        let result = slow_computation(1_000_000);
        assert_eq!(result, 499_999_500_000);
    }

    // ---- 自定义失败信息 ----

    /// assert! 系列宏支持格式化的自定义错误信息
    /// 当测试失败时，自定义信息有助于快速定位问题
    #[test]
    fn test_custom_failure_messages() {
        let x = 42;
        assert!(x > 0, "x 的值 {} 应该大于 0", x);
        assert_eq!(
            x, 42,
            "期望 x 等于 42，但实际值是 {}",
            x
        );
        assert_ne!(
            x, 0,
            "x 不应该为 0，但实际值是 {}",
            x
        );
    }

    // ---- 测试私有函数 ----

    /// Rust 允许在单元测试中测试私有函数
    /// 因为测试模块是被测模块的子模块，可以访问私有项
    /// 这是 Rust 与很多其他语言不同的地方
    #[test]
    fn test_private_functions() {
        // add、is_even 等都是私有函数（没有 pub），
        // 但在 #[cfg(test)] mod tests 中可以正常访问
        assert_eq!(add(1, 2), 3);
        assert!(is_even(10));
    }
}
