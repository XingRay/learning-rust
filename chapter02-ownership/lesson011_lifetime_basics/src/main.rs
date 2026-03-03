/// # Lesson 011 - 生命周期基础 (Lifetime Basics)
///
/// 生命周期是 Rust 中引用的有效作用域，编译器通过生命周期来确保所有引用都是有效的。
///
/// ## 学习目标
/// - 理解为什么需要生命周期
/// - 掌握生命周期标注语法 `'a`
/// - 学会在函数中使用生命周期参数
/// - 学会在结构体中使用生命周期
/// - 理解生命周期省略规则（Elision Rules）
/// - 理解 `'static` 生命周期
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson011_lifetime_basics
/// ```

// =============================================================
// Lesson 011: 生命周期基础 (Lifetime Basics)
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 为什么需要生命周期？
    // ---------------------------------------------------------
    // 生命周期的核心目的：防止悬垂引用（dangling references）
    // 编译器的「借用检查器」（borrow checker）会比较引用的生命周期
    // 来确保所有引用在使用时都指向有效数据
    println!("=== 1. 为什么需要生命周期？ ===");

    // 悬垂引用示例（无法编译）：
    //
    // let r;                   // --------- 'a 开始
    // {
    //     let x = 5;           // --------- 'b 开始
    //     r = &x;              // r 引用 x
    // }                        // --------- 'b 结束，x 被释放
    // println!("{}", r);       // ❌ r 引用了已被释放的 x！
    //                          // --------- 'a 结束
    //
    // 编译器发现 'b（x 的生命周期）比 'a（r 的生命周期）短
    // 所以拒绝编译

    // 正确的写法：确保引用的生命周期不超过被引用数据的生命周期
    let x = 5;                   // --------- 'a 开始
    let r = &x;                  // r 引用 x，x 的生命周期足够长
    println!("r = {}", r);       // ✅ 安全！x 还活着
                                 // --------- 'a 结束

    // ---------------------------------------------------------
    // 2. 生命周期标注语法
    // ---------------------------------------------------------
    // 生命周期标注不改变引用的生命周期长短，它只是描述多个引用之间
    // 生命周期的关系，帮助编译器进行检查
    //
    // 语法：以撇号 ' 开头，通常用小写字母
    //   &'a i32      - 带有生命周期 'a 的不可变引用
    //   &'a mut i32  - 带有生命周期 'a 的可变引用
    println!("\n=== 2. 生命周期标注语法 ===");
    println!("生命周期标注以 ' 开头，如 'a, 'b, 'input 等");
    println!("标注本身不改变生命周期，只是描述引用间的关系");

    // ---------------------------------------------------------
    // 3. 函数中的生命周期参数
    // ---------------------------------------------------------
    // 当函数返回一个引用时，编译器需要知道返回值的生命周期
    // 与哪个输入参数的生命周期相关
    println!("\n=== 3. 函数中的生命周期参数 ===");

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        // result 的生命周期 = min(string1 的生命周期, string2 的生命周期)
        // 所以 result 在 string2 有效期间可以安全使用
        println!("更长的字符串: {}", result);
    }
    // println!("{}", result); // ⚠️ 如果取消注释，会编译错误
    // 因为 result 的生命周期受限于 string2，而 string2 已离开作用域

    // 另一个安全的用法
    let string3 = String::from("I am long");
    let result2;
    {
        let string4 = String::from("hi");
        result2 = longest(string3.as_str(), string4.as_str());
        println!("更长的字符串: {}", result2);
    }

    // 只与一个参数相关的生命周期
    let s = String::from("Hello World");
    let first = first_word_lt(&s);
    println!("第一个单词: {}", first);

    // ---------------------------------------------------------
    // 4. 生命周期标注的深入理解
    // ---------------------------------------------------------
    println!("\n=== 4. 生命周期标注的深入理解 ===");

    // 返回值的生命周期必须与某个输入参数相关
    // 如果返回值的引用不与任何输入参数相关，那它一定指向函数内部创建的值
    // 这会导致悬垂引用
    //
    // fn bad_function<'a>() -> &'a str {
    //     let s = String::from("hello");
    //     s.as_str()  // ❌ s 在函数结束时被释放，返回的引用无效
    // }
    //
    // 正确做法：返回拥有所有权的值
    // fn good_function() -> String {
    //     String::from("hello")
    // }

    // 当只有一个参数的生命周期与返回值相关时
    let text = String::from("Rust Programming Language");
    let announcement = first_sentence(&text);
    println!("提取结果: {}", announcement);

    // 多个生命周期参数的示例
    let s1 = "Hello";
    let s2 = "Hi";
    let (first, second) = first_and_second(s1, s2);
    println!("first={}, second={}", first, second);

    // ---------------------------------------------------------
    // 5. 结构体中的生命周期
    // ---------------------------------------------------------
    // 如果结构体持有引用，必须标注生命周期
    // 这确保结构体的实例不会比其引用的数据活得更久
    println!("\n=== 5. 结构体中的生命周期 ===");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence_text = novel.split('.').next().expect("找不到句号");
    let excerpt = ImportantExcerpt {
        part: first_sentence_text,
    };
    println!("摘录: {}", excerpt.part);
    println!("摘录长度: {}", excerpt.len());
    excerpt.announce_and_return("注意！");

    // 结构体的生命周期意味着：
    // ImportantExcerpt 的实例不能比 part 字段中的引用活得更久
    //
    // 以下代码会编译错误：
    // let excerpt2;
    // {
    //     let short_lived = String::from("temp");
    //     excerpt2 = ImportantExcerpt { part: &short_lived };
    // } // short_lived 在这里被释放
    // println!("{}", excerpt2.part); // ❌ 悬垂引用！

    // 结构体包含多个引用
    let key = String::from("name");
    let value = String::from("Rust");
    let pair = KeyValue {
        key: &key,
        value: &value,
    };
    println!("键值对: {} = {}", pair.key, pair.value);

    // ---------------------------------------------------------
    // 6. 生命周期省略规则 (Elision Rules)
    // ---------------------------------------------------------
    // 在某些常见的模式中，Rust 编译器可以自动推断生命周期
    // 这些规则被称为「生命周期省略规则」
    println!("\n=== 6. 生命周期省略规则 ===");

    // Rust 编译器使用三条规则来推断生命周期：
    //
    // 规则一（输入生命周期）：
    //   每个引用参数都获得自己的生命周期参数
    //   fn foo(x: &str)           -> fn foo<'a>(x: &'a str)
    //   fn foo(x: &str, y: &str)  -> fn foo<'a, 'b>(x: &'a str, y: &'b str)
    //
    // 规则二（输出生命周期 - 单参数）：
    //   如果只有一个输入生命周期参数，它被赋予所有输出生命周期
    //   fn foo(x: &str) -> &str   -> fn foo<'a>(x: &'a str) -> &'a str
    //
    // 规则三（输出生命周期 - 方法）：
    //   如果方法有 &self 或 &mut self 参数，self 的生命周期
    //   被赋予所有输出生命周期
    //   fn method(&self, x: &str) -> &str
    //       -> fn method<'a, 'b>(&'a self, x: &'b str) -> &'a str

    // 以下函数不需要手动标注生命周期（规则二自动推断）
    let msg = String::from("Hello, World!");
    let first = first_word_elided(&msg);
    println!("省略规则示例 - 第一个单词: {}", first);

    // 以下函数需要手动标注（编译器无法自动推断）
    // 因为有两个输入生命周期，编译器不知道返回值与哪个相关
    let a = String::from("alpha");
    let b = String::from("beta gamma");
    let longer = longest(&a, &b);
    println!("需要标注 - 更长的: {}", longer);

    println!("\n--- 省略规则总结 ---");
    println!("能省略: fn first_word(s: &str) -> &str");
    println!("  (只有一个输入引用，规则二自动推断)");
    println!("不能省略: fn longest(a: &str, b: &str) -> &str");
    println!("  (两个输入引用，编译器不知道返回值与哪个相关)");

    // ---------------------------------------------------------
    // 7. 'static 生命周期
    // ---------------------------------------------------------
    // 'static 表示引用可以在整个程序运行期间有效
    println!("\n=== 7. 'static 生命周期 ===");

    // 字符串字面量的生命周期是 'static
    // 因为它们被直接编码在程序的二进制文件中
    let s: &'static str = "I live for the entire program!";
    println!("静态字符串: {}", s);

    // 'static 并不意味着变量永远活着
    // 它意味着这个引用「可以」活得和程序一样久
    // 实际的生命周期仍然取决于作用域

    // 常见的 'static 数据
    let static_str: &'static str = "编译期嵌入的字符串";
    println!("字符串字面量: {}", static_str);

    // 使用 'static 作为 trait bound
    let owned = String::from("I own my data");
    print_static_or_owned(&owned); // &String 不是 'static，但函数接受 &str

    // 注意：不要滥用 'static！
    // 大多数情况下，问题不是需要 'static 生命周期
    // 而是需要正确地使用生命周期标注

    // ---------------------------------------------------------
    // 8. 综合示例：生命周期的实际应用
    // ---------------------------------------------------------
    println!("\n=== 8. 综合示例 ===");

    // 示例 1: 使用带生命周期的结构体构建文本解析器
    let document = String::from("Title: Rust Guide\nAuthor: Community\nVersion: 1.0");
    let parser = TextParser::new(&document);
    println!("行数: {}", parser.line_count());
    if let Some(title) = parser.find_value("Title") {
        println!("标题: {}", title);
    }
    if let Some(author) = parser.find_value("Author") {
        println!("作者: {}", author);
    }

    // 示例 2: 生命周期与泛型结合
    let numbers = vec![1, 5, 3, 9, 2, 8];
    let biggest = largest_ref(&numbers);
    println!("最大值: {}", biggest);

    // 示例 3: 函数中的多个生命周期
    let context = String::from("默认消息");
    let custom = String::from("自定义消息");
    let chosen = choose_message(&context, &custom, true);
    println!("选择的消息: {}", chosen);
    let chosen2 = choose_message(&context, &custom, false);
    println!("选择的消息: {}", chosen2);

    println!("\n=== 生命周期核心要点 ===");
    println!("📌 生命周期确保引用始终有效，防止悬垂引用");
    println!("📌 标注语法 'a 描述引用间的关系，不改变实际生命周期");
    println!("📌 结构体持有引用时必须标注生命周期");
    println!("📌 省略规则让大多数情况不需要手动标注");
    println!("📌 'static 表示引用可以活得和整个程序一样久");

    println!("\n🎉 恭喜！你已经掌握了生命周期的基础知识！");
    println!("💡 提示：生命周期是 Rust 最独特的概念之一，");
    println!("   随着实践的增加，你会越来越熟练地使用它。");
}

// =============================================================
// 函数定义
// =============================================================

/// 返回两个字符串切片中较长的那个
///
/// 生命周期标注 'a 表示：返回值的生命周期等于两个输入参数中较短的那个
/// 这是因为返回值可能是 x 也可能是 y，所以必须在两者都有效时才能使用
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// 返回第一个单词 - 需要显式标注生命周期
fn first_word_lt<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// 返回第一个单词 - 利用生命周期省略规则（不需要显式标注）
/// 编译器自动推断为: fn first_word_elided<'a>(s: &'a str) -> &'a str
fn first_word_elided(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}

/// 提取第一个句子
fn first_sentence(text: &str) -> &str {
    match text.find('.') {
        Some(pos) => &text[..pos],
        None => text,
    }
}

/// 返回两个引用（各自有独立的生命周期）
fn first_and_second<'a, 'b>(first: &'a str, second: &'b str) -> (&'a str, &'b str) {
    (first, second)
}

/// 根据条件选择消息
fn choose_message<'a>(default: &'a str, custom: &'a str, use_custom: bool) -> &'a str {
    if use_custom {
        custom
    } else {
        default
    }
}

/// 查找切片中的最大值引用
fn largest_ref<'a>(list: &'a [i32]) -> &'a i32 {
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// 打印字符串（接受 &str 即可，不需要 'static）
fn print_static_or_owned(s: &str) {
    println!("打印: {}", s);
}

// =============================================================
// 结构体定义
// =============================================================

/// 包含引用的结构体 - 必须标注生命周期
///
/// 'a 表示：ImportantExcerpt 的实例不能比 part 中的引用活得更久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// 为带生命周期的结构体实现方法
impl<'a> ImportantExcerpt<'a> {
    /// 返回 part 的长度（利用省略规则三：&self 的生命周期赋予返回值）
    fn len(&self) -> usize {
        self.part.len()
    }

    /// 方法中的生命周期省略（规则三）
    /// 完整签名: fn announce_and_return(&'a self, announcement: &str) -> &'a str
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("请注意: {}", announcement);
        self.part
    }
}

/// 包含两个引用的结构体
struct KeyValue<'a> {
    key: &'a str,
    value: &'a str,
}

/// 简单的文本解析器，持有对文本的引用
struct TextParser<'a> {
    content: &'a str,
}

impl<'a> TextParser<'a> {
    /// 创建新的解析器
    fn new(content: &'a str) -> Self {
        TextParser { content }
    }

    /// 统计行数
    fn line_count(&self) -> usize {
        self.content.lines().count()
    }

    /// 查找 "Key: Value" 格式中的值
    fn find_value(&self, key: &str) -> Option<&'a str> {
        for line in self.content.lines() {
            if let Some(stripped) = line.strip_prefix(key) {
                if let Some(value) = stripped.strip_prefix(": ") {
                    return Some(value);
                }
            }
        }
        None
    }
}
