// ============================================================
// Lesson 048: Mutex 与 RwLock
// ============================================================
// 本课深入对比 Mutex 和 RwLock 两种锁机制：
// - Mutex 互斥锁回顾
// - RwLock 读写锁 (read/write)
// - 读多写少场景的选择
// - 避免死锁的策略
// - Lock ordering（锁排序）
// ============================================================

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    println!("===== Lesson 048: Mutex 与 RwLock =====\n");

    // ---------------------------------------------------------
    // 1. Mutex 互斥锁回顾
    // ---------------------------------------------------------
    // Mutex 是最基本的锁：同一时刻只允许一个线程访问数据
    // 不区分读和写，所有访问都是互斥的
    println!("--- 1. Mutex 互斥锁回顾 ---");

    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // 即使所有线程只是读取，也必须互斥访问
    for i in 0..5 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let val = data.lock().unwrap();
            println!("  线程 {} 读取到值: {}", i, *val);
            // 即使只是读取，也持有了独占锁
            thread::sleep(Duration::from_millis(10));
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // ---------------------------------------------------------
    // 2. RwLock 读写锁基础
    // ---------------------------------------------------------
    // RwLock 允许多个读者 OR 一个写者
    // - read()  获取读锁（共享锁），多个线程可以同时读
    // - write() 获取写锁（独占锁），只有一个线程可以写
    println!("\n--- 2. RwLock 读写锁基础 ---");

    let rwlock = RwLock::new(5);

    // 多个读锁可以同时存在
    {
        let r1 = rwlock.read().unwrap();
        let r2 = rwlock.read().unwrap();
        println!("  同时读取: r1={}, r2={}", *r1, *r2);
        // r1 和 r2 同时持有读锁，这是允许的
    } // 两个读锁都在这里释放

    // 写锁是独占的
    {
        let mut w = rwlock.write().unwrap();
        *w = 10;
        println!("  写入后: {}", *w);
        // 写锁被持有时，不能获取读锁或其他写锁
    } // 写锁在这里释放

    // 写锁释放后，又可以读了
    {
        let r = rwlock.read().unwrap();
        println!("  最终读取: {}", *r);
    }

    // ---------------------------------------------------------
    // 3. RwLock 在多线程中使用
    // ---------------------------------------------------------
    println!("\n--- 3. RwLock 多线程示例 ---");

    let config = Arc::new(RwLock::new(AppConfig {
        max_connections: 100,
        timeout_ms: 5000,
        debug_mode: false,
    }));

    let mut handles = vec![];

    // 启动多个读取线程
    for i in 0..5 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            // 读锁：多个线程可以同时获取
            let cfg = config.read().unwrap();
            println!(
                "  读取线程 {}: max_conn={}, timeout={}ms, debug={}",
                i, cfg.max_connections, cfg.timeout_ms, cfg.debug_mode
            );
            thread::sleep(Duration::from_millis(10));
        }));
    }

    // 启动一个写入线程
    {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            // 写锁：独占访问
            let mut cfg = config.write().unwrap();
            cfg.debug_mode = true;
            cfg.timeout_ms = 3000;
            println!("  写入线程: 已更新配置");
        }));
    }

    // 等待所有线程完成后再启动新的读取线程
    for handle in handles {
        handle.join().unwrap();
    }

    // 验证配置已更新
    let cfg = config.read().unwrap();
    println!(
        "  更新后配置: max_conn={}, timeout={}ms, debug={}",
        cfg.max_connections, cfg.timeout_ms, cfg.debug_mode
    );
    drop(cfg);

    // ---------------------------------------------------------
    // 4. 读多写少场景的性能对比
    // ---------------------------------------------------------
    // RwLock 在读多写少的场景下比 Mutex 有更好的并发性能
    // 因为多个读者可以并行，而 Mutex 会让所有访问串行化
    println!("\n--- 4. 读多写少场景性能对比 ---");

    // 使用 Mutex 的读密集场景
    let mutex_data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&mutex_data);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let guard = data.lock().unwrap();
                let _sum: i32 = guard.iter().sum();
                // 持有锁，其他线程都得等
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mutex_duration = start.elapsed();
    println!("  Mutex  (10线程 x 100次读): {:?}", mutex_duration);

    // 使用 RwLock 的读密集场景
    let rwlock_data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..10 {
        let data = Arc::clone(&rwlock_data);
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let guard = data.read().unwrap();
                let _sum: i32 = guard.iter().sum();
                // 读锁可以并行
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let rwlock_duration = start.elapsed();
    println!("  RwLock (10线程 x 100次读): {:?}", rwlock_duration);

    println!("  提示: RwLock 在读密集场景通常更快（取决于系统调度）");

    // ---------------------------------------------------------
    // 5. try_read / try_write — 非阻塞获取锁
    // ---------------------------------------------------------
    println!("\n--- 5. try_read / try_write ---");

    let lock = RwLock::new(42);

    // 持有读锁时，try_read 可以成功，try_write 会失败
    {
        let _r = lock.read().unwrap();
        match lock.try_read() {
            Ok(val) => println!("  try_read 成功（读锁期间）: {}", *val),
            Err(_) => println!("  try_read 失败"),
        }
        match lock.try_write() {
            Ok(_) => println!("  try_write 成功"),
            Err(_) => println!("  try_write 失败（读锁期间，预期行为）"),
        }
    }

    // 持有写锁时，try_read 和 try_write 都会失败
    {
        let _w = lock.write().unwrap();
        match lock.try_read() {
            Ok(_) => println!("  try_read 成功"),
            Err(_) => println!("  try_read 失败（写锁期间，预期行为）"),
        }
        match lock.try_write() {
            Ok(_) => println!("  try_write 成功"),
            Err(_) => println!("  try_write 失败（写锁期间，预期行为）"),
        }
    }

    // ---------------------------------------------------------
    // 6. 避免死锁的策略
    // ---------------------------------------------------------
    println!("\n--- 6. 避免死锁的策略 ---");

    // 策略一：锁排序（Lock Ordering）
    // 当需要获取多个锁时，始终按照固定顺序获取
    println!("  策略1: 锁排序");

    let lock_a = Arc::new(Mutex::new("A".to_string()));
    let lock_b = Arc::new(Mutex::new("B".to_string()));

    let lock_a_clone = Arc::clone(&lock_a);
    let lock_b_clone = Arc::clone(&lock_b);

    // 两个线程都按 A -> B 的顺序获取锁，不会死锁
    let h1 = thread::spawn(move || {
        let a = lock_a_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let b = lock_b_clone.lock().unwrap();
        println!("    线程1: 获取了 {} 和 {}", *a, *b);
    });

    let lock_a_clone = Arc::clone(&lock_a);
    let lock_b_clone = Arc::clone(&lock_b);

    let h2 = thread::spawn(move || {
        // 也是先 A 后 B，和线程1相同的顺序
        let a = lock_a_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let b = lock_b_clone.lock().unwrap();
        println!("    线程2: 获取了 {} 和 {}", *a, *b);
    });

    h1.join().unwrap();
    h2.join().unwrap();
    println!("  ✓ 按相同顺序获取锁 — 不会死锁");

    // 策略二：减少锁的粒度
    println!("\n  策略2: 减少锁的粒度");
    println!("    - 只在需要时获取锁");
    println!("    - 尽快释放锁（用花括号限制作用域）");
    println!("    - 避免在持有锁时做耗时操作");

    // 示例：在锁外做计算，只在写入时获取锁
    let shared_results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let results = Arc::clone(&shared_results);
        handles.push(thread::spawn(move || {
            // 耗时计算在锁外进行
            let computed_value = expensive_computation(i);

            // 只在写入时获取锁，持有时间很短
            results.lock().unwrap().push(computed_value);
        }));
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("    计算结果: {:?}", *shared_results.lock().unwrap());

    // 策略三：使用 try_lock 避免永久等待
    println!("\n  策略3: 使用 try_lock + 退避重试");

    let resource = Arc::new(Mutex::new(0));
    let resource_clone = Arc::clone(&resource);

    let h = thread::spawn(move || {
        // 先占住锁
        let _guard = resource_clone.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
    });

    // 主线程用 try_lock 尝试获取
    let mut retries = 0;
    loop {
        match resource.try_lock() {
            Ok(mut val) => {
                *val = 42;
                println!("    经过 {} 次重试后成功获取锁", retries);
                break;
            }
            Err(_) => {
                retries += 1;
                thread::sleep(Duration::from_millis(10)); // 退避
                if retries > 20 {
                    println!("    超过最大重试次数，放弃");
                    break;
                }
            }
        }
    }
    h.join().unwrap();

    // ---------------------------------------------------------
    // 7. 何时使用 Mutex vs RwLock
    // ---------------------------------------------------------
    println!("\n--- 7. Mutex vs RwLock 选择指南 ---");

    println!("  ┌─────────────┬───────────────────────────────────────┐");
    println!("  │ 选择 Mutex  │ 写操作频繁 / 读写比例相近              │");
    println!("  │             │ 临界区很短（锁竞争不是瓶颈）           │");
    println!("  │             │ 实现更简单，开销略低                   │");
    println!("  ├─────────────┼───────────────────────────────────────┤");
    println!("  │ 选择 RwLock │ 读操作远多于写操作                     │");
    println!("  │             │ 读操作耗时较长                        │");
    println!("  │             │ 需要最大化读取并发度                   │");
    println!("  └─────────────┴───────────────────────────────────────┘");

    println!("  注意: RwLock 有额外的开销（维护读者计数等），");
    println!("        如果锁竞争不严重，Mutex 可能反而更快。");
    println!("        建议通过基准测试来决定！");

    println!("\n===== Lesson 048 完成 =====");
}

/// 配置结构体
#[derive(Debug)]
struct AppConfig {
    max_connections: u32,
    timeout_ms: u64,
    debug_mode: bool,
}

/// 模拟耗时计算
fn expensive_computation(input: i32) -> i32 {
    thread::sleep(Duration::from_millis(10));
    input * input + input
}
