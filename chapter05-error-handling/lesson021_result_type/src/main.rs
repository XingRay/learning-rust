/// # Lesson 021 - Result 类型
///
/// 本课介绍 Rust 中的 `Result<T, E>` 类型，用于处理可恢复的错误。
///
/// ## 学习目标
/// - 理解 `Result<T, E>` 的定义和用途
/// - 掌握 `Ok` 和 `Err` 变体的使用
/// - 学习 `unwrap` 和 `expect` 的使用场景
/// - 掌握 `match` 处理 Result 的模式
/// - 学习 `map`、`map_err`、`and_then` 等组合器
/// - 通过文件操作实例巩固 Result 的使用
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson021_result_type
/// ```

// =============================================================
// Lesson 021: Result 类型
// =============================================================

use std::fs::File;
use std::io::{self, Read, Write};
use std::num::ParseIntError;

fn main() {
    println!("===== Lesson 021: Result 类型 =====\n");

    // ---------------------------------------------------------
    // 1. Result<T, E> 的定义
    // ---------------------------------------------------------
    // Result 是 Rust 标准库中定义的枚举，用于表示操作可能成功或失败：
    //
    // enum Result<T, E> {
    //     Ok(T),   // 成功，包含成功的值，类型为 T
    //     Err(E),  // 失败，包含错误信息，类型为 E
    // }
    //
    // 与 Option 的区别：
    //   Option<T> = Some(T) | None          —— 有值或无值
    //   Result<T, E> = Ok(T) | Err(E)       —— 成功或失败（失败带有错误信息）
    println!("--- 1. Result<T, E> 的定义 ---");

    // 定义一个返回 Result 的函数：除法运算
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    let result1 = divide(10.0, 3.0);
    let result2 = divide(10.0, 0.0);
    println!("10 / 3 = {:?}", result1); // Ok(3.333...)
    println!("10 / 0 = {:?}", result2); // Err("除数不能为零")

    println!();

    // ---------------------------------------------------------
    // 2. Ok 和 Err
    // ---------------------------------------------------------
    println!("--- 2. Ok 和 Err ---");

    // 直接构造 Result 值
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("发生了错误");

    // 使用 is_ok() 和 is_err() 检查状态
    println!("success.is_ok() = {}", success.is_ok()); // true
    println!("success.is_err() = {}", success.is_err()); // false
    println!("failure.is_ok() = {}", failure.is_ok()); // false
    println!("failure.is_err() = {}", failure.is_err()); // true

    // ok() 将 Result<T, E> 转为 Option<T>（丢弃错误信息）
    println!("success.ok() = {:?}", success.ok()); // Some(42)
    println!("failure.ok() = {:?}", failure.ok()); // None

    // err() 将 Result<T, E> 转为 Option<E>（丢弃成功值）
    let success2: Result<i32, &str> = Ok(42);
    let failure2: Result<i32, &str> = Err("错误");
    println!("success.err() = {:?}", success2.err()); // None
    println!("failure.err() = {:?}", failure2.err()); // Some("错误")

    println!();

    // ---------------------------------------------------------
    // 3. unwrap 和 expect
    // ---------------------------------------------------------
    println!("--- 3. unwrap 和 expect ---");

    // unwrap: 成功时返回 Ok 中的值，失败时 panic
    let value: Result<i32, &str> = Ok(42);
    println!("Ok(42).unwrap() = {}", value.unwrap());

    // expect: 与 unwrap 类似，但可以自定义 panic 消息
    let value: Result<i32, &str> = Ok(100);
    println!("Ok(100).expect(\"...\") = {}", value.expect("应该有值"));

    // ⚠️ 注意：对 Err 调用 unwrap/expect 会 panic！
    // let bad: Result<i32, &str> = Err("error");
    // bad.unwrap(); // panic!

    // 更安全的替代方案：
    let err_result: Result<i32, &str> = Err("找不到");
    println!(
        "Err.unwrap_or(0) = {}",
        err_result.unwrap_or(0) // 提供默认值
    );

    let err_result: Result<i32, &str> = Err("找不到");
    println!(
        "Err.unwrap_or_else(|e| ...) = {}",
        err_result.unwrap_or_else(|e| {
            println!("  处理错误: {}", e);
            -1
        })
    );

    let err_result: Result<i32, &str> = Err("找不到");
    println!(
        "Err.unwrap_or_default() = {}",
        err_result.unwrap_or_default() // i32 的默认值是 0
    );

    println!();

    // ---------------------------------------------------------
    // 4. match 处理 Result
    // ---------------------------------------------------------
    // match 是处理 Result 最常见、最安全的方式。
    println!("--- 4. match 处理 Result ---");

    fn parse_number(s: &str) -> Result<i32, ParseIntError> {
        s.parse::<i32>()
    }

    let inputs = vec!["42", "hello", "100", "-7", "99999999999999"];

    for input in inputs {
        match parse_number(input) {
            Ok(num) => println!("  \"{}\" -> 解析成功: {}", input, num),
            Err(e) => println!("  \"{}\" -> 解析失败: {}", input, e),
        }
    }

    // 也可以使用 if let 只关注成功或失败的情况
    println!("\n  使用 if let 只处理成功情况：");
    if let Ok(num) = parse_number("256") {
        println!("  解析成功: {}", num);
    }

    // 使用 let else（Rust 1.65+）处理错误并提前返回
    // 注意：let else 要求发散（diverge），在 main 中我们用闭包演示
    let demo_let_else = || -> Result<(), String> {
        let Ok(num) = parse_number("42") else {
            return Err("解析失败".to_string());
        };
        println!("  let else 模式: 解析得到 {}", num);
        Ok(())
    };
    let _ = demo_let_else();

    println!();

    // ---------------------------------------------------------
    // 5. map 和 map_err
    // ---------------------------------------------------------
    // map:     对 Ok 中的值进行转换，Err 保持不变
    // map_err: 对 Err 中的错误进行转换，Ok 保持不变
    println!("--- 5. map 和 map_err ---");

    // map: 转换成功值
    let result: Result<i32, &str> = Ok(5);
    let doubled = result.map(|x| x * 2);
    println!("Ok(5).map(|x| x * 2) = {:?}", doubled); // Ok(10)

    let result: Result<i32, &str> = Err("错误");
    let doubled = result.map(|x| x * 2);
    println!("Err.map(|x| x * 2) = {:?}", doubled); // Err("错误")，不执行 map

    // map_err: 转换错误类型
    let result: Result<i32, &str> = Err("not a number");
    let mapped = result.map_err(|e| format!("解析错误: {}", e));
    println!("Err.map_err(...) = {:?}", mapped);

    // 链式调用 map
    let result: Result<String, &str> = Ok("  hello  ".to_string());
    let processed = result
        .map(|s| s.trim().to_string()) // 去除空白
        .map(|s| s.to_uppercase()); // 转大写
    println!("链式 map: {:?}", processed); // Ok("HELLO")

    println!();

    // ---------------------------------------------------------
    // 6. and_then（flatmap）
    // ---------------------------------------------------------
    // and_then 类似 map，但闭包返回的是 Result 而不是普通值。
    // 这允许你将多个可能失败的操作串联起来。
    println!("--- 6. and_then ---");

    fn parse_and_double(s: &str) -> Result<i32, String> {
        s.parse::<i32>()
            .map_err(|e| format!("解析错误: {}", e)) // 转换错误类型
            .and_then(|n| {
                // 检查是否溢出
                n.checked_mul(2)
                    .ok_or_else(|| format!("乘法溢出: {} * 2", n))
            })
    }

    println!("parse_and_double(\"21\") = {:?}", parse_and_double("21"));
    println!("parse_and_double(\"abc\") = {:?}", parse_and_double("abc"));
    println!(
        "parse_and_double(\"2000000000\") = {:?}",
        parse_and_double("2000000000")
    );

    // and_then 链式调用示例：验证用户年龄
    fn validate_age_string(input: &str) -> Result<u32, String> {
        input
            .trim()
            .parse::<u32>()
            .map_err(|e| format!("不是有效数字: {}", e))
            .and_then(|age| {
                if age == 0 {
                    Err("年龄不能为0".to_string())
                } else {
                    Ok(age)
                }
            })
            .and_then(|age| {
                if age > 150 {
                    Err(format!("年龄 {} 不合理", age))
                } else {
                    Ok(age)
                }
            })
    }

    let test_ages = vec!["25", "0", "200", "abc", " 30 "];
    for age_str in test_ages {
        println!(
            "  validate_age(\"{}\") = {:?}",
            age_str,
            validate_age_string(age_str)
        );
    }

    // or_else: 与 and_then 相反，在 Err 时执行
    let result: Result<i32, String> = Err("第一次失败".to_string());
    let recovered = result.or_else(|_| Ok::<i32, String>(0));
    println!("\n  Err.or_else(|_| Ok(0)) = {:?}", recovered); // Ok(0)

    println!();

    // ---------------------------------------------------------
    // 7. 文件操作实例 (File::open)
    // ---------------------------------------------------------
    // 文件操作是 Result 最常见的实际应用之一。
    // File::open 返回 Result<File, io::Error>。
    println!("--- 7. 文件操作实例 ---");

    // 尝试打开一个不存在的文件
    let result = File::open("这个文件不存在.txt");
    match &result {
        Ok(_) => println!("文件打开成功"),
        Err(e) => println!("文件打开失败: {} (错误类型: {:?})", e, e.kind()),
    }

    // 根据错误类型做不同处理
    match File::open("不存在的文件.txt") {
        Ok(file) => println!("打开成功: {:?}", file),
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                println!("文件未找到，可以尝试创建它");
            }
            io::ErrorKind::PermissionDenied => {
                println!("权限不足");
            }
            other => {
                println!("其他错误: {:?}", other);
            }
        },
    }

    // 完整的文件读写示例
    println!("\n  完整的文件读写示例：");
    let file_path = "lesson021_temp_test.txt";

    // 写入文件
    match File::create(file_path) {
        Ok(mut file) => {
            match file.write_all(b"Hello, Rust Result!\nThis is a test file.") {
                Ok(_) => println!("  写入文件成功"),
                Err(e) => println!("  写入失败: {}", e),
            }
        }
        Err(e) => println!("  创建文件失败: {}", e),
    }

    // 读取文件
    match File::open(file_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(bytes) => {
                    println!("  读取了 {} 字节: \"{}\"", bytes, contents);
                }
                Err(e) => println!("  读取失败: {}", e),
            }
        }
        Err(e) => println!("  打开文件失败: {}", e),
    }

    // 使用更简洁的方式（利用 and_then）
    let read_result = File::open(file_path).and_then(|mut f| {
        let mut s = String::new();
        f.read_to_string(&mut s).map(|_| s)
    });
    println!("  and_then 方式读取: {:?}", read_result);

    // 清理临时文件
    let _ = std::fs::remove_file(file_path);
    println!("  临时文件已清理");

    println!();

    // ---------------------------------------------------------
    // 8. Result 的其他实用方法
    // ---------------------------------------------------------
    println!("--- 8. Result 的其他实用方法 ---");

    // flatten: 将 Result<Result<T, E>, E> 压平为 Result<T, E>
    let nested: Result<Result<i32, &str>, &str> = Ok(Ok(42));
    println!("Ok(Ok(42)).flatten() = {:?}", nested.flatten());

    // transpose: 将 Result<Option<T>, E> 转为 Option<Result<T, E>>
    let result: Result<Option<i32>, &str> = Ok(Some(42));
    println!("Ok(Some(42)).transpose() = {:?}", result.transpose());

    let result: Result<Option<i32>, &str> = Ok(None);
    println!("Ok(None).transpose() = {:?}", result.transpose());

    // 收集 Result 的迭代器
    let strings = vec!["1", "2", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("收集成功: {:?}", numbers); // Ok([1, 2, 3])

    let strings = vec!["1", "abc", "3"];
    let numbers: Result<Vec<i32>, _> = strings.iter().map(|s| s.parse::<i32>()).collect();
    println!("收集失败: {:?}", numbers); // Err(...)，遇到第一个错误就停止

    println!("\n🎉 恭喜！你已经完成了 Lesson 021 —— Result 类型！");
}
