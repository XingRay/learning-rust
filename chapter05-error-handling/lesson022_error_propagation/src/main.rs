/// # Lesson 022 - 错误传播
///
/// 本课介绍 Rust 中的错误传播机制，学习如何优雅地将错误传递给调用者。
///
/// ## 学习目标
/// - 掌握 `?` 操作符的使用
/// - 了解 `?` 在 `main` 函数中的使用方式
/// - 理解 `From` trait 的自动错误类型转换
/// - 学习连续 `?` 链式调用
/// - 掌握 `Box<dyn Error>` 作为通用错误类型
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson022_error_propagation
/// ```

// =============================================================
// Lesson 022: 错误传播
// =============================================================

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("===== Lesson 022: 错误传播 =====\n");

    // ---------------------------------------------------------
    // 1. 没有 ? 操作符时的错误传播（繁琐的方式）
    // ---------------------------------------------------------
    println!("--- 1. 手动传播错误（繁琐方式） ---");

    // 不使用 ? 的情况下，需要手动 match 每一步
    fn read_number_verbose(s: &str) -> Result<i32, String> {
        // 手动匹配并传播错误
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err("输入为空".to_string());
        }
        match trimmed.parse::<i32>() {
            Ok(n) => Ok(n * 2),
            Err(e) => Err(format!("解析失败: {}", e)),
        }
    }

    println!("手动传播: {:?}", read_number_verbose("  42  "));
    println!("手动传播: {:?}", read_number_verbose("abc"));
    println!("手动传播: {:?}", read_number_verbose("  "));

    println!();

    // ---------------------------------------------------------
    // 2. ? 操作符
    // ---------------------------------------------------------
    // `?` 操作符是 Rust 中错误传播的语法糖。
    //
    // 当用在 Result<T, E> 上时：
    //   - 如果是 Ok(v)，取出 v 继续执行
    //   - 如果是 Err(e)，立即从当前函数返回 Err(e)
    //
    // 等价于：
    //   let value = match result {
    //       Ok(v) => v,
    //       Err(e) => return Err(e.into()),  // 注意 .into() 的自动转换
    //   };
    println!("--- 2. ? 操作符 ---");

    // 使用 ? 的简洁写法
    fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
        let x = a.parse::<i32>()?; // 如果解析失败，直接返回 Err
        let y = b.parse::<i32>()?; // 同上
        Ok(x + y)
    }

    println!("parse_and_add(\"10\", \"20\") = {:?}", parse_and_add("10", "20"));
    println!(
        "parse_and_add(\"10\", \"abc\") = {:?}",
        parse_and_add("10", "abc")
    );

    // ? 也可以用在 Option 上（返回类型必须是 Option）
    fn first_even(numbers: &[i32]) -> Option<i32> {
        let first = numbers.first()?; // 如果列表为空，返回 None
        if first % 2 == 0 {
            Some(*first)
        } else {
            None
        }
    }

    println!("first_even(&[2, 3, 4]) = {:?}", first_even(&[2, 3, 4]));
    println!("first_even(&[1, 3, 5]) = {:?}", first_even(&[1, 3, 5]));
    println!("first_even(&[]) = {:?}", first_even(&[]));

    println!();

    // ---------------------------------------------------------
    // 3. From trait 自动转换
    // ---------------------------------------------------------
    // `?` 操作符不仅仅返回错误，它还会自动调用 `From::from()` 进行类型转换。
    // 这意味着如果你的函数返回 Result<T, MyError>，
    // 而某个操作返回 Result<_, OtherError>，
    // 只要 MyError 实现了 From<OtherError>，? 就能自动转换。
    println!("--- 3. From trait 自动转换 ---");

    // 定义自定义错误类型
    #[derive(Debug)]
    enum AppError {
        IoError(io::Error),
        ParseError(ParseIntError),
        CustomError(String),
    }

    // 实现 From trait 以支持自动转换
    impl From<io::Error> for AppError {
        fn from(e: io::Error) -> Self {
            AppError::IoError(e)
        }
    }

    impl From<ParseIntError> for AppError {
        fn from(e: ParseIntError) -> Self {
            AppError::ParseError(e)
        }
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::IoError(e) => write!(f, "IO 错误: {}", e),
                AppError::ParseError(e) => write!(f, "解析错误: {}", e),
                AppError::CustomError(msg) => write!(f, "自定义错误: {}", msg),
            }
        }
    }

    // 使用 ? 自动转换不同的错误类型
    fn read_config_value(path: &str) -> Result<i32, AppError> {
        let content = fs::read_to_string(path)?; // io::Error 自动转为 AppError
        let number = content.trim().parse::<i32>()?; // ParseIntError 自动转为 AppError
        Ok(number)
    }

    // 演示错误转换
    match read_config_value("不存在的配置文件.txt") {
        Ok(v) => println!("配置值: {}", v),
        Err(e) => println!("错误（自动从 io::Error 转换）: {}", e),
    }

    // 先写一个测试文件，再读取
    let test_path = "lesson022_temp_config.txt";
    fs::write(test_path, "not_a_number").unwrap();
    match read_config_value(test_path) {
        Ok(v) => println!("配置值: {}", v),
        Err(e) => println!("错误（自动从 ParseIntError 转换）: {}", e),
    }
    let _ = fs::remove_file(test_path);

    // 写入有效数字
    fs::write(test_path, "42").unwrap();
    match read_config_value(test_path) {
        Ok(v) => println!("配置值（成功）: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    let _ = fs::remove_file(test_path);

    println!();

    // ---------------------------------------------------------
    // 4. 连续 ? 链式调用
    // ---------------------------------------------------------
    // 多个 ? 可以链式使用，使代码非常简洁。
    println!("--- 4. 连续 ? 链式调用 ---");

    // 示例：读取文件 -> 解析配置 -> 验证 -> 计算结果
    #[derive(Debug)]
    struct Config {
        width: u32,
        height: u32,
    }

    fn parse_config(content: &str) -> Result<Config, AppError> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() < 2 {
            return Err(AppError::CustomError("配置文件至少需要两行".to_string()));
        }
        let width = lines[0].trim().parse::<u32>()?; // ? 链式调用 1
        let height = lines[1].trim().parse::<u32>()?; // ? 链式调用 2
        Ok(Config { width, height })
    }

    fn load_and_compute_area(path: &str) -> Result<u32, AppError> {
        let content = fs::read_to_string(path)?; // ? 链式调用 1
        let config = parse_config(&content)?; // ? 链式调用 2
        let area = config
            .width
            .checked_mul(config.height)
            .ok_or_else(|| AppError::CustomError("面积计算溢出".to_string()))?; // ? 链式调用 3
        Ok(area)
    }

    // 测试链式调用
    let config_path = "lesson022_temp_config2.txt";
    fs::write(config_path, "800\n600").unwrap();
    match load_and_compute_area(config_path) {
        Ok(area) => println!("计算面积: {} (800 * 600)", area),
        Err(e) => println!("错误: {}", e),
    }
    let _ = fs::remove_file(config_path);

    // 测试链式调用中的错误情况
    fs::write(config_path, "800\nabc").unwrap();
    match load_and_compute_area(config_path) {
        Ok(area) => println!("面积: {}", area),
        Err(e) => println!("链式调用中的错误: {}", e),
    }
    let _ = fs::remove_file(config_path);

    // ? 用在方法调用链中也很方便
    fn read_first_line(path: &str) -> Result<String, io::Error> {
        // 读取文件、取第一行 —— 链式操作
        let content = fs::read_to_string(path)?;
        Ok(content.lines().next().unwrap_or("").to_string())
    }

    let test_path = "lesson022_temp_firstline.txt";
    fs::write(test_path, "第一行内容\n第二行内容").unwrap();
    match read_first_line(test_path) {
        Ok(line) => println!("第一行: \"{}\"", line),
        Err(e) => println!("错误: {}", e),
    }
    let _ = fs::remove_file(test_path);

    println!();

    // ---------------------------------------------------------
    // 5. Box<dyn Error> 通用错误类型
    // ---------------------------------------------------------
    // 当函数可能返回多种不同的错误类型时，可以使用 Box<dyn Error> 作为统一的错误类型。
    // 这是一种简单但灵活的方式，适合应用程序代码和原型代码。
    //
    // dyn Error 是一个 trait 对象，任何实现了 std::error::Error 的类型都能转换为它。
    println!("--- 5. Box<dyn Error> ---");

    fn do_complex_operation(input: &str) -> Result<i64, Box<dyn Error>> {
        // parse 可能返回 ParseIntError
        let number: i64 = input.parse()?;

        // 假设我们需要做文件操作（可能返回 io::Error）
        // 这里用 fs::metadata 作为示例
        let _current_dir = std::env::current_dir()?; // io::Error

        // 自定义错误也可以通过 Box 返回
        if number < 0 {
            return Err("数字不能为负数".into()); // &str -> Box<dyn Error>
        }

        Ok(number * 100)
    }

    println!(
        "do_complex_operation(\"42\") = {:?}",
        do_complex_operation("42")
    );
    println!(
        "do_complex_operation(\"abc\") = {:?}",
        do_complex_operation("abc")
    );
    println!(
        "do_complex_operation(\"-5\") = {:?}",
        do_complex_operation("-5")
    );

    // 类型别名简化签名
    // 在实际项目中，经常定义类型别名来简化签名：
    type MyResult<T> = Result<T, Box<dyn Error>>;

    fn another_operation(s: &str) -> MyResult<u32> {
        let n: u32 = s.parse()?;
        Ok(n + 1)
    }
    println!(
        "another_operation(\"99\") = {:?}",
        another_operation("99")
    );

    println!();

    // ---------------------------------------------------------
    // 6. ? 在 main 中的使用
    // ---------------------------------------------------------
    // main 函数默认返回 ()，不能直接使用 ?。
    // 但 main 函数可以返回 Result<(), E>，这样就能使用 ? 了。
    //
    // 方式一：main() -> Result<(), Box<dyn Error>>
    // 方式二：main() -> Result<(), 自定义Error>
    //
    // 当 main 返回 Err 时，程序会打印 Debug 格式的错误信息并以非零状态码退出。
    println!("--- 6. ? 在 main 中的使用 ---");
    println!("main 函数可以这样声明以支持 ?：");
    println!("  fn main() -> Result<(), Box<dyn Error>> {{");
    println!("      let content = fs::read_to_string(\"config.txt\")?;");
    println!("      // ... 更多使用 ? 的操作");
    println!("      Ok(())");
    println!("  }}");
    println!();
    println!("本课的 main 函数保持返回 ()，以便演示更多内容。");
    println!("实际项目中，让 main 返回 Result 是很常见的做法。");

    // 用子函数模拟 main 返回 Result 的效果
    fn app_main() -> Result<(), Box<dyn Error>> {
        let test_path = "lesson022_temp_main_test.txt";
        fs::write(test_path, "12345")?;
        let content = fs::read_to_string(test_path)?;
        let number: i32 = content.trim().parse()?;
        println!("  模拟 main -> Result: 读取并解析得到 {}", number);
        fs::remove_file(test_path)?;
        Ok(())
    }

    match app_main() {
        Ok(()) => println!("  app_main 执行成功"),
        Err(e) => println!("  app_main 执行失败: {}", e),
    }

    // std::process::ExitCode 也可以作为 main 的返回类型（Rust 1.61+）
    println!("\n  也可以使用 ExitCode（Rust 1.61+）：");
    println!("  fn main() -> ExitCode {{ ... }}");

    println!();

    // ---------------------------------------------------------
    // 7. 实战综合示例
    // ---------------------------------------------------------
    println!("--- 7. 实战综合示例 ---");

    // 模拟一个简单的学生成绩处理系统
    #[derive(Debug)]
    struct Student {
        name: String,
        score: f64,
    }

    fn parse_student(line: &str) -> Result<Student, Box<dyn Error>> {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("格式错误，期望 '姓名,分数'，得到: '{}'", line).into());
        }
        let name = parts[0].trim().to_string();
        let score: f64 = parts[1].trim().parse()?;
        if !(0.0..=100.0).contains(&score) {
            return Err(format!("分数 {} 超出范围 [0, 100]", score).into());
        }
        Ok(Student { name, score })
    }

    fn process_student_data(data: &str) -> Result<f64, Box<dyn Error>> {
        let students: Result<Vec<Student>, _> =
            data.lines().filter(|l| !l.trim().is_empty()).map(parse_student).collect();
        let students = students?;
        if students.is_empty() {
            return Err("没有学生数据".into());
        }
        let total: f64 = students.iter().map(|s| s.score).sum();
        let avg = total / students.len() as f64;
        for s in &students {
            println!("  {} : {:.1} 分", s.name, s.score);
        }
        Ok(avg)
    }

    let data = "张三,85.5\n李四,92.0\n王五,78.3";
    match process_student_data(data) {
        Ok(avg) => println!("  平均分: {:.1}", avg),
        Err(e) => println!("  处理失败: {}", e),
    }

    println!("\n  测试错误数据：");
    let bad_data = "张三,85.5\n李四,abc";
    match process_student_data(bad_data) {
        Ok(avg) => println!("  平均分: {:.1}", avg),
        Err(e) => println!("  处理失败: {}", e),
    }

    println!("\n🎉 恭喜！你已经完成了 Lesson 022 —— 错误传播！");
}
