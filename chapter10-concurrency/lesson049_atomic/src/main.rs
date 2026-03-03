// ============================================================
// Lesson 049: 原子操作 (Atomic Operations)
// ============================================================
// 本课介绍 Rust 中的原子类型和原子操作：
// - AtomicBool / AtomicI32 / AtomicUsize 等原子类型
// - load / store / fetch_add 等原子操作
// - Ordering（内存序）简要说明
// - 原子操作 vs Mutex 的对比
// ============================================================
// 原子操作是硬件层面的不可分割操作，不需要锁就能保证线程安全。
// 适用于简单的共享状态（计数器、标志位等）。
// ============================================================

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("===== Lesson 049: 原子操作 =====\n");

    // ---------------------------------------------------------
    // 1. AtomicBool — 原子布尔值
    // ---------------------------------------------------------
    // 常用于标志位：停止信号、初始化标志等
    println!("--- 1. AtomicBool 原子布尔值 ---");

    // 创建一个共享的停止标志
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = Arc::clone(&running);

    let worker = thread::spawn(move || {
        let mut count = 0;
        // load: 读取当前值
        while running_clone.load(Ordering::Relaxed) {
            count += 1;
            thread::sleep(Duration::from_millis(10));
            if count >= 10 {
                break; // 安全退出，防止无限循环
            }
        }
        println!("  工作线程：执行了 {} 次循环", count);
    });

    // 主线程等待一段时间后发送停止信号
    thread::sleep(Duration::from_millis(50));
    // store: 写入新值
    running.store(false, Ordering::Relaxed);
    println!("  主线程：已发送停止信号");

    worker.join().unwrap();

    // ---------------------------------------------------------
    // 2. AtomicI32 / AtomicI64 — 原子整数
    // ---------------------------------------------------------
    println!("\n--- 2. AtomicI32 原子整数 ---");

    let counter = Arc::new(AtomicI32::new(0));

    // store: 设置值
    counter.store(42, Ordering::SeqCst);
    println!("  store 后: {}", counter.load(Ordering::SeqCst));

    // fetch_add: 原子加法，返回旧值
    let old = counter.fetch_add(10, Ordering::SeqCst);
    println!("  fetch_add(10): 旧值={}, 新值={}", old, counter.load(Ordering::SeqCst));

    // fetch_sub: 原子减法，返回旧值
    let old = counter.fetch_sub(5, Ordering::SeqCst);
    println!("  fetch_sub(5): 旧值={}, 新值={}", old, counter.load(Ordering::SeqCst));

    // fetch_min / fetch_max: 原子取最小/最大值
    let value = AtomicI32::new(10);
    value.fetch_min(5, Ordering::SeqCst);
    println!("  fetch_min(5) on 10: {}", value.load(Ordering::SeqCst)); // 5
    value.fetch_max(20, Ordering::SeqCst);
    println!("  fetch_max(20) on 5: {}", value.load(Ordering::SeqCst)); // 20

    // ---------------------------------------------------------
    // 3. AtomicUsize — 原子无符号整数
    // ---------------------------------------------------------
    // AtomicUsize 常用于计数器和索引
    println!("\n--- 3. AtomicUsize 多线程计数器 ---");

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..1000 {
                // fetch_add 是原子操作，多线程并发也不会丢失计数
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    // 10 个线程，每个加 1000 次 = 10000
    let final_count = counter.load(Ordering::Relaxed);
    println!("  10 个线程 × 1000 次 = {}", final_count);
    assert_eq!(final_count, 10000);
    println!("  ✓ 结果正确！原子操作保证了计数的准确性");

    // ---------------------------------------------------------
    // 4. compare_exchange — 比较并交换 (CAS)
    // ---------------------------------------------------------
    // CAS 是构建无锁数据结构的基础操作
    // 只有当前值等于 expected 时，才会更新为 new
    println!("\n--- 4. compare_exchange (CAS) ---");

    let value = AtomicI32::new(5);

    // 成功的 CAS：当前值是 5，更新为 10
    match value.compare_exchange(5, 10, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(old) => println!("  CAS 成功: 旧值={}, 新值={}", old, value.load(Ordering::SeqCst)),
        Err(actual) => println!("  CAS 失败: 期望=5, 实际={}", actual),
    }

    // 失败的 CAS：当前值是 10（不是 5），不会更新
    match value.compare_exchange(5, 20, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(old) => println!("  CAS 成功: 旧值={}", old),
        Err(actual) => println!("  CAS 失败: 期望=5, 实际={} (值未改变)", actual),
    }

    // compare_exchange_weak：可能虚假失败，但在循环中性能更好
    let value = AtomicI32::new(0);
    // 用 CAS 循环实现原子加法（演示原理）
    loop {
        let current = value.load(Ordering::Relaxed);
        match value.compare_exchange_weak(current, current + 42, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_) => {
                println!("  CAS 循环成功: {} -> {}", current, value.load(Ordering::SeqCst));
                break;
            }
            Err(_) => continue, // 虚假失败，重试
        }
    }

    // ---------------------------------------------------------
    // 5. 位操作：fetch_and / fetch_or / fetch_xor
    // ---------------------------------------------------------
    println!("\n--- 5. 原子位操作 ---");

    let flags = AtomicUsize::new(0b0000);

    // fetch_or: 设置标志位
    flags.fetch_or(0b0001, Ordering::SeqCst); // 设置第0位
    flags.fetch_or(0b0100, Ordering::SeqCst); // 设置第2位
    println!("  设置位后: {:04b}", flags.load(Ordering::SeqCst));

    // fetch_and: 清除标志位
    flags.fetch_and(!0b0001, Ordering::SeqCst); // 清除第0位
    println!("  清除位后: {:04b}", flags.load(Ordering::SeqCst));

    // fetch_xor: 翻转标志位
    flags.fetch_xor(0b0110, Ordering::SeqCst); // 翻转第1和第2位
    println!("  翻转位后: {:04b}", flags.load(Ordering::SeqCst));

    // ---------------------------------------------------------
    // 6. Ordering（内存序）简要说明
    // ---------------------------------------------------------
    println!("\n--- 6. Ordering 内存序说明 ---");

    println!("  Ordering 控制原子操作的内存可见性和指令重排序：");
    println!();
    println!("  Relaxed  — 最弱的保证，只保证操作本身是原子的");
    println!("              不保证与其他操作的顺序关系");
    println!("              适用于：简单计数器");
    println!();
    println!("  Acquire  — 读操作使用，保证后续读写不会被重排到此操作之前");
    println!("              \"获取\"语义：看到写者发布的所有数据");
    println!();
    println!("  Release  — 写操作使用，保证之前的读写不会被重排到此操作之后");
    println!("              \"发布\"语义：让读者看到之前写的所有数据");
    println!();
    println!("  AcqRel   — 同时具有 Acquire 和 Release 语义");
    println!("              用于 read-modify-write 操作（如 fetch_add）");
    println!();
    println!("  SeqCst   — 最强保证，所有线程看到相同的操作顺序");
    println!("              性能开销最大，但最容易推理正确性");
    println!("              不确定时就用这个！");

    // Acquire/Release 配对示例
    println!("\n  Acquire/Release 配对示例:");
    let data = Arc::new(AtomicI64::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let data_c = Arc::clone(&data);
    let ready_c = Arc::clone(&ready);

    let writer = thread::spawn(move || {
        // 先写入数据
        data_c.store(42, Ordering::Relaxed);
        // 然后用 Release 标记 "数据已就绪"
        ready_c.store(true, Ordering::Release);
    });

    let data_c = Arc::clone(&data);
    let ready_c = Arc::clone(&ready);

    let reader = thread::spawn(move || {
        // 用 Acquire 读取 ready 标志
        while !ready_c.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // Acquire 保证：如果我们看到 ready=true，
        // 那么 writer 在 Release 之前写入的 data=42 也一定可见
        let val = data_c.load(Ordering::Relaxed);
        println!("    读取到 data = {} (应该是 42)", val);
    });

    writer.join().unwrap();
    reader.join().unwrap();

    // ---------------------------------------------------------
    // 7. 原子操作 vs Mutex 性能对比
    // ---------------------------------------------------------
    println!("\n--- 7. 原子操作 vs Mutex 性能对比 ---");

    let iterations = 100_000;
    let num_threads = 8;

    // Atomic 版本
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..num_threads {
        let counter = Arc::clone(&atomic_counter);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let atomic_duration = start.elapsed();
    let atomic_result = atomic_counter.load(Ordering::Relaxed);

    // Mutex 版本
    let mutex_counter = Arc::new(Mutex::new(0usize));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..num_threads {
        let counter = Arc::clone(&mutex_counter);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                *counter.lock().unwrap() += 1;
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mutex_duration = start.elapsed();
    let mutex_result = *mutex_counter.lock().unwrap();

    println!("  {} 个线程 × {} 次递增:", num_threads, iterations);
    println!("  Atomic: {:?} (结果: {})", atomic_duration, atomic_result);
    println!("  Mutex:  {:?} (结果: {})", mutex_duration, mutex_result);
    println!("  原子操作通常比 Mutex 快得多（尤其在高竞争场景）");

    // ---------------------------------------------------------
    // 8. 原子 vs Mutex 选择指南
    // ---------------------------------------------------------
    println!("\n--- 8. 原子 vs Mutex 选择指南 ---");

    println!("  ┌───────────────┬──────────────────────────────────┐");
    println!("  │ 选择 Atomic   │ 简单数值操作（计数器/标志位）      │");
    println!("  │               │ 性能关键路径                      │");
    println!("  │               │ 单个值的原子更新                  │");
    println!("  ├───────────────┼──────────────────────────────────┤");
    println!("  │ 选择 Mutex    │ 复杂数据结构（Vec, HashMap等）    │");
    println!("  │               │ 需要多个值的一致性更新             │");
    println!("  │               │ 临界区包含多个操作                │");
    println!("  │               │ 代码逻辑更容易理解                │");
    println!("  └───────────────┴──────────────────────────────────┘");
    println!("  提示：不确定时先用 Mutex，性能瓶颈时再考虑 Atomic");

    println!("\n===== Lesson 049 完成 =====");
}
