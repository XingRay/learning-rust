// ============================================================
// Lesson 050: Send 与 Sync Trait
// ============================================================
// 本课介绍 Rust 并发安全的基石 — Send 和 Sync 两个标记 trait：
// - Send trait：可以在线程间转移所有权
// - Sync trait：可以在线程间共享引用（即 &T 是 Send）
// - 自动实现规则
// - Rc 为什么不是 Send
// - Arc 为什么是 Send + Sync
// - !Send / !Sync 类型
// ============================================================
// Send 和 Sync 是 Rust 类型系统保证线程安全的核心机制。
// 它们是自动推导的标记 trait（marker traits），
// 编译器在编译时就能检测出线程安全问题。
// ============================================================

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

fn main() {
    println!("===== Lesson 050: Send 与 Sync =====\n");

    // ---------------------------------------------------------
    // 1. Send Trait — 跨线程转移所有权
    // ---------------------------------------------------------
    // Send 表示一个类型的值可以安全地从一个线程移动到另一个线程。
    // 几乎所有的 Rust 类型都是 Send 的。
    println!("--- 1. Send Trait ---");

    // String 是 Send 的，可以 move 到另一个线程
    let s = String::from("hello");
    let handle = thread::spawn(move || {
        // s 的所有权从主线程转移到了这个线程
        println!("  String 在新线程中: {}", s);
    });
    handle.join().unwrap();

    // Vec<T> 是 Send 的（当 T 是 Send 时）
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("  Vec 在新线程中: {:?}", v);
    });
    handle.join().unwrap();

    // Box<T> 是 Send 的（当 T 是 Send 时）
    let boxed = Box::new(42);
    let handle = thread::spawn(move || {
        println!("  Box 在新线程中: {}", boxed);
    });
    handle.join().unwrap();

    println!("  ✓ String, Vec, Box 等都是 Send 的");

    // ---------------------------------------------------------
    // 2. Sync Trait — 跨线程共享引用
    // ---------------------------------------------------------
    // Sync 表示 &T 可以安全地在多个线程间共享。
    // 等价地说：T 是 Sync 当且仅当 &T 是 Send。
    //
    // 如果一个类型是 Sync 的，那么多个线程可以同时持有它的不可变引用。
    println!("\n--- 2. Sync Trait ---");

    // i32 是 Sync 的，多个线程可以同时读取它的引用
    let number = 42;
    thread::scope(|s| {
        s.spawn(|| {
            println!("  线程1 读取 &i32: {}", &number);
        });
        s.spawn(|| {
            println!("  线程2 读取 &i32: {}", &number);
        });
    });

    // Mutex<T> 是 Sync 的（这是它存在的意义之一）
    // 多个线程可以同时持有 &Mutex<T>，并通过 lock() 安全地访问内部数据
    let mutex = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            *mutex.lock().unwrap() += 1;
        });
        s.spawn(|| {
            *mutex.lock().unwrap() += 1;
        });
    });
    println!("  Mutex 是 Sync 的，结果: {}", *mutex.lock().unwrap());

    // ---------------------------------------------------------
    // 3. Rc<T> 为什么不是 Send（也不是 Sync）
    // ---------------------------------------------------------
    // Rc 使用非原子的引用计数，在多线程中并发修改计数会导致数据竞争
    println!("\n--- 3. Rc<T> 不是 Send ---");

    // 以下代码如果取消注释会编译失败：
    // use std::rc::Rc;
    // let rc = Rc::new(5);
    // let handle = thread::spawn(move || {
    //     println!("{}", rc);
    // });
    // 编译错误: `Rc<i32>` cannot be sent between threads safely
    // 原因: Rc 的引用计数不是原子操作，多线程并发修改会导致竞争条件

    println!("  Rc<T> 不是 Send，因为：");
    println!("    - Rc 使用非原子引用计数");
    println!("    - 多线程并发 clone/drop 会导致计数不准确");
    println!("    - 可能导致内存泄漏或 use-after-free");
    println!("  编译器会阻止你把 Rc 移到另一个线程中 ✓");

    // ---------------------------------------------------------
    // 4. Arc<T> 为什么是 Send + Sync
    // ---------------------------------------------------------
    // Arc 使用原子引用计数，线程安全
    println!("\n--- 4. Arc<T> 是 Send + Sync ---");

    // Arc 可以在线程间移动（Send）
    let arc = Arc::new(String::from("shared data"));
    let arc_clone = Arc::clone(&arc);

    let handle = thread::spawn(move || {
        // arc_clone 的所有权移动到了这个线程（Send）
        println!("  线程中使用 Arc: {}", arc_clone);
    });
    handle.join().unwrap();

    // Arc 的引用可以在线程间共享（因为 Arc: Sync）
    let arc = Arc::new(vec![1, 2, 3]);
    thread::scope(|s| {
        // 多个线程同时持有 &Arc<Vec<i32>>
        s.spawn(|| println!("  线程1 通过 Arc 读取: {:?}", *arc));
        s.spawn(|| println!("  线程2 通过 Arc 读取: {:?}", *arc));
    });

    println!("  Arc<T> 是 Send + Sync，因为：");
    println!("    - 使用原子操作管理引用计数");
    println!("    - 多线程 clone/drop 是线程安全的");
    println!("    - 当 T: Send + Sync 时, Arc<T>: Send + Sync");

    // ---------------------------------------------------------
    // 5. 自动实现规则
    // ---------------------------------------------------------
    println!("\n--- 5. 自动实现规则 ---");

    println!("  Send/Sync 的自动推导规则：");
    println!("    1. 如果一个类型的所有字段都是 Send，那么该类型自动实现 Send");
    println!("    2. 如果一个类型的所有字段都是 Sync，那么该类型自动实现 Sync");
    println!("    3. 原始类型（i32, f64, bool 等）都是 Send + Sync");
    println!("    4. 只包含 Send+Sync 字段的 struct/enum 自动是 Send+Sync");

    // 自定义结构体自动实现 Send + Sync
    #[allow(dead_code)]
    struct MyData {
        value: i32,
        name: String,
        items: Vec<f64>,
    }

    let data = Arc::new(Mutex::new(MyData {
        value: 42,
        name: "test".to_string(),
        items: vec![1.0, 2.0, 3.0],
    }));

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let d = data_clone.lock().unwrap();
        println!("  自定义结构体 MyData 在线程中: value={}, name={}", d.value, d.name);
    });
    handle.join().unwrap();
    println!("  ✓ MyData 自动实现了 Send (所有字段都是 Send)");

    // ---------------------------------------------------------
    // 6. !Send 和 !Sync 类型
    // ---------------------------------------------------------
    println!("\n--- 6. !Send 和 !Sync 类型 ---");

    println!("  常见的 !Send 类型:");
    println!("    - Rc<T>       : 非原子引用计数，不能跨线程");
    println!("    - *const T    : 裸指针，没有线程安全保证");
    println!("    - *mut T      : 裸指针，没有线程安全保证");
    println!();
    println!("  常见的 !Sync 类型:");
    println!("    - Cell<T>     : 内部可变性，非线程安全");
    println!("    - RefCell<T>  : 内部可变性，非线程安全");
    println!("    - Rc<T>       : 既不是 Send 也不是 Sync");
    println!();
    println!("  线程安全的替代品:");
    println!("    - Rc<T>       → Arc<T>");
    println!("    - Cell<T>     → AtomicXxx");
    println!("    - RefCell<T>  → Mutex<T> / RwLock<T>");

    // RefCell 不是 Sync 的演示
    // 以下代码如果取消注释会编译失败：
    // let cell = RefCell::new(5);
    // thread::scope(|s| {
    //     s.spawn(|| {
    //         *cell.borrow_mut() += 1; // ❌ RefCell 不是 Sync
    //     });
    // });

    // 正确的做法：使用 Mutex
    let cell = Mutex::new(5);
    thread::scope(|s| {
        s.spawn(|| {
            *cell.lock().unwrap() += 1;
        });
        s.spawn(|| {
            *cell.lock().unwrap() += 1;
        });
    });
    println!("  用 Mutex 替代 RefCell: {}", *cell.lock().unwrap());

    // ---------------------------------------------------------
    // 7. 手动实现 Send / Sync（unsafe）
    // ---------------------------------------------------------
    // 在极少数情况下，你可能需要手动实现 Send/Sync
    // 这需要 unsafe，因为你在向编译器保证线程安全性
    println!("\n--- 7. 手动实现 Send/Sync (unsafe) ---");

    // 一个包含裸指针的结构体，默认不是 Send/Sync
    #[allow(dead_code)]
    struct RawWrapper {
        ptr: *mut i32,
    }

    // 我们可以手动声明它是 Send + Sync
    // ⚠️ 只有在你确信它确实是线程安全时才这样做！
    unsafe impl Send for RawWrapper {}
    unsafe impl Sync for RawWrapper {}

    println!("  unsafe impl Send/Sync 是手动承诺线程安全性");
    println!("  ⚠️ 仅在确信安全时使用，否则可能导致未定义行为！");

    // 更安全的做法：使用 PhantomData 标记不发送
    struct NotSendType {
        _data: i32,
        // PhantomData<*const ()> 使得这个类型自动变成 !Send + !Sync
        _marker: PhantomData<*const ()>,
    }

    let _not_send = NotSendType {
        _data: 42,
        _marker: PhantomData,
    };
    // 以下代码如果取消注释会编译失败：
    // thread::spawn(move || {
    //     println!("{}", _not_send._data); // ❌ NotSendType 不是 Send
    // });
    println!("  PhantomData<*const ()> 可以让类型变成 !Send + !Sync");

    // ---------------------------------------------------------
    // 8. 常见类型的 Send/Sync 一览表
    // ---------------------------------------------------------
    println!("\n--- 8. 常见类型 Send/Sync 一览表 ---");

    println!("  ┌──────────────────────┬──────┬──────┐");
    println!("  │ 类型                 │ Send │ Sync │");
    println!("  ├──────────────────────┼──────┼──────┤");
    println!("  │ i32, f64, bool       │  ✓   │  ✓   │");
    println!("  │ String, Vec<T>       │  ✓   │  ✓   │");
    println!("  │ Box<T>               │  ✓   │  ✓   │");
    println!("  │ Arc<T>               │  ✓   │  ✓   │");
    println!("  │ Mutex<T>             │  ✓   │  ✓   │");
    println!("  │ RwLock<T>            │  ✓   │  ✓   │");
    println!("  │ AtomicXxx            │  ✓   │  ✓   │");
    println!("  │ mpsc::Sender<T>      │  ✓   │  ✗   │");
    println!("  │ mpsc::Receiver<T>    │  ✓   │  ✗   │");
    println!("  │ Rc<T>                │  ✗   │  ✗   │");
    println!("  │ Cell<T>              │  ✓   │  ✗   │");
    println!("  │ RefCell<T>           │  ✓   │  ✗   │");
    println!("  │ *const T / *mut T    │  ✗   │  ✗   │");
    println!("  └──────────────────────┴──────┴──────┘");
    println!("  (T 需要满足对应的 Send/Sync 要求)");

    // ---------------------------------------------------------
    // 9. 实际应用：理解编译器的线程安全检查
    // ---------------------------------------------------------
    println!("\n--- 9. 编译器的线程安全检查 ---");

    // thread::spawn 的签名要求闭包是 Send 的
    // pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    // where
    //     F: FnOnce() -> T + Send + 'static,
    //     T: Send + 'static,
    //
    // 这意味着：
    // 1. 闭包本身必须是 Send（所有捕获的变量必须是 Send）
    // 2. 闭包的返回值必须是 Send
    // 3. 'static 生命周期：不能借用栈上的数据（除非用 thread::scope）

    // thread::scope 放宽了 'static 要求
    let local_data = vec![1, 2, 3];
    thread::scope(|s| {
        // 可以借用 local_data，不需要 'static
        s.spawn(|| {
            println!("  scope 中可以借用局部变量: {:?}", &local_data);
        });
    });

    // Arc<Mutex<T>> 的完整路径
    // Arc<T>: Send + Sync (当 T: Send + Sync)
    // Mutex<T>: Send + Sync (当 T: Send)
    // 所以 Arc<Mutex<T>> 是 Send + Sync (当 T: Send)
    let shared = Arc::new(Mutex::new(vec![1, 2, 3]));
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        // Arc<Mutex<Vec<i32>>> 满足 Send + 'static
        shared_clone.lock().unwrap().push(4);
    });
    handle.join().unwrap();
    println!("  Arc<Mutex<Vec>> 在线程间安全共享: {:?}", *shared.lock().unwrap());

    // Arc<RwLock<T>> 也是 Send + Sync (当 T: Send + Sync)
    let shared = Arc::new(RwLock::new(String::from("hello")));
    let shared_clone = Arc::clone(&shared);

    let handle = thread::spawn(move || {
        shared_clone.write().unwrap().push_str(" world");
    });
    handle.join().unwrap();
    println!("  Arc<RwLock<String>> 在线程间安全共享: {}", *shared.read().unwrap());

    // ---------------------------------------------------------
    // 10. 总结
    // ---------------------------------------------------------
    println!("\n--- 10. 总结 ---");
    println!("  Send 和 Sync 是 Rust 并发安全的基石：");
    println!("  • Send: 类型的值可以安全地移动到另一个线程");
    println!("  • Sync: 类型的不可变引用可以安全地在多个线程间共享");
    println!("  • 大多数类型自动实现 Send + Sync");
    println!("  • 编译器在编译时检查线程安全性 → 零运行时开销");
    println!("  • \"如果它能编译通过，那它就是线程安全的\" — 这就是 Rust 的承诺！");

    println!("\n===== Lesson 050 完成 =====");
}
