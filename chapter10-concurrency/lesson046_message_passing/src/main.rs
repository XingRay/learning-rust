// ============================================================
// Lesson 046: 消息传递 (Message Passing)
// ============================================================
// 本课介绍 Rust 中基于通道的消息传递并发模型：
// - mpsc::channel 创建通道
// - tx.send() / rx.recv() 发送和接收消息
// - 多个发送者 (tx.clone())
// - try_recv 非阻塞接收
// - 遍历接收 for msg in rx
// - 发送不同类型的消息（枚举包装）
// ============================================================
// Rust 的并发哲学：
// "Do not communicate by sharing memory; instead, share memory by communicating."
// "不要通过共享内存来通信，而要通过通信来共享内存。"
// ============================================================

use std::sync::mpsc; // mpsc = Multiple Producer, Single Consumer
use std::thread;
use std::time::Duration;

fn main() {
    println!("===== Lesson 046: 消息传递 =====\n");

    // ---------------------------------------------------------
    // 1. mpsc::channel 创建通道
    // ---------------------------------------------------------
    // channel() 返回一个元组 (Sender<T>, Receiver<T>)
    // tx = transmitter(发送端), rx = receiver(接收端)
    println!("--- 1. 基本的 channel 用法 ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("你好，主线程！");
        // send() 返回 Result<(), SendError<T>>
        // 如果接收端已经被 drop，send 会返回 Err
        tx.send(msg).unwrap();
        // 注意：msg 的所有权已被 send 转移，不能再使用
        // println!("{}", msg); // ❌ 编译错误
    });

    // recv() 会阻塞当前线程直到收到消息
    // 返回 Result<T, RecvError>
    // 如果所有发送端都被 drop 了，recv 会返回 Err
    let received = rx.recv().unwrap();
    println!("  收到消息: {}", received);

    // ---------------------------------------------------------
    // 2. 发送多条消息
    // ---------------------------------------------------------
    println!("\n--- 2. 发送多条消息 ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("消息1: 你好"),
            String::from("消息2: Rust"),
            String::from("消息3: 并发"),
            String::from("消息4: 很棒"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        // 线程结束后，tx 被 drop，通道关闭
    });

    // 使用 for 循环遍历接收端
    // 当通道关闭（所有发送端被 drop）时，迭代器结束
    for msg in rx {
        println!("  收到: {}", msg);
    }

    // ---------------------------------------------------------
    // 3. 多个发送者 (Multiple Producers)
    // ---------------------------------------------------------
    // mpsc 的含义就是 "多生产者，单消费者"
    // 通过 clone() 创建多个发送端
    println!("\n--- 3. 多个发送者 ---");

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 克隆发送端
    let tx2 = tx.clone();

    // 注意：原始的 tx 也要用掉或 drop，否则通道不会关闭
    drop(tx);

    thread::spawn(move || {
        let messages = vec!["线程1: A", "线程1: B", "线程1: C"];
        for msg in messages {
            tx1.send(msg.to_string()).unwrap();
            thread::sleep(Duration::from_millis(30));
        }
    });

    thread::spawn(move || {
        let messages = vec!["线程2: X", "线程2: Y", "线程2: Z"];
        for msg in messages {
            tx2.send(msg.to_string()).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    // 接收所有消息（来自不同的发送者）
    for msg in rx {
        println!("  收到: {}", msg);
    }

    // ---------------------------------------------------------
    // 4. try_recv 非阻塞接收
    // ---------------------------------------------------------
    // try_recv 不会阻塞，如果没有消息则立即返回 Err
    println!("\n--- 4. try_recv 非阻塞接收 ---");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("延迟消息").unwrap();
    });

    // 用 try_recv 轮询检查消息
    let mut attempts = 0;
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("  第 {} 次尝试: 收到消息 \"{}\"", attempts + 1, msg);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                // 通道为空，还没有消息
                attempts += 1;
                if attempts % 10 == 0 {
                    println!("  已尝试 {} 次，尚无消息...", attempts);
                }
                thread::sleep(Duration::from_millis(10));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("  通道已关闭！");
                break;
            }
        }
    }

    // ---------------------------------------------------------
    // 5. recv_timeout — 带超时的接收
    // ---------------------------------------------------------
    println!("\n--- 5. recv_timeout 带超时接收 ---");

    let (tx, rx) = mpsc::channel::<String>();

    // 不发送任何消息，直接 drop 发送端之前先测试超时
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let _ = tx.send("终于到了".to_string());
    });

    // 第一次：超时
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(msg) => println!("  收到: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("  50ms 超时：没有收到消息"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("  通道已断开"),
    }

    // 第二次：应该能收到
    match rx.recv_timeout(Duration::from_millis(300)) {
        Ok(msg) => println!("  收到: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("  300ms 超时：没有收到消息"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("  通道已断开"),
    }

    // ---------------------------------------------------------
    // 6. 发送不同类型的消息（枚举包装）
    // ---------------------------------------------------------
    // 通道是强类型的，只能发送一种类型
    // 如果要发送不同类型的消息，可以用枚举包装
    println!("\n--- 6. 发送不同类型的消息（枚举包装） ---");

    // 定义一个消息枚举，包含不同类型的变体
    #[derive(Debug)]
    enum WorkerMessage {
        Text(String),
        Number(i64),
        Data(Vec<u8>),
        Quit,
    }

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(WorkerMessage::Text("开始处理".to_string())).unwrap();
        tx.send(WorkerMessage::Number(42)).unwrap();
        tx.send(WorkerMessage::Data(vec![1, 2, 3, 4, 5])).unwrap();
        tx.send(WorkerMessage::Text("处理中...".to_string())).unwrap();
        tx.send(WorkerMessage::Number(100)).unwrap();
        tx.send(WorkerMessage::Quit).unwrap();
    });

    // 接收并匹配不同类型的消息
    loop {
        match rx.recv().unwrap() {
            WorkerMessage::Text(text) => {
                println!("  [文本] {}", text);
            }
            WorkerMessage::Number(n) => {
                println!("  [数字] {}", n);
            }
            WorkerMessage::Data(data) => {
                println!("  [数据] {:?} (长度: {})", data, data.len());
            }
            WorkerMessage::Quit => {
                println!("  [退出] 收到退出信号，停止接收");
                break;
            }
        }
    }

    // ---------------------------------------------------------
    // 7. sync_channel — 同步通道（有界通道）
    // ---------------------------------------------------------
    // mpsc::channel 是无界的（异步通道），发送不会阻塞
    // mpsc::sync_channel 是有界的，缓冲区满时发送会阻塞
    println!("\n--- 7. sync_channel 同步通道 ---");

    // 创建一个缓冲区大小为 2 的同步通道
    let (tx, rx) = mpsc::sync_channel(2);

    thread::spawn(move || {
        for i in 1..=5 {
            println!("  发送: {}", i);
            tx.send(i).unwrap();
            println!("  发送完成: {}", i);
        }
    });

    // 稍微延迟接收，让发送方体验到缓冲区满的阻塞
    thread::sleep(Duration::from_millis(100));
    for msg in rx {
        println!("  接收: {}", msg);
        thread::sleep(Duration::from_millis(30));
    }

    // ---------------------------------------------------------
    // 8. 实际应用：生产者-消费者模式
    // ---------------------------------------------------------
    println!("\n--- 8. 生产者-消费者模式 ---");

    #[derive(Debug)]
    struct Task {
        id: u32,
        payload: String,
    }

    let (tx, rx) = mpsc::channel();

    // 多个生产者
    for producer_id in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            for task_num in 0..2 {
                let task = Task {
                    id: producer_id * 10 + task_num,
                    payload: format!("来自生产者{}的任务{}", producer_id, task_num),
                };
                tx.send(task).unwrap();
            }
        });
    }
    // drop 原始的 tx，这样当所有 clone 都完成后通道才会关闭
    drop(tx);

    // 消费者处理所有任务
    let mut task_count = 0;
    for task in rx {
        println!("  处理任务 #{}: {}", task.id, task.payload);
        task_count += 1;
    }
    println!("  共处理了 {} 个任务", task_count);

    println!("\n===== Lesson 046 完成 =====");
}
