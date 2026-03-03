/// # Lesson 007 - 字符串
///
/// 本课学习 Rust 中的字符串类型及操作。
///
/// ## 学习目标
/// - 理解 `String` 与 `&str` 的区别
/// - 掌握字符串的多种创建方式
/// - 学会字符串拼接（+, format!, push_str）
/// - 了解字符串切片
/// - 掌握字符和字节的遍历
/// - 理解 UTF-8 编码
/// - 掌握常用字符串方法
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson007_strings
/// ```

// =============================================================
// Lesson 007: 字符串
// =============================================================

fn main() {
    println!("=== Lesson 007: 字符串 ===\n");

    // ---------------------------------------------------------
    // 1. String 与 &str 的区别
    // ---------------------------------------------------------
    println!("--- 1. String 与 &str 的区别 ---");

    // &str（字符串切片）：
    // - 不可变引用，指向 UTF-8 编码的字符串数据
    // - 可以指向字符串字面量（存在于二进制文件中）或 String 的一部分
    // - 固定大小：一个指针 + 一个长度
    let str_literal: &str = "Hello, Rust!"; // 字符串字面量，类型是 &str
    println!("&str: {}", str_literal);

    // String：
    // - 可增长、可修改、拥有所有权的 UTF-8 字符串
    // - 分配在堆上
    // - 由三部分组成：指针、长度、容量
    let string_owned: String = String::from("Hello, Rust!");
    println!("String: {}", string_owned);

    // 两者的关系
    // &str 可以看作 String 的视图/切片
    let s: String = String::from("你好世界");
    let s_ref: &str = &s; // String 可以自动解引用为 &str
    println!("String: {}", s);
    println!("&str:   {}", s_ref);

    // 大小对比
    println!(
        "\n内存大小: &str = {} 字节, String = {} 字节",
        std::mem::size_of::<&str>(),    // 16 字节（指针 + 长度）
        std::mem::size_of::<String>()   // 24 字节（指针 + 长度 + 容量）
    );

    // 何时使用哪个？
    // - 函数参数优先使用 &str（更灵活，同时接受 &str 和 &String）
    // - 需要拥有所有权或修改时使用 String
    print_info("直接传 &str");
    print_info(&String::from("也可以传 &String"));

    // ---------------------------------------------------------
    // 2. 字符串创建方式
    // ---------------------------------------------------------
    println!("\n--- 2. 字符串创建方式 ---");

    // 方式 1: 字符串字面量（&str）
    let s1 = "Hello";
    println!("字面量: {}", s1);

    // 方式 2: String::from()
    let s2 = String::from("你好");
    println!("String::from: {}", s2);

    // 方式 3: .to_string()
    let s3 = "世界".to_string();
    println!(".to_string(): {}", s3);

    // 方式 4: String::new() 创建空字符串
    let s4 = String::new();
    println!("String::new(): '{}' (空字符串，长度={})", s4, s4.len());

    // 方式 5: String::with_capacity() 预分配容量
    let s5 = String::with_capacity(100);
    println!(
        "with_capacity: 长度={}, 容量={}",
        s5.len(),
        s5.capacity()
    );

    // 方式 6: format! 宏
    let name = "Rust";
    let version = 2021;
    let s6 = format!("{} Edition {}", name, version);
    println!("format!: {}", s6);

    // 方式 7: 从字符集合创建
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let s7: String = chars.into_iter().collect();
    println!("collect: {}", s7);

    // 方式 8: 重复字符串
    let s8 = "ha".repeat(3);
    println!("repeat: {}", s8);

    // 方式 9: 从字节创建（需确保是有效 UTF-8）
    let bytes = vec![72, 101, 108, 108, 111]; // "Hello" 的 ASCII
    let s9 = String::from_utf8(bytes).expect("无效的 UTF-8");
    println!("from_utf8: {}", s9);

    // ---------------------------------------------------------
    // 3. 字符串拼接
    // ---------------------------------------------------------
    println!("\n--- 3. 字符串拼接 ---");

    // 方式 1: + 运算符
    // 注意: + 左边是 String（会被移动），右边是 &str
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let greeting = hello + &world; // hello 被移动，不能再用
    println!("+ 运算符: {}", greeting);
    // println!("{}", hello); // ❌ hello 已被移动
    println!("world 仍可用: {}", world); // ✅ world 只是借用

    // 连续拼接
    let s = String::from("tic") + "-" + "tac" + "-" + "toe";
    println!("连续 +: {}", s);

    // 方式 2: format! 宏（推荐！不会移动任何值）
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = format!("{}, {}!", s1, s2);
    println!("format!: {}", s3);
    println!("s1 仍可用: {}", s1); // ✅ format! 使用引用
    println!("s2 仍可用: {}", s2); // ✅

    // 方式 3: push_str() 追加字符串切片
    let mut s = String::from("Hello");
    s.push_str(", ");
    s.push_str("Rust");
    s.push_str("!");
    println!("push_str: {}", s);

    // 方式 4: push() 追加单个字符
    let mut s = String::from("Rust");
    s.push(' ');
    s.push('🦀');
    println!("push: {}", s);

    // 方式 5: 使用 extend
    let mut s = String::from("abc");
    s.extend(['d', 'e', 'f']);
    println!("extend: {}", s);

    // 方式 6: 使用迭代器 join
    let parts = vec!["Hello", "Beautiful", "World"];
    let joined = parts.join(" ");
    println!("join: {}", joined);

    let csv = vec!["apple", "banana", "cherry"];
    let csv_line = csv.join(", ");
    println!("join CSV: {}", csv_line);

    // ---------------------------------------------------------
    // 4. 字符串切片
    // ---------------------------------------------------------
    println!("\n--- 4. 字符串切片 ---");

    let s = String::from("Hello, World!");

    // 通过字节索引获取切片（注意：索引必须在 UTF-8 字符边界上）
    let hello = &s[0..5]; // "Hello"
    let world = &s[7..12]; // "World"
    println!("s[0..5] = '{}'", hello);
    println!("s[7..12] = '{}'", world);

    // 简写形式
    let hello = &s[..5]; // 从开头到索引 5
    let world = &s[7..]; // 从索引 7 到结尾
    let full = &s[..]; // 整个字符串
    println!("s[..5] = '{}'", hello);
    println!("s[7..] = '{}'", world);
    println!("s[..] = '{}'", full);

    // ⚠️ 中文字符串切片要小心！每个中文字符占 3 个字节（UTF-8）
    let chinese = "你好世界";
    let ni_hao = &chinese[0..6]; // "你好"（每个字 3 字节）
    println!("'{}' 的前 6 字节: '{}'", chinese, ni_hao);

    // 以下会 panic（在字符边界中间切割）：
    // let bad = &chinese[0..1]; // ❌ panic: byte index 1 is not a char boundary

    // 安全的方式：使用 char_indices
    println!("\n安全地切片中文字符串:");
    for (i, c) in chinese.char_indices() {
        println!("  字节索引 {}: '{}'", i, c);
    }

    // ---------------------------------------------------------
    // 5. 遍历字符与字节
    // ---------------------------------------------------------
    println!("\n--- 5. 遍历字符与字节 ---");

    let text = "Hello你好🦀";

    // 按字符遍历
    print!("字符遍历: ");
    for ch in text.chars() {
        print!("'{}' ", ch);
    }
    println!();
    println!("字符数量: {}", text.chars().count());

    // 按字节遍历
    print!("字节遍历: ");
    for byte in text.bytes() {
        print!("{:#04x} ", byte);
    }
    println!();
    println!("字节数量: {}", text.len()); // len() 返回字节数，不是字符数！

    // 按字符索引遍历
    println!("\nchar_indices 遍历:");
    for (byte_index, ch) in text.char_indices() {
        println!("  字节偏移 {:2}: '{}' (占 {} 字节)", byte_index, ch, ch.len_utf8());
    }

    // ⚠️ 重要：len() vs chars().count()
    let emoji = "🦀🎉🚀";
    println!("\n'{}' 的 len() = {} 字节", emoji, emoji.len());
    println!("'{}' 的 chars().count() = {} 个字符", emoji, emoji.chars().count());

    // 获取第 n 个字符
    let third_char = text.chars().nth(2);
    println!("第 3 个字符: {:?}", third_char);

    // ---------------------------------------------------------
    // 6. UTF-8 编码
    // ---------------------------------------------------------
    println!("\n--- 6. UTF-8 编码 ---");

    // Rust 的字符串都是 UTF-8 编码
    // 不同字符占用不同字节数：
    // - ASCII (a-z, 0-9 等): 1 字节
    // - 常见中文/日文/韩文: 3 字节
    // - Emoji: 4 字节

    let examples = [
        ("ASCII", "A"),
        ("中文", "中"),
        ("日文", "あ"),
        ("Emoji", "🦀"),
    ];

    println!("{:<10} {:<6} {:<6} {:<15}", "类型", "字符", "字节数", "UTF-8 编码");
    println!("{}", "-".repeat(40));
    for (category, s) in &examples {
        let bytes: Vec<String> = s.bytes().map(|b| format!("{:#04x}", b)).collect();
        println!(
            "{:<10} {:<6} {:<6} {}",
            category,
            s,
            s.len(),
            bytes.join(" ")
        );
    }

    // 验证字符串是否是有效 UTF-8
    let valid_bytes = vec![0xe4, 0xb8, 0xad]; // "中" 的 UTF-8 编码
    let invalid_bytes = vec![0xff, 0xfe]; // 无效的 UTF-8

    match String::from_utf8(valid_bytes) {
        Ok(s) => println!("\n有效 UTF-8: '{}'", s),
        Err(e) => println!("\n无效 UTF-8: {}", e),
    }
    match String::from_utf8(invalid_bytes) {
        Ok(s) => println!("有效 UTF-8: '{}'", s),
        Err(e) => println!("无效 UTF-8: {}", e),
    }

    // ---------------------------------------------------------
    // 7. 常用字符串方法
    // ---------------------------------------------------------
    println!("\n--- 7. 常用字符串方法 ---");

    // -- 长度相关 --
    let s = "Hello, 世界!";
    println!("字符串: '{}'", s);
    println!("  len() = {} (字节数)", s.len());
    println!("  chars().count() = {} (字符数)", s.chars().count());
    println!("  is_empty() = {}", s.is_empty());
    println!("  \"\".is_empty() = {}", "".is_empty());

    // -- 搜索与包含 --
    println!("\n搜索与包含:");
    let s = "Hello, beautiful World!";
    println!("  contains(\"beautiful\") = {}", s.contains("beautiful"));
    println!("  contains(\"ugly\") = {}", s.contains("ugly"));
    println!("  starts_with(\"Hello\") = {}", s.starts_with("Hello"));
    println!("  ends_with(\"!\") = {}", s.ends_with("!"));
    println!("  find(\"World\") = {:?}", s.find("World"));
    println!("  find(\"xyz\") = {:?}", s.find("xyz"));

    // -- 替换 --
    println!("\n替换:");
    let s = "Hello World Hello Rust";
    println!("  原始: '{}'", s);
    println!("  replace(\"Hello\", \"Hi\") = '{}'", s.replace("Hello", "Hi"));
    println!(
        "  replacen(\"Hello\", \"Hi\", 1) = '{}'",
        s.replacen("Hello", "Hi", 1)
    );

    // -- 大小写 --
    println!("\n大小写转换:");
    let s = "Hello World";
    println!("  to_uppercase() = '{}'", s.to_uppercase());
    println!("  to_lowercase() = '{}'", s.to_lowercase());

    // -- 去除空白 --
    println!("\n去除空白:");
    let s = "  Hello, World!  \n";
    println!("  原始: '{}'", s.trim_end());
    println!("  trim() = '{}'", s.trim());
    println!("  trim_start() = '{}'", s.trim_start().trim_end());
    println!("  trim_end() = '{}'", s.trim_end());

    // -- 分割 --
    println!("\n分割:");
    let csv = "apple,banana,cherry,date";
    let parts: Vec<&str> = csv.split(',').collect();
    println!("  split(','): {:?}", parts);

    let sentence = "  Hello   World   Rust  ";
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("  split_whitespace(): {:?}", words);

    let path = "/usr/local/bin";
    let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    println!("  split('/'): {:?}", components);

    // splitn 限制分割次数
    let data = "name:age:city:country";
    let fields: Vec<&str> = data.splitn(3, ':').collect();
    println!("  splitn(3, ':'): {:?}", fields);

    // -- 其他实用方法 --
    println!("\n其他实用方法:");

    // chars 和 集合转换
    let s = "hello";
    let capitalized: String = s
        .chars()
        .enumerate()
        .map(|(i, c)| if i == 0 { c.to_uppercase().next().unwrap() } else { c })
        .collect();
    println!("  首字母大写: '{}'", capitalized);

    // 数字和字符串转换
    let num = 42;
    let s = num.to_string();
    println!("  数字转字符串: {}", s);

    let parsed: i32 = "123".parse().unwrap();
    println!("  字符串转数字: {}", parsed);

    // 检查类型
    let s = "Hello123";
    println!("\n  '{}' 各字符属性:", s);
    for ch in s.chars() {
        println!(
            "    '{}': 字母={}, 数字={}, 字母或数字={}",
            ch,
            ch.is_alphabetic(),
            ch.is_numeric(),
            ch.is_alphanumeric()
        );
    }

    // 填充和对齐（通过 format! 实现）
    println!("\n格式化对齐:");
    println!("  左对齐:   '{:<20}'", "Hello");
    println!("  右对齐:   '{:>20}'", "Hello");
    println!("  居中:     '{:^20}'", "Hello");
    println!("  填充字符: '{:*^20}'", "Hello");
    println!("  填充字符: '{:-<20}'", "Hello");

    // ---------------------------------------------------------
    // 8. 综合示例
    // ---------------------------------------------------------
    println!("\n--- 8. 综合示例 ---");

    // 统计字符串中的单词数
    let text = "Rust is a systems programming language that runs blazingly fast";
    let word_count = text.split_whitespace().count();
    println!("文本: '{}'", text);
    println!("单词数: {}", word_count);

    // 反转字符串
    let original = "Hello, 世界!";
    let reversed: String = original.chars().rev().collect();
    println!("原始: '{}' → 反转: '{}'", original, reversed);

    // 检查回文
    let palindrome = "racecar";
    let is_palindrome = palindrome == palindrome.chars().rev().collect::<String>();
    println!("'{}' 是回文? {}", palindrome, is_palindrome);

    // 统计字符出现次数
    let text = "hello world";
    let l_count = text.chars().filter(|&c| c == 'l').count();
    println!("'{}' 中 'l' 出现 {} 次", text, l_count);

    // 截取安全（按字符而非字节）
    let text = "你好世界，欢迎来到Rust！";
    let first_four: String = text.chars().take(4).collect();
    println!("前 4 个字符: '{}'", first_four);

    // ---------------------------------------------------------
    // 9. 小结
    // ---------------------------------------------------------
    println!("\n--- 小结 ---");
    println!("✅ &str 是字符串切片（不可变引用），String 是拥有所有权的堆字符串");
    println!("✅ 多种创建方式: String::from, .to_string(), format!, String::new()");
    println!("✅ 拼接: + (移动左值), format!(不移动), push_str/push");
    println!("✅ 切片使用字节索引，需注意 UTF-8 字符边界");
    println!("✅ .chars() 遍历字符，.bytes() 遍历字节");
    println!("✅ Rust 字符串是 UTF-8 编码，len() 返回字节数");
    println!("✅ 常用方法: contains, replace, trim, split, find 等");

    println!("\n🎉 恭喜！你已经完成了第七课！");
}

/// 接受 &str 参数的函数（同时兼容 &str 和 &String）
fn print_info(info: &str) {
    println!("信息: {}", info);
}
