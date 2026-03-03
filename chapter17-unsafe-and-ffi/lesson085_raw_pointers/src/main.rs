/// # Lesson 085 - 裸指针 (Raw Pointers)
///
/// 本课深入讲解 Rust 中的裸指针 `*const T` 和 `*mut T`。
///
/// ## 学习目标
/// - 理解 `*const T`（不可变裸指针）和 `*mut T`（可变裸指针）
/// - 掌握从引用创建裸指针（安全操作）
/// - 掌握解引用裸指针（需要 unsafe）
/// - 学会使用指针运算 offset/add
/// - 了解 ptr::null 和 ptr::null_mut
/// - 理解裸指针与引用的区别
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson085_raw_pointers
/// ```

// =============================================================
// Lesson 085: 裸指针 - 直接操作内存地址
// =============================================================

use std::ptr;

fn main() {
    println!("=== Lesson 085: 裸指针 (Raw Pointers) ===\n");

    // ---------------------------------------------------------
    // 1. *const T 和 *mut T 基础
    // ---------------------------------------------------------
    // Rust 有两种裸指针：
    // - *const T：不可变裸指针（类似 C 的 const T*）
    // - *mut T：可变裸指针（类似 C 的 T*）
    //
    // 注意：这里的 * 是类型名的一部分，不是解引用操作符
    println!("--- 1. *const T 和 *mut T 基础 ---");

    let value: i32 = 42;
    let mut mutable_value: i32 = 100;

    // 不可变裸指针
    let const_ptr: *const i32 = &value;
    // 可变裸指针
    let mut_ptr: *mut i32 = &mut mutable_value;

    println!("  const_ptr 地址: {:p}", const_ptr);
    println!("  mut_ptr 地址: {:p}", mut_ptr);

    // 裸指针允许同时存在多个指向同一数据的指针（包括可变的）
    // 这在引用系统中是不允许的
    let another_const_ptr: *const i32 = &value;
    println!("  another_const_ptr 地址: {:p} (与 const_ptr 相同)", another_const_ptr);

    // ---------------------------------------------------------
    // 2. 从引用创建裸指针（安全操作）
    // ---------------------------------------------------------
    // 创建裸指针本身不需要 unsafe，因为创建指针不会造成危害
    // 只有解引用（读写指针指向的数据）才需要 unsafe
    println!("\n--- 2. 从引用创建裸指针（安全操作）---");

    let x = 10;
    let y = &x;

    // 方式 1: 通过 as 转换引用为裸指针
    let ptr1: *const i32 = &x as *const i32;
    // 方式 2: 隐式转换（Rust 可以自动将引用转为裸指针）
    let ptr2: *const i32 = &x;
    // 方式 3: 从另一个引用创建
    let ptr3: *const i32 = y;

    println!("  ptr1: {:p}", ptr1);
    println!("  ptr2: {:p}", ptr2);
    println!("  ptr3: {:p}", ptr3);
    println!("  三个指针指向同一地址: {}", ptr1 == ptr2 && ptr2 == ptr3);

    // 从可变引用创建可变裸指针
    let mut z = 99;
    let mut_ptr: *mut i32 = &mut z;
    println!("  mut_ptr: {:p}", mut_ptr);

    // 也可以创建指向特定地址的指针（不推荐，仅做演示）
    // 注意：创建是安全的，但解引用可能导致未定义行为
    let arbitrary_ptr = 0x12345usize as *const i32;
    println!("  任意地址指针: {:p}（创建是安全的，但不能解引用）", arbitrary_ptr);

    // ---------------------------------------------------------
    // 3. 解引用裸指针（需要 unsafe）
    // ---------------------------------------------------------
    // 解引用是通过指针读取或写入数据的操作
    // 这需要 unsafe，因为 Rust 无法保证指针的有效性
    println!("\n--- 3. 解引用裸指针（需要 unsafe）---");

    let mut data = 42;
    let ptr_read: *const i32 = &data;
    let ptr_write: *mut i32 = &mut data;

    unsafe {
        // 读取
        println!("  通过 *const 读取: {}", *ptr_read);

        // 写入
        *ptr_write = 100;
        println!("  通过 *mut 写入后: {}", *ptr_write);
    }
    println!("  data 的最终值: {}", data);

    // 使用 read/write 方法（更推荐的方式）
    let mut val = 77;
    let p = &mut val as *mut i32;
    unsafe {
        let read_val = ptr::read(p);
        println!("  ptr::read 读取: {}", read_val);

        ptr::write(p, 88);
        println!("  ptr::write 写入后: {}", ptr::read(p));
    }

    // ---------------------------------------------------------
    // 4. 指针运算: offset / add / sub
    // ---------------------------------------------------------
    // 裸指针支持算术运算，可以在内存中移动指针
    println!("\n--- 4. 指针运算: offset / add / sub ---");

    let arr = [10, 20, 30, 40, 50];
    let base_ptr: *const i32 = arr.as_ptr();

    println!("  数组: {:?}", arr);
    println!("  基地址: {:p}", base_ptr);

    unsafe {
        // add(n): 向前移动 n 个元素（不是字节！）
        for i in 0..arr.len() {
            let elem_ptr = base_ptr.add(i);
            println!("  arr[{}] 地址: {:p}, 值: {}", i, elem_ptr, *elem_ptr);
        }

        println!();

        // offset(n): 与 add 类似，但参数是 isize（可以为负）
        let mid_ptr = base_ptr.add(2); // 指向 arr[2]
        println!("  mid_ptr 指向: {} (arr[2])", *mid_ptr);

        let prev = mid_ptr.offset(-1); // 向前移动一个元素
        println!("  mid_ptr.offset(-1): {} (arr[1])", *prev);

        let next = mid_ptr.offset(1);  // 向后移动一个元素
        println!("  mid_ptr.offset(1): {} (arr[3])", *next);

        // sub(n): 向前（地址减小方向）移动 n 个元素
        let end_ptr = base_ptr.add(4); // 指向 arr[4]
        let back2 = end_ptr.sub(2);    // 退回 2 个元素
        println!("  end_ptr.sub(2): {} (arr[2])", *back2);
    }

    // 使用指针遍历数组
    println!("\n  使用指针遍历数组:");
    let data = [100, 200, 300, 400];
    let ptr = data.as_ptr();
    let len = data.len();

    unsafe {
        let mut current = ptr;
        let end = ptr.add(len);
        while current < end {
            print!("  {} ", *current);
            current = current.add(1);
        }
        println!();
    }

    // ---------------------------------------------------------
    // 5. ptr::null 和 ptr::null_mut
    // ---------------------------------------------------------
    // Rust 提供了创建空指针的方式（类似 C 的 NULL）
    println!("\n--- 5. ptr::null 和 ptr::null_mut ---");

    let null_const: *const i32 = ptr::null();
    let null_mut: *mut i32 = ptr::null_mut();

    println!("  null_const: {:p}", null_const);
    println!("  null_mut: {:p}", null_mut);

    // 检查指针是否为空
    println!("  null_const.is_null(): {}", null_const.is_null());
    println!("  null_mut.is_null(): {}", null_mut.is_null());

    let valid_ptr: *const i32 = &42;
    println!("  valid_ptr.is_null(): {}", valid_ptr.is_null());

    // 在使用指针前检查是否为空是良好实践
    fn safe_deref(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            // 注意：非空不代表一定有效，但在受控环境下这是常见模式
            unsafe { Some(*ptr) }
        }
    }

    println!("  safe_deref(valid_ptr): {:?}", safe_deref(valid_ptr));
    println!("  safe_deref(null_const): {:?}", safe_deref(null_const));

    // ---------------------------------------------------------
    // 6. 裸指针 vs 引用 对比
    // ---------------------------------------------------------
    println!("\n--- 6. 裸指针 vs 引用 对比 ---");
    println!("  ┌──────────────────┬────────────────────────┬──────────────────────┐");
    println!("  │     特性         │       引用 (&T/&mut T) │    裸指针 (*const/*mut)│");
    println!("  ├──────────────────┼────────────────────────┼──────────────────────┤");
    println!("  │ 可以为空         │       ✗ 不可以         │    ✓ 可以            │");
    println!("  │ 自动解引用       │       ✓ 支持           │    ✗ 不支持          │");
    println!("  │ 借用检查         │       ✓ 编译时检查     │    ✗ 无检查          │");
    println!("  │ 保证有效性       │       ✓ 保证有效       │    ✗ 不保证          │");
    println!("  │ 需要 unsafe 读写 │       ✗ 不需要         │    ✓ 需要            │");
    println!("  │ 可多个可变指针   │       ✗ 不允许         │    ✓ 允许            │");
    println!("  │ 指针运算         │       ✗ 不支持         │    ✓ 支持            │");
    println!("  │ 跨 FFI 边界     │       ✗ 不安全         │    ✓ 常用方式        │");
    println!("  └──────────────────┴────────────────────────┴──────────────────────┘");

    // 实际演示对比
    println!("\n  演示：引用不允许但裸指针允许的操作");

    let mut value = 10;

    // 引用系统：同时只能有一个 &mut
    // let r1 = &mut value;
    // let r2 = &mut value; // 编译错误！

    // 裸指针：可以同时有多个可变指针（但使用时需要自己保证安全）
    let p1: *mut i32 = &mut value;
    let p2: *mut i32 = p1; // 合法！

    unsafe {
        *p1 = 20;
        println!("  通过 p1 写入后: {}", *p2);
    }

    // ---------------------------------------------------------
    // 7. 综合示例：用裸指针实现 swap
    // ---------------------------------------------------------
    println!("\n--- 7. 综合示例：用裸指针实现 swap ---");

    let mut a = 111;
    let mut b = 222;
    println!("  交换前: a = {}, b = {}", a, b);

    raw_swap(&mut a, &mut b);
    println!("  交换后: a = {}, b = {}", a, b);

    // 使用裸指针实现的简单内存拷贝
    println!("\n  简单内存拷贝示例:");
    let src = [1, 2, 3, 4, 5];
    let mut dst = [0i32; 5];
    raw_copy(&src, &mut dst);
    println!("  源: {:?}", src);
    println!("  目标: {:?}", dst);

    println!("\n🎉 恭喜！你已经掌握了裸指针的核心知识！");
}

// ---------------------------------------------------------
// 辅助函数
// ---------------------------------------------------------

/// 使用裸指针实现 swap
fn raw_swap(a: &mut i32, b: &mut i32) {
    let pa: *mut i32 = a;
    let pb: *mut i32 = b;

    unsafe {
        let tmp = ptr::read(pa);
        ptr::copy_nonoverlapping(pb, pa, 1);
        ptr::write(pb, tmp);
    }
}

/// 使用裸指针实现简单的内存拷贝
fn raw_copy(src: &[i32], dst: &mut [i32]) {
    assert_eq!(src.len(), dst.len(), "源和目标长度必须相同");

    let src_ptr = src.as_ptr();
    let dst_ptr = dst.as_mut_ptr();
    let len = src.len();

    unsafe {
        // copy_nonoverlapping 类似 C 的 memcpy
        // 要求源和目标内存区域不重叠
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, len);
    }
}
