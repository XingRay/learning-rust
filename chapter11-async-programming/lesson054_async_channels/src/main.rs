// ============================================================
// Lesson 054: 异步通道
// ============================================================
//
// 本课学习 Tokio 提供的异步通道（Channel），用于任务间通信。
//
// 知识点：
// 1. tokio::sync::mpsc —— 多生产者单消费者通道
// 2. tokio::sync::oneshot —— 一次性通道
// 3. tokio::sync::broadcast —— 广播通道
// 4. tokio::sync::watch —— 观察通道
// 5. 异步消息传递模式
//
// ============================================================
// 通道类型对比
// ============================================================
//
//  ┌──────────┬────────────┬────────────┬──────────────────────┐
//  │ 通道类型  │ 生产者数量  │ 消费者数量  │ 消息数量              │
//  ├──────────┼────────────┼────────────┼──────────────────────┤
//  │ mpsc     │ 多个       │ 1 个       │ 多条消息              │
//  │ oneshot  │ 1 个       │ 1 个       │ 仅 1 条消息           │
//  │ broadcast│ 多个       │ 多个       │ 多条（每人都收到）      │
//  │ watch    │ 1 个       │ 多个       │ 只保留最新值           │
//  └──────────┴────────────┴────────────┴──────────────────────┘
//
// ============================================================

use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Lesson 054: 异步通道 ===\n");

    // 1. mpsc 通道
    demo_mpsc().await;

    // 2. oneshot 通道
    demo_oneshot().await;

    // 3. broadcast 通道
    demo_broadcast().await;

    // 4. watch 通道
    demo_watch().await;

    // 5. 实际应用模式
    demo_request_response().await;

    // ============================================================
    // 总结
    // ============================================================
    println!("=== 总结 ===");
    println!("  1. mpsc: 多个生产者 → 一个消费者，最常用");
    println!("  2. oneshot: 一次性发送单个值，适合请求-响应模式");
    println!("  3. broadcast: 所有消费者都收到每条消息");
    println!("  4. watch: 只保留最新值，消费者看到最新状态");
    println!("  5. 选择通道类型取决于通信模式和需求");
}

// ============================================================
// 1. mpsc —— 多生产者单消费者
// ============================================================

async fn demo_mpsc() {
    println!("--- 1. mpsc 通道（多生产者，单消费者）---");

    // 创建有界通道，缓冲区大小为 32
    // tx: 发送端（可以 clone 给多个生产者）
    // rx: 接收端（只有一个）
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // 生产者 1
    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            let msg = format!("来自生产者1的消息-{}", i);
            tx1.send(msg).await.unwrap();
            sleep(Duration::from_millis(50)).await;
        }
        println!("  生产者1: 发送完毕");
        // tx1 在这里被 drop
    });

    // 生产者 2
    let tx2 = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            let msg = format!("来自生产者2的消息-{}", i);
            tx2.send(msg).await.unwrap();
            sleep(Duration::from_millis(70)).await;
        }
        println!("  生产者2: 发送完毕");
    });

    // 必须 drop 原始的 tx，否则通道不会关闭
    drop(tx);

    // 消费者：接收消息直到通道关闭
    let mut count = 0;
    while let Some(msg) = rx.recv().await {
        count += 1;
        println!("  消费者收到[{}]: {}", count, msg);
    }
    println!("  通道已关闭，共收到 {} 条消息\n", count);

    // ---- 有界 vs 无界通道 ----
    println!("  补充：有界 vs 无界通道");
    println!("    mpsc::channel(N)   → 有界通道，缓冲区满时 send 会阻塞");
    println!("    mpsc::unbounded_channel() → 无界通道，send 永不阻塞");

    // 无界通道示例
    let (tx, mut rx) = mpsc::unbounded_channel::<i32>();
    tx.send(1).unwrap(); // 注意：unbounded 的 send 不是异步的
    tx.send(2).unwrap();
    tx.send(3).unwrap();
    drop(tx);

    let mut values = Vec::new();
    while let Some(v) = rx.recv().await {
        values.push(v);
    }
    println!("    无界通道收到: {:?}\n", values);
}

// ============================================================
// 2. oneshot —— 一次性通道
// ============================================================

async fn demo_oneshot() {
    println!("--- 2. oneshot 通道（一次性，单发单收）---");

    // oneshot 通道只能发送一个值
    // 常用于：请求-响应模式、任务间传递结果
    let (tx, rx) = oneshot::channel::<String>();

    // 在另一个任务中发送结果
    tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        let result = "异步计算的结果".to_string();
        // send 不是异步的，因为只发一次
        if tx.send(result).is_err() {
            println!("  接收者已经被 drop 了");
        }
    });

    // 等待结果
    match rx.await {
        Ok(value) => println!("  收到 oneshot 结果: {}", value),
        Err(_) => println!("  发送者被 drop 了，没有发送值"),
    }

    // oneshot 也可以用来取消任务
    println!("\n  oneshot 用于取消信号：");
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();

    let task = tokio::spawn(async move {
        tokio::select! {
            _ = cancel_rx => {
                println!("  任务收到取消信号，停止执行");
            }
            _ = async {
                // 模拟长时间运行的操作
                for i in 1..=10 {
                    sleep(Duration::from_millis(50)).await;
                    println!("  任务执行中... 步骤 {}/10", i);
                }
            } => {
                println!("  任务正常完成");
            }
        }
    });

    // 等一会儿后发送取消信号
    sleep(Duration::from_millis(160)).await;
    let _ = cancel_tx.send(());
    task.await.unwrap();
    println!();
}

// ============================================================
// 3. broadcast —— 广播通道
// ============================================================

async fn demo_broadcast() {
    println!("--- 3. broadcast 通道（广播，多对多）---");

    // 创建广播通道，缓冲区大小为 16
    // 每个消费者都会收到所有消息
    let (tx, _rx) = broadcast::channel::<String>(16);

    // 创建多个消费者（通过 subscribe）
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    // 生产者
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        for i in 1..=3 {
            let msg = format!("广播消息-{}", i);
            // send 返回接收者数量
            let receivers = tx_clone.send(msg).unwrap();
            println!("  发送者: 消息 {} 发送给 {} 个接收者", i, receivers);
            sleep(Duration::from_millis(50)).await;
        }
    });

    // 消费者 1
    let consumer1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("  消费者1 收到: {}", msg);
        }
    });

    // 消费者 2
    let consumer2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("  消费者2 收到: {}", msg);
        }
    });

    // 等待生产者完成后 drop tx，关闭通道
    sleep(Duration::from_millis(250)).await;
    drop(tx);

    let _ = consumer1.await;
    let _ = consumer2.await;

    println!("  ⚠️  注意：如果消费者太慢，广播通道会丢弃旧消息 (Lagged error)");
    println!();
}

// ============================================================
// 4. watch —— 观察通道
// ============================================================

async fn demo_watch() {
    println!("--- 4. watch 通道（观察最新值）---");

    // watch 通道只保留最新值
    // 适合：配置更新、状态监控
    let (tx, mut rx1) = watch::channel("初始状态".to_string());
    let mut rx2 = rx1.clone();

    // 发送者：更新状态
    tokio::spawn(async move {
        let states = ["加载中", "就绪", "运行中", "完成"];
        for state in states {
            sleep(Duration::from_millis(80)).await;
            tx.send(state.to_string()).unwrap();
            println!("  发送者: 状态更新为 '{}'", state);
        }
        // tx drop 后通道关闭
    });

    // 观察者 1：监听变化
    let observer1 = tokio::spawn(async move {
        while rx1.changed().await.is_ok() {
            // borrow() 获取当前最新值的引用
            let value = rx1.borrow().clone();
            println!("  观察者1 看到状态变化: '{}'", value);
        }
        println!("  观察者1: 通道已关闭");
    });

    // 观察者 2：偶尔检查（可能跳过中间值）
    let observer2 = tokio::spawn(async move {
        sleep(Duration::from_millis(200)).await;
        // 可能跳过了前几个状态变化
        let current = rx2.borrow().clone();
        println!("  观察者2 延迟查看，当前状态: '{}'", current);

        // 继续等待后续变化
        while rx2.changed().await.is_ok() {
            let value = rx2.borrow().clone();
            println!("  观察者2 看到状态变化: '{}'", value);
        }
        println!("  观察者2: 通道已关闭");
    });

    observer1.await.unwrap();
    observer2.await.unwrap();

    println!("  watch 特点：");
    println!("    - 只保留最新值，慢消费者不会阻塞生产者");
    println!("    - changed() 只在值实际改变时通知");
    println!("    - 适合状态广播、配置热更新");
    println!();
}

// ============================================================
// 5. 实际应用模式：请求-响应（使用 mpsc + oneshot）
// ============================================================

/// 定义命令消息
enum Command {
    Get {
        key: String,
        /// 用 oneshot 回传结果
        resp_tx: oneshot::Sender<Option<String>>,
    },
    Set {
        key: String,
        value: String,
        resp_tx: oneshot::Sender<bool>,
    },
}

async fn demo_request_response() {
    println!("--- 5. 请求-响应模式（mpsc + oneshot）---");
    println!("  模拟一个简单的 Key-Value 存储服务\n");

    // 创建命令通道
    let (cmd_tx, mut cmd_rx) = mpsc::channel::<Command>(32);

    // 启动"服务器"任务：处理命令
    tokio::spawn(async move {
        use std::collections::HashMap;
        let mut store = HashMap::new();

        while let Some(cmd) = cmd_rx.recv().await {
            match cmd {
                Command::Get { key, resp_tx } => {
                    let value = store.get(&key).cloned();
                    println!("  [服务器] GET '{}' -> {:?}", key, value);
                    let _ = resp_tx.send(value);
                }
                Command::Set {
                    key,
                    value,
                    resp_tx,
                } => {
                    println!("  [服务器] SET '{}' = '{}'", key, value);
                    store.insert(key, value);
                    let _ = resp_tx.send(true);
                }
            }
        }
        println!("  [服务器] 命令通道关闭，服务器退出");
    });

    // 客户端 1：设置值
    let tx1 = cmd_tx.clone();
    let client1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        tx1.send(Command::Set {
            key: "name".to_string(),
            value: "Rust".to_string(),
            resp_tx,
        })
        .await
        .unwrap();
        let ok = resp_rx.await.unwrap();
        println!("  [客户端1] SET 结果: {}", ok);
    });

    // 客户端 2：设置另一个值，然后读取
    let tx2 = cmd_tx.clone();
    let client2 = tokio::spawn(async move {
        // SET
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Set {
            key: "version".to_string(),
            value: "1.0".to_string(),
            resp_tx,
        })
        .await
        .unwrap();
        resp_rx.await.unwrap();

        // GET
        let (resp_tx, resp_rx) = oneshot::channel();
        tx2.send(Command::Get {
            key: "name".to_string(),
            resp_tx,
        })
        .await
        .unwrap();
        let value = resp_rx.await.unwrap();
        println!("  [客户端2] GET 'name' -> {:?}", value);
    });

    // 等待客户端完成
    client1.await.unwrap();
    client2.await.unwrap();

    // 用原始发送端做最后一次查询
    let (resp_tx, resp_rx) = oneshot::channel();
    cmd_tx
        .send(Command::Get {
            key: "version".to_string(),
            resp_tx,
        })
        .await
        .unwrap();
    let value = resp_rx.await.unwrap();
    println!("  [主任务] GET 'version' -> {:?}", value);

    // drop 发送端，关闭通道
    drop(cmd_tx);
    // 给服务器一点时间完成
    sleep(Duration::from_millis(50)).await;
    println!();
}
