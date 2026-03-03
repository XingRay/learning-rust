/// # Lesson 008 - 所有权基础 (Ownership)
///
/// 所有权是 Rust 最独特和核心的特性，它让 Rust 无需垃圾回收器即可保证内存安全。
///
/// ## 学习目标
/// - 理解所有权的三大规则
/// - 理解栈（Stack）与堆（Heap）的区别
/// - 掌握移动语义（Move）
/// - 掌握 Clone 深拷贝
/// - 理解 Copy trait 与 Copy 语义
/// - 理解函数调用中的所有权转移
/// - 理解返回值与所有权
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson008_ownership
/// ```

// =============================================================
// Lesson 008: 所有权基础 (Ownership)
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 所有权的三大规则
    // ---------------------------------------------------------
    // Rust 的所有权系统基于三条核心规则：
    //   规则一：Rust 中每一个值都有一个「所有者」（owner）变量
    //   规则二：每个值在任一时刻有且仅有一个所有者
    //   规则三：当所有者离开作用域（scope），该值将被丢弃（drop）
    println!("=== 1. 所有权的三大规则 ===");

    {
        // s 在这里还不存在
        let s = String::from("hello"); // 从这里开始，s 是 "hello" 的所有者
        println!("s = {}", s);         // s 有效，可以使用
    } // <-- s 离开作用域，Rust 自动调用 drop 函数，"hello" 的内存被释放

    // println!("{}", s); // ❌ 编译错误！s 已经离开作用域，不再有效

    // ---------------------------------------------------------
    // 2. 栈（Stack）与堆（Heap）
    // ---------------------------------------------------------
    // 栈：后进先出（LIFO），速度快，存储固定大小的数据
    //     - 整数、浮点数、布尔值、字符、固定大小的元组和数组
    //     - 数据直接存储在栈上
    //
    // 堆：动态分配，速度较慢，存储大小不确定的数据
    //     - String、Vec、Box 等
    //     - 数据存在堆上，栈上保存指向堆的指针 + 长度 + 容量
    println!("\n=== 2. 栈与堆 ===");

    // 栈上的数据（固定大小）
    let x: i32 = 42;           // i32 固定 4 字节，存在栈上
    let y: f64 = 3.14;         // f64 固定 8 字节，存在栈上
    let z: bool = true;        // bool 固定 1 字节，存在栈上
    println!("栈上数据: x = {}, y = {}, z = {}", x, y, z);

    // 堆上的数据（动态大小）
    let s1 = String::from("hello"); // 堆上分配字符串数据
    // s1 在栈上存储：指针(ptr) + 长度(len=5) + 容量(capacity>=5)
    // 实际的 "hello" 字节数据存储在堆上
    println!("堆上数据: s1 = {}, 长度 = {}, 容量 = {}", s1, s1.len(), s1.capacity());

    // ---------------------------------------------------------
    // 3. 移动语义（Move）
    // ---------------------------------------------------------
    // 对于堆上的数据，赋值操作会发生「移动」而不是「拷贝」
    // 移动后，原变量将不再有效
    println!("\n=== 3. 移动语义 (Move) ===");

    let s2 = String::from("world");
    let s3 = s2; // s2 的所有权「移动」给了 s3
    // 此时 s2 已经无效，s3 是 "world" 的新所有者
    // println!("{}", s2); // ❌ 编译错误！value borrowed here after move
    println!("s3 = {} (从 s2 移动而来)", s3);

    // 为什么要移动而不是拷贝？
    // 如果 s2 和 s3 都指向同一块堆内存，当它们离开作用域时
    // 会两次释放同一块内存（double free），这是严重的内存错误！
    // Rust 通过移动语义避免了这个问题。

    // 移动的可视化：
    //
    // 移动前:
    //   s2: [ptr | len:5 | cap:5] ---> 堆上: "world"
    //
    // 移动后:
    //   s2: (已失效)
    //   s3: [ptr | len:5 | cap:5] ---> 堆上: "world"

    // ---------------------------------------------------------
    // 4. Clone 深拷贝
    // ---------------------------------------------------------
    // 如果确实需要深拷贝堆上的数据，使用 clone() 方法
    println!("\n=== 4. Clone 深拷贝 ===");

    let s4 = String::from("Rust");
    let s5 = s4.clone(); // 深拷贝：堆上的数据被完整复制了一份
    // s4 和 s5 各自拥有独立的堆内存，互不影响
    println!("s4 = {}, s5 = {}", s4, s5);
    println!("s4 和 s5 是独立的副本，修改一个不会影响另一个");

    // clone 的可视化：
    //
    //   s4: [ptr | len:4 | cap:4] ---> 堆上: "Rust" (第一块内存)
    //   s5: [ptr | len:4 | cap:4] ---> 堆上: "Rust" (第二块内存，独立副本)

    let mut s6 = String::from("Hello");
    let s7 = s6.clone();
    s6.push_str(", World!"); // 修改 s6 不会影响 s7
    println!("s6 = \"{}\" (已修改)", s6);
    println!("s7 = \"{}\" (保持不变)", s7);

    // ---------------------------------------------------------
    // 5. Copy trait 与 Copy 语义
    // ---------------------------------------------------------
    // 对于栈上的数据（实现了 Copy trait 的类型），赋值是「拷贝」而非「移动」
    // 因为栈上数据的拷贝非常快，不存在 double free 问题
    println!("\n=== 5. Copy trait 与 Copy 语义 ===");

    // 以下类型实现了 Copy trait：
    //   - 所有整数类型：i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
    //   - 浮点类型：f32, f64
    //   - 布尔类型：bool
    //   - 字符类型：char
    //   - 元组（当所有元素都实现了 Copy）：如 (i32, f64)
    //   - 固定大小数组（当元素实现了 Copy）：如 [i32; 5]
    //   - 不可变引用 &T

    let a = 10;
    let b = a; // 这里是 Copy，不是 Move！因为 i32 实现了 Copy trait
    println!("a = {}, b = {} (整数赋值是拷贝)", a, b);
    // a 仍然有效！

    let t1 = (1, 2.0, true);
    let t2 = t1; // 元组中所有元素都实现了 Copy，所以元组也是 Copy
    println!("t1 = {:?}, t2 = {:?} (元组拷贝)", t1, t2);

    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1; // 固定大小数组中元素实现了 Copy，数组也是 Copy
    println!("arr1 = {:?}, arr2 = {:?} (数组拷贝)", arr1, arr2);

    // 注意：String 没有实现 Copy trait，所以赋值是 Move
    // 规则：如果一个类型实现了 Drop trait，就不能实现 Copy trait
    // String 有 Drop（需要释放堆内存），所以不能 Copy

    // ---------------------------------------------------------
    // 6. 函数与所有权转移
    // ---------------------------------------------------------
    // 将值传递给函数，与赋值操作类似：
    //   - 对于 Move 类型（如 String），传参会移动所有权
    //   - 对于 Copy 类型（如 i32），传参会拷贝值
    println!("\n=== 6. 函数与所有权转移 ===");

    let msg = String::from("Hello from ownership!");
    takes_ownership(msg); // msg 的所有权移动到函数中
    // println!("{}", msg); // ❌ 编译错误！msg 已经被移动

    let num = 42;
    makes_copy(num); // num 被拷贝到函数中
    println!("num 仍然可用: {}", num); // ✅ 没问题！i32 是 Copy 的

    // ---------------------------------------------------------
    // 7. 返回值与所有权
    // ---------------------------------------------------------
    // 函数的返回值也可以转移所有权
    println!("\n=== 7. 返回值与所有权 ===");

    let s8 = gives_ownership(); // 函数将返回值的所有权移交给 s8
    println!("获得所有权: s8 = {}", s8);

    let s9 = String::from("take me");
    let s10 = takes_and_gives_back(s9); // s9 移入函数，返回值移入 s10
    // println!("{}", s9); // ❌ 编译错误！s9 已被移动
    println!("转交所有权: s10 = {}", s10);

    // 使用元组返回多个值（一种保留所有权的技巧，但不优雅）
    let s11 = String::from("calculate my length");
    let (s12, len) = calculate_length_tuple(s11);
    println!("\"{}\" 的长度是 {}", s12, len);
    // 注意：更好的做法是使用引用（下一课的内容）

    // ---------------------------------------------------------
    // 8. 综合示例
    // ---------------------------------------------------------
    println!("\n=== 8. 综合示例 ===");

    // 展示所有权在实际场景中的应用
    let names = create_greeting_list();
    println!("问候列表:");
    for name in &names {
        println!("  - {}", name);
    }

    // Vec 也是堆上的数据，遵循移动语义
    let v1 = vec![1, 2, 3];
    let v2 = v1; // v1 的所有权移动到 v2
    // println!("{:?}", v1); // ❌ 编译错误！v1 已被移动
    println!("v2 = {:?} (从 v1 移动而来)", v2);

    // Vec 的深拷贝
    let v3 = vec![4, 5, 6];
    let v4 = v3.clone();
    println!("v3 = {:?}, v4 = {:?} (独立副本)", v3, v4);

    println!("\n🎉 恭喜！你已经掌握了 Rust 所有权的基础知识！");
    println!("💡 提示：所有权系统是 Rust 内存安全的基石，");
    println!("   下一课我们将学习「引用与借用」，让数据共享更加灵活。");
}

/// 获取 String 的所有权，函数结束后 String 被 drop
fn takes_ownership(s: String) {
    println!("函数获得了所有权: {}", s);
} // s 在这里离开作用域，drop 被调用，内存被释放

/// 获取 i32 的拷贝，原始值不受影响
fn makes_copy(n: i32) {
    println!("函数获得了拷贝: {}", n);
} // n 离开作用域，但因为是栈上的 Copy 类型，没什么特别的事情发生

/// 函数创建一个 String 并将其所有权返回给调用者
fn gives_ownership() -> String {
    let s = String::from("I am yours now");
    s // 返回 s，所有权移交给调用者
}

/// 获取一个 String 的所有权，然后将其返回
fn takes_and_gives_back(s: String) -> String {
    println!("暂时持有: {}", s);
    s // 将所有权归还给调用者
}

/// 使用元组返回字符串和长度（这样可以归还所有权）
fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // 将 s 的所有权和长度一起返回
}

/// 创建一个包含问候语的列表
fn create_greeting_list() -> Vec<String> {
    let mut greetings = Vec::new();
    greetings.push(String::from("你好，Rust！"));
    greetings.push(String::from("Hello, Ownership！"));
    greetings.push(String::from("所有权让内存安全成为可能！"));
    greetings // 所有权转移给调用者
}
