// ============================================================
// Lesson 045: 线程基础 (Threads)
// ============================================================
// 本课介绍 Rust 中的线程编程基础：
// - thread::spawn 创建线程
// - JoinHandle 和 join() 等待线程结束
// - move 闭包转移所有权到线程
// - thread::sleep 让线程休眠
// - 线程返回值
// - 多线程并发示例
// ============================================================

use std::thread;
use std::time::Duration;

fn main() {
    println!("===== Lesson 045: 线程基础 =====\n");

    // ---------------------------------------------------------
    // 1. 使用 thread::spawn 创建线程
    // ---------------------------------------------------------
    // thread::spawn 接受一个闭包，返回 JoinHandle
    // 新线程会与主线程并发执行
    println!("--- 1. 创建线程 ---");

    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  子线程: 计数 {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=3 {
        println!("  主线程: 计数 {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // ---------------------------------------------------------
    // 2. JoinHandle 和 join() 等待线程完成
    // ---------------------------------------------------------
    // join() 会阻塞当前线程，直到被 join 的线程执行完毕
    // 如果不调用 join()，主线程结束时子线程可能还未执行完
    println!("\n--- 2. join() 等待线程完成 ---");

    // handle.join() 返回 Result<T, Box<dyn Any + Send>>
    // 如果子线程 panic 了，join() 会返回 Err
    handle.join().unwrap();
    println!("  子线程已完成！");

    // ---------------------------------------------------------
    // 3. move 闭包 — 转移所有权到线程
    // ---------------------------------------------------------
    // 如果线程需要使用外部变量，通常需要使用 move 闭包
    // 将变量的所有权转移到线程中
    println!("\n--- 3. move 闭包转移所有权 ---");

    let name = String::from("Rust");
    let numbers = vec![1, 2, 3, 4, 5];

    // 使用 move 关键字将 name 和 numbers 的所有权转移到线程中
    let handle = thread::spawn(move || {
        println!("  线程中使用: name = {}", name);
        println!("  线程中使用: numbers = {:?}", numbers);
        // name 和 numbers 的所有权现在属于这个线程
    });

    // 编译错误：name 和 numbers 的所有权已经被 move 到线程中了
    // println!("{}", name);  // ❌ 不能再使用

    handle.join().unwrap();

    // ---------------------------------------------------------
    // 4. thread::sleep 线程休眠
    // ---------------------------------------------------------
    // thread::sleep 让当前线程休眠指定的时间
    // 注意：sleep 的时间是 *至少* 休眠这么久，操作系统可能会让它多睡一会
    println!("\n--- 4. thread::sleep 休眠 ---");

    let start = std::time::Instant::now();
    println!("  开始休眠...");
    thread::sleep(Duration::from_millis(100));
    let elapsed = start.elapsed();
    println!("  实际休眠时间: {:?}", elapsed);

    // ---------------------------------------------------------
    // 5. 线程返回值
    // ---------------------------------------------------------
    // 线程闭包的返回值可以通过 join() 获取
    // join() 返回 Result<T, ...>，其中 T 是闭包的返回类型
    println!("\n--- 5. 线程返回值 ---");

    let handle = thread::spawn(|| {
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum // 返回计算结果
    });

    // 通过 join() 获取线程的返回值
    let result = handle.join().unwrap();
    println!("  1 到 100 的和 = {}", result);

    // 返回更复杂的类型
    let handle = thread::spawn(|| -> Vec<i32> {
        (1..=10).map(|x| x * x).collect()
    });

    let squares = handle.join().unwrap();
    println!("  1 到 10 的平方: {:?}", squares);

    // ---------------------------------------------------------
    // 6. 多线程并发示例
    // ---------------------------------------------------------
    // 创建多个线程并行执行任务，最后汇总结果
    println!("\n--- 6. 多线程并发示例 ---");

    let mut handles = vec![];

    // 启动 5 个线程，每个线程计算一段区间的和
    for i in 0..5 {
        let handle = thread::spawn(move || {
            let start = i * 20 + 1;
            let end = (i + 1) * 20;
            let sum: i64 = (start..=end).sum();
            println!("  线程 {} 计算 {} 到 {} 的和 = {}", i, start, end, sum);
            sum
        });
        handles.push(handle);
    }

    // 等待所有线程完成并汇总结果
    let mut total: i64 = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    println!("  1 到 100 的总和 = {}", total);

    // ---------------------------------------------------------
    // 7. 使用 Builder 自定义线程
    // ---------------------------------------------------------
    // thread::Builder 允许设置线程名称和栈大小
    println!("\n--- 7. thread::Builder 自定义线程 ---");

    let builder = thread::Builder::new()
        .name("my-worker".to_string())  // 设置线程名称
        .stack_size(4 * 1024 * 1024);   // 设置栈大小为 4MB

    let handle = builder.spawn(|| {
        // 在线程内部获取当前线程的信息
        let current = thread::current();
        println!("  当前线程名称: {:?}", current.name());
        println!("  线程 ID: {:?}", current.id());
    }).unwrap();

    handle.join().unwrap();

    // 主线程的信息
    let main_thread = thread::current();
    println!("  主线程名称: {:?}", main_thread.name());

    // ---------------------------------------------------------
    // 8. 线程 panic 的处理
    // ---------------------------------------------------------
    // 子线程的 panic 不会影响主线程
    // 但可以通过 join() 的返回值检测到
    println!("\n--- 8. 线程 panic 的处理 ---");

    let handle = thread::spawn(|| {
        panic!("子线程故意 panic！");
    });

    // join() 返回 Err 表示子线程 panic 了
    match handle.join() {
        Ok(_) => println!("  子线程正常结束"),
        Err(e) => {
            // e 的类型是 Box<dyn Any + Send>
            // 可以尝试 downcast 为具体类型
            if let Some(msg) = e.downcast_ref::<&str>() {
                println!("  子线程 panic 了: {}", msg);
            } else if let Some(msg) = e.downcast_ref::<String>() {
                println!("  子线程 panic 了: {}", msg);
            } else {
                println!("  子线程 panic 了（未知类型）");
            }
        }
    }

    println!("  主线程继续正常运行！");

    // ---------------------------------------------------------
    // 9. thread::scope — 作用域线程 (Rust 1.63+)
    // ---------------------------------------------------------
    // 作用域线程允许借用外部数据而无需 move
    // 因为编译器可以确保线程在作用域结束前完成
    println!("\n--- 9. thread::scope 作用域线程 ---");

    let data = vec![1, 2, 3, 4, 5];
    let prefix = "结果";

    thread::scope(|s| {
        // 可以直接借用 data 和 prefix，无需 move
        s.spawn(|| {
            let sum: i32 = data.iter().sum();
            println!("  {}: 数据之和 = {}", prefix, sum);
        });

        s.spawn(|| {
            let product: i32 = data.iter().product();
            println!("  {}: 数据之积 = {}", prefix, product);
        });
    });
    // scope 结束后，所有线程都已完成，data 和 prefix 仍可使用
    println!("  scope 结束后, data 仍然可用: {:?}", data);

    println!("\n===== Lesson 045 完成 =====");
}
