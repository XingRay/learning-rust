// ============================================================
// Lesson 042: RefCell<T> 内部可变性
// ============================================================
// RefCell<T> 提供"内部可变性"（interior mutability）：
// 即使只有不可变引用，也能修改其内部的数据。
//
// Rust 的借用规则在编译时检查：
//   - 同一时刻，要么有一个可变引用，要么有多个不可变引用
// RefCell<T> 将这些检查推迟到运行时：
//   - 违反规则会导致 panic，而不是编译错误
//
// RefCell<T> 仅用于单线程场景。

use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    println!("=== Lesson 042: RefCell<T> 内部可变性 ===\n");

    // -------------------------------------------------------
    // 1. RefCell::new 基本用法
    // -------------------------------------------------------
    println!("--- 1. RefCell::new 基本用法 ---");

    let data = RefCell::new(vec![1, 2, 3]);
    println!("初始数据: {:?}", data.borrow()); // 不可变借用

    // 即使 data 本身是不可变绑定，也能修改内部数据
    data.borrow_mut().push(4);
    println!("添加元素后: {:?}", data.borrow());

    // RefCell 包裹的值可以随时修改
    *data.borrow_mut() = vec![10, 20, 30];
    println!("替换后: {:?}", data.borrow());

    println!();

    // -------------------------------------------------------
    // 2. borrow 与 borrow_mut
    // -------------------------------------------------------
    println!("--- 2. borrow 与 borrow_mut ---");

    let value = RefCell::new(String::from("hello"));

    // borrow() 返回 Ref<T> — 不可变借用
    {
        let borrowed = value.borrow();
        println!("不可变借用: {}", borrowed);
        // 可以同时有多个不可变借用
        let borrowed2 = value.borrow();
        println!("第二个不可变借用: {}", borrowed2);
        // borrowed 和 borrowed2 在此处被 drop
    }

    // borrow_mut() 返回 RefMut<T> — 可变借用
    {
        let mut borrowed_mut = value.borrow_mut();
        borrowed_mut.push_str(", world!");
        println!("可变借用修改后: {}", borrowed_mut);
        // borrowed_mut 在此处被 drop
    }

    println!("最终值: {}", value.borrow());

    println!();

    // -------------------------------------------------------
    // 3. 运行时借用检查（panic 场景说明）
    // -------------------------------------------------------
    println!("--- 3. 运行时借用检查 ---");

    // ⚠️ 以下场景会导致 panic（已注释掉以避免程序崩溃）
    //
    // 场景 1：同时存在不可变借用和可变借用
    // let data = RefCell::new(5);
    // let r1 = data.borrow();     // 不可变借用
    // let r2 = data.borrow_mut(); // panic! 已有不可变借用时尝试可变借用
    //
    // 场景 2：同时存在多个可变借用
    // let data = RefCell::new(5);
    // let r1 = data.borrow_mut(); // 可变借用
    // let r2 = data.borrow_mut(); // panic! 已有可变借用时再次可变借用

    // 安全的做法：使用 try_borrow / try_borrow_mut 避免 panic
    let safe_data = RefCell::new(42);

    let borrow1 = safe_data.borrow();
    match safe_data.try_borrow_mut() {
        Ok(_) => println!("成功获取可变借用"),
        Err(e) => println!("获取可变借用失败（符合预期）: {}", e),
    }
    drop(borrow1); // 释放不可变借用

    // 现在可以安全地获取可变借用
    match safe_data.try_borrow_mut() {
        Ok(mut val) => {
            *val = 100;
            println!("成功修改值为: {}", val);
        }
        Err(e) => println!("获取可变借用失败: {}", e),
    }

    // 多个不可变借用是允许的
    let r1 = safe_data.borrow();
    let r2 = safe_data.borrow();
    println!("多个不可变借用: r1={}, r2={}", r1, r2);
    drop(r1);
    drop(r2);

    println!();

    // -------------------------------------------------------
    // 4. Rc<RefCell<T>> 组合模式
    // -------------------------------------------------------
    println!("--- 4. Rc<RefCell<T>> 组合模式 ---");

    // Rc 提供共享所有权，RefCell 提供内部可变性
    // 组合起来就能实现"多个所有者都能修改数据"

    // 示例：多个观察者共享并修改一个计数器
    #[derive(Debug)]
    struct SharedCounter {
        count: Rc<RefCell<i32>>,
        name: String,
    }

    impl SharedCounter {
        fn new(name: &str, counter: Rc<RefCell<i32>>) -> Self {
            SharedCounter {
                count: counter,
                name: String::from(name),
            }
        }

        fn increment(&self) {
            *self.count.borrow_mut() += 1;
            println!(
                "{} 增加计数，当前值: {}",
                self.name,
                self.count.borrow()
            );
        }

        fn get_count(&self) -> i32 {
            *self.count.borrow()
        }
    }

    let counter = Rc::new(RefCell::new(0));

    let observer_a = SharedCounter::new("观察者A", Rc::clone(&counter));
    let observer_b = SharedCounter::new("观察者B", Rc::clone(&counter));
    let observer_c = SharedCounter::new("观察者C", Rc::clone(&counter));

    observer_a.increment(); // 1
    observer_b.increment(); // 2
    observer_c.increment(); // 3
    observer_a.increment(); // 4

    println!("最终计数: {}", observer_a.get_count());
    println!("Rc 引用计数: {}", Rc::strong_count(&counter));

    println!();

    // 更实际的例子：共享可变的图节点
    #[derive(Debug)]
    struct Node {
        value: i32,
        neighbors: Vec<Rc<RefCell<Node>>>,
    }

    impl Node {
        fn new(value: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node {
                value,
                neighbors: vec![],
            }))
        }

        fn add_neighbor(node: &Rc<RefCell<Node>>, neighbor: &Rc<RefCell<Node>>) {
            node.borrow_mut().neighbors.push(Rc::clone(neighbor));
        }
    }

    let node_a = Node::new(1);
    let node_b = Node::new(2);
    let node_c = Node::new(3);

    // 构建连接关系
    Node::add_neighbor(&node_a, &node_b);
    Node::add_neighbor(&node_a, &node_c);
    Node::add_neighbor(&node_b, &node_c);

    println!(
        "节点A(值={})的邻居数: {}",
        node_a.borrow().value,
        node_a.borrow().neighbors.len()
    );
    println!(
        "节点B(值={})的邻居数: {}",
        node_b.borrow().value,
        node_b.borrow().neighbors.len()
    );

    // 修改节点的值
    node_a.borrow_mut().value = 100;
    println!("修改后节点A的值: {}", node_a.borrow().value);

    println!();

    // -------------------------------------------------------
    // 5. Cell<T> — 更简单的内部可变性
    // -------------------------------------------------------
    println!("--- 5. Cell<T> ---");

    // Cell<T> 适用于 Copy 类型，通过整体替换值来实现内部可变性
    // 不需要借用，因此没有运行时借用检查的开销

    let cell = Cell::new(42);
    println!("Cell 初始值: {}", cell.get());

    cell.set(100);
    println!("Cell 设置后: {}", cell.get());

    // Cell 的一个实际用途：在不可变结构体中追踪状态
    struct Logger {
        message: String,
        call_count: Cell<u32>, // 可以在 &self 方法中修改
    }

    impl Logger {
        fn new(message: &str) -> Self {
            Logger {
                message: String::from(message),
                call_count: Cell::new(0),
            }
        }

        fn log(&self) {
            // 注意：这是 &self，不是 &mut self
            // 但仍然可以修改 call_count
            self.call_count.set(self.call_count.get() + 1);
            println!(
                "[第{}次调用] {}",
                self.call_count.get(),
                self.message
            );
        }
    }

    let logger = Logger::new("系统启动");
    logger.log(); // 第1次
    logger.log(); // 第2次
    logger.log(); // 第3次
    println!("总调用次数: {}", logger.call_count.get());

    println!();

    // -------------------------------------------------------
    // 6. Cell vs RefCell 对比
    // -------------------------------------------------------
    println!("--- 6. Cell vs RefCell 对比 ---");
    println!("┌──────────────┬──────────────────┬───────────────────────┐");
    println!("│ 特性         │ Cell<T>          │ RefCell<T>            │");
    println!("├──────────────┼──────────────────┼───────────────────────┤");
    println!("│ 适用类型     │ Copy 类型        │ 任意类型              │");
    println!("│ 获取值       │ get() 返回副本   │ borrow() 返回引用     │");
    println!("│ 修改值       │ set() 整体替换   │ borrow_mut() 可变引用 │");
    println!("│ 运行时检查   │ 无               │ 有（可能 panic）      │");
    println!("│ 性能         │ 更快             │ 稍慢                  │");
    println!("│ 线程安全     │ 否               │ 否                    │");
    println!("└──────────────┴──────────────────┴───────────────────────┘");

    println!("\n=== Lesson 042 完成 ===");
}
