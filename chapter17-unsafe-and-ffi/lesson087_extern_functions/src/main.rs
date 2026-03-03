/// # Lesson 087 - 外部函数与高级 unsafe 特性
///
/// 本课讲解外部函数的更多细节和 unsafe 的高级用法。
///
/// ## 学习目标
/// - 掌握 extern "C" fn 导出函数
/// - 理解不同 ABI（"C" / "stdcall" / "system"）
/// - 了解全局分配器概念
/// - 掌握 unsafe trait 实现（Send / Sync）
/// - 理解 static mut 全局可变变量的使用和风险
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson087_extern_functions
/// ```

// =============================================================
// Lesson 087: 外部函数与高级 unsafe 特性
// =============================================================

use std::os::raw::c_int;

fn main() {
    println!("=== Lesson 087: 外部函数与高级 unsafe 特性 ===\n");

    // ---------------------------------------------------------
    // 1. extern "C" fn 导出函数
    // ---------------------------------------------------------
    // 使用 extern "C" 可以定义遵循 C 调用约定的函数
    // 这些函数可以被 C 代码（或其他语言的 FFI）调用
    println!("--- 1. extern \"C\" fn 导出函数 ---");

    // 直接在 Rust 中调用 extern "C" 函数（也是合法的）
    let result = exported_add(10, 20);
    println!("  exported_add(10, 20) = {}", result);

    let result = exported_multiply(7, 6);
    println!("  exported_multiply(7, 6) = {}", result);

    // 带有 #[no_mangle] 的函数保持原名，不会被 Rust 名称修饰
    let greeting = get_greeting();
    println!("  get_greeting() = \"{}\"", greeting);

    // ---------------------------------------------------------
    // 2. 不同 ABI 类型
    // ---------------------------------------------------------
    // ABI (Application Binary Interface) 定义了函数的调用约定
    // 包括：参数如何传递、返回值如何传递、栈如何管理等
    println!("\n--- 2. 不同 ABI 类型 ---");

    println!("  常见 ABI 类型说明:");
    println!("  ┌──────────────┬──────────────────────────────────────────┐");
    println!("  │ ABI          │ 说明                                     │");
    println!("  ├──────────────┼──────────────────────────────────────────┤");
    println!("  │ \"Rust\"       │ 默认 Rust ABI，不保证稳定               │");
    println!("  │ \"C\"          │ C 语言调用约定，最常用的 FFI ABI        │");
    println!("  │ \"stdcall\"    │ Windows API 常用（Win32 API）           │");
    println!("  │ \"system\"     │ Windows 上等同 stdcall，其他平台等同 C  │");
    println!("  │ \"fastcall\"   │ 寄存器传参，某些平台优化调用            │");
    println!("  │ \"cdecl\"      │ C 声明调用约定                          │");
    println!("  └──────────────┴──────────────────────────────────────────┘");

    // extern "C" - C 调用约定（最常用）
    let r1 = c_abi_function(5, 3);
    println!("\n  extern \"C\" fn result: {}", r1);

    // extern "system" - 系统调用约定
    // 在 Windows 上等同于 "stdcall"，在其他平台等同于 "C"
    let r2 = system_abi_function(5, 3);
    println!("  extern \"system\" fn result: {}", r2);

    // 默认 Rust ABI（可以省略 extern 或写 extern "Rust"）
    let r3 = rust_abi_function(5, 3);
    println!("  Rust ABI fn result: {}", r3);

    // 函数指针也可以指定 ABI
    let fn_ptr: extern "C" fn(c_int, c_int) -> c_int = c_abi_function;
    println!("  通过函数指针调用: {}", fn_ptr(10, 5));

    // ---------------------------------------------------------
    // 3. 全局分配器概念
    // ---------------------------------------------------------
    println!("\n--- 3. 全局分配器概念 ---");

    println!("  Rust 使用全局分配器来管理堆内存分配:");
    println!("  • 默认使用系统分配器（malloc/free 的封装）");
    println!("  • 可以通过 #[global_allocator] 替换为自定义分配器");
    println!("  • 常用的替代分配器: jemalloc, mimalloc, wee_alloc");
    println!();
    println!("  自定义全局分配器的基本结构:");
    println!("    use std::alloc::{{GlobalAlloc, Layout}};");
    println!("    ");
    println!("    struct MyAllocator;");
    println!("    ");
    println!("    unsafe impl GlobalAlloc for MyAllocator {{");
    println!("        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {{ ... }}");
    println!("        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {{ ... }}");
    println!("    }}");
    println!("    ");
    println!("    #[global_allocator]");
    println!("    static ALLOCATOR: MyAllocator = MyAllocator;");

    // 演示使用标准库的 Layout 和 alloc API
    demonstrate_alloc_api();

    // ---------------------------------------------------------
    // 4. unsafe trait 实现（Send / Sync）
    // ---------------------------------------------------------
    println!("\n--- 4. unsafe trait 实现（Send / Sync）---");

    println!("  Send 和 Sync 是 Rust 最重要的两个 unsafe trait：");
    println!("  • Send: 类型可以安全地在线程间转移所有权");
    println!("  • Sync: 类型可以安全地被多个线程共享引用");
    println!("  • 大多数类型自动实现了 Send 和 Sync");
    println!("  • Rc<T> 没有实现 Send/Sync（只能单线程使用）");
    println!("  • 裸指针 *const T / *mut T 没有实现 Send/Sync");

    // 演示自定义类型实现 Send 和 Sync
    let wrapper = SafePointerWrapper::new(42);
    println!("\n  SafePointerWrapper 值: {}", wrapper.get());

    // 因为我们为 SafePointerWrapper 实现了 Send，它可以跨线程传递
    let handle = std::thread::spawn(move || {
        println!("  在新线程中访问: {}", wrapper.get());
        wrapper
    });

    let wrapper = handle.join().unwrap();
    println!("  回到主线程: {}", wrapper.get());

    // 演示自定义 unsafe trait
    println!("\n  自定义 unsafe trait 示例:");
    let data = ThreadSafeCounter::new();
    data.describe();

    // ---------------------------------------------------------
    // 5. static mut 全局可变变量
    // ---------------------------------------------------------
    println!("\n--- 5. static mut 全局可变变量 ---");

    println!("  static mut 的特点和风险:");
    println!("  • 全局可变变量，程序的整个生命周期都存在");
    println!("  • 读取和写入都需要 unsafe 块");
    println!("  • 在多线程环境下可能导致数据竞争 (data race)");
    println!("  • 应尽量避免使用，优先考虑 Mutex/RwLock/Atomic");

    // 访问 static mut 需要 unsafe
    unsafe {
        let val = std::ptr::addr_of!(REQUEST_COUNT).read();
        println!("\n  REQUEST_COUNT (初始): {}", val);

        increment_request_count();
        increment_request_count();
        increment_request_count();

        let val = std::ptr::addr_of!(REQUEST_COUNT).read();
        println!("  REQUEST_COUNT (增加 3 次后): {}", val);
    }

    // static mut 的替代方案
    println!("\n  更安全的替代方案:");
    println!("  • std::sync::atomic::AtomicXxx - 原子类型");
    println!("  • std::sync::Mutex<T> - 互斥锁");
    println!("  • std::sync::RwLock<T> - 读写锁");
    println!("  • std::sync::OnceLock<T> - 一次性初始化");

    // 演示 Atomic 作为 static mut 的安全替代
    demonstrate_atomic_alternative();

    // ---------------------------------------------------------
    // 6. 综合示例：带有 unsafe 的完整模式
    // ---------------------------------------------------------
    println!("\n--- 6. 综合示例 ---");

    // 使用 extern "C" 函数作为排序的比较函数
    let mut numbers = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    println!("  排序前: {:?}", numbers);

    // 使用我们的 extern "C" 比较函数通过安全的 sort_by 排序
    numbers.sort_by(|a, b| {
        let result = compare_ints(*a, *b);
        if result < 0 {
            std::cmp::Ordering::Less
        } else if result > 0 {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    println!("  排序后: {:?}", numbers);

    // 演示 unsafe 与安全代码的正确边界
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum = safe_parallel_sum(&data);
    println!("  安全并行求和: {}", sum);

    println!("\n🎉 恭喜！你已经掌握了外部函数与高级 unsafe 特性！");
}

// =============================================================
// 辅助定义
// =============================================================

// ---------------------------------------------------------
// extern "C" fn 导出函数
// ---------------------------------------------------------

/// 使用 C ABI 导出的加法函数
#[no_mangle]
pub extern "C" fn exported_add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// 使用 C ABI 导出的乘法函数
#[no_mangle]
pub extern "C" fn exported_multiply(a: c_int, b: c_int) -> c_int {
    a * b
}

/// 返回问候语（演示 Rust 函数导出）
fn get_greeting() -> &'static str {
    "Hello from Rust extern function!"
}

// ---------------------------------------------------------
// 不同 ABI 的函数
// ---------------------------------------------------------

/// C 调用约定
extern "C" fn c_abi_function(a: c_int, b: c_int) -> c_int {
    a + b
}

/// 系统调用约定（Windows 上是 stdcall，其他平台是 C）
extern "system" fn system_abi_function(a: c_int, b: c_int) -> c_int {
    a + b
}

/// 默认 Rust 调用约定
fn rust_abi_function(a: c_int, b: c_int) -> c_int {
    a + b
}

/// extern "C" 的比较函数（可用于 C 的 qsort 等）
extern "C" fn compare_ints(a: i32, b: i32) -> c_int {
    a - b
}

// ---------------------------------------------------------
// 全局分配器演示
// ---------------------------------------------------------

fn demonstrate_alloc_api() {
    use std::alloc::{alloc, dealloc, Layout};

    println!("\n  手动内存分配演示:");

    unsafe {
        // 分配 4 字节，对齐到 4 字节边界
        let layout = Layout::new::<i32>();
        let ptr = alloc(layout) as *mut i32;

        if ptr.is_null() {
            println!("    内存分配失败！");
            return;
        }

        // 写入值
        *ptr = 12345;
        println!("    分配并写入值: {}", *ptr);
        println!("    分配的地址: {:p}", ptr);
        println!("    Layout 大小: {} 字节, 对齐: {} 字节", layout.size(), layout.align());

        // 释放内存
        dealloc(ptr as *mut u8, layout);
        println!("    内存已释放");
    }
}

// ---------------------------------------------------------
// unsafe trait 实现
// ---------------------------------------------------------

/// 一个包装裸指针的类型
/// 默认不实现 Send 和 Sync（因为包含裸指针）
struct SafePointerWrapper {
    // 使用 Box 确保数据有效，但用裸指针存储
    // 实际上我们完全控制这个指针的生命周期
    ptr: *mut i32,
}

impl SafePointerWrapper {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        SafePointerWrapper {
            ptr: Box::into_raw(boxed),
        }
    }

    fn get(&self) -> i32 {
        // 安全：因为我们保证 ptr 在 drop 之前一直有效
        unsafe { *self.ptr }
    }
}

impl Drop for SafePointerWrapper {
    fn drop(&mut self) {
        // 必须手动释放 Box::into_raw 转换的内存
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

/// 我们可以保证 SafePointerWrapper 在线程间传递是安全的
/// 因为：
/// 1. 它独占其指向的数据（不与其他实例共享）
/// 2. i32 本身是 Send 的
///
/// # Safety
/// 实现者保证内部指针的独占访问
unsafe impl Send for SafePointerWrapper {}

/// 自定义 unsafe trait
unsafe trait ThreadSafe {
    fn describe(&self);
}

struct ThreadSafeCounter {
    count: std::sync::atomic::AtomicU32,
}

impl ThreadSafeCounter {
    fn new() -> Self {
        ThreadSafeCounter {
            count: std::sync::atomic::AtomicU32::new(0),
        }
    }
}

/// 实现自定义 unsafe trait
/// # Safety
/// ThreadSafeCounter 使用原子操作，天然线程安全
unsafe impl ThreadSafe for ThreadSafeCounter {
    fn describe(&self) {
        println!("  ThreadSafeCounter 实现了自定义 unsafe trait ThreadSafe");
        println!("  当前计数: {}", self.count.load(std::sync::atomic::Ordering::Relaxed));
    }
}

// ---------------------------------------------------------
// static mut
// ---------------------------------------------------------

/// 全局可变变量 —— 需要 unsafe 来访问
/// 警告：在多线程环境下可能导致数据竞争
static mut REQUEST_COUNT: u64 = 0;

/// 增加请求计数（unsafe 函数）
///
/// # Safety
/// 调用者必须确保没有其他线程同时访问 REQUEST_COUNT
unsafe fn increment_request_count() {
    let ptr = std::ptr::addr_of_mut!(REQUEST_COUNT);
    *ptr += 1;
}

// ---------------------------------------------------------
// Atomic 作为安全替代
// ---------------------------------------------------------

use std::sync::atomic::{AtomicU64, Ordering};

/// 使用 Atomic 的安全计数器（不需要 unsafe）
static SAFE_COUNTER: AtomicU64 = AtomicU64::new(0);

fn demonstrate_atomic_alternative() {
    println!("\n  Atomic 替代方案演示:");

    // 不需要 unsafe！
    SAFE_COUNTER.store(0, Ordering::SeqCst);

    for _ in 0..5 {
        SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    }

    let value = SAFE_COUNTER.load(Ordering::SeqCst);
    println!("  AtomicU64 计数器: {} (无需 unsafe!)", value);
}

// ---------------------------------------------------------
// 综合示例辅助函数
// ---------------------------------------------------------

/// 安全的并行求和（演示如何用安全 API 包装 unsafe 概念）
fn safe_parallel_sum(data: &[i32]) -> i32 {
    // 实际上这里不需要 unsafe，这正是 Rust 的优势——
    // 大多数情况下，标准库提供了安全的高层 API
    data.iter().sum()
}
