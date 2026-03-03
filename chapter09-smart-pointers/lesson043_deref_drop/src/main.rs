// ============================================================
// Lesson 043: Deref 与 Drop trait
// ============================================================
// Deref trait：允许自定义解引用行为，使智能指针像普通引用一样使用。
// Drop trait：允许自定义值离开作用域时的清理行为（析构函数）。
//
// 这两个 trait 是 Rust 智能指针的核心基础。

use std::fmt;
use std::ops::{Deref, DerefMut};

fn main() {
    println!("=== Lesson 043: Deref 与 Drop trait ===\n");

    // -------------------------------------------------------
    // 1. Deref trait 实现
    // -------------------------------------------------------
    println!("--- 1. Deref trait 实现 ---");

    // 首先回顾普通引用的解引用
    let x = 5;
    let y = &x;
    assert_eq!(5, *y); // * 解引用获取值
    println!("x = {}, *y = {}", x, *y);

    // 自定义智能指针，实现 Deref
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(value: T) -> MyBox<T> {
            MyBox(value)
        }
    }

    // 实现 Deref，使 MyBox 可以用 * 解引用
    impl<T> Deref for MyBox<T> {
        type Target = T; // 关联类型，指定解引用后的目标类型

        fn deref(&self) -> &T {
            &self.0 // 返回内部值的引用
        }
    }

    let my_box = MyBox::new(42);
    // *my_box 实际上被编译器转换为 *(my_box.deref())
    println!("MyBox 解引用: {}", *my_box);
    assert_eq!(42, *my_box);

    // 更复杂的例子：包裹字符串的智能指针
    struct SmartString {
        data: String,
        access_count: std::cell::Cell<u32>,
    }

    impl SmartString {
        fn new(s: &str) -> Self {
            SmartString {
                data: String::from(s),
                access_count: std::cell::Cell::new(0),
            }
        }

        fn access_count(&self) -> u32 {
            self.access_count.get()
        }
    }

    impl Deref for SmartString {
        type Target = String;

        fn deref(&self) -> &String {
            self.access_count.set(self.access_count.get() + 1);
            &self.data
        }
    }

    let smart = SmartString::new("Rust");
    println!("长度: {}", smart.len()); // 自动解引用调用 String::len
    println!("大写: {}", smart.to_uppercase()); // 自动解引用调用 String 的方法
    println!("访问次数: {}", smart.access_count());

    println!();

    // -------------------------------------------------------
    // 2. 自动解引用（Deref Coercion）
    // -------------------------------------------------------
    println!("--- 2. 自动解引用（Deref Coercion）---");

    // Deref coercion：当类型不匹配时，Rust 会自动插入 .deref() 调用链

    fn greet(name: &str) {
        println!("你好, {}!", name);
    }

    let boxed_string = Box::new(String::from("世界"));

    // 以下调用展示了自动解引用链：
    // &Box<String> -> &String -> &str
    greet(&boxed_string); // 自动完成两次解引用

    // 没有 deref coercion 的话，需要写成：
    greet(&(*boxed_string)[..]); // 手动解引用 Box，再取切片

    // MyBox 的自动解引用
    let my_str = MyBox::new(String::from("Hello"));
    greet(&my_str); // &MyBox<String> -> &String -> &str

    // 多层嵌套也能自动解引用
    let nested = Box::new(Box::new(String::from("嵌套")));
    greet(&nested); // &Box<Box<String>> -> &Box<String> -> &String -> &str

    // 函数参数的自动解引用
    fn first_char(s: &str) -> Option<char> {
        s.chars().next()
    }

    let boxed = Box::new(String::from("Rust 语言"));
    println!("第一个字符: {:?}", first_char(&boxed)); // 自动解引用

    println!();

    // -------------------------------------------------------
    // 3. DerefMut — 可变解引用
    // -------------------------------------------------------
    println!("--- 3. DerefMut ---");

    // DerefMut 允许可变解引用，需要先实现 Deref

    struct MutableBox<T> {
        value: T,
    }

    impl<T> MutableBox<T> {
        fn new(value: T) -> Self {
            MutableBox { value }
        }
    }

    impl<T> Deref for MutableBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.value
        }
    }

    impl<T> DerefMut for MutableBox<T> {
        fn deref_mut(&mut self) -> &mut T {
            &mut self.value
        }
    }

    let mut my_vec = MutableBox::new(vec![1, 2, 3]);
    println!("原始: {:?}", *my_vec);

    // DerefMut 允许通过 * 修改内部值
    my_vec.push(4); // 自动调用 deref_mut，然后调用 Vec::push
    my_vec.push(5);
    println!("修改后: {:?}", *my_vec);

    // 自动解引用规则总结：
    // 1. &T      -> &U  当 T: Deref<Target=U>    （不可变到不可变）
    // 2. &mut T  -> &mut U  当 T: DerefMut<Target=U>（可变到可变）
    // 3. &mut T  -> &U  当 T: Deref<Target=U>    （可变到不可变，自动降级）

    fn modify_vec(v: &mut Vec<i32>) {
        v.push(100);
    }

    modify_vec(&mut my_vec); // &mut MutableBox<Vec<i32>> -> &mut Vec<i32>
    println!("通过函数修改后: {:?}", *my_vec);

    println!();

    // -------------------------------------------------------
    // 4. Drop trait — 自定义析构
    // -------------------------------------------------------
    println!("--- 4. Drop trait ---");

    struct Resource {
        name: String,
    }

    impl Resource {
        fn new(name: &str) -> Self {
            println!("  [创建] 资源 '{}' 已分配", name);
            Resource {
                name: String::from(name),
            }
        }
    }

    impl Drop for Resource {
        fn drop(&mut self) {
            println!("  [释放] 资源 '{}' 已清理", self.name);
        }
    }

    println!("进入作用域...");
    {
        let _r1 = Resource::new("数据库连接");
        let _r2 = Resource::new("文件句柄");
        let _r3 = Resource::new("网络套接字");
        println!("  作用域内使用资源...");
        // 注意：drop 的顺序与创建顺序相反（LIFO，类似栈）
    }
    println!("离开作用域");

    println!();

    // 更实际的 Drop 示例：自定义智能指针
    struct CustomSmartPointer<T: fmt::Debug> {
        data: T,
        label: String,
    }

    impl<T: fmt::Debug> CustomSmartPointer<T> {
        fn new(label: &str, data: T) -> Self {
            println!("  SmartPointer[{}] 创建，数据: {:?}", label, data);
            CustomSmartPointer {
                data,
                label: String::from(label),
            }
        }
    }

    impl<T: fmt::Debug> Drop for CustomSmartPointer<T> {
        fn drop(&mut self) {
            println!(
                "  SmartPointer[{}] 被释放，数据: {:?}",
                self.label, self.data
            );
        }
    }

    println!("创建智能指针...");
    {
        let _sp1 = CustomSmartPointer::new("A", vec![1, 2, 3]);
        let _sp2 = CustomSmartPointer::new("B", "hello");
        println!("  使用中...");
    }
    println!("智能指针已释放");

    println!();

    // -------------------------------------------------------
    // 5. std::mem::drop — 提前释放
    // -------------------------------------------------------
    println!("--- 5. std::mem::drop 提前释放 ---");

    // 有时需要在作用域结束之前释放资源
    // 不能直接调用 .drop() 方法（编译器不允许，会导致 double free）
    // 应该使用 std::mem::drop 函数（也可以直接写 drop）

    struct Lock {
        name: String,
    }

    impl Lock {
        fn new(name: &str) -> Self {
            println!("  🔒 获取锁: {}", name);
            Lock {
                name: String::from(name),
            }
        }
    }

    impl Drop for Lock {
        fn drop(&mut self) {
            println!("  🔓 释放锁: {}", self.name);
        }
    }

    let lock1 = Lock::new("mutex_a");
    println!("  持有 lock1 做一些工作...");

    // 提前释放锁，让其他代码可以获取
    drop(lock1); // 等同于 std::mem::drop(lock1)
    println!("  lock1 已提前释放");

    // 此时 lock1 已不可用（所有权已移走）
    // println!("{}", lock1.name); // 编译错误！

    let lock2 = Lock::new("mutex_a"); // 可以重新获取
    println!("  重新获取锁，做更多工作...");
    drop(lock2);

    println!();

    // Drop 的实际应用场景
    println!("--- Drop 的常见应用场景 ---");
    println!("1. 释放堆内存（Box, Vec, String 等）");
    println!("2. 关闭文件句柄");
    println!("3. 释放互斥锁（MutexGuard）");
    println!("4. 关闭网络连接");
    println!("5. 刷新缓冲区（如 BufWriter）");
    println!("6. 向管道发送关闭信号");

    println!();

    // -------------------------------------------------------
    // 6. 综合示例：带引用计数的资源管理器
    // -------------------------------------------------------
    println!("--- 6. 综合示例 ---");

    struct ManagedResource {
        name: String,
        data: Vec<u8>,
    }

    impl ManagedResource {
        fn new(name: &str, size: usize) -> Self {
            println!("  分配 {} 字节给 '{}'", size, name);
            ManagedResource {
                name: String::from(name),
                data: vec![0; size],
            }
        }
    }

    impl Deref for ManagedResource {
        type Target = Vec<u8>;
        fn deref(&self) -> &Vec<u8> {
            &self.data
        }
    }

    impl DerefMut for ManagedResource {
        fn deref_mut(&mut self) -> &mut Vec<u8> {
            &mut self.data
        }
    }

    impl Drop for ManagedResource {
        fn drop(&mut self) {
            println!(
                "  释放 {} 字节 ('{}' 被清理)",
                self.data.len(),
                self.name
            );
        }
    }

    {
        let mut resource = ManagedResource::new("缓冲区", 1024);

        // 通过 Deref 和 DerefMut 像操作 Vec 一样操作
        resource[0] = 42;
        resource[1] = 128;
        println!("  数据长度: {}", resource.len()); // Deref -> Vec::len
        println!("  前两个字节: [{}, {}]", resource[0], resource[1]);
    }
    // resource 被 drop，打印释放信息

    println!("\n=== Lesson 043 完成 ===");
}
