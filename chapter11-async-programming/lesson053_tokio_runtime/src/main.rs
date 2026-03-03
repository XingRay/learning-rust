// ============================================================
// Lesson 053: Tokio 运行时
// ============================================================
//
// 本课学习 Tokio —— Rust 最流行的异步运行时。
//
// 知识点：
// 1. #[tokio::main] 属性宏
// 2. tokio::spawn 创建异步任务
// 3. JoinHandle 获取任务结果
// 4. tokio::time::sleep 异步休眠
// 5. tokio::task::spawn_blocking 处理阻塞操作
//
// ============================================================
// Tokio 运行时架构
// ============================================================
//
//   ┌────────────────────────────────────┐
//   │          Tokio Runtime             │
//   │                                    │
//   │  ┌──────────┐  ┌──────────┐       │
//   │  │ Worker 1 │  │ Worker 2 │ ...   │  ← 工作线程（默认 = CPU 核数）
//   │  └──────────┘  └──────────┘       │
//   │       │              │             │
//   │  ┌────┴────┐   ┌────┴────┐        │
//   │  │ Task A  │   │ Task C  │        │
//   │  │ Task B  │   │ Task D  │        │  ← 异步任务分配到工作线程
//   │  └─────────┘   └─────────┘        │
//   │                                    │
//   │  ┌─────────────────────────┐      │
//   │  │    Blocking Pool        │      │  ← 专门处理阻塞操作的线程池
//   │  │  (spawn_blocking 用)     │      │
//   │  └─────────────────────────┘      │
//   │                                    │
//   │  ┌─────────────────────────┐      │
//   │  │    I/O Driver (epoll)   │      │  ← 系统级 I/O 事件驱动
//   │  └─────────────────────────┘      │
//   │                                    │
//   │  ┌─────────────────────────┐      │
//   │  │    Timer (时间轮)        │      │  ← sleep/timeout 等计时
//   │  └─────────────────────────┘      │
//   └────────────────────────────────────┘
//
// ============================================================

use std::time::Instant;
use tokio::time::{sleep, Duration};

// ============================================================
// 1. #[tokio::main] —— 启动 Tokio 运行时
// ============================================================
//
// #[tokio::main] 是一个过程宏，它会：
// - 创建多线程运行时（默认）
// - 将 async fn main() 的函数体放入 block_on()
//
// 等价展开：
//   fn main() {
//       tokio::runtime::Builder::new_multi_thread()
//           .enable_all()
//           .build()
//           .unwrap()
//           .block_on(async { ... })
//   }
//
// 也可以使用单线程运行时：
//   #[tokio::main(flavor = "current_thread")]
//   async fn main() { ... }
//

#[tokio::main]
async fn main() {
    println!("=== Lesson 053: Tokio 运行时 ===\n");

    // ----- 演示 1: tokio::spawn 基础 -----
    demo_spawn_basics().await;

    // ----- 演示 2: JoinHandle 获取返回值 -----
    demo_join_handle().await;

    // ----- 演示 3: 多任务并发 -----
    demo_concurrent_tasks().await;

    // ----- 演示 4: tokio::time::sleep -----
    demo_sleep().await;

    // ----- 演示 5: spawn_blocking -----
    demo_spawn_blocking().await;

    // ----- 演示 6: 手动构建运行时 -----
    demo_manual_runtime();

    // ============================================================
    // 总结
    // ============================================================
    println!("=== 总结 ===");
    println!("  1. #[tokio::main] 创建运行时并执行 async main");
    println!("  2. tokio::spawn() 创建独立的异步任务，返回 JoinHandle");
    println!("  3. JoinHandle.await 获取任务结果（Result 类型，可能 panic）");
    println!("  4. tokio::time::sleep() 异步休眠，不阻塞线程");
    println!("  5. spawn_blocking() 将阻塞操作放到专用线程池");
    println!("  6. 可以用 Builder 手动配置运行时参数");
}

// ============================================================
// 2. tokio::spawn —— 创建异步任务
// ============================================================

async fn demo_spawn_basics() {
    println!("--- 1. tokio::spawn 基础 ---");

    // tokio::spawn 创建一个新的异步任务
    // 任务会被调度到运行时的某个工作线程上执行
    // 注意：spawn 的闭包/Future 必须是 'static + Send
    let handle = tokio::spawn(async {
        println!("  [任务] 我在一个 spawn 的任务中运行！");
        println!("  [任务] 当前线程: {:?}", std::thread::current().id());
        42
    });

    println!("  [主线程] spawn 之后，主任务继续执行");
    println!("  [主线程] 当前线程: {:?}", std::thread::current().id());

    // 等待任务完成
    let result = handle.await.unwrap();
    println!("  [主线程] 任务返回: {}", result);
    println!();
}

// ============================================================
// 3. JoinHandle —— 获取任务结果
// ============================================================

async fn demo_join_handle() {
    println!("--- 2. JoinHandle 详解 ---");

    // JoinHandle.await 返回 Result<T, JoinError>
    // JoinError 发生在任务 panic 时

    // 正常完成的任务
    let handle = tokio::spawn(async {
        "任务成功完成".to_string()
    });
    match handle.await {
        Ok(result) => println!("  正常结果: {}", result),
        Err(e) => println!("  任务出错: {}", e),
    }

    // panic 的任务
    let handle = tokio::spawn(async {
        if true {
            panic!("故意 panic！");
        }
        "永远到不了这里".to_string()
    });
    match handle.await {
        Ok(result) => println!("  结果: {}", result),
        Err(e) => println!("  任务 panic 了: {} (这是预期的)", e),
    }

    // 取消任务：drop JoinHandle 不会取消任务！
    // 需要使用 abort() 来取消
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(10)).await;
        println!("  这行不会被打印（任务已被取消）");
    });

    // 取消任务
    handle.abort();
    match handle.await {
        Ok(_) => println!("  任务完成了（不应该到这里）"),
        Err(e) if e.is_cancelled() => println!("  任务已被取消 (abort): {}", e),
        Err(e) => println!("  其他错误: {}", e),
    }
    println!();
}

// ============================================================
// 4. 多任务并发执行
// ============================================================

async fn demo_concurrent_tasks() {
    println!("--- 3. 多任务并发执行 ---");

    let start = Instant::now();

    // 启动多个并发任务
    let mut handles = Vec::new();

    for i in 1..=5 {
        let handle = tokio::spawn(async move {
            // 每个任务模拟不同时长的操作
            let duration = Duration::from_millis(100 * i);
            sleep(duration).await;
            println!(
                "  任务 {} 完成（耗时 {:?}，运行在线程 {:?}）",
                i,
                duration,
                std::thread::current().id()
            );
            i * 10 // 返回结果
        });
        handles.push(handle);
    }

    // 收集所有结果
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap();
        results.push(result);
    }

    println!("  所有任务完成！结果: {:?}", results);
    println!("  总耗时: {:?}（并发执行，约 500ms 而非 1500ms）", start.elapsed());
    println!();
}

// ============================================================
// 5. tokio::time::sleep —— 异步休眠
// ============================================================

async fn demo_sleep() {
    println!("--- 4. tokio::time::sleep ---");

    // tokio::time::sleep 是异步的，不会阻塞线程
    println!("  开始异步休眠...");
    let start = Instant::now();

    // 同时休眠两个不同时长
    let ((), ()) = tokio::join!(
        async {
            sleep(Duration::from_millis(100)).await;
            println!("  短休眠完成（{:?}）", start.elapsed());
        },
        async {
            sleep(Duration::from_millis(200)).await;
            println!("  长休眠完成（{:?}）", start.elapsed());
        },
    );
    println!("  总耗时: {:?}（约 200ms，因为是并发的）", start.elapsed());

    // 对比：std::thread::sleep 会阻塞线程！
    // 在异步代码中永远不要使用 std::thread::sleep
    // 如果需要在异步上下文中执行阻塞操作，使用 spawn_blocking
    println!("  ⚠️ 注意：在 async 中不要使用 std::thread::sleep！");
    println!();
}

// ============================================================
// 6. tokio::task::spawn_blocking —— 处理阻塞操作
// ============================================================

async fn demo_spawn_blocking() {
    println!("--- 5. tokio::task::spawn_blocking ---");

    // 某些操作天生就是阻塞的（如 CPU 密集计算、同步文件 I/O）
    // 这些操作不应该在异步任务中直接执行，否则会阻塞整个工作线程

    let start = Instant::now();

    // 将阻塞操作放入专用的阻塞线程池
    let blocking_handle = tokio::task::spawn_blocking(move || {
        println!("  [阻塞任务] 开始 CPU 密集计算...");
        println!(
            "  [阻塞任务] 运行在线程: {:?}",
            std::thread::current().id()
        );

        // 模拟 CPU 密集操作
        let mut sum: u64 = 0;
        for i in 0..1_000_000 {
            sum += i;
        }

        println!("  [阻塞任务] 计算完成");
        sum
    });

    // 同时可以执行其他异步任务
    let async_handle = tokio::spawn(async {
        sleep(Duration::from_millis(10)).await;
        println!("  [异步任务] 在阻塞任务执行期间，异步任务也在运行！");
    });

    // 等待两个任务完成
    let (blocking_result, _) = tokio::join!(blocking_handle, async_handle);
    let sum = blocking_result.unwrap();
    println!("  阻塞任务结果: sum = {}", sum);
    println!("  总耗时: {:?}", start.elapsed());
    println!();

    // spawn_blocking 的使用场景：
    println!("  spawn_blocking 适用场景：");
    println!("    - CPU 密集型计算");
    println!("    - 同步文件 I/O 操作");
    println!("    - 调用不支持异步的第三方库");
    println!("    - 数据库同步驱动");
    println!();
}

// ============================================================
// 7. 手动构建运行时
// ============================================================

fn demo_manual_runtime() {
    println!("--- 6. 手动构建运行时 ---");

    // 方式一：多线程运行时
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2) // 设置工作线程数
        .thread_name("my-worker") // 线程名前缀
        .enable_all() // 启用所有功能（IO、Timer 等）
        .build()
        .unwrap();

    rt.block_on(async {
        println!("  [多线程运行时] 运行在手动创建的运行时中");
        println!(
            "  [多线程运行时] 线程: {:?}",
            std::thread::current().name()
        );

        let handle = tokio::spawn(async {
            format!(
                "子任务运行在: {:?}",
                std::thread::current().name()
            )
        });
        println!("  [多线程运行时] {}", handle.await.unwrap());
    });

    // 方式二：单线程运行时（current_thread）
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        println!("  [单线程运行时] 所有任务在同一个线程上运行");
        println!(
            "  [单线程运行时] 线程: {:?}",
            std::thread::current().id()
        );

        // 在单线程运行时中，spawn 的任务也在同一个线程上执行
        let handle = tokio::spawn(async {
            std::thread::current().id()
        });
        let task_thread = handle.await.unwrap();
        println!(
            "  [单线程运行时] 子任务线程: {:?}（与主任务相同）",
            task_thread
        );
    });

    println!();
    println!("  运行时选择指南：");
    println!("    - 多线程运行时：适合服务器、高并发场景（默认）");
    println!("    - 单线程运行时：适合简单脚本、嵌入式场景");
    println!();
}
