/// # Lesson 020 - panic! 与不可恢复错误
///
/// 本课介绍 Rust 中的 panic 机制，用于处理不可恢复的错误。
///
/// ## 学习目标
/// - 理解 `panic!` 宏的作用与触发场景
/// - 了解数组越界导致的 panic
/// - 掌握 `unwrap` 导致 panic 的情况
/// - 了解 `RUST_BACKTRACE` 环境变量的作用
/// - 学习 `panic::catch_unwind` 捕获 panic
/// - 理解 panic 策略：展开（unwind）vs 中止（abort）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson020_panic
/// ```

// =============================================================
// Lesson 020: panic! 与不可恢复错误
// =============================================================

use std::panic;

fn main() {
    println!("===== Lesson 020: panic! 与不可恢复错误 =====\n");

    // ---------------------------------------------------------
    // 1. panic! 宏基础
    // ---------------------------------------------------------
    // panic! 宏用于产生一个不可恢复的错误，程序会打印错误信息，
    // 展开（unwind）并清理栈数据，然后退出。
    //
    // 直接调用 panic! 会中断程序，所以我们用 catch_unwind 来演示：
    println!("--- 1. panic! 宏基础 ---");

    // 使用 catch_unwind 捕获 panic，这样程序不会终止
    let result = panic::catch_unwind(|| {
        panic!("这是一个手动触发的 panic！");
    });

    match result {
        Ok(_) => println!("没有发生 panic"),
        Err(e) => {
            // 尝试将 panic 信息转为字符串
            if let Some(msg) = e.downcast_ref::<&str>() {
                println!("捕获到 panic: {}", msg);
            } else if let Some(msg) = e.downcast_ref::<String>() {
                println!("捕获到 panic: {}", msg);
            } else {
                println!("捕获到未知类型的 panic");
            }
        }
    }

    // panic! 也支持格式化字符串
    let result = panic::catch_unwind(|| {
        let error_code = 42;
        panic!("出现严重错误，错误码: {}", error_code);
    });

    if let Err(e) = result {
        if let Some(msg) = e.downcast_ref::<String>() {
            println!("捕获到格式化 panic: {}", msg);
        }
    }

    println!();

    // ---------------------------------------------------------
    // 2. 数组越界导致的 panic
    // ---------------------------------------------------------
    // Rust 在运行时会检查数组访问是否越界，越界时会 panic。
    // 这是 Rust 安全性的重要保障——不会像 C/C++ 那样允许非法内存访问。
    println!("--- 2. 数组越界 panic ---");

    let numbers = vec![1, 2, 3, 4, 5];
    println!("数组内容: {:?}", numbers);

    // 安全访问：使用 get() 方法返回 Option
    match numbers.get(10) {
        Some(val) => println!("索引 10 的值: {}", val),
        None => println!("索引 10 越界了！get() 返回 None，不会 panic"),
    }

    // 越界访问会 panic，我们用 catch_unwind 捕获
    let result = panic::catch_unwind(|| {
        let v = vec![1, 2, 3];
        let _value = v[99]; // 越界！
    });

    if result.is_err() {
        println!("捕获到数组越界 panic！直接索引访问越界会导致 panic");
    }

    println!();

    // ---------------------------------------------------------
    // 3. unwrap 导致的 panic
    // ---------------------------------------------------------
    // unwrap() 方法在值为 None 或 Err 时会 panic。
    // 在生产代码中应尽量避免使用 unwrap()，改用更安全的处理方式。
    println!("--- 3. unwrap 导致的 panic ---");

    // Option 的 unwrap
    let some_value: Option<i32> = Some(42);
    println!("Some(42).unwrap() = {}", some_value.unwrap()); // 安全，因为是 Some

    // None.unwrap() 会 panic
    let result = panic::catch_unwind(|| {
        let none_value: Option<i32> = None;
        none_value.unwrap(); // panic: called `Option::unwrap()` on a `None` value
    });
    if result.is_err() {
        println!("None.unwrap() 触发了 panic！");
    }

    // Result 的 unwrap
    let ok_value: Result<i32, &str> = Ok(100);
    println!("Ok(100).unwrap() = {}", ok_value.unwrap()); // 安全

    let result = panic::catch_unwind(|| {
        let err_value: Result<i32, &str> = Err("出错了");
        err_value.unwrap(); // panic!
    });
    if result.is_err() {
        println!("Err.unwrap() 触发了 panic！");
    }

    // expect 与 unwrap 类似，但可以自定义 panic 信息
    let result = panic::catch_unwind(|| {
        let none_value: Option<i32> = None;
        none_value.expect("期望得到一个值，但得到了 None");
    });
    if result.is_err() {
        println!("None.expect() 触发了带自定义消息的 panic！");
    }

    // 更好的做法：使用 unwrap_or、unwrap_or_else、unwrap_or_default
    let value = None::<i32>.unwrap_or(0);
    println!("None.unwrap_or(0) = {}（安全，不会 panic）", value);

    let value = None::<i32>.unwrap_or_else(|| {
        println!("  计算默认值...");
        42
    });
    println!("None.unwrap_or_else(|| 42) = {}（安全，不会 panic）", value);

    let value = None::<i32>.unwrap_or_default();
    println!(
        "None::<i32>.unwrap_or_default() = {}（使用类型默认值）",
        value
    );

    println!();

    // ---------------------------------------------------------
    // 4. RUST_BACKTRACE 环境变量
    // ---------------------------------------------------------
    // 当 panic 发生时，可以通过设置 RUST_BACKTRACE 环境变量来查看完整的调用栈信息。
    //
    // 用法：
    //   RUST_BACKTRACE=1 cargo run    # 显示简要回溯
    //   RUST_BACKTRACE=full cargo run # 显示完整回溯
    //
    // 回溯信息对于调试非常有用，它会显示从 panic 发生点到 main 函数的完整调用链。
    println!("--- 4. RUST_BACKTRACE 环境变量 ---");
    println!("设置方式：");
    println!("  RUST_BACKTRACE=1 cargo run -p lesson020_panic     # 简要回溯");
    println!("  RUST_BACKTRACE=full cargo run -p lesson020_panic   # 完整回溯");
    println!();

    // 演示：获取当前 RUST_BACKTRACE 的值
    match std::env::var("RUST_BACKTRACE") {
        Ok(val) => println!("当前 RUST_BACKTRACE = \"{}\"", val),
        Err(_) => println!("当前 RUST_BACKTRACE 未设置（默认不显示回溯）"),
    }

    println!();

    // ---------------------------------------------------------
    // 5. panic::catch_unwind 捕获 panic
    // ---------------------------------------------------------
    // catch_unwind 可以捕获 panic，防止程序终止。
    // 注意：这不应该用于常规错误处理！它主要用于：
    //   - FFI 边界（防止 panic 跨越 C 代码）
    //   - 线程池（防止一个任务的 panic 影响整个池）
    //   - 测试框架
    println!("--- 5. panic::catch_unwind ---");

    // 基本用法
    let result = panic::catch_unwind(|| {
        println!("  这段代码正常执行");
        42 // 返回值
    });
    println!("正常执行的结果: {:?}", result); // Ok(42)

    let result = panic::catch_unwind(|| -> i32 {
        panic!("糟糕！");
    });
    println!("panic 的结果: {:?}", result.is_err()); // true

    // 实际应用：在任务执行中防止 panic 导致整个程序崩溃
    let tasks: Vec<Box<dyn Fn() -> i32>> = vec![
        Box::new(|| 1 + 1),
        Box::new(|| panic!("任务2失败了")),
        Box::new(|| 3 + 3),
    ];

    println!("\n  执行多个任务（其中一个会 panic）：");
    for (i, task) in tasks.iter().enumerate() {
        // 使用 AssertUnwindSafe 包裹闭包引用，告诉编译器这里是安全的
        match panic::catch_unwind(panic::AssertUnwindSafe(|| task())) {
            Ok(result) => println!("  任务 {} 成功，结果: {}", i, result),
            Err(_) => println!("  任务 {} 发生了 panic，已被捕获", i),
        }
    }

    println!();

    // ---------------------------------------------------------
    // 6. panic 策略：展开（unwind）vs 中止（abort）
    // ---------------------------------------------------------
    // Rust 提供两种 panic 策略：
    //
    // 【展开 (unwind)】 —— 默认策略
    //   - panic 发生时，Rust 会沿着调用栈逐层回退
    //   - 每一层的局部变量都会被正确析构（调用 Drop trait）
    //   - 资源（文件句柄、内存等）会被正确释放
    //   - 可以被 catch_unwind 捕获
    //   - 缺点：生成的二进制文件较大（包含展开信息）
    //
    // 【中止 (abort)】
    //   - panic 发生时，程序立即终止，不做任何清理
    //   - 无法被 catch_unwind 捕获
    //   - 优点：生成的二进制文件更小
    //   - 在 Cargo.toml 中配置：
    //     [profile.release]
    //     panic = "abort"
    //
    println!("--- 6. panic 策略：展开 vs 中止 ---");
    println!("展开 (unwind) - 默认策略：");
    println!("  - panic 时逐层回退调用栈，正确析构局部变量");
    println!("  - 可以被 catch_unwind 捕获");
    println!("  - 二进制文件较大\n");
    println!("中止 (abort)：");
    println!("  - panic 时立即终止程序，不做清理");
    println!("  - 无法被 catch_unwind 捕获");
    println!("  - 二进制文件更小");
    println!("  - 在 Cargo.toml 中配置：");
    println!("    [profile.release]");
    println!("    panic = \"abort\"");

    // 演示展开策略下 Drop 的执行
    println!("\n  演示展开时 Drop 的执行：");

    struct DropDemo {
        name: String,
    }

    impl Drop for DropDemo {
        fn drop(&mut self) {
            println!("    [Drop] {} 被析构了", self.name);
        }
    }

    let result = panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _a = DropDemo {
            name: "变量A".to_string(),
        };
        let _b = DropDemo {
            name: "变量B".to_string(),
        };
        panic!("在创建两个变量后 panic");
        // 注意：即使发生了 panic，_a 和 _b 的 Drop 仍然会被调用（展开策略下）
    }));

    if result.is_err() {
        println!("  panic 已被捕获，上面的 Drop 信息证明了展开策略的清理行为");
    }

    println!();

    // ---------------------------------------------------------
    // 7. 何时使用 panic
    // ---------------------------------------------------------
    println!("--- 7. 何时使用 panic（最佳实践） ---");
    println!("适合使用 panic 的场景：");
    println!("  1. 原型代码和示例 —— 快速实验时用 unwrap()");
    println!("  2. 测试代码 —— 断言失败时自动 panic");
    println!("  3. 违反程序不变量 —— 逻辑上不可能的情况");
    println!("  4. 无法恢复的错误 —— 如配置文件格式严重错误");
    println!();
    println!("不适合 panic 的场景：");
    println!("  1. 可预期的错误 —— 文件不存在、网络超时（用 Result）");
    println!("  2. 用户输入错误 —— 格式不对（用 Result 或 Option）");
    println!("  3. 库代码 —— 应该让调用者决定如何处理错误");

    println!("\n🎉 恭喜！你已经完成了 Lesson 020 —— panic! 与不可恢复错误！");
}
