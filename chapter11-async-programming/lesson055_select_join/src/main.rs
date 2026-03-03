// ============================================================
// Lesson 055: select! 与 join!
// ============================================================
//
// 本课学习 Tokio 中的两个核心并发控制宏：
// - tokio::select! —— 同时等待多个 Future，第一个完成的"赢"
// - tokio::join!   —— 同时等待所有 Future 全部完成
//
// 知识点：
// 1. tokio::select! 多分支等待
// 2. tokio::join! 并发等待
// 3. tokio::time::timeout 超时控制
// 4. 竞争与取消语义
//
// ============================================================
// select! vs join! 图解
// ============================================================
//
//   select! （竞争模式 —— 取第一个完成的）
//   ┌─────────┐
//   │ Future A │───→ 完成 ✅  ←── select! 返回这个结果
//   │ Future B │───→ ...     ←── 被取消 ❌
//   │ Future C │───→ ...     ←── 被取消 ❌
//   └─────────┘
//
//   join! （并发模式 —— 等所有都完成）
//   ┌─────────┐
//   │ Future A │───→ 完成 ✅
//   │ Future B │───→ 完成 ✅  ←── join! 等待所有结果
//   │ Future C │───→ 完成 ✅
//   └─────────┘
//
// ============================================================

use tokio::time::{sleep, timeout, Duration, Instant};

#[tokio::main]
async fn main() {
    println!("=== Lesson 055: select! 与 join! ===\n");

    // 1. select! 基础
    demo_select_basic().await;

    // 2. select! 模式匹配
    demo_select_patterns().await;

    // 3. select! 循环
    demo_select_loop().await;

    // 4. join! 基础
    demo_join_basic().await;

    // 5. try_join!
    demo_try_join().await;

    // 6. timeout
    demo_timeout().await;

    // 7. 竞争取消
    demo_race_cancellation().await;

    // ============================================================
    // 总结
    // ============================================================
    println!("=== 总结 ===");
    println!("  1. select! 等待多个 Future，返回第一个完成的结果");
    println!("  2. select! 中未完成的分支会被取消（drop）");
    println!("  3. join! 并发执行所有 Future，等待全部完成");
    println!("  4. try_join! 类似 join!，但遇到 Err 立即返回");
    println!("  5. timeout 为 Future 设置超时时间");
    println!("  6. select! + loop 是实现事件循环的常用模式");
}

// ============================================================
// 1. select! 基础
// ============================================================

async fn demo_select_basic() {
    println!("--- 1. select! 基础 ---");

    let start = Instant::now();

    // select! 等待多个分支，第一个完成的"赢"
    tokio::select! {
        // 分支 1：100ms 后完成
        val = async {
            sleep(Duration::from_millis(100)).await;
            "快速任务"
        } => {
            println!("  第一个完成: {} ({:?})", val, start.elapsed());
        }
        // 分支 2：500ms 后完成
        val = async {
            sleep(Duration::from_millis(500)).await;
            "慢速任务"
        } => {
            println!("  第一个完成: {} ({:?})", val, start.elapsed());
        }
    }
    // 输出：快速任务先完成，慢速任务被取消

    println!();
}

// ============================================================
// 2. select! 模式匹配和条件分支
// ============================================================

async fn demo_select_patterns() {
    println!("--- 2. select! 模式匹配 ---");

    // 使用通道演示模式匹配
    let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(1);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel::<String>(1);

    // 发送数据
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        tx1.send(42).await.unwrap();
    });

    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        tx2.send("hello".to_string()).await.unwrap();
    });

    // select! 支持模式匹配
    tokio::select! {
        // Some(val) 模式匹配 —— 只匹配有值的情况
        Some(val) = rx1.recv() => {
            println!("  从通道1收到整数: {}", val);
        }
        Some(val) = rx2.recv() => {
            println!("  从通道2收到字符串: {}", val);
        }
    }

    // select! 带 else 分支
    println!("\n  select! 带 else 分支：");
    let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(1);
    drop(tx); // 立即关闭通道

    tokio::select! {
        Some(val) = rx.recv() => {
            println!("  收到: {}", val);
        }
        // 当所有分支的模式都不匹配时，执行 else
        else => {
            println!("  所有通道已关闭（else 分支执行）");
        }
    }

    // select! 带条件守卫 (if guard)
    println!("\n  select! 带条件守卫：");
    let flag = true;
    tokio::select! {
        _ = sleep(Duration::from_millis(50)), if flag => {
            println!("  分支1执行（flag = true，条件满足）");
        }
        _ = sleep(Duration::from_millis(50)), if !flag => {
            println!("  分支2执行（flag = false，条件满足）");
        }
    }
    println!();
}

// ============================================================
// 3. select! 在循环中使用（事件循环模式）
// ============================================================

async fn demo_select_loop() {
    println!("--- 3. select! 循环（事件循环模式）---");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

    // 模拟消息发送者
    tokio::spawn(async move {
        let messages = ["你好", "世界", "完成"];
        for (i, msg) in messages.iter().enumerate() {
            sleep(Duration::from_millis(80 * (i as u64 + 1))).await;
            tx.send(msg.to_string()).await.unwrap();
        }
        // tx 被 drop，通道关闭
    });

    // 超时计时器
    let deadline = sleep(Duration::from_millis(500));
    // pin! 宏将 Future 固定在栈上，以便在循环中重用
    tokio::pin!(deadline);

    let mut message_count = 0;

    loop {
        tokio::select! {
            // 分支 1：收到消息
            msg = rx.recv() => {
                match msg {
                    Some(m) => {
                        message_count += 1;
                        println!("  收到消息[{}]: {}", message_count, m);
                    }
                    None => {
                        println!("  通道已关闭，退出循环");
                        break;
                    }
                }
            }
            // 分支 2：超时（注意用 &mut 引用，以便在循环中重用）
            _ = &mut deadline => {
                println!("  超时！强制退出");
                break;
            }
        }
    }
    println!("  共处理 {} 条消息\n", message_count);
}

// ============================================================
// 4. join! 基础
// ============================================================

async fn demo_join_basic() {
    println!("--- 4. join! 并发等待所有 Future ---");

    let start = Instant::now();

    // join! 并发执行所有 Future，等待全部完成
    let (a, b, c) = tokio::join!(
        async {
            sleep(Duration::from_millis(100)).await;
            println!("  任务A 完成 ({:?})", start.elapsed());
            1
        },
        async {
            sleep(Duration::from_millis(200)).await;
            println!("  任务B 完成 ({:?})", start.elapsed());
            2
        },
        async {
            sleep(Duration::from_millis(150)).await;
            println!("  任务C 完成 ({:?})", start.elapsed());
            3
        },
    );

    println!("  所有任务完成: a={}, b={}, c={}", a, b, c);
    println!("  总耗时: {:?}（约 200ms，取决于最慢的任务）", start.elapsed());
    println!();

    // join! 用于并发请求
    println!("  模拟并发网络请求：");
    let start = Instant::now();

    async fn fetch_user() -> String {
        sleep(Duration::from_millis(100)).await;
        "用户: Alice".to_string()
    }
    async fn fetch_orders() -> Vec<String> {
        sleep(Duration::from_millis(150)).await;
        vec!["订单1".to_string(), "订单2".to_string()]
    }
    async fn fetch_settings() -> String {
        sleep(Duration::from_millis(80)).await;
        "设置: 深色模式".to_string()
    }

    let (user, orders, settings) = tokio::join!(
        fetch_user(),
        fetch_orders(),
        fetch_settings(),
    );

    println!("  {}", user);
    println!("  订单: {:?}", orders);
    println!("  {}", settings);
    println!("  并发获取耗时: {:?}\n", start.elapsed());
}

// ============================================================
// 5. try_join! —— 遇到错误提前返回
// ============================================================

async fn demo_try_join() {
    println!("--- 5. try_join! ---");

    // 定义可能失败的异步函数
    async fn might_fail(id: u32, should_fail: bool) -> Result<String, String> {
        sleep(Duration::from_millis(50 * id as u64)).await;
        if should_fail {
            Err(format!("任务 {} 失败了！", id))
        } else {
            Ok(format!("任务 {} 成功", id))
        }
    }

    // 全部成功的情况
    let result = tokio::try_join!(
        might_fail(1, false),
        might_fail(2, false),
        might_fail(3, false),
    );
    match result {
        Ok((a, b, c)) => println!("  全部成功: {}, {}, {}", a, b, c),
        Err(e) => println!("  出错: {}", e),
    }

    // 有失败的情况：try_join! 遇到第一个 Err 就返回
    let result = tokio::try_join!(
        might_fail(1, false),
        might_fail(2, true),  // 这个会失败
        might_fail(3, false),
    );
    match result {
        Ok((a, b, c)) => println!("  全部成功: {}, {}, {}", a, b, c),
        Err(e) => println!("  出错: {}（其他任务被取消）", e),
    }
    println!();
}

// ============================================================
// 6. timeout —— 超时控制
// ============================================================

async fn demo_timeout() {
    println!("--- 6. tokio::time::timeout ---");

    // timeout 为 Future 设置最大等待时间
    // 超时返回 Err(Elapsed)，正常完成返回 Ok(value)

    // 情况 1：任务在超时前完成
    let result = timeout(
        Duration::from_millis(200),
        async {
            sleep(Duration::from_millis(50)).await;
            "及时完成"
        },
    )
    .await;
    match result {
        Ok(val) => println!("  及时完成: {}", val),
        Err(_) => println!("  超时了！"),
    }

    // 情况 2：任务超时
    let result = timeout(
        Duration::from_millis(50),
        async {
            sleep(Duration::from_millis(200)).await;
            "太慢了"
        },
    )
    .await;
    match result {
        Ok(val) => println!("  完成: {}", val),
        Err(e) => println!("  超时: {}（任务被取消）", e),
    }

    // 使用 timeout 包装实际操作
    println!("\n  实际应用：带超时的网络请求模拟");
    async fn mock_request(delay_ms: u64) -> String {
        sleep(Duration::from_millis(delay_ms)).await;
        "响应数据".to_string()
    }

    for delay in [50, 150, 250] {
        let result = timeout(Duration::from_millis(200), mock_request(delay)).await;
        match result {
            Ok(data) => println!("  请求({}ms) -> 成功: {}", delay, data),
            Err(_) => println!("  请求({}ms) -> 超时！", delay),
        }
    }
    println!();
}

// ============================================================
// 7. 竞争与取消
// ============================================================

async fn demo_race_cancellation() {
    println!("--- 7. 竞争取消（Cancellation Safety）---");

    // 重要概念：select! 中未被选中的分支会被 drop（取消）
    // 这可能导致一些数据丢失的风险

    println!("  示例：安全的取消");

    // 创建一个可以被安全取消的任务
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

    // 发送一些消息
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for i in 1..=5 {
            sleep(Duration::from_millis(30)).await;
            let _ = tx_clone.send(format!("消息{}", i)).await;
        }
    });

    // 使用 select! 在超时后停止接收
    let deadline = sleep(Duration::from_millis(100));
    tokio::pin!(deadline);

    let mut received = Vec::new();
    loop {
        tokio::select! {
            // recv() 是取消安全的（cancel-safe）
            // 即使被取消，也不会丢失消息
            Some(msg) = rx.recv() => {
                received.push(msg);
            }
            _ = &mut deadline => {
                println!("  超时，停止接收");
                break;
            }
        }
    }
    println!("  已收到的消息: {:?}", received);

    drop(tx);

    // ---- 取消安全性说明 ----
    println!("\n  取消安全性（Cancel Safety）说明：");
    println!("    ✅ 取消安全的操作：");
    println!("       - tokio::sync::mpsc::Receiver::recv()");
    println!("       - tokio::sync::oneshot::Receiver (在 select! 中使用 &mut)");
    println!("       - tokio::time::sleep()");
    println!("       - tokio::net::TcpListener::accept()");
    println!("    ⚠️  不安全的操作（可能丢失数据）：");
    println!("       - tokio::io::AsyncReadExt::read()（可能读了但没处理）");
    println!("       - 需要特别小心自定义的异步操作");

    // ---- 使用 biased 控制优先级 ----
    println!("\n  select! 带 biased：");
    println!("    默认情况下 select! 随机选择就绪的分支");
    println!("    使用 biased 关键字可以按声明顺序优先");

    let (tx, mut rx) = tokio::sync::mpsc::channel::<&str>(10);
    tx.send("数据").await.unwrap();

    tokio::select! {
        biased; // 按声明顺序优先选择

        val = rx.recv() => {
            println!("    biased select! 优先选择第一个就绪分支: {:?}", val);
        }
        _ = sleep(Duration::from_millis(0)) => {
            println!("    选择了 sleep 分支");
        }
    }
    println!();
}
