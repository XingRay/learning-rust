/// # Lesson 034 - 关联类型（Associated Types）
///
/// 关联类型是 trait 中定义的类型占位符，让 trait 更灵活、更清晰。
///
/// ## 学习目标
/// - 理解关联类型 `type Output` 的语法
/// - 掌握 Iterator trait 中的 Item 关联类型
/// - 区分关联类型与泛型参数的使用场景
/// - 学会通过 Add trait 实现运算符重载
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson034_associated_types
/// ```

// =============================================================
// Lesson 034: 关联类型 - Trait 中的类型占位符
// =============================================================

use std::fmt;
use std::ops::{Add, Mul, Neg};

// ---------------------------------------------------------
// 1. 关联类型基础
// ---------------------------------------------------------
// 关联类型在 trait 定义中使用 `type` 关键字声明。
// 实现者在 impl 块中指定具体类型。

/// 定义一个转换器 trait，将输入转换为输出
/// `Output` 就是关联类型
trait Converter {
    /// 关联类型：转换的输出类型
    type Output;

    /// 执行转换
    fn convert(&self) -> Self::Output;
}

/// 温度结构体（摄氏度）
#[derive(Debug)]
struct CelsiusTemp(f64);

/// 华氏度结构体
#[derive(Debug)]
struct FahrenheitTemp(f64);

/// 将摄氏度转换为华氏度
impl Converter for CelsiusTemp {
    type Output = FahrenheitTemp; // 指定关联类型为 FahrenheitTemp

    fn convert(&self) -> Self::Output {
        FahrenheitTemp(self.0 * 9.0 / 5.0 + 32.0)
    }
}

/// 将华氏度转换为摄氏度
impl Converter for FahrenheitTemp {
    type Output = CelsiusTemp; // 指定关联类型为 CelsiusTemp

    fn convert(&self) -> Self::Output {
        CelsiusTemp((self.0 - 32.0) * 5.0 / 9.0)
    }
}

/// 字符串转换为大写
impl Converter for String {
    type Output = String;

    fn convert(&self) -> Self::Output {
        self.to_uppercase()
    }
}

// ---------------------------------------------------------
// 2. Iterator trait —— 关联类型的经典应用
// ---------------------------------------------------------
// Iterator trait 的定义（标准库中）：
// pub trait Iterator {
//     type Item;  // 关联类型
//     fn next(&mut self) -> Option<Self::Item>;
// }

/// 斐波那契数列迭代器
struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

/// 为 Fibonacci 实现 Iterator，Item 为 u64
impl Iterator for Fibonacci {
    type Item = u64; // 每次迭代产出 u64

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current = self.next;
        self.next = result + self.next;
        Some(result)
    }
}

/// 范围计数器 —— 产出 (index, value) 元组
struct RangeCounter {
    start: i32,
    end: i32,
    current: i32,
}

impl RangeCounter {
    fn new(start: i32, end: i32) -> Self {
        RangeCounter {
            start,
            end,
            current: start,
        }
    }
}

impl Iterator for RangeCounter {
    type Item = (usize, i32); // 关联类型是元组

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let index = (self.current - self.start) as usize;
            let value = self.current;
            self.current += 1;
            Some((index, value))
        } else {
            None
        }
    }
}

// ---------------------------------------------------------
// 3. 关联类型 vs 泛型参数
// ---------------------------------------------------------
// 关键区别：
//   泛型参数：一个类型可以对同一 trait 实现多次（不同的类型参数）
//   关联类型：一个类型对同一 trait 只能实现一次

/// 使用泛型参数的 trait —— 同一类型可以有多种实现
trait ConvertTo<T> {
    fn convert_to(&self) -> T;
}

/// i32 可以转换为 f64
impl ConvertTo<f64> for i32 {
    fn convert_to(&self) -> f64 {
        *self as f64
    }
}

/// i32 也可以转换为 String
impl ConvertTo<String> for i32 {
    fn convert_to(&self) -> String {
        self.to_string()
    }
}

// 对比：
// 使用关联类型时：impl Iterator for Fibonacci { type Item = u64; }
//   → Fibonacci 只能有一种 Item 类型
//
// 如果 Iterator 用泛型：trait Iterator<T> { fn next(&mut self) -> Option<T>; }
//   → Fibonacci 可以同时是 Iterator<u64> 和 Iterator<String>
//   → 调用时需要指定要用哪个实现，很混乱
//
// 结论：当一个类型对 trait 只应有一种实现时，用关联类型更好

// ---------------------------------------------------------
// 4. 运算符重载 —— Add trait
// ---------------------------------------------------------
// std::ops::Add 的定义：
// pub trait Add<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }

/// 二维向量
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

/// 实现 Vec2 + Vec2
impl Add for Vec2 {
    type Output = Vec2; // 两个向量相加仍然是向量

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// 实现 Vec2 * f64（标量乘法）
impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Self::Output {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// 实现 -Vec2（取反）
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

// ---------------------------------------------------------
// 更多运算符重载的例子
// ---------------------------------------------------------

/// 表示毫米的类型
#[derive(Debug, Clone, Copy)]
struct Millimeters(f64);

/// 表示米的类型
#[derive(Debug, Clone, Copy)]
struct Meters(f64);

impl fmt::Display for Millimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}mm", self.0)
    }
}

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}m", self.0)
    }
}

/// 毫米 + 毫米 = 毫米
impl Add for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Millimeters) -> Self::Output {
        Millimeters(self.0 + rhs.0)
    }
}

/// 毫米 + 米 = 毫米（不同类型相加！）
/// 注意 Add<Rhs> 中的 Rhs 泛型参数
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000.0)
    }
}

// ---------------------------------------------------------
// 5. 带关联类型的 trait 作为约束
// ---------------------------------------------------------

/// 要求 Iterator 的 Item 必须实现 Display
fn print_first_n<I>(iter: I, n: usize)
where
    I: Iterator,
    I::Item: fmt::Display, // 约束关联类型
{
    for (i, item) in iter.enumerate().take(n) {
        println!("  [{}] {}", i, item);
    }
}

/// 要求 Converter 的 Output 必须实现 Debug
fn convert_and_debug<T>(item: &T)
where
    T: Converter + fmt::Debug,
    T::Output: fmt::Debug, // 约束关联类型
{
    println!("  输入: {:?}", item);
    println!("  输出: {:?}", item.convert());
}

fn main() {
    println!("=== Lesson 034: 关联类型 ===\n");

    // ---------------------------------------------------------
    // 演示 1: 基本关联类型
    // ---------------------------------------------------------
    println!("--- 1. 基本关联类型 ---");

    let celsius = CelsiusTemp(100.0);
    let fahrenheit = celsius.convert();
    println!("100°C = {:?}", fahrenheit);

    let fahrenheit = FahrenheitTemp(72.0);
    let celsius = fahrenheit.convert();
    println!("72°F = {:?}", celsius);

    let text = String::from("hello rust");
    let upper = text.convert();
    println!("\"{}\" → \"{}\"", text, upper);

    // ---------------------------------------------------------
    // 演示 2: Iterator 中的关联类型
    // ---------------------------------------------------------
    println!("\n--- 2. Iterator 的 Item 关联类型 ---");

    println!("斐波那契数列前 10 项:");
    let fib = Fibonacci::new();
    print_first_n(fib, 10);

    println!("\n范围计数器 (5..10):");
    let counter = RangeCounter::new(5, 10);
    for (idx, val) in counter {
        println!("  索引 {}: 值 {}", idx, val);
    }

    // Iterator 的链式调用也依赖关联类型
    println!("\n斐波那契数列 —— 前 8 项中的偶数:");
    let even_fibs: Vec<u64> = Fibonacci::new()
        .take(8)
        .filter(|n| n % 2 == 0)
        .collect();
    println!("  {:?}", even_fibs);

    // ---------------------------------------------------------
    // 演示 3: 关联类型 vs 泛型参数
    // ---------------------------------------------------------
    println!("\n--- 3. 关联类型 vs 泛型参数 ---");

    // 泛型参数版本：同一类型有多种实现，需要指定类型
    let num: i32 = 42;
    let as_f64: f64 = num.convert_to();
    let as_string: String = num.convert_to();
    println!("i32 → f64:    {}", as_f64);
    println!("i32 → String: {}", as_string);

    println!("\n📝 何时用关联类型 vs 泛型参数:");
    println!("   关联类型: 每个类型只有一种实现 (如 Iterator::Item)");
    println!("   泛型参数: 一个类型可有多种实现 (如 From<T>, Add<Rhs>)");

    // ---------------------------------------------------------
    // 演示 4: 运算符重载
    // ---------------------------------------------------------
    println!("\n--- 4. 运算符重载 (Add/Mul/Neg) ---");

    let v1 = Vec2::new(1.0, 2.0);
    let v2 = Vec2::new(3.0, 4.0);

    // Vec2 + Vec2
    let v3 = v1 + v2;
    println!("{} + {} = {}", v1, v2, v3);

    // Vec2 * f64
    let v4 = v1 * 3.0;
    println!("{} * 3.0 = {}", v1, v4);

    // -Vec2
    let v5 = -v1;
    println!("-{} = {}", v1, v5);

    // 向量长度
    println!("|{}| = {:.2}", v2, v2.magnitude());

    // 不同类型之间的运算符重载
    println!("\n不同类型的加法:");
    let mm1 = Millimeters(500.0);
    let mm2 = Millimeters(300.0);
    let m1 = Meters(1.5);

    println!("{} + {} = {}", mm1, mm2, mm1 + mm2);
    println!("{} + {} = {}", mm1, m1, mm1 + m1);

    // ---------------------------------------------------------
    // 演示 5: 约束关联类型
    // ---------------------------------------------------------
    println!("\n--- 5. 约束关联类型 ---");

    println!("转换并调试:");
    convert_and_debug(&CelsiusTemp(37.0));
    println!();
    convert_and_debug(&FahrenheitTemp(98.6));

    println!("\n打印迭代器（约束 Item: Display）:");
    let nums = vec![10, 20, 30, 40, 50];
    print_first_n(nums.into_iter(), 5);

    // ---------------------------------------------------------
    // 演示 6: 标准库中的关联类型示例
    // ---------------------------------------------------------
    println!("\n--- 6. 标准库中的关联类型 ---");

    println!("常见的使用关联类型的 trait:");
    println!("  Iterator       → type Item");
    println!("  Add<Rhs=Self>  → type Output");
    println!("  Sub<Rhs=Self>  → type Output");
    println!("  Mul<Rhs=Self>  → type Output");
    println!("  Deref          → type Target");
    println!("  IntoIterator   → type Item, type IntoIter");
    println!("  FromStr        → type Err");

    println!("\n🎉 恭喜！你已经完成了关联类型的学习！");
}
