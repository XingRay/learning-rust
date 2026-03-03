// ============================================================
// Lesson 041: Rc<T> 与 Arc<T> — 引用计数智能指针
// ============================================================
// Rc<T>（Reference Counted）允许多个所有者共享同一数据的所有权。
// 每次 clone 时引用计数 +1，drop 时 -1，计数归零时释放内存。
//
// Rc<T> 仅用于单线程场景。
// Arc<T>（Atomic Reference Counted）是线程安全版本，用于多线程。

use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Lesson 041: Rc<T> 与 Arc<T> ===\n");

    // -------------------------------------------------------
    // 1. Rc::new 与 Rc::clone 基本用法
    // -------------------------------------------------------
    println!("--- 1. Rc::new 与 Rc::clone ---");

    // 创建一个 Rc 包裹的字符串
    let original = Rc::new(String::from("共享数据"));
    println!("创建 original: {}", original);

    // Rc::clone 不会深拷贝数据，只是增加引用计数
    let clone1 = Rc::clone(&original);
    let clone2 = Rc::clone(&original);

    // 三个变量指向同一块堆内存
    println!("original: {}", original);
    println!("clone1:   {}", clone1);
    println!("clone2:   {}", clone2);

    // 验证它们指向同一地址
    println!(
        "是否指向同一地址: {}",
        Rc::ptr_eq(&original, &clone1) && Rc::ptr_eq(&clone1, &clone2)
    );

    println!();

    // -------------------------------------------------------
    // 2. 引用计数 — Rc::strong_count
    // -------------------------------------------------------
    println!("--- 2. Rc::strong_count ---");

    let data = Rc::new(vec![1, 2, 3]);
    println!("创建后引用计数: {}", Rc::strong_count(&data)); // 1

    {
        let _ref1 = Rc::clone(&data);
        println!("clone 一次后: {}", Rc::strong_count(&data)); // 2

        {
            let _ref2 = Rc::clone(&data);
            println!("clone 两次后: {}", Rc::strong_count(&data)); // 3
            // _ref2 在此处被 drop
        }

        println!("内层作用域结束: {}", Rc::strong_count(&data)); // 2
        // _ref1 在此处被 drop
    }

    println!("外层作用域结束: {}", Rc::strong_count(&data)); // 1

    // 当最后一个 Rc 被 drop 时，数据才会被释放
    println!("数据仍然可用: {:?}", data);

    println!();

    // -------------------------------------------------------
    // 3. 共享所有权场景 — 图结构
    // -------------------------------------------------------
    println!("--- 3. 共享所有权场景 ---");

    // 场景：多个列表共享同一个尾部
    // 例如: list_b 和 list_c 都引用 list_a 作为尾部
    //
    //   list_b: 4 -> 1 -> 2 -> 3 -> Nil
    //   list_c: 5 -> 1 -> 2 -> 3 -> Nil
    //                ^^^^^^^^^^^^^^^^
    //                  共享部分 (list_a)

    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // 共享的尾部
    let list_a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("list_a 创建后引用计数: {}", Rc::strong_count(&list_a));

    // list_b 和 list_c 共享 list_a
    let list_b = Cons(4, Rc::clone(&list_a));
    println!("list_b 创建后引用计数: {}", Rc::strong_count(&list_a));

    let list_c = Cons(5, Rc::clone(&list_a));
    println!("list_c 创建后引用计数: {}", Rc::strong_count(&list_a));

    println!("list_b: {:?}", list_b);
    println!("list_c: {:?}", list_c);

    // 更实际的例子：共享配置
    #[derive(Debug)]
    struct Config {
        database_url: String,
        max_connections: u32,
    }

    let config = Rc::new(Config {
        database_url: String::from("postgres://localhost/mydb"),
        max_connections: 10,
    });

    // 多个服务共享同一份配置
    struct ServiceA {
        config: Rc<Config>,
    }

    struct ServiceB {
        config: Rc<Config>,
    }

    let service_a = ServiceA {
        config: Rc::clone(&config),
    };
    let service_b = ServiceB {
        config: Rc::clone(&config),
    };

    println!("ServiceA 使用配置: {}", service_a.config.database_url);
    println!(
        "ServiceB 最大连接数: {}",
        service_b.config.max_connections
    );
    println!("配置引用计数: {}", Rc::strong_count(&config)); // 3

    println!();

    // -------------------------------------------------------
    // 4. Arc<T> — 线程安全的引用计数
    // -------------------------------------------------------
    println!("--- 4. Arc<T> 线程安全版本 ---");

    // Rc<T> 不能在线程间传递（没有实现 Send trait）
    // Arc<T> 使用原子操作来维护引用计数，可以安全地在线程间共享

    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("Arc 数据: {:?}", shared_data);
    println!("初始引用计数: {}", Arc::strong_count(&shared_data));

    // Arc::clone 的用法和 Rc::clone 完全一样
    let clone_for_display = Arc::clone(&shared_data);
    println!("clone 后引用计数: {}", Arc::strong_count(&shared_data));
    println!("克隆的数据: {:?}", clone_for_display);

    println!();

    // -------------------------------------------------------
    // 5. Arc 配合线程使用
    // -------------------------------------------------------
    println!("--- 5. Arc 配合线程使用 ---");

    let data = Arc::new(vec![10, 20, 30, 40, 50]);
    let mut handles = vec![];

    // 启动 5 个线程，每个线程读取共享数据
    for i in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            // 每个线程拥有 Arc 的一个克隆，共享底层数据
            println!("线程 {} 读取到: data[{}] = {}", i, i, data_clone[i]);
            data_clone[i] * 2 // 返回计算结果
        });
        handles.push(handle);
    }

    // 等待所有线程完成，收集结果
    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("所有线程的结果: {:?}", results);
    println!("原始数据仍然可用: {:?}", data);
    println!("最终引用计数: {}", Arc::strong_count(&data)); // 1 (线程都结束了)

    println!();

    // -------------------------------------------------------
    // 6. Arc 多线程共享只读资源的实际示例
    // -------------------------------------------------------
    println!("--- 6. 多线程共享只读资源 ---");

    // 模拟共享一个大型查找表
    let lookup_table = Arc::new(
        (0..100)
            .map(|i| (format!("key_{}", i), i * i))
            .collect::<std::collections::HashMap<String, i32>>(),
    );

    let mut handles = vec![];

    for i in 0..4 {
        let table = Arc::clone(&lookup_table);
        let handle = thread::spawn(move || {
            let key = format!("key_{}", i * 10);
            match table.get(&key) {
                Some(value) => println!("线程 {}: {} => {}", i, key, value),
                None => println!("线程 {}: {} 未找到", i, key),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!();

    // -------------------------------------------------------
    // 7. Rc vs Arc 总结
    // -------------------------------------------------------
    println!("--- 7. Rc vs Arc 总结 ---");
    println!("┌──────────┬──────────────┬──────────────┐");
    println!("│ 特性     │ Rc<T>        │ Arc<T>       │");
    println!("├──────────┼──────────────┼──────────────┤");
    println!("│ 线程安全 │ ❌ 否        │ ✅ 是        │");
    println!("│ 性能     │ ⚡ 更快      │ 🔒 稍慢      │");
    println!("│ 引用计数 │ 非原子操作   │ 原子操作     │");
    println!("│ 适用场景 │ 单线程共享   │ 多线程共享   │");
    println!("│ 可变性   │ 需配合RefCell│ 需配合Mutex  │");
    println!("└──────────┴──────────────┴──────────────┘");
    println!();
    println!("注意：Rc 和 Arc 提供的都是不可变引用！");
    println!("如果需要修改共享数据，需要配合内部可变性：");
    println!("  - 单线程：Rc<RefCell<T>>");
    println!("  - 多线程：Arc<Mutex<T>> 或 Arc<RwLock<T>>");

    println!("\n=== Lesson 041 完成 ===");
}
