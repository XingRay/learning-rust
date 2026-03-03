// ============================================================
// Lesson 051: async/await 基础
// ============================================================
//
// 本课介绍 Rust 异步编程的核心语法：async/await。
//
// 知识点：
// 1. async fn 定义异步函数
// 2. .await 调用异步函数
// 3. 异步 vs 同步对比
// 4. 使用 #[tokio::main] 运行 Future
// 5. async 块（async block）
//
// ============================================================
// 什么是异步编程？
// ============================================================
//
// 同步编程：代码逐行执行，遇到耗时操作（如 I/O）会阻塞当前线程。
// 异步编程：遇到耗时操作时，当前任务可以"让出"控制权，
//           让运行时去执行其他任务，从而提高整体吞吐量。
//
// 在 Rust 中，async/await 是"零成本抽象"——
// 编译器会将 async 函数转换为状态机，没有运行时开销。
//
// ============================================================
// 核心概念图解
// ============================================================
//
//   async fn foo() -> u32 { 42 }
//        ↓ 编译器转换
//   fn foo() -> impl Future<Output = u32> { ... }
//
//   调用 foo() 不会立即执行函数体！
//   它只是返回一个 Future（惰性的）。
//   只有 .await 或交给运行时 poll 时，才会真正执行。
//
// ============================================================

use std::time::Instant;
use tokio::time::{sleep, Duration};

// ============================================================
// 1. async fn —— 定义异步函数
// ============================================================

/// 一个简单的异步函数，模拟网络请求
async fn fetch_data(id: u32) -> String {
    // sleep 是异步的，不会阻塞线程
    sleep(Duration::from_millis(100)).await;
    format!("数据-{}", id)
}

/// 带有多个 .await 调用的异步函数
async fn process_data() -> String {
    // 每个 .await 都是一个"挂起点"（suspension point）
    // 在挂起期间，运行时可以执行其他任务
    let data1 = fetch_data(1).await; // 第一个挂起点
    let data2 = fetch_data(2).await; // 第二个挂起点
    format!("处理完成: {} + {}", data1, data2)
}

// ============================================================
// 2. 异步 vs 同步对比
// ============================================================

/// 同步版本：模拟耗时操作
fn sync_task(id: u32) -> String {
    // 注意：这里用 std::thread::sleep，它会阻塞当前线程！
    std::thread::sleep(Duration::from_millis(100));
    format!("同步任务-{}", id)
}

/// 异步版本：模拟耗时操作
async fn async_task(id: u32) -> String {
    // 注意：这里用 tokio::time::sleep，它不会阻塞线程！
    sleep(Duration::from_millis(100)).await;
    format!("异步任务-{}", id)
}

// ============================================================
// 3. async 块
// ============================================================

/// 演示 async 块的使用
async fn demo_async_block() {
    // async 块可以捕获环境变量，就像闭包一样
    let name = String::from("Rust");

    // async 块返回一个 Future
    let greeting_future = async {
        // 可以在 async 块中使用 .await
        sleep(Duration::from_millis(50)).await;
        format!("你好, {}!", name)
    };

    // Future 是惰性的——上面的代码还没有执行
    println!("  async 块已创建，但还未执行...");

    // .await 触发执行
    let greeting = greeting_future.await;
    println!("  async 块执行结果: {}", greeting);

    // async move 块——获取变量所有权
    let data = vec![1, 2, 3];
    let sum_future = async move {
        // data 的所有权被移入 async 块
        data.iter().sum::<i32>()
    };
    // 此时 data 已经被移走，不能再使用
    // println!("{:?}", data); // 编译错误！

    let sum = sum_future.await;
    println!("  async move 块结果: 求和 = {}", sum);
}

// ============================================================
// 4. #[tokio::main] —— 运行 Future
// ============================================================
//
// Rust 的 Future 是惰性的，需要一个"运行时"（runtime）来驱动它。
// #[tokio::main] 宏会：
//   1. 创建一个 Tokio 运行时
//   2. 将 main 函数体包装成一个 Future
//   3. 使用 block_on 运行这个 Future
//
// 等价于：
//   fn main() {
//       tokio::runtime::Runtime::new()
//           .unwrap()
//           .block_on(async { ... })
//   }
// ============================================================

#[tokio::main]
async fn main() {
    println!("=== Lesson 051: async/await 基础 ===\n");

    // ----- 演示 1: 基本的 async fn 和 .await -----
    println!("--- 1. async fn 和 .await ---");

    // 调用异步函数，获得 Future（此时函数体还未执行）
    let future = fetch_data(42);
    println!("  Future 已创建（fetch_data 还未执行）");

    // .await 驱动 Future 执行
    let result = future.await;
    println!("  fetch_data 执行完毕: {}", result);

    // 链式调用
    let result = process_data().await;
    println!("  process_data 结果: {}", result);
    println!();

    // ----- 演示 2: 异步 vs 同步对比 -----
    println!("--- 2. 异步 vs 同步对比 ---");

    // 同步顺序执行：3 个任务各 100ms，总共约 300ms
    let start = Instant::now();
    let r1 = sync_task(1);
    let r2 = sync_task(2);
    let r3 = sync_task(3);
    let sync_elapsed = start.elapsed();
    println!("  同步结果: {}, {}, {}", r1, r2, r3);
    println!("  同步耗时: {:?}", sync_elapsed);

    // 异步并发执行：3 个任务并发运行，总共约 100ms
    let start = Instant::now();
    let (r1, r2, r3) = tokio::join!(
        async_task(1),
        async_task(2),
        async_task(3),
    );
    let async_elapsed = start.elapsed();
    println!("  异步结果: {}, {}, {}", r1, r2, r3);
    println!("  异步耗时: {:?}（并发执行，远快于同步）", async_elapsed);
    println!();

    // ----- 演示 3: async 块 -----
    println!("--- 3. async 块 ---");
    demo_async_block().await;
    println!();

    // ----- 演示 4: async 块作为表达式 -----
    println!("--- 4. async 块作为表达式 ---");

    // async 块可以用在需要 Future 的任何地方
    let values = vec![10, 20, 30];
    let mut results = Vec::new();

    for v in values {
        // 每次循环创建一个 async 块
        let result = async move {
            sleep(Duration::from_millis(10)).await;
            v * 2
        }.await;
        results.push(result);
    }
    println!("  处理结果: {:?}", results);
    println!();

    // ----- 演示 5: 异步函数的返回值 -----
    println!("--- 5. 异步函数中的控制流 ---");

    async fn async_with_condition(flag: bool) -> &'static str {
        if flag {
            sleep(Duration::from_millis(10)).await;
            "条件为真的结果"
        } else {
            // 即使不需要 await，函数仍然是异步的
            "条件为假的结果"
        }
    }

    let r1 = async_with_condition(true).await;
    let r2 = async_with_condition(false).await;
    println!("  true  -> {}", r1);
    println!("  false -> {}", r2);
    println!();

    // ----- 演示 6: 异步函数作为参数 -----
    println!("--- 6. 将 Future 传递给函数 ---");

    // 接受一个 Future 作为参数
    async fn execute_future<F>(future: F) -> String
    where
        F: std::future::Future<Output = String>,
    {
        println!("  准备执行 Future...");
        let result = future.await;
        println!("  Future 执行完毕!");
        result
    }

    let result = execute_future(fetch_data(99)).await;
    println!("  结果: {}", result);
    println!();

    // ============================================================
    // 总结
    // ============================================================
    println!("=== 总结 ===");
    println!("  1. async fn 定义异步函数，返回 impl Future");
    println!("  2. .await 驱动 Future 执行（只能在 async 上下文中使用）");
    println!("  3. Future 是惰性的，不 .await 就不会执行");
    println!("  4. async 块可以创建匿名的 Future");
    println!("  5. async move 块会获取捕获变量的所有权");
    println!("  6. 需要运行时（如 Tokio）来驱动异步代码执行");
}
