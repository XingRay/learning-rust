/// # Lesson 084 - unsafe 基础
///
/// 本课介绍 Rust 中的 unsafe 关键字及其核心概念。
///
/// ## 学习目标
/// - 理解 unsafe 块和 unsafe fn 的区别
/// - 掌握 unsafe 的五种超能力
/// - 理解何时使用 unsafe 以及安全抽象的概念
/// - 明白 unsafe 不意味着代码一定有错
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson084_unsafe_basics
/// ```

// =============================================================
// Lesson 084: unsafe 基础 - 突破 Rust 安全系统的边界
// =============================================================

fn main() {
    println!("=== Lesson 084: unsafe 基础 ===\n");

    // ---------------------------------------------------------
    // 1. unsafe 块 (unsafe block)
    // ---------------------------------------------------------
    // unsafe 块告诉编译器："我（程序员）已经手动验证了这段代码的安全性"
    // 编译器在 unsafe 块内不会进行某些安全检查
    println!("--- 1. unsafe 块 ---");

    let mut num = 42;

    // 创建裸指针是安全的，不需要 unsafe
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 但解引用裸指针需要 unsafe 块
    unsafe {
        println!("r1 指向的值: {}", *r1);
        println!("r2 指向的值: {}", *r2);
    }

    // ---------------------------------------------------------
    // 2. unsafe fn (unsafe 函数)
    // ---------------------------------------------------------
    // unsafe fn 声明整个函数体都是 unsafe 的上下文
    // 调用 unsafe fn 也需要在 unsafe 块中
    println!("\n--- 2. unsafe fn ---");

    unsafe {
        dangerous_function();
    }

    // 也可以在 unsafe fn 内部直接进行 unsafe 操作
    unsafe {
        let value = read_pointer_value(&42 as *const i32);
        println!("通过 unsafe fn 读取的值: {}", value);
    }

    // ---------------------------------------------------------
    // 3. unsafe 的五种超能力
    // ---------------------------------------------------------
    println!("\n--- 3. unsafe 的五种超能力 ---");

    // 超能力 1: 解引用裸指针 (*const T 和 *mut T)
    println!("\n  超能力 1: 解引用裸指针");
    let x = 10;
    let ptr = &x as *const i32;
    unsafe {
        println!("  解引用裸指针: *ptr = {}", *ptr);
    }

    // 超能力 2: 调用 unsafe 函数或方法
    println!("\n  超能力 2: 调用 unsafe 函数");
    unsafe {
        let result = add_unchecked(100, 200);
        println!("  unsafe 函数结果: {}", result);
    }

    // 超能力 3: 访问或修改可变静态变量
    println!("\n  超能力 3: 访问可变静态变量");
    // 不可变静态变量可以安全访问
    println!("  不可变静态变量 MAX_POINTS = {}", MAX_POINTS);
    // 可变静态变量需要 unsafe
    unsafe {
        COUNTER += 1;
        // 使用 addr_of! 或直接拷贝值来避免创建引用
        let val = std::ptr::addr_of!(COUNTER).read();
        println!("  可变静态变量 COUNTER = {}", val);
        COUNTER += 1;
        let val = std::ptr::addr_of!(COUNTER).read();
        println!("  修改后 COUNTER = {}", val);
    }

    // 超能力 4: 实现 unsafe trait
    println!("\n  超能力 4: 实现 unsafe trait");
    let marker = MyType;
    marker.do_something();

    // 超能力 5: 访问 union 的字段
    println!("\n  超能力 5: 访问 union 字段");
    let my_union = IntOrFloat { i: 42 };
    unsafe {
        // 访问 union 字段是 unsafe 的，因为 Rust 无法保证
        // 当前存储的是哪个字段的数据
        println!("  union 作为 i32: {}", my_union.i);
    }

    // 也可以写入另一个字段
    let my_union2 = IntOrFloat { f: 3.14 };
    unsafe {
        println!("  union 作为 f32: {}", my_union2.f);
    }

    // ---------------------------------------------------------
    // 4. 何时使用 unsafe
    // ---------------------------------------------------------
    println!("\n--- 4. 何时使用 unsafe ---");
    println!("  适合使用 unsafe 的场景:");
    println!("  ✓ 调用 C 语言库（FFI）");
    println!("  ✓ 实现底层数据结构（如链表、无锁队列）");
    println!("  ✓ 性能关键路径，且你能证明安全性");
    println!("  ✓ 调用硬件或操作系统 API");
    println!();
    println!("  不应该使用 unsafe 的场景:");
    println!("  ✗ 仅仅为了绕过借用检查器");
    println!("  ✗ 不确定是否安全的情况");
    println!("  ✗ 有安全替代方案时");

    // ---------------------------------------------------------
    // 5. 安全抽象 (Safe Abstraction)
    // ---------------------------------------------------------
    // 关键思想：在 unsafe 代码外面包一层安全的 API
    // 标准库中有很多这样的例子，如 Vec、String 等
    println!("\n--- 5. 安全抽象 (Safe Abstraction) ---");

    let mut v = vec![1, 2, 3, 4, 5, 6];
    println!("  原始向量: {:?}", v);

    // split_at_mut 内部使用了 unsafe，但对外提供安全的接口
    let (left, right) = v.split_at_mut(3);
    println!("  左半部分: {:?}", left);
    println!("  右半部分: {:?}", right);

    // 我们自己实现一个安全抽象的例子
    let mut data = vec![10, 20, 30, 40, 50];
    let (first_half, second_half) = my_split_at_mut(&mut data, 2);
    println!("  自定义 split - 前半: {:?}", first_half);
    println!("  自定义 split - 后半: {:?}", second_half);

    // ---------------------------------------------------------
    // 6. unsafe 不意味着代码一定有错
    // ---------------------------------------------------------
    println!("\n--- 6. unsafe ≠ 代码有错 ---");
    println!("  unsafe 的含义:");
    println!("  • unsafe 是对编译器说：'相信我，我知道我在做什么'");
    println!("  • unsafe 代码可以是完全正确的，只是编译器无法自动验证");
    println!("  • 标准库中大量使用了 unsafe（如 Vec、HashMap 等）");
    println!("  • 关键是：程序员承担了证明安全性的责任");
    println!();

    // 演示一个虽然使用 unsafe 但完全正确的例子
    let numbers = [1, 2, 3, 4, 5];
    let sum = safe_sum_via_pointer(&numbers);
    println!("  通过裸指针安全地计算数组和: {}", sum);

    println!("\n🎉 恭喜！你已经掌握了 unsafe 的基础知识！");
}

// ---------------------------------------------------------
// 辅助定义
// ---------------------------------------------------------

/// 不可变静态变量（访问是安全的）
static MAX_POINTS: u32 = 100_000;

/// 可变静态变量（访问需要 unsafe，因为多线程下可能引发数据竞争）
static mut COUNTER: u32 = 0;

/// 一个 unsafe 函数 —— 调用者必须确保调用条件满足
///
/// # Safety
/// 此函数仅用于演示，无特殊安全前提条件。
unsafe fn dangerous_function() {
    println!("  这是一个 unsafe 函数，已被成功调用！");
}

/// 从裸指针读取值
///
/// # Safety
/// 调用者必须确保 `ptr` 是有效的、已对齐的，并且指向已初始化的 i32 数据。
unsafe fn read_pointer_value(ptr: *const i32) -> i32 {
    *ptr
}

/// 一个不检查溢出的加法（仅做演示）
///
/// # Safety
/// 调用者需确保 a + b 不会溢出 i32。
unsafe fn add_unchecked(a: i32, b: i32) -> i32 {
    a + b
}

// --- unsafe trait ---

/// unsafe trait 表示实现者必须保证某些不变量
/// 例如标准库的 Send 和 Sync 就是 unsafe trait
unsafe trait SafeMarker {
    fn do_something(&self);
}

struct MyType;

/// 实现 unsafe trait 需要使用 `unsafe impl`
/// 实现者承诺满足 trait 要求的安全不变量
unsafe impl SafeMarker for MyType {
    fn do_something(&self) {
        println!("  MyType 实现了 unsafe trait SafeMarker");
    }
}

// --- union ---

/// union 类似 C 语言的联合体，所有字段共享同一块内存
/// 访问 union 字段是 unsafe 的，因为 Rust 不知道当前存的是什么类型
#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

// --- 安全抽象示例 ---

/// 自定义的 split_at_mut：内部使用 unsafe，对外提供安全接口
///
/// 这是安全抽象的典型例子：
/// - 函数签名是安全的（不需要 unsafe 调用）
/// - 内部使用 unsafe 实现，但我们能保证安全性
/// - 外部用户无需关心内部实现
fn my_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    assert!(mid <= len, "mid 不能超过 slice 长度");

    let ptr = slice.as_mut_ptr();

    // 这里的 unsafe 是安全的，因为：
    // 1. ptr 来自有效的 slice，一定是有效指针
    // 2. mid <= len，所以两个子切片不会越界
    // 3. 两个子切片不重叠，满足可变引用的唯一性要求
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

/// 通过裸指针遍历数组求和 —— unsafe 但逻辑安全
fn safe_sum_via_pointer(data: &[i32]) -> i32 {
    let mut sum = 0;
    let ptr = data.as_ptr();
    let len = data.len();

    for i in 0..len {
        // 安全的，因为 i < len，不会越界
        unsafe {
            sum += *ptr.add(i);
        }
    }
    sum
}
