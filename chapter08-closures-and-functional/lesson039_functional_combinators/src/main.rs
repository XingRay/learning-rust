/// # Lesson 039 - 函数式组合子
///
/// Rust 提供了丰富的函数式组合子（combinators），让我们可以用链式调用
/// 的风格优雅地处理 Option、Result 和 Iterator。
///
/// ## 学习目标
/// - 掌握 Option 的常用组合子：map、and_then、unwrap_or、filter
/// - 掌握 Result 的常用组合子：map、map_err、and_then、or_else
/// - 理解链式调用风格的优势
/// - 将 Iterator 与函数式组合子结合使用
///
/// ## 运行方式
/// 在项目根目录下执行:
/// ```bash
/// cargo run -p lesson039_functional_combinators
/// ```

// =============================================================
// Lesson 039: 函数式组合子 - Functional Combinators
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. Option 的 map
    // ---------------------------------------------------------
    // map 对 Some 中的值应用函数，None 则直接传递
    // 签名：fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Option<U>
    println!("=== 1. Option::map ===\n");

    let some_number: Option<i32> = Some(42);
    let none_number: Option<i32> = None;

    // 对 Some 值进行转换
    let doubled = some_number.map(|x| x * 2);
    let doubled_none = none_number.map(|x| x * 2);

    println!("Some(42).map(|x| x * 2) = {:?}", doubled);       // Some(84)
    println!("None.map(|x| x * 2)     = {:?}", doubled_none);   // None

    // map 可以改变内部类型
    let num_str = some_number.map(|x| format!("数字是: {}", x));
    println!("Some(42).map(格式化) = {:?}", num_str); // Some("数字是: 42")

    // 链式 map
    let result = Some(3)
        .map(|x| x + 1)       // Some(4)
        .map(|x| x * 10)      // Some(40)
        .map(|x| x.to_string()); // Some("40")
    println!("链式 map: {:?}", result);

    // 实际应用：安全地获取字符串长度
    let name: Option<String> = Some("Rustacean".to_string());
    let name_len = name.as_ref().map(|s| s.len());
    println!("名字长度: {:?}", name_len);
    println!();

    // ---------------------------------------------------------
    // 2. Option 的 and_then（flatmap）
    // ---------------------------------------------------------
    // and_then 类似 map，但闭包返回 Option，避免嵌套 Option
    // 签名：fn and_then<U, F: FnOnce(T) -> Option<U>>(self, f: F) -> Option<U>
    println!("=== 2. Option::and_then ===\n");

    // 如果使用 map，返回的是 Option<Option<T>>（嵌套）
    fn parse_number(s: &str) -> Option<i32> {
        s.parse::<i32>().ok()
    }

    let input: Option<&str> = Some("42");

    // 使用 map 会导致嵌套
    let nested = input.map(parse_number);
    println!("map 导致嵌套: {:?}", nested); // Some(Some(42))

    // 使用 and_then 展平
    let flat = input.and_then(parse_number);
    println!("and_then 展平: {:?}", flat); // Some(42)

    // None 的情况
    let invalid: Option<&str> = Some("abc");
    let result = invalid.and_then(parse_number);
    println!("解析 \"abc\": {:?}", result); // None

    let empty: Option<&str> = None;
    let result = empty.and_then(parse_number);
    println!("None.and_then: {:?}", result); // None

    // 链式 and_then：模拟多步骤解析
    fn get_first_char(s: &str) -> Option<char> {
        s.chars().next()
    }

    fn char_to_digit(c: char) -> Option<u32> {
        c.to_digit(10)
    }

    let result = Some("7hello")
        .and_then(|s| get_first_char(s))
        .and_then(char_to_digit);
    println!("解析第一个字符为数字: {:?}", result); // Some(7)

    let result = Some("hello")
        .and_then(|s| get_first_char(s))
        .and_then(char_to_digit);
    println!("非数字开头: {:?}", result); // None
    println!();

    // ---------------------------------------------------------
    // 3. Option 的 unwrap_or 系列
    // ---------------------------------------------------------
    // 提供默认值的不同方式
    println!("=== 3. Option::unwrap_or 系列 ===\n");

    let some_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;

    // unwrap_or：提供一个默认值
    println!("Some(42).unwrap_or(0) = {}", some_val.unwrap_or(0));
    println!("None.unwrap_or(0)     = {}", none_val.unwrap_or(0));

    // unwrap_or_else：惰性计算默认值（闭包只在 None 时执行）
    let default_val = none_val.unwrap_or_else(|| {
        println!("  (计算默认值...)");
        100 * 2
    });
    println!("None.unwrap_or_else = {}", default_val);

    // unwrap_or_default：使用类型的 Default trait 值
    let none_string: Option<String> = None;
    println!("None.unwrap_or_default() = \"{}\"", none_string.unwrap_or_default());

    let none_vec: Option<Vec<i32>> = None;
    println!("None.unwrap_or_default() = {:?}", none_vec.unwrap_or_default());

    // 实际应用：配置系统
    fn get_config(key: &str) -> Option<String> {
        match key {
            "host" => Some("localhost".to_string()),
            "port" => Some("8080".to_string()),
            _ => None,
        }
    }

    let host = get_config("host").unwrap_or_else(|| "127.0.0.1".to_string());
    let port = get_config("port").unwrap_or_else(|| "3000".to_string());
    let timeout = get_config("timeout").unwrap_or_else(|| "30".to_string());

    println!("配置 - host: {}, port: {}, timeout: {}", host, port, timeout);
    println!();

    // ---------------------------------------------------------
    // 4. Option 的 filter
    // ---------------------------------------------------------
    // filter 在 Some 的值满足条件时保留，否则变为 None
    // 签名：fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Option<T>
    println!("=== 4. Option::filter ===\n");

    let even_check = Some(4).filter(|x| x % 2 == 0);
    let odd_check = Some(3).filter(|x| x % 2 == 0);
    let none_check: Option<i32> = None;
    let none_filter = none_check.filter(|x| x % 2 == 0);

    println!("Some(4).filter(是偶数) = {:?}", even_check);   // Some(4)
    println!("Some(3).filter(是偶数) = {:?}", odd_check);     // None
    println!("None.filter(是偶数)    = {:?}", none_filter);   // None

    // 链式使用 map + filter
    let result = Some(15)
        .filter(|&x| x > 10)       // 大于10？是 -> Some(15)
        .map(|x| x * 2)             // 乘以2 -> Some(30)
        .filter(|&x| x < 50);       // 小于50？是 -> Some(30)
    println!("链式 filter + map: {:?}", result);

    // 实际应用：验证输入
    fn validate_age(age: Option<u32>) -> Option<u32> {
        age.filter(|&a| a >= 18 && a <= 120)
    }

    println!("验证年龄 25: {:?}", validate_age(Some(25)));   // Some(25)
    println!("验证年龄 10: {:?}", validate_age(Some(10)));   // None
    println!("验证年龄 None: {:?}", validate_age(None));     // None
    println!();

    // ---------------------------------------------------------
    // 5. Result 的 map
    // ---------------------------------------------------------
    // map 对 Ok 中的值应用函数，Err 则直接传递
    println!("=== 5. Result::map ===\n");

    let ok_val: Result<i32, String> = Ok(42);
    let err_val: Result<i32, String> = Err("出错了".to_string());

    let doubled_ok = ok_val.map(|x| x * 2);
    let doubled_err = err_val.map(|x| x * 2);

    println!("Ok(42).map(|x| x * 2) = {:?}", doubled_ok);     // Ok(84)
    println!("Err.map(|x| x * 2)    = {:?}", doubled_err);     // Err("出错了")

    // 链式 map
    let result: Result<String, String> = Ok(5)
        .map(|x| x + 1)
        .map(|x| x * 10)
        .map(|x| format!("结果: {}", x));
    println!("链式 map: {:?}", result); // Ok("结果: 60")
    println!();

    // ---------------------------------------------------------
    // 6. Result 的 map_err
    // ---------------------------------------------------------
    // map_err 对 Err 中的错误值应用函数，Ok 则直接传递
    // 常用于转换错误类型
    println!("=== 6. Result::map_err ===\n");

    let result: Result<i32, &str> = Err("not a number");

    // 转换错误类型
    let mapped: Result<i32, String> = result.map_err(|e| format!("解析错误: {}", e));
    println!("map_err: {:?}", mapped);

    // Ok 值不受影响
    let ok_result: Result<i32, &str> = Ok(42);
    let mapped_ok: Result<i32, String> = ok_result.map_err(|e| format!("错误: {}", e));
    println!("Ok.map_err: {:?}", mapped_ok); // Ok(42)

    // 实际应用：统一错误类型
    #[derive(Debug)]
    #[allow(dead_code)] // 教学示例，通过 Debug trait 输出字段值
    enum AppError {
        ParseError(String),
        ValidationError(String),
    }

    fn parse_age(input: &str) -> Result<u32, AppError> {
        input
            .parse::<u32>()
            .map_err(|e| AppError::ParseError(format!("无法解析 '{}': {}", input, e)))
            .and_then(|age| {
                if age > 0 && age <= 150 {
                    Ok(age)
                } else {
                    Err(AppError::ValidationError(format!("年龄 {} 不在有效范围内", age)))
                }
            })
    }

    println!("parse_age(\"25\") = {:?}", parse_age("25"));
    println!("parse_age(\"abc\") = {:?}", parse_age("abc"));
    println!("parse_age(\"200\") = {:?}", parse_age("200"));
    println!();

    // ---------------------------------------------------------
    // 7. Result 的 and_then
    // ---------------------------------------------------------
    // and_then 链式连接可能失败的操作
    // 签名：fn and_then<U, F: FnOnce(T) -> Result<U, E>>(self, op: F) -> Result<U, E>
    println!("=== 7. Result::and_then ===\n");

    fn parse_and_validate(input: &str) -> Result<u32, String> {
        input
            .parse::<u32>()
            .map_err(|e| format!("解析失败: {}", e))
            .and_then(|age| {
                if age >= 18 {
                    Ok(age)
                } else {
                    Err(format!("年龄 {} 不满18岁", age))
                }
            })
    }

    println!("验证 \"25\": {:?}", parse_and_validate("25"));
    println!("验证 \"10\": {:?}", parse_and_validate("10"));
    println!("验证 \"abc\": {:?}", parse_and_validate("abc"));

    // 多步链式操作
    fn process_input(input: &str) -> Result<String, String> {
        input
            .parse::<i32>()
            .map_err(|e| format!("步骤1失败: {}", e))
            .and_then(|n| {
                if n > 0 {
                    Ok(n)
                } else {
                    Err("步骤2失败: 数字必须为正".to_string())
                }
            })
            .and_then(|n| {
                if n <= 1000 {
                    Ok(format!("有效数字: {}", n))
                } else {
                    Err("步骤3失败: 数字太大".to_string())
                }
            })
    }

    println!("process(\"42\"):   {:?}", process_input("42"));
    println!("process(\"-5\"):   {:?}", process_input("-5"));
    println!("process(\"9999\"): {:?}", process_input("9999"));
    println!("process(\"xyz\"):  {:?}", process_input("xyz"));
    println!();

    // ---------------------------------------------------------
    // 8. Result 的 or_else
    // ---------------------------------------------------------
    // or_else 在 Err 时尝试恢复，提供备选操作
    // 签名：fn or_else<F, O: FnOnce(E) -> Result<T, F>>(self, op: O) -> Result<T, F>
    println!("=== 8. Result::or_else ===\n");

    fn parse_int(s: &str) -> Result<i32, String> {
        s.parse::<i32>().map_err(|e| e.to_string())
    }

    fn parse_with_fallback(s: &str) -> Result<i32, String> {
        parse_int(s).or_else(|_| {
            // 如果直接解析失败，尝试去掉空格后再解析
            parse_int(s.trim())
        }).or_else(|_| {
            // 还是失败，尝试解析为浮点数后取整
            s.trim()
                .parse::<f64>()
                .map(|f| f as i32)
                .map_err(|e| format!("所有解析尝试都失败了: {}", e))
        })
    }

    println!("parse_with_fallback(\"42\"):    {:?}", parse_with_fallback("42"));
    println!("parse_with_fallback(\" 42 \"):  {:?}", parse_with_fallback(" 42 "));
    println!("parse_with_fallback(\"3.14\"): {:?}", parse_with_fallback("3.14"));
    println!("parse_with_fallback(\"abc\"):  {:?}", parse_with_fallback("abc"));

    // Ok 值不受 or_else 影响
    let ok_result: Result<i32, String> = Ok(100);
    let result = ok_result.or_else(|_| Ok::<i32, String>(0));
    println!("Ok(100).or_else: {:?}", result); // Ok(100)
    println!();

    // ---------------------------------------------------------
    // 9. 链式调用风格
    // ---------------------------------------------------------
    // 组合多个 combinator 形成清晰的数据处理流水线
    println!("=== 9. 链式调用风格 ===\n");

    // 传统写法（嵌套 if/match）
    fn find_user_age_verbose(db: &[(String, Option<u32>)], name: &str) -> String {
        let mut found = None;
        for (n, age) in db {
            if n == name {
                found = Some(age);
                break;
            }
        }
        match found {
            Some(age_opt) => match age_opt {
                Some(age) => {
                    if *age >= 18 {
                        format!("{} 是成年人（{} 岁）", name, age)
                    } else {
                        format!("{} 是未成年人（{} 岁）", name, age)
                    }
                }
                None => format!("{} 没有填写年龄", name),
            },
            None => format!("找不到用户 {}", name),
        }
    }

    // 函数式写法（链式调用）
    fn find_user_age_functional(db: &[(String, Option<u32>)], name: &str) -> String {
        db.iter()
            .find(|(n, _)| n == name)                           // 查找用户
            .map(|(_, age)| age)                                 // 提取年龄字段
            .and_then(|age| *age)                                // 展平 Option<Option<u32>> -> Option<u32>
            .map(|age| {
                if age >= 18 {
                    format!("{} 是成年人（{} 岁）", name, age)
                } else {
                    format!("{} 是未成年人（{} 岁）", name, age)
                }
            })
            .unwrap_or_else(|| format!("找不到用户 {} 或未填写年龄", name))
    }

    let db = vec![
        ("Alice".to_string(), Some(25u32)),
        ("Bob".to_string(), Some(15u32)),
        ("Charlie".to_string(), None),
    ];

    println!("--- 传统写法 ---");
    println!("  {}", find_user_age_verbose(&db, "Alice"));
    println!("  {}", find_user_age_verbose(&db, "Bob"));
    println!("  {}", find_user_age_verbose(&db, "Charlie"));
    println!("  {}", find_user_age_verbose(&db, "Dave"));

    println!("--- 函数式写法 ---");
    println!("  {}", find_user_age_functional(&db, "Alice"));
    println!("  {}", find_user_age_functional(&db, "Bob"));
    println!("  {}", find_user_age_functional(&db, "Charlie"));
    println!("  {}", find_user_age_functional(&db, "Dave"));
    println!();

    // ---------------------------------------------------------
    // 10. Iterator 与函数式组合子结合
    // ---------------------------------------------------------
    println!("=== 10. Iterator 与函数式组合子结合 ===\n");

    // 示例数据
    let raw_scores = vec!["85", "92", "abc", "78", "invalid", "95", "88"];

    // 综合使用 Iterator + Option/Result 组合子
    // 解析成绩 → 过滤无效值 → 过滤及格成绩 → 计算平均分
    let valid_scores: Vec<i32> = raw_scores
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // 解析并过滤掉无效值
        .filter(|&score| score >= 0 && score <= 100) // 确保分数范围有效
        .collect();

    println!("原始数据: {:?}", raw_scores);
    println!("有效成绩: {:?}", valid_scores);

    let average = if valid_scores.is_empty() {
        0.0
    } else {
        valid_scores.iter().sum::<i32>() as f64 / valid_scores.len() as f64
    };
    println!("平均分: {:.1}", average);

    // 使用 flat_map 展平嵌套结构
    let sentences = vec!["hello world", "rust is great", "functional programming"];
    let words: Vec<&str> = sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("\n单词列表: {:?}", words);

    // 使用 fold 进行累积计算
    let numbers = vec![1, 2, 3, 4, 5];
    let sum_of_squares = numbers
        .iter()
        .map(|&x| x * x)
        .fold(0, |acc, x| acc + x);
    println!("平方和: {}", sum_of_squares);

    // 使用 zip + map 组合两个迭代器
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];

    let report: Vec<String> = names
        .iter()
        .zip(scores.iter())
        .map(|(name, score)| format!("{}: {} 分", name, score))
        .collect();
    println!("成绩报告: {:?}", report);

    // 复杂的链式操作：学生成绩处理
    println!("\n--- 学生成绩处理系统 ---");

    #[derive(Debug)]
    struct Student {
        name: String,
        scores: Vec<Option<i32>>,
    }

    let students = vec![
        Student {
            name: "Alice".to_string(),
            scores: vec![Some(85), Some(92), Some(78), None, Some(95)],
        },
        Student {
            name: "Bob".to_string(),
            scores: vec![Some(70), None, Some(65), Some(80), Some(75)],
        },
        Student {
            name: "Charlie".to_string(),
            scores: vec![Some(95), Some(98), Some(92), Some(96), Some(94)],
        },
    ];

    // 对每个学生：过滤有效分数 → 计算平均分 → 判定等级
    let results: Vec<String> = students
        .iter()
        .map(|student| {
            let valid_scores: Vec<i32> = student
                .scores
                .iter()
                .filter_map(|&s| s)                // 过滤掉 None
                .filter(|&s| s >= 0 && s <= 100)   // 有效范围
                .collect();

            let avg = if valid_scores.is_empty() {
                0.0
            } else {
                valid_scores.iter().sum::<i32>() as f64 / valid_scores.len() as f64
            };

            let grade = if avg >= 90.0 {
                "优秀"
            } else if avg >= 80.0 {
                "良好"
            } else if avg >= 70.0 {
                "中等"
            } else if avg >= 60.0 {
                "及格"
            } else {
                "不及格"
            };

            format!(
                "  {} - 有效成绩: {:?}, 平均分: {:.1}, 等级: {}",
                student.name, valid_scores, avg, grade
            )
        })
        .collect();

    for r in &results {
        println!("{}", r);
    }

    // 找出优秀学生
    let excellent_students: Vec<&str> = students
        .iter()
        .filter(|s| {
            let valid: Vec<i32> = s.scores.iter().filter_map(|&x| x).collect();
            let avg = if valid.is_empty() {
                0.0
            } else {
                valid.iter().sum::<i32>() as f64 / valid.len() as f64
            };
            avg >= 90.0
        })
        .map(|s| s.name.as_str())
        .collect();
    println!("\n优秀学生: {:?}", excellent_students);

    // ---------------------------------------------------------
    // 11. 其他常用 Option/Result 组合子
    // ---------------------------------------------------------
    println!("\n=== 11. 其他常用组合子 ===\n");

    // Option::zip - 将两个 Option 合并为元组
    let x: Option<i32> = Some(1);
    let y: Option<&str> = Some("hello");
    let z: Option<i32> = None;
    println!("Some(1).zip(Some(\"hello\")) = {:?}", x.zip(y));  // Some((1, "hello"))
    println!("Some(1).zip(None)          = {:?}", x.zip(z));    // None

    // Option::or - 如果是 None，返回另一个 Option
    let a: Option<i32> = None;
    let b: Option<i32> = Some(10);
    println!("None.or(Some(10))    = {:?}", a.or(b));           // Some(10)
    println!("Some(5).or(Some(10)) = {:?}", Some(5).or(b));     // Some(5)

    // Option::xor - 恰好一个为 Some 时返回 Some
    println!("Some(1).xor(None)    = {:?}", Some(1).xor(None::<i32>));    // Some(1)
    println!("Some(1).xor(Some(2)) = {:?}", Some(1).xor(Some(2)));       // None
    println!("None.xor(Some(2))    = {:?}", None::<i32>.xor(Some(2)));   // Some(2)

    // Result::unwrap_or_else
    let result: Result<i32, &str> = Err("错误");
    let val = result.unwrap_or_else(|e| {
        println!("  处理错误: {}", e);
        -1
    });
    println!("unwrap_or_else 结果: {}", val);

    // Option::is_some_and (Rust 1.70+)
    let x: Option<i32> = Some(42);
    println!("Some(42).is_some_and(|x| x > 40) = {}", x.is_some_and(|x| x > 40));
    println!("Some(42).is_some_and(|x| x > 50) = {}", x.is_some_and(|x| x > 50));

    println!("\n🎉 恭喜！你已经完成了函数式组合子的学习！");
}
