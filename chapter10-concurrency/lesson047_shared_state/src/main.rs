// ============================================================
// Lesson 047: 共享状态 (Shared State)
// ============================================================
// 本课介绍 Rust 中基于共享状态的并发编程：
// - Mutex::new / lock() 互斥锁基础
// - Arc<Mutex<T>> 在多线程中共享可变数据
// - MutexGuard 自动释放锁
// - 多线程计数器示例
// - Mutex 中毒 (poisoning) 机制
// ============================================================

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("===== Lesson 047: 共享状态 =====\n");

    // ---------------------------------------------------------
    // 1. Mutex 基础用法
    // ---------------------------------------------------------
    // Mutex<T> 提供了内部可变性 + 互斥访问
    // 同一时刻只有一个线程能访问内部数据
    println!("--- 1. Mutex 基础用法 ---");

    let m = Mutex::new(5);

    {
        // lock() 返回 Result<MutexGuard<T>, PoisonError<MutexGuard<T>>>
        // MutexGuard 实现了 Deref 和 DerefMut，可以像引用一样使用
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("  修改后的值: {}", *num);
        // MutexGuard 在离开作用域时自动释放锁
    }

    // 锁已释放，可以再次获取
    println!("  最终值: {:?}", m.lock().unwrap());

    // ---------------------------------------------------------
    // 2. MutexGuard 自动释放
    // ---------------------------------------------------------
    // MutexGuard 实现了 Drop trait，离开作用域时自动释放锁
    // 这是 Rust RAII（资源获取即初始化）的体现
    println!("\n--- 2. MutexGuard 自动释放 ---");

    let data = Mutex::new(vec![1, 2, 3]);

    {
        let mut guard = data.lock().unwrap();
        guard.push(4);
        guard.push(5);
        println!("  作用域内: {:?}", *guard);
        // guard 在这里被 drop，锁自动释放
    }

    // 再次获取锁
    {
        let guard = data.lock().unwrap();
        println!("  作用域外重新获取: {:?}", *guard);
    }

    // 注意：如果不小心持有锁太久，可能会导致性能问题
    // 技巧：用花括号限制 MutexGuard 的作用域
    let counter = Mutex::new(0);
    // 好的做法 — 锁的持有时间很短
    *counter.lock().unwrap() += 1;
    println!("  快速更新后: {}", *counter.lock().unwrap());

    // ---------------------------------------------------------
    // 3. Arc<Mutex<T>> 在多线程中使用
    // ---------------------------------------------------------
    // Mutex 本身不能在多个线程间共享（没有实现 Clone）
    // Rc<T> 不能用于多线程（不是 Send）
    // 所以需要 Arc<T>（原子引用计数）来在多线程间共享 Mutex
    println!("\n--- 3. Arc<Mutex<T>> 多线程共享 ---");

    // 创建一个线程安全的共享计数器
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        // Arc::clone 增加引用计数（原子操作，线程安全）
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  线程 {} 将计数器增加到 {}", i, *num);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("  最终计数: {}", *counter.lock().unwrap());

    // ---------------------------------------------------------
    // 4. 多线程计数器 — 更复杂的示例
    // ---------------------------------------------------------
    println!("\n--- 4. 多线程累加器 ---");

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for thread_id in 0..5 {
        let results = Arc::clone(&results);
        let handle = thread::spawn(move || {
            // 模拟一些计算
            let partial_sum: i64 = ((thread_id * 20 + 1)..=(thread_id + 1) * 20).sum();

            // 获取锁，将结果添加到共享 Vec 中
            let mut data = results.lock().unwrap();
            data.push((thread_id, partial_sum));
            println!("  线程 {} 计算完成: 部分和 = {}", thread_id, partial_sum);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let results = results.lock().unwrap();
    let total: i64 = results.iter().map(|(_, sum)| sum).sum();
    println!("  各线程结果: {:?}", *results);
    println!("  总和: {}", total);
    drop(results); // 显式释放锁

    // ---------------------------------------------------------
    // 5. Mutex 中毒 (Poisoning)
    // ---------------------------------------------------------
    // 如果一个线程在持有锁时 panic，Mutex 会被"中毒"
    // 后续尝试获取锁会得到 PoisonError
    // 这是一种安全机制：防止使用可能不一致的数据
    println!("\n--- 5. Mutex 中毒 (Poisoning) ---");

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data_clone = Arc::clone(&data);

    // 这个线程会在持有锁时 panic
    let handle = thread::spawn(move || {
        let mut guard = data_clone.lock().unwrap();
        guard.push(4);
        panic!("哎呀！线程在持有锁时 panic 了！");
        // guard 在 panic 展开时被 drop，但 Mutex 已经中毒
    });

    // 等待 panic 的线程
    let _ = handle.join(); // 忽略 panic 的错误

    // 尝试再次获取锁
    match data.lock() {
        Ok(guard) => {
            println!("  成功获取锁: {:?}", *guard);
        }
        Err(poisoned) => {
            println!("  Mutex 中毒了！");
            // 但我们仍然可以通过 into_inner() 获取数据
            let guard = poisoned.into_inner();
            println!("  中毒后的数据: {:?}", *guard);
            // 数据可能处于不一致状态，使用时要小心
        }
    }

    // ---------------------------------------------------------
    // 6. 避免死锁的基本原则
    // ---------------------------------------------------------
    println!("\n--- 6. 避免死锁的提示 ---");

    // 死锁示例说明（不实际运行，因为会卡住）：
    // 线程 A: 先锁 lock1, 再锁 lock2
    // 线程 B: 先锁 lock2, 再锁 lock1
    // → 两个线程互相等待，死锁！

    println!("  避免死锁的策略:");
    println!("  1. 始终按相同顺序获取多个锁");
    println!("  2. 尽量减少锁的持有时间");
    println!("  3. 避免在持有锁时调用外部函数");
    println!("  4. 考虑使用 try_lock() 避免永久等待");

    // try_lock() 的使用示例
    let lock = Mutex::new(42);

    // 先获取一次锁
    let _guard = lock.lock().unwrap();

    // try_lock 不会阻塞，如果锁已被占用则立即返回 Err
    match lock.try_lock() {
        Ok(val) => println!("  try_lock 成功: {}", *val),
        Err(_) => println!("  try_lock 失败: 锁已被占用（预期行为）"),
    }
    drop(_guard);

    // ---------------------------------------------------------
    // 7. 共享状态的实际应用：线程安全的缓存
    // ---------------------------------------------------------
    println!("\n--- 7. 线程安全的简单缓存 ---");

    use std::collections::HashMap;

    let cache: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = vec![];

    // 多个线程写入缓存
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i * 10);
            cache.lock().unwrap().insert(key.clone(), value.clone());
            println!("  线程 {} 写入: {} -> {}", i, key, value);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 读取缓存
    let cache = cache.lock().unwrap();
    println!("  缓存内容: {:?}", *cache);
    println!("  缓存大小: {}", cache.len());

    println!("\n===== Lesson 047 完成 =====");
}
