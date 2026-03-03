#![allow(dead_code)]

/// # Lesson 035 - Newtype 模式
///
/// Newtype 模式是 Rust 中的重要设计模式，使用元组结构体包装类型。
///
/// ## 学习目标
/// - 理解 Newtype 模式（用元组结构体包装类型）
/// - 学会绕过孤儿规则为外部类型实现外部 trait
/// - 掌握类型安全（区分语义不同的相同底层类型）
/// - 了解通过 Deref 实现透明访问
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson035_newtype_pattern
/// ```

// =============================================================
// Lesson 035: Newtype 模式 - 元组结构体的妙用
// =============================================================

use std::fmt;
use std::ops::{Add, Deref, DerefMut};

// ---------------------------------------------------------
// 1. Newtype 基础 —— 元组结构体包装
// ---------------------------------------------------------
// Newtype 模式就是创建一个只有一个字段的元组结构体，
// 用来包装已有的类型。在运行时没有任何开销（零成本抽象）。

/// 包装 Vec<String>，使其可以实现外部 trait
struct Wrapper(Vec<String>);

/// 包装 String，赋予它特定的语义
struct EmailAddress(String);

impl EmailAddress {
    /// 创建邮箱地址（带简单验证）
    fn new(email: &str) -> Result<Self, String> {
        if email.contains('@') && email.contains('.') {
            Ok(EmailAddress(email.to_string()))
        } else {
            Err(format!("无效的邮箱地址: {}", email))
        }
    }

    /// 获取域名部分
    fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }

    /// 获取用户名部分
    fn username(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------
// 2. 绕过孤儿规则
// ---------------------------------------------------------
// 孤儿规则：不能为外部类型实现外部 trait
// 但用 Newtype 包装后，包装类型是我们的，就可以实现了！

// ❌ 不可以直接这样做（Vec 和 Display 都不是我们定义的）：
// impl fmt::Display for Vec<String> { ... }

// ✅ 用 Newtype 包装后就可以了！
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// 另一个例子：为 Vec<i32> 实现自定义 trait
struct IntList(Vec<i32>);

impl IntList {
    fn new() -> Self {
        IntList(Vec::new())
    }

    fn sum(&self) -> i32 {
        self.0.iter().sum()
    }

    fn average(&self) -> f64 {
        if self.0.is_empty() {
            0.0
        } else {
            self.sum() as f64 / self.0.len() as f64
        }
    }
}

impl fmt::Display for IntList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.0.iter().map(|n| n.to_string()).collect();
        write!(f, "[{}]", items.join(", "))
    }
}

// ---------------------------------------------------------
// 3. 类型安全 —— 区分语义不同的相同底层类型
// ---------------------------------------------------------
// Newtype 最强大的用途之一：让编译器帮你区分语义不同的值

/// 米（长度单位）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

/// 千米（长度单位）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilometers(f64);

/// 秒（时间单位）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Seconds(f64);

/// 米/秒（速度单位）
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct MetersPerSecond(f64);

// 为 Meters 实现 Display
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}m", self.0)
    }
}

impl fmt::Display for Kilometers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}km", self.0)
    }
}

impl fmt::Display for Seconds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}s", self.0)
    }
}

impl fmt::Display for MetersPerSecond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}m/s", self.0)
    }
}

// Meters + Meters = Meters
impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Self::Output {
        Meters(self.0 + rhs.0)
    }
}

// Kilometers + Kilometers = Kilometers
impl Add for Kilometers {
    type Output = Kilometers;
    fn add(self, rhs: Kilometers) -> Self::Output {
        Kilometers(self.0 + rhs.0)
    }
}

// 类型间的转换
impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Self {
        Meters(km.0 * 1000.0)
    }
}

impl From<Meters> for Kilometers {
    fn from(m: Meters) -> Self {
        Kilometers(m.0 / 1000.0)
    }
}

/// 计算速度（只接受正确的类型，编译时保证安全）
fn calculate_speed(distance: Meters, time: Seconds) -> MetersPerSecond {
    MetersPerSecond(distance.0 / time.0)
}

// 如果用裸 f64，以下错误不会被编译器发现：
// fn calculate_speed_unsafe(distance: f64, time: f64) -> f64 {
//     distance / time  // 如果不小心传反了参数呢？编译器不会报错！
// }

// ---------------------------------------------------------
// 4. Deref 实现透明访问
// ---------------------------------------------------------
// 通过实现 Deref trait，可以让 Newtype 像底层类型一样使用。
// 这让我们既保留了类型安全，又不失便利性。

/// 一个包装了 String 的用户名类型
#[derive(Debug, Clone)]
struct Username(String);

impl Username {
    fn new(name: &str) -> Result<Self, String> {
        if name.len() >= 3 && name.len() <= 20 {
            Ok(Username(name.to_string()))
        } else {
            Err(format!("用户名长度必须在 3-20 之间，当前: {}", name.len()))
        }
    }
}

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

/// 实现 Deref，使 Username 可以像 &str 一样使用
impl Deref for Username {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 一个包装了 Vec<T> 的栈结构
#[derive(Debug)]
struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    fn new() -> Self {
        Stack(Vec::new())
    }

    fn push(&mut self, item: T) {
        self.0.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn size(&self) -> usize {
        self.0.len()
    }
}

/// 实现 Deref，可以透明访问底层 Vec 的只读方法
impl<T> Deref for Stack<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// 实现 DerefMut，可以透明访问底层 Vec 的可变方法
impl<T> DerefMut for Stack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let items: Vec<String> = self.0.iter().map(|i| i.to_string()).collect();
        write!(f, "Stack[{}]", items.join(" → "))
    }
}

// ---------------------------------------------------------
// 5. Newtype 与泛型结合
// ---------------------------------------------------------

/// 非空列表 —— 通过 Newtype 在编译时保证列表不为空
#[derive(Debug)]
struct NonEmpty<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmpty<T> {
    /// 创建至少包含一个元素的列表
    fn new(first: T) -> Self {
        NonEmpty {
            head: first,
            tail: Vec::new(),
        }
    }

    /// 从 Vec 创建（如果 Vec 为空则失败）
    fn from_vec(mut vec: Vec<T>) -> Result<Self, &'static str> {
        if vec.is_empty() {
            Err("无法从空 Vec 创建 NonEmpty")
        } else {
            let head = vec.remove(0);
            Ok(NonEmpty { head, tail: vec })
        }
    }

    /// 追加元素
    fn push(&mut self, item: T) {
        self.tail.push(item);
    }

    /// 获取第一个元素（保证存在，无需 Option）
    fn first(&self) -> &T {
        &self.head
    }

    /// 获取长度（保证 >= 1）
    fn len(&self) -> usize {
        1 + self.tail.len()
    }
}

impl<T: fmt::Display> fmt::Display for NonEmpty<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}", self.head)?;
        for item in &self.tail {
            write!(f, ", {}", item)?;
        }
        write!(f, "]")
    }
}

fn main() {
    println!("=== Lesson 035: Newtype 模式 ===\n");

    // ---------------------------------------------------------
    // 演示 1: 基本 Newtype
    // ---------------------------------------------------------
    println!("--- 1. 基本 Newtype ---");

    let email = EmailAddress::new("user@example.com").unwrap();
    println!("邮箱: {}", email);
    println!("用户名: {}", email.username());
    println!("域名: {}", email.domain());

    // 无效邮箱
    match EmailAddress::new("invalid-email") {
        Ok(e) => println!("邮箱: {}", e),
        Err(e) => println!("错误: {}", e),
    }

    // ---------------------------------------------------------
    // 演示 2: 绕过孤儿规则
    // ---------------------------------------------------------
    println!("\n--- 2. 绕过孤儿规则 ---");

    let wrapper = Wrapper(vec![
        "Rust".to_string(),
        "是".to_string(),
        "最好的".to_string(),
        "语言".to_string(),
    ]);
    // 因为 Wrapper 是我们的类型，可以为它实现 Display
    println!("Wrapper Display: {}", wrapper);

    let mut list = IntList::new();
    list.0.push(10);
    list.0.push(20);
    list.0.push(30);
    list.0.push(40);
    println!("IntList: {}", list);
    println!("总和: {}", list.sum());
    println!("平均值: {:.1}", list.average());

    // ---------------------------------------------------------
    // 演示 3: 类型安全
    // ---------------------------------------------------------
    println!("\n--- 3. 类型安全 ---");

    let distance1 = Meters(100.0);
    let distance2 = Meters(50.0);
    let total_distance = distance1 + distance2;
    println!("{} + {} = {}", distance1, distance2, total_distance);

    let km1 = Kilometers(5.0);
    let km2 = Kilometers(3.0);
    println!("{} + {} = {}", km1, km2, km1 + km2);

    // 类型转换
    let meters_from_km: Meters = Meters::from(km1);
    println!("{} = {}", km1, meters_from_km);

    let km_from_m: Kilometers = Kilometers::from(distance1);
    println!("{} = {}", distance1, km_from_m);

    // 计算速度（编译器保证参数类型正确）
    let distance = Meters(1000.0);
    let time = Seconds(50.0);
    let speed = calculate_speed(distance, time);
    println!("{} / {} = {}", distance, time, speed);

    // 以下代码会编译错误 —— 这正是我们想要的！
    // let wrong_speed = calculate_speed(time, distance); // ❌ 参数顺序错误
    // let wrong_add = distance1 + km1;                   // ❌ 不能直接相加不同类型

    println!("\n📝 类型安全的好处:");
    println!("   编译器会阻止你把「秒」当作「米」使用");
    println!("   不需要在运行时检查，零成本！");

    // ---------------------------------------------------------
    // 演示 4: Deref 透明访问
    // ---------------------------------------------------------
    println!("\n--- 4. Deref 透明访问 ---");

    let user = Username::new("rustacean").unwrap();
    println!("用户名: {}", user);

    // 通过 Deref，Username 可以直接使用 String/str 的方法
    println!("长度: {}", user.len());           // String::len
    println!("大写: {}", user.to_uppercase());  // str::to_uppercase
    println!("包含 'rust': {}", user.contains("rust")); // str::contains

    // 可以传给接受 &str 的函数（Deref 强制转换）
    fn greet(name: &str) {
        println!("你好, {}!", name);
    }
    greet(&user); // Username → &String → &str（两次 Deref 强制转换）

    println!("\n栈的 Deref 示例:");
    let mut stack: Stack<i32> = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("栈: {}", stack);
    println!("栈顶: {:?}", stack.peek());
    println!("大小: {}", stack.size());

    // 通过 Deref 访问 Vec 的方法
    println!("迭代器求和: {}", stack.iter().sum::<i32>());
    println!("包含 2: {}", stack.contains(&2));

    // pop
    let popped = stack.pop();
    println!("弹出: {:?}, 剩余: {}", popped, stack);

    // ---------------------------------------------------------
    // 演示 5: NonEmpty —— 编译时保证非空
    // ---------------------------------------------------------
    println!("\n--- 5. NonEmpty 类型安全 ---");

    let mut ne = NonEmpty::new(1);
    ne.push(2);
    ne.push(3);
    println!("NonEmpty: {}", ne);
    println!("第一个元素: {} (保证存在，无需 unwrap)", ne.first());
    println!("长度: {} (保证 >= 1)", ne.len());

    // 从 Vec 创建
    let ne2 = NonEmpty::from_vec(vec!["a", "b", "c"]).unwrap();
    println!("从 Vec 创建: {}", ne2);

    // 空 Vec 会失败
    let result: Result<NonEmpty<i32>, _> = NonEmpty::from_vec(vec![]);
    match result {
        Ok(ne) => println!("创建成功: {}", ne),
        Err(e) => println!("创建失败: {}", e),
    }

    // ---------------------------------------------------------
    // 总结
    // ---------------------------------------------------------
    println!("\n--- 总结 ---");
    println!("Newtype 模式的用途:");
    println!("  1️⃣  绕过孤儿规则，为外部类型实现外部 trait");
    println!("  2️⃣  类型安全: 区分 Meters/Kilometers/Seconds");
    println!("  3️⃣  封装: 隐藏内部实现细节");
    println!("  4️⃣  验证: 在创建时验证数据（如 EmailAddress）");
    println!("  5️⃣  Deref: 实现透明访问底层类型的方法");
    println!("  6️⃣  零成本: 运行时没有额外开销");

    println!("\n🎉 恭喜！你已经完成了 Newtype 模式的学习！");
}
