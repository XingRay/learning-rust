/// # Lesson 009 - 引用与借用 (References & Borrowing)
///
/// 引用允许你在不获取所有权的情况下使用值，这就是「借用」。
///
/// ## 学习目标
/// - 理解不可变引用 `&T`
/// - 理解可变引用 `&mut T`
/// - 掌握借用规则（核心安全保证）
/// - 理解悬垂引用的防护机制
/// - 了解引用的作用域与非词法生命周期（NLL）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson009_references
/// ```

// =============================================================
// Lesson 009: 引用与借用 (References & Borrowing)
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 不可变引用 &T
    // ---------------------------------------------------------
    // 引用就像一个指针：它是一个地址，指向某个数据
    // 与指针不同的是，引用保证指向某个特定类型的有效值
    // 创建引用的操作称为「借用」（borrowing）
    println!("=== 1. 不可变引用 &T ===");

    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递 s1 的引用，而不是转移所有权
    // s1 仍然有效！因为我们只是「借出」了它，没有转移所有权
    println!("\"{}\" 的长度是 {}", s1, len);

    // 引用的可视化：
    //
    //   s1: [ptr | len:5 | cap:5] ---> 堆上: "hello"
    //            ↑
    //   &s1: [ptr] （指向 s1 的地址）
    //
    // &s1 只是「借用」了 s1，s1 仍然是 "hello" 的所有者

    // 可以同时存在多个不可变引用
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("三个不可变引用: r1={}, r2={}, r3={}", r1, r2, r3);
    // 多个不可变引用是安全的，因为没有人能修改数据

    // ---------------------------------------------------------
    // 2. 可变引用 &mut T
    // ---------------------------------------------------------
    // 如果需要通过引用修改数据，需要使用可变引用
    println!("\n=== 2. 可变引用 &mut T ===");

    let mut s2 = String::from("hello");
    println!("修改前: {}", s2);

    change(&mut s2); // 传递可变引用
    println!("修改后: {}", s2);

    // 可变引用允许修改借用的值
    let mut score = 100;
    let score_ref = &mut score;
    *score_ref += 50; // 通过解引用操作符 * 修改值
    println!("分数: {}", score_ref);
    // 注意：score_ref 的作用域到这里就结束了（NLL）

    println!("最终分数: {}", score); // 现在可以直接使用 score 了

    // ---------------------------------------------------------
    // 3. 借用规则（Rust 的核心安全保证）
    // ---------------------------------------------------------
    // 规则：在任意给定时间，你只能拥有以下两者之一：
    //   - 一个可变引用（&mut T）
    //   - 任意数量的不可变引用（&T）
    //
    // 这两者不能同时存在！这防止了数据竞争（data race）
    println!("\n=== 3. 借用规则 ===");

    // 规则 3a: 同一时间只能有一个可变引用
    let mut s3 = String::from("hello");
    let r_mut1 = &mut s3;
    // let r_mut2 = &mut s3; // ❌ 编译错误！不能同时有两个可变引用
    println!("唯一的可变引用: {}", r_mut1);
    // r_mut1 的作用域在最后一次使用后结束（NLL）

    // 现在可以创建新的可变引用了
    let r_mut2 = &mut s3;
    r_mut2.push_str(" world");
    println!("新的可变引用: {}", r_mut2);

    // 规则 3b: 可变引用和不可变引用不能同时存在
    let mut s4 = String::from("hello");
    let r_imm1 = &s4;
    let r_imm2 = &s4;
    // let r_mut3 = &mut s4; // ❌ 编译错误！已经有不可变引用了
    println!("不可变引用: {} 和 {}", r_imm1, r_imm2);
    // r_imm1 和 r_imm2 的作用域在最后一次使用后结束（NLL）

    // 现在可以创建可变引用了
    let r_mut3 = &mut s4;
    r_mut3.push_str(" rust");
    println!("可变引用: {}", r_mut3);

    // 为什么需要这些规则？——防止数据竞争
    // 数据竞争发生在以下三个条件同时满足时：
    //   1. 两个或更多指针同时访问同一数据
    //   2. 至少有一个指针被用来写入数据
    //   3. 没有同步数据访问的机制
    // Rust 在编译时就阻止了数据竞争！

    // ---------------------------------------------------------
    // 4. 悬垂引用防护
    // ---------------------------------------------------------
    // 悬垂引用（dangling reference）：引用指向的内存已经被释放
    // 在 C/C++ 中这是一个常见的 bug，但 Rust 编译器会阻止它
    println!("\n=== 4. 悬垂引用防护 ===");

    // 以下代码会编译错误，因为会产生悬垂引用：
    //
    // fn dangle() -> &String {      // ❌ 返回一个引用
    //     let s = String::from("hello");
    //     &s                        // ❌ 返回 s 的引用
    // }  // s 在这里离开作用域被释放，但引用仍然存在！
    //
    // Rust 编译器会报错：missing lifetime specifier
    // 这是 Rust 保证「引用永远有效」的方式

    // 正确的做法：返回拥有所有权的值
    let valid_string = no_dangle();
    println!("安全的返回值: {}", valid_string);

    // ---------------------------------------------------------
    // 5. 引用的作用域与 NLL (Non-Lexical Lifetimes)
    // ---------------------------------------------------------
    // 从 Rust 2018 edition 开始，引用的作用域从声明处开始，
    // 到最后一次使用处结束（而不是到大括号 } 结束）
    // 这就是非词法生命周期（NLL）
    println!("\n=== 5. 引用的作用域与 NLL ===");

    let mut data = String::from("initial");

    // NLL 示例：引用的作用域在最后一次使用处结束
    let ref1 = &data;          // -- ref1 作用域开始
    let ref2 = &data;          // -- ref2 作用域开始
    println!("ref1={}, ref2={}", ref1, ref2);
    // -- ref1 和 ref2 作用域结束（最后一次使用）

    // 因为 ref1 和 ref2 已经不再使用，所以可以创建可变引用
    let ref3 = &mut data;      // ✅ 没问题！
    ref3.push_str(" + modified");
    println!("ref3={}", ref3);

    // 没有 NLL 的话（老版本 Rust），上面的代码会报错
    // 因为 ref1/ref2 的作用域会延伸到大括号结束

    // ---------------------------------------------------------
    // 6. 引用的实际应用示例
    // ---------------------------------------------------------
    println!("\n=== 6. 引用的实际应用示例 ===");

    // 示例 1: 使用引用比较字符串
    let greeting1 = String::from("Hello");
    let greeting2 = String::from("Hello");
    let greeting3 = String::from("World");
    println!("{} == {}? {}", greeting1, greeting2, are_equal(&greeting1, &greeting2));
    println!("{} == {}? {}", greeting1, greeting3, are_equal(&greeting1, &greeting3));

    // 示例 2: 使用可变引用修改 Vec
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("原始: {:?}", numbers);
    double_values(&mut numbers);
    println!("翻倍: {:?}", numbers);

    // 示例 3: 函数接收引用，避免所有权转移
    let user = String::from("Alice");
    print_greeting(&user);    // 借用 user
    print_farewell(&user);    // 再次借用 user，没有问题！
    println!("用户 {} 仍然有效", user); // user 仍然可用

    // ---------------------------------------------------------
    // 7. 引用规则总结
    // ---------------------------------------------------------
    println!("\n=== 7. 引用规则总结 ===");
    println!("📌 规则一: 在任意时刻，要么只有一个可变引用，要么有多个不可变引用");
    println!("📌 规则二: 引用必须始终有效（不能悬垂）");
    println!("📌 规则三: 引用的作用域从声明到最后一次使用（NLL）");
    println!("📌 核心理念: 共享不可变，可变不共享（Shared XOR Mutable）");

    println!("\n🎉 恭喜！你已经掌握了引用与借用的核心概念！");
    println!("💡 提示：引用让你无需转移所有权即可使用数据，");
    println!("   下一课我们将学习「切片」，一种特殊的引用类型。");
}

/// 通过不可变引用计算字符串长度（不获取所有权）
fn calculate_length(s: &String) -> usize {
    // s 是一个引用，我们只是「借用」了它
    // 不能修改借用的值：
    // s.push_str("!"); // ❌ 编译错误！不能通过不可变引用修改值
    s.len()
} // s 离开作用域，但因为它没有所有权，所以不会释放任何东西

/// 通过可变引用修改字符串
fn change(s: &mut String) {
    s.push_str(", world!"); // ✅ 可以通过可变引用修改值
}

/// 安全地返回一个 String（返回拥有所有权的值，而不是引用）
fn no_dangle() -> String {
    let s = String::from("safe string");
    s // 所有权转移给调用者，不会产生悬垂引用
}

/// 比较两个字符串是否相等（使用不可变引用）
fn are_equal(a: &String, b: &String) -> bool {
    a == b
}

/// 将 Vec 中的每个元素翻倍（使用可变引用）
fn double_values(v: &mut Vec<i32>) {
    for val in v.iter_mut() {
        *val *= 2;
    }
}

/// 打印问候语
fn print_greeting(name: &String) {
    println!("你好, {}！欢迎回来！", name);
}

/// 打印告别语
fn print_farewell(name: &String) {
    println!("再见, {}！期待下次相见！", name);
}
