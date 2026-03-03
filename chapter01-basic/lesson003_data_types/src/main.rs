/// # Lesson 003 - 数据类型
///
/// 本课学习 Rust 中所有基本数据类型。
///
/// ## 学习目标
/// - 掌握整数类型（有符号与无符号）
/// - 了解浮点类型（f32, f64）
/// - 理解布尔类型与字符类型
/// - 掌握复合类型：元组（tuple）和数组（array）
/// - 学会类型转换（as 关键字）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson003_data_types
/// ```

// =============================================================
// Lesson 003: 数据类型
// =============================================================

fn main() {
    println!("=== Lesson 003: 数据类型 ===\n");

    // ---------------------------------------------------------
    // 1. 整数类型
    // ---------------------------------------------------------
    println!("--- 1. 整数类型 ---");

    // 有符号整数（可以表示负数）
    // i8:   -128 到 127
    // i16:  -32,768 到 32,767
    // i32:  -2,147,483,648 到 2,147,483,647（默认）
    // i64:  约 ±9.2 × 10^18
    // i128: 约 ±1.7 × 10^38
    // isize: 取决于平台（32位或64位）

    let a: i8 = 127; // i8 最大值
    let b: i16 = 32_767; // 下划线分隔提高可读性
    let c: i32 = 2_147_483_647;
    let d: i64 = 9_223_372_036_854_775_807;
    let e: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let f: isize = 42; // 在 64 位系统上等同于 i64

    println!("i8   最大值: {}", a);
    println!("i16  最大值: {}", b);
    println!("i32  最大值: {}", c);
    println!("i64  最大值: {}", d);
    println!("i128 最大值: {}", e);
    println!("isize 示例值: {} (指针大小: {} 字节)", f, std::mem::size_of::<isize>());

    // 无符号整数（只能表示非负数）
    // u8:   0 到 255
    // u16:  0 到 65,535
    // u32:  0 到 4,294,967,295
    // u64:  约 1.8 × 10^19
    // u128: 约 3.4 × 10^38
    // usize: 取决于平台（常用于索引和长度）

    let g: u8 = 255;
    let h: u16 = 65_535;
    let i: u32 = 4_294_967_295;
    let j: u64 = 18_446_744_073_709_551_615;
    let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let l: usize = 100; // 常用于数组索引

    println!("\nu8   最大值: {}", g);
    println!("u16  最大值: {}", h);
    println!("u32  最大值: {}", i);
    println!("u64  最大值: {}", j);
    println!("u128 最大值: {}", k);
    println!("usize 示例值: {} (大小: {} 字节)", l, std::mem::size_of::<usize>());

    // 不同进制的整数字面量
    println!("\n不同进制表示:");
    let decimal = 98_222; // 十进制
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节字面量（仅 u8）

    println!("十进制: {}", decimal);
    println!("十六进制 0xff = {}", hex);
    println!("八进制 0o77 = {}", octal);
    println!("二进制 0b1111_0000 = {}", binary);
    println!("字节 b'A' = {}", byte);

    // 使用标准库获取各类型的最大/最小值
    println!("\n通过标准库获取范围:");
    println!("i8:  {} 到 {}", i8::MIN, i8::MAX);
    println!("u8:  {} 到 {}", u8::MIN, u8::MAX);
    println!("i32: {} 到 {}", i32::MIN, i32::MAX);

    // ---------------------------------------------------------
    // 2. 浮点类型
    // ---------------------------------------------------------
    println!("\n--- 2. 浮点类型 ---");

    // f32: 单精度（32位），约6-7位有效数字
    // f64: 双精度（64位），约15-16位有效数字（默认）

    let float64: f64 = 3.141_592_653_589_793; // 默认推断为 f64
    let float32: f32 = 3.141_592_7; // 显式标注 f32

    println!("f64: {}", float64);
    println!("f32: {}", float32);
    println!("f64 大小: {} 字节", std::mem::size_of::<f64>());
    println!("f32 大小: {} 字节", std::mem::size_of::<f32>());

    // 浮点运算
    let sum = 5.0 + 10.0;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.0;
    let quotient = 56.7 / 32.2;
    let remainder = 43.0_f64 % 5.0;

    println!("\n浮点运算:");
    println!("5.0 + 10.0 = {}", sum);
    println!("95.5 - 4.3 = {}", difference);
    println!("4.0 * 30.0 = {}", product);
    println!("56.7 / 32.2 = {}", quotient);
    println!("43.0 % 5.0 = {}", remainder);

    // 特殊浮点值
    let infinity = f64::INFINITY;
    let neg_infinity = f64::NEG_INFINITY;
    let nan = f64::NAN;

    println!("\n特殊浮点值:");
    println!("正无穷: {}", infinity);
    println!("负无穷: {}", neg_infinity);
    println!("NaN: {}", nan);
    println!("NaN == NaN? {}", nan == nan); // false！NaN 不等于自身
    println!("NaN 检测: {}", nan.is_nan());

    // 浮点精度问题（所有语言共有）
    println!("\n浮点精度问题:");
    println!("0.1 + 0.2 = {}", 0.1 + 0.2); // 不精确等于 0.3
    println!("0.1 + 0.2 == 0.3 ? {}", (0.1_f64 + 0.2 - 0.3).abs() < f64::EPSILON);

    // ---------------------------------------------------------
    // 3. 布尔类型
    // ---------------------------------------------------------
    println!("\n--- 3. 布尔类型 ---");

    let is_active: bool = true;
    let is_deleted = false; // 类型推断为 bool

    println!("is_active: {}", is_active);
    println!("is_deleted: {}", is_deleted);

    // 布尔运算
    let a_bool = true;
    let b_bool = false;

    println!("AND: true && false = {}", a_bool && b_bool);
    println!("OR:  true || false = {}", a_bool || b_bool);
    println!("NOT: !true = {}", !a_bool);

    // 布尔值占 1 字节
    println!("bool 大小: {} 字节", std::mem::size_of::<bool>());

    // 比较运算产生布尔值
    let x = 10;
    let y = 20;
    println!("\n{} > {} ? {}", x, y, x > y);
    println!("{} < {} ? {}", x, y, x < y);
    println!("{} == {} ? {}", x, y, x == y);
    println!("{} != {} ? {}", x, y, x != y);
    println!("{} >= {} ? {}", x, y, x >= y);
    println!("{} <= {} ? {}", x, y, x <= y);

    // ---------------------------------------------------------
    // 4. 字符类型
    // ---------------------------------------------------------
    println!("\n--- 4. 字符类型（char） ---");

    // Rust 的 char 是 Unicode 标量值，占 4 字节
    let letter = 'A';
    let chinese = '中';
    let emoji = '🦀'; // Rust 吉祥物 Ferris
    let heart = '❤';

    println!("英文字母: {}", letter);
    println!("中文字符: {}", chinese);
    println!("Emoji: {}", emoji);
    println!("特殊符号: {}", heart);

    // char 占 4 字节（32位），存储 Unicode 标量值
    println!("\nchar 大小: {} 字节", std::mem::size_of::<char>());

    // Unicode 转义
    let unicode_char = '\u{4e2d}'; // '中' 的 Unicode 编码
    println!("Unicode 转义 \\u{{4e2d}} = {}", unicode_char);

    // char 的一些方法
    let ch = 'a';
    println!("\n字符 '{}' 的方法:", ch);
    println!("  是字母? {}", ch.is_alphabetic());
    println!("  是数字? {}", ch.is_numeric());
    println!("  是大写? {}", ch.is_uppercase());
    println!("  转大写: {}", ch.to_uppercase().next().unwrap());

    let digit = '7';
    println!("字符 '{}' 是数字? {}", digit, digit.is_numeric());
    println!("字符 '{}' 转为数字: {}", digit, digit.to_digit(10).unwrap());

    // 注意区分 char 和 u8（字节）
    // 'A' 是 char 类型（4字节）
    // b'A' 是 u8 类型（1字节）

    // ---------------------------------------------------------
    // 5. 元组（Tuple）
    // ---------------------------------------------------------
    println!("\n--- 5. 元组（Tuple） ---");

    // 元组可以包含不同类型的值，长度固定
    let tup: (i32, f64, char) = (500, 6.4, '是');
    println!("元组: {:?}", tup);

    // 通过模式匹配（解构）获取元组元素
    let (tx, ty, tz) = tup;
    println!("解构: x={}, y={}, z={}", tx, ty, tz);

    // 通过索引访问元组元素（从 0 开始）
    println!("tup.0 = {}", tup.0);
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    // 单元元组（unit tuple），即空元组 ()
    // 函数没有返回值时，实际返回的就是 ()
    let unit: () = ();
    println!("单元类型: {:?}", unit);
    println!("() 大小: {} 字节", std::mem::size_of::<()>());

    // 嵌套元组
    let nested = ((1, 2), (3.0, 4.0), 'X');
    println!("嵌套元组: {:?}", nested);
    println!("访问嵌套: (nested.0).1 = {}", (nested.0).1);

    // 元组可以用于函数返回多个值（后面课程详讲）
    let (min, max) = min_max(3, 7);
    println!("min_max(3, 7) = ({}, {})", min, max);

    // ---------------------------------------------------------
    // 6. 数组（Array）
    // ---------------------------------------------------------
    println!("\n--- 6. 数组（Array） ---");

    // 数组：同类型元素，固定长度，分配在栈上
    let arr = [1, 2, 3, 4, 5];
    println!("数组: {:?}", arr);

    // 显式标注类型和长度
    let arr2: [i32; 5] = [10, 20, 30, 40, 50];
    println!("显式标注: {:?}", arr2);

    // 初始化相同值的数组
    let zeros = [0; 10]; // 创建包含 10 个 0 的数组
    println!("10 个零: {:?}", zeros);

    let filled = [42; 5]; // 5 个 42
    println!("5 个 42: {:?}", filled);

    // 通过索引访问
    println!("\narr[0] = {}", arr[0]);
    println!("arr[4] = {}", arr[4]);

    // 数组长度
    println!("数组长度: {}", arr.len());

    // 数组切片
    let slice = &arr[1..4]; // 获取索引 1 到 3 的切片
    println!("切片 [1..4]: {:?}", slice);

    // 遍历数组
    print!("遍历数组: ");
    for element in arr.iter() {
        print!("{} ", element);
    }
    println!();

    // 带索引遍历
    for (index, value) in arr.iter().enumerate() {
        if index > 0 {
            print!(", ");
        }
        print!("[{}]={}", index, value);
    }
    println!();

    // 可变数组
    let mut mutable_arr = [1, 2, 3];
    println!("\n修改前: {:?}", mutable_arr);
    mutable_arr[0] = 100;
    println!("修改后: {:?}", mutable_arr);

    // 数组的一些方法
    let numbers = [5, 2, 8, 1, 9, 3];
    println!("\nnumbers: {:?}", numbers);
    println!("包含 8? {}", numbers.contains(&8));
    println!("包含 7? {}", numbers.contains(&7));

    // 数组大小在编译时确定
    println!("数组占用: {} 字节", std::mem::size_of_val(&numbers));

    // 二维数组
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6],
    ];
    println!("\n二维数组:");
    for row in &matrix {
        println!("  {:?}", row);
    }
    println!("matrix[1][2] = {}", matrix[1][2]);

    // ---------------------------------------------------------
    // 7. 类型转换（as）
    // ---------------------------------------------------------
    println!("\n--- 7. 类型转换（as） ---");

    // 整数之间的转换
    let big: i32 = 256;
    let small: u8 = big as u8; // 截断：256 mod 256 = 0
    println!("i32 {} as u8 = {} (截断！)", big, small);

    let negative: i32 = -1;
    let unsigned: u32 = negative as u32;
    println!("i32 {} as u32 = {} (按位解释)", negative, unsigned);

    // 浮点到整数（截断小数部分）
    let float_val = 3.99;
    let int_val = float_val as i32;
    println!("f64 {} as i32 = {} (截断，不是四舍五入)", float_val, int_val);

    let negative_float = -3.7;
    let int_neg = negative_float as i32;
    println!("f64 {} as i32 = {}", negative_float, int_neg);

    // 整数到浮点
    let integer = 42;
    let float = integer as f64;
    println!("i32 {} as f64 = {}", integer, float);

    // char 与整数之间的转换
    let ch = 'A';
    let ascii = ch as u32;
    println!("char '{}' as u32 = {}", ch, ascii);

    let num = 66u8;
    let character = num as char;
    println!("u8 {} as char = '{}'", num, character);

    // 安全转换：使用 From/Into trait（推荐方式）
    let small_val: i32 = 42;
    let big_val: i64 = i64::from(small_val); // 无损转换
    println!("i32 {} -> i64 {} (From trait)", small_val, big_val);

    let another: i64 = small_val.into(); // 同上，使用 Into trait
    println!("i32 {} -> i64 {} (Into trait)", small_val, another);

    // ---------------------------------------------------------
    // 8. 小结
    // ---------------------------------------------------------
    println!("\n--- 小结 ---");
    println!("✅ 整数: i8/i16/i32/i64/i128/isize 和 u8/.../usize");
    println!("✅ 浮点: f32（单精度）、f64（双精度，默认）");
    println!("✅ 布尔: bool，值为 true 或 false");
    println!("✅ 字符: char，4 字节 Unicode 标量值");
    println!("✅ 元组: 固定长度，可包含不同类型");
    println!("✅ 数组: 固定长度，元素类型相同，栈上分配");
    println!("✅ 类型转换: as 关键字（可能截断），From/Into（安全）");

    println!("\n🎉 恭喜！你已经完成了第三课！");
}

/// 辅助函数：返回两个数的最小值和最大值
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}
