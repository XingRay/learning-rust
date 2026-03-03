/// # Lesson 086 - FFI 与 C 互操作（概念讲解）
///
/// 本课讲解 Rust 与 C 语言互操作的核心概念。
///
/// ## 学习目标
/// - 理解 `extern "C"` 函数声明
/// - 掌握 `#[no_mangle]` 的用途
/// - 学会用 `extern "C"` 块调用 C 标准库函数
/// - 理解 `#[repr(C)]` 结构体布局
/// - 了解 CStr/CString 字符串转换概念
/// - 了解 bindgen 工具的作用
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson086_ffi_c_interop
/// ```

// =============================================================
// Lesson 086: FFI 与 C 互操作 - 跨越语言边界
// =============================================================

use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int};

// ---------------------------------------------------------
// 1. extern "C" 块：声明外部 C 函数
// ---------------------------------------------------------
// 使用 extern "C" 块来声明我们想调用的 C 标准库函数
// "C" 表示使用 C 语言的调用约定 (ABI)
//
// 注意：extern 块中声明的函数都隐式地是 unsafe 的，
// 因为 Rust 编译器无法验证外部代码的安全性
extern "C" {
    /// C 标准库的 abs 函数：返回整数的绝对值
    /// int abs(int n);
    fn abs(n: c_int) -> c_int;

    /// C 标准库的 sqrt 函数：返回平方根
    /// double sqrt(double x);
    fn sqrt(x: c_double) -> c_double;

    /// C 标准库的 strlen 函数：返回字符串长度
    /// size_t strlen(const char *s);
    fn strlen(s: *const c_char) -> usize;
}

// ---------------------------------------------------------
// 2. #[repr(C)] 结构体布局
// ---------------------------------------------------------
// 默认情况下，Rust 的结构体字段顺序和内存布局是不确定的
// （编译器可能重新排列字段以优化内存对齐）
//
// #[repr(C)] 告诉编译器按照 C 语言的规则来布局结构体：
// - 字段按声明顺序排列
// - 遵循 C 的对齐规则

/// C 兼容的点结构体
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CPoint {
    x: f64,
    y: f64,
}

/// C 兼容的矩形结构体
#[repr(C)]
#[derive(Debug)]
struct CRect {
    origin: CPoint,
    width: f64,
    height: f64,
}

/// 普通 Rust 结构体（对比用）
#[derive(Debug)]
#[allow(dead_code)]
struct RustPoint {
    x: f64,
    y: f64,
}

// ---------------------------------------------------------
// 3. #[no_mangle] 和导出函数
// ---------------------------------------------------------
// 当我们要将 Rust 函数暴露给 C 代码调用时，需要：
// 1. #[no_mangle]: 防止 Rust 编译器修改函数名（name mangling）
// 2. extern "C": 使用 C 调用约定
//
// 以下函数可以被编译为动态/静态库后供 C 代码调用

/// 这个函数可以被 C 代码调用
/// 在实际的库项目中，这个函数会被导出
#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// 计算两点之间的距离（可被 C 调用）
#[no_mangle]
pub extern "C" fn point_distance(p1: CPoint, p2: CPoint) -> c_double {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    // 我们用纯 Rust 计算，不调用 C 的 sqrt
    (dx * dx + dy * dy).sqrt()
}

fn main() {
    println!("=== Lesson 086: FFI 与 C 互操作 ===\n");

    // ---------------------------------------------------------
    // 演示 1: 调用 C 标准库函数
    // ---------------------------------------------------------
    println!("--- 1. 调用 C 标准库函数 ---");

    // 调用 C 的 abs 函数
    unsafe {
        let result = abs(-42);
        println!("  C abs(-42) = {}", result);

        let result2 = abs(10);
        println!("  C abs(10) = {}", result2);
    }

    // 调用 C 的 sqrt 函数
    unsafe {
        let result = sqrt(144.0);
        println!("  C sqrt(144.0) = {}", result);

        let result2 = sqrt(2.0);
        println!("  C sqrt(2.0) = {:.6}", result2);
    }

    // 调用 C 的 strlen 函数
    unsafe {
        // 需要以 null 结尾的 C 字符串
        let c_str = CString::new("Hello, FFI!").expect("CString 创建失败");
        let len = strlen(c_str.as_ptr());
        println!("  C strlen(\"Hello, FFI!\") = {}", len);
    }

    // ---------------------------------------------------------
    // 演示 2: #[repr(C)] 结构体
    // ---------------------------------------------------------
    println!("\n--- 2. #[repr(C)] 结构体布局 ---");

    let p1 = CPoint { x: 1.0, y: 2.0 };
    let p2 = CPoint { x: 4.0, y: 6.0 };

    println!("  CPoint p1: {:?}", p1);
    println!("  CPoint p2: {:?}", p2);

    // 调用我们导出的 extern "C" 函数（在 Rust 内部也可以调用）
    let dist = point_distance(p1, p2);
    println!("  两点距离: {:.2}", dist);

    let rect = CRect {
        origin: CPoint { x: 0.0, y: 0.0 },
        width: 10.0,
        height: 5.0,
    };
    println!("  CRect: {:?}", rect);

    // repr(C) 的内存布局是确定的
    println!("\n  内存布局对比:");
    println!("  CPoint (repr(C)) 大小: {} 字节", std::mem::size_of::<CPoint>());
    println!("  RustPoint (默认) 大小: {} 字节", std::mem::size_of::<RustPoint>());
    println!("  CRect (repr(C)) 大小: {} 字节", std::mem::size_of::<CRect>());

    // ---------------------------------------------------------
    // 演示 3: #[no_mangle] 导出的函数
    // ---------------------------------------------------------
    println!("\n--- 3. #[no_mangle] 导出的函数 ---");

    // 即使是 extern "C" 函数，在同一个 Rust 程序中也可以直接调用
    let sum = rust_add(10, 20);
    println!("  rust_add(10, 20) = {}", sum);
    println!("  这个函数因为 #[no_mangle]，编译后函数名保持为 'rust_add'");
    println!("  C 代码可以通过 'rust_add' 这个名字来链接和调用");

    // ---------------------------------------------------------
    // 演示 4: CStr 和 CString 字符串转换
    // ---------------------------------------------------------
    println!("\n--- 4. CStr / CString 字符串转换 ---");

    // CString: 拥有所有权的 C 风格字符串（Rust -> C）
    // 特点：以 null (\0) 结尾，不能包含内部 null 字节
    println!("  [Rust -> C] CString 示例:");

    let rust_string = String::from("Hello from Rust!");
    let c_string = CString::new(rust_string.clone()).expect("字符串中不能包含 null 字节");

    println!("    原始 Rust 字符串: \"{}\"", rust_string);
    println!("    CString 字节 (含 null): {:?}", c_string.as_bytes_with_nul());
    println!("    CString 指针: {:p}", c_string.as_ptr());

    // 包含 null 字节的字符串会导致 CString::new 失败
    let result = CString::new("hello\0world");
    println!("    包含 null 的字符串: {:?} (会失败)", result);

    // CStr: 借用的 C 风格字符串（C -> Rust）
    // 用于从 C 函数接收字符串
    println!("\n  [C -> Rust] CStr 示例:");

    // 模拟从 C 代码接收一个字符串指针
    let c_string = CString::new("Hello from C!").unwrap();
    let c_ptr: *const c_char = c_string.as_ptr();

    // 在实际 FFI 中，c_ptr 会来自 C 函数返回值
    unsafe {
        // 从裸指针创建 CStr（不获取所有权）
        let c_str = CStr::from_ptr(c_ptr);

        // 转换为 Rust 的 &str
        match c_str.to_str() {
            Ok(rust_str) => println!("    转换为 &str: \"{}\"", rust_str),
            Err(e) => println!("    UTF-8 转换失败: {}", e),
        }

        // 或者转换为 String（拥有所有权的拷贝）
        let owned_string = c_str.to_string_lossy().into_owned();
        println!("    转换为 String: \"{}\"", owned_string);
    }

    // 从字节切片创建 CStr（安全方式）
    let bytes_with_nul = b"Safe creation\0";
    let c_str = CStr::from_bytes_with_nul(bytes_with_nul).expect("必须以 null 结尾");
    println!("    从字节创建 CStr: {:?}", c_str);

    // ---------------------------------------------------------
    // 演示 5: FFI 类型映射
    // ---------------------------------------------------------
    println!("\n--- 5. C/Rust 类型映射表 ---");
    println!("  ┌───────────────────┬──────────────────────┐");
    println!("  │    C 类型         │    Rust 类型         │");
    println!("  ├───────────────────┼──────────────────────┤");
    println!("  │ int               │ c_int (i32)          │");
    println!("  │ unsigned int      │ c_uint (u32)         │");
    println!("  │ long              │ c_long               │");
    println!("  │ float             │ c_float (f32)        │");
    println!("  │ double            │ c_double (f64)       │");
    println!("  │ char              │ c_char (i8/u8)       │");
    println!("  │ void              │ ()                   │");
    println!("  │ void*             │ *mut c_void          │");
    println!("  │ const char*       │ *const c_char        │");
    println!("  │ size_t            │ usize                │");
    println!("  │ bool              │ bool (C99)           │");
    println!("  └───────────────────┴──────────────────────┘");

    // ---------------------------------------------------------
    // 演示 6: 回调函数（函数指针跨 FFI）
    // ---------------------------------------------------------
    println!("\n--- 6. 回调函数模式（函数指针跨 FFI）---");

    // 在实际 FFI 中，C 代码可能需要调用 Rust 提供的回调函数
    // 函数指针可以跨 FFI 边界传递

    // 定义一个符合 C ABI 的回调函数
    extern "C" fn my_callback(value: c_int) -> c_int {
        println!("    回调被调用，参数: {}", value);
        value * 2
    }

    // 模拟 C 代码调用回调
    let callback: extern "C" fn(c_int) -> c_int = my_callback;
    let result = callback(21);
    println!("    回调返回值: {}", result);

    // ---------------------------------------------------------
    // 说明: bindgen 工具
    // ---------------------------------------------------------
    println!("\n--- 7. bindgen 工具说明 ---");
    println!("  bindgen 是一个自动生成 Rust FFI 绑定的工具：");
    println!("  • 输入：C/C++ 头文件（.h）");
    println!("  • 输出：Rust 的 extern 声明和类型定义");
    println!("  • 安装：cargo install bindgen-cli");
    println!("  • 使用：bindgen input.h -o bindings.rs");
    println!("  • 也可以在 build.rs 中集成，编译时自动生成");
    println!();
    println!("  示例 build.rs:");
    println!("    extern crate bindgen;");
    println!("    fn main() {{");
    println!("        let bindings = bindgen::Builder::default()");
    println!("            .header(\"wrapper.h\")");
    println!("            .generate()");
    println!("            .expect(\"Unable to generate bindings\");");
    println!("        bindings.write_to_file(\"src/bindings.rs\").unwrap();");
    println!("    }}");
    println!();
    println!("  cbindgen 则相反：从 Rust 代码生成 C/C++ 头文件");

    println!("\n🎉 恭喜！你已经掌握了 FFI 与 C 互操作的核心概念！");
}
