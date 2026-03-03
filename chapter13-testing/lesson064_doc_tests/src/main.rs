// ============================================================
// Lesson 064: 文档测试 (Doc Tests)
// ============================================================
// 本课学习 Rust 中的文档测试，包括：
// - /// 文档注释中的代码块会被自动当作测试运行
// - # 隐藏行：在文档中隐藏但在测试中运行的代码
// - should_panic、no_run、ignore 等标记
// - 文档测试确保文档中的示例代码始终正确
//
// 文档测试的核心理念：
// 代码文档中的示例也是测试！如果示例过时了、编译不过了，
// `cargo test` 会报错，迫使你更新文档。
//
// 运行文档测试：cargo test --doc
//
// 注意：文档测试只对 lib crate 有效。
// 由于本文件是 binary crate (main.rs)，文档测试不会被 cargo test 运行。
// 但我们仍然编写完整的文档注释来展示正确的写法。
// 在真实项目中，这些代码应放在 src/lib.rs 中。
// ============================================================

/// 将两个整数相加并返回结果。
///
/// # 参数
///
/// * `a` - 第一个加数
/// * `b` - 第二个加数
///
/// # 返回值
///
/// 返回两个整数的和。
///
/// # 示例
///
/// 文档注释中的代码块（```）会自动成为文档测试：
///
/// ```
/// let result = lesson064_doc_tests::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// 测试负数：
///
/// ```
/// assert_eq!(lesson064_doc_tests::add(-1, 1), 0);
/// assert_eq!(lesson064_doc_tests::add(-5, -3), -8);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 安全除法，当除数为零时返回错误。
///
/// # 参数
///
/// * `a` - 被除数
/// * `b` - 除数
///
/// # 错误
///
/// 当 `b` 为零时，返回 `Err("除数不能为零")`。
///
/// # 示例
///
/// 正常除法：
///
/// ```
/// let result = lesson064_doc_tests::divide(10.0, 3.0);
/// assert!(result.is_ok());
/// let value = result.unwrap();
/// assert!((value - 3.3333).abs() < 0.001);
/// ```
///
/// 除以零会返回错误：
///
/// ```
/// let result = lesson064_doc_tests::divide(10.0, 0.0);
/// assert!(result.is_err());
/// assert_eq!(result.unwrap_err(), "除数不能为零");
/// ```
///
/// # 使用 `#` 隐藏行
///
/// 用 `# ` 开头的行在渲染的文档中不显示，但在测试中会运行。
/// 这对于需要 setup 代码但不想在文档中展示的情况很有用：
///
/// ```
/// # // 这一行在文档中不可见，但会参与编译
/// # fn setup() -> (f64, f64) { (100.0, 4.0) }
/// # let (a, b) = setup();
/// let result = lesson064_doc_tests::divide(a, b).unwrap();
/// assert_eq!(result, 25.0);
/// ```
pub fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 一个会在输入为空时 panic 的函数。
///
/// # Panics
///
/// 当 `name` 为空字符串时会 panic。
///
/// # 示例
///
/// 正常使用：
///
/// ```
/// let greeting = lesson064_doc_tests::greet("Rust");
/// assert_eq!(greeting, "你好，Rust！");
/// ```
///
/// ## should_panic 标记
///
/// 使用 `should_panic` 标记测试预期的 panic：
///
/// ```should_panic
/// lesson064_doc_tests::greet(""); // 这会 panic
/// ```
pub fn greet(name: &str) -> String {
    if name.is_empty() {
        panic!("名字不能为空！");
    }
    format!("你好，{}！", name)
}

/// 一个连接网络的函数（模拟）。
///
/// # 示例
///
/// ## no_run 标记
///
/// `no_run` 标记的代码块会被编译检查，但不会实际运行。
/// 适用于需要网络、文件系统等外部资源的示例：
///
/// ```no_run
/// // 这段代码会被编译，但不会执行
/// let data = lesson064_doc_tests::fetch_data("https://example.com");
/// println!("获取到的数据: {}", data);
/// ```
///
/// ## ignore 标记
///
/// `ignore` 标记的代码块完全被跳过，不编译也不运行。
/// 适用于伪代码或需要特殊环境的示例：
///
/// ```ignore
/// // 这段代码不会被编译或运行
/// let result = some_hypothetical_function();
/// ```
///
/// ## compile_fail 标记
///
/// `compile_fail` 标记期望代码编译失败。
/// 用于展示哪些用法是错误的：
///
/// ```compile_fail
/// // 这段代码应该编译失败
/// let x: i32 = "不是数字";
/// ```
pub fn fetch_data(url: &str) -> String {
    // 模拟网络请求
    format!("来自 {} 的数据", url)
}

/// 计算斐波那契数列的第 n 项。
///
/// 使用迭代法实现，时间复杂度为 O(n)。
///
/// # 参数
///
/// * `n` - 要计算的项数（从 0 开始）
///
/// # 示例
///
/// ```
/// assert_eq!(lesson064_doc_tests::fibonacci(0), 0);
/// assert_eq!(lesson064_doc_tests::fibonacci(1), 1);
/// assert_eq!(lesson064_doc_tests::fibonacci(10), 55);
/// ```
///
/// 使用隐藏行做更完整的测试：
///
/// ```
/// # // 测试前几项
/// let fibs: Vec<u64> = (0..8).map(lesson064_doc_tests::fibonacci).collect();
/// assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
/// ```
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// 一个简单的栈结构。
///
/// # 示例
///
/// ```
/// let mut stack = lesson064_doc_tests::Stack::new();
/// assert!(stack.is_empty());
///
/// stack.push(1);
/// stack.push(2);
/// stack.push(3);
/// assert_eq!(stack.len(), 3);
///
/// assert_eq!(stack.pop(), Some(3));
/// assert_eq!(stack.peek(), Some(&2));
/// assert_eq!(stack.len(), 2);
/// ```
///
/// 空栈的行为：
///
/// ```
/// let mut stack: lesson064_doc_tests::Stack<i32> = lesson064_doc_tests::Stack::new();
/// assert_eq!(stack.pop(), None);
/// assert_eq!(stack.peek(), None);
/// ```
pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    /// 创建一个新的空栈。
    ///
    /// ```
    /// let stack: lesson064_doc_tests::Stack<i32> = lesson064_doc_tests::Stack::new();
    /// assert!(stack.is_empty());
    /// ```
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    /// 将元素压入栈顶。
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    /// 弹出栈顶元素。如果栈为空，返回 `None`。
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// 查看栈顶元素（不弹出）。
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    /// 返回栈中元素的数量。
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// 判断栈是否为空。
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

/// 模块级文档也可以包含示例。
///
/// 使用 `//!` 编写模块级文档（写在文件开头）。
/// 使用 `///` 编写条目级文档（写在函数/结构体前面）。
///
/// # 文档注释的标记总结
///
/// | 标记             | 编译检查 | 运行 | 用途                             |
/// |-----------------|---------|------|----------------------------------|
/// | ` ```rust `     | ✅      | ✅   | 默认，正常的文档测试              |
/// | ` ``` `         | ✅      | ✅   | 同上，Rust 代码块默认就是 rust     |
/// | ` ```should_panic ` | ✅  | ✅   | 预期 panic                       |
/// | ` ```no_run `   | ✅      | ❌   | 只编译不运行（如网络代码）         |
/// | ` ```ignore `   | ❌      | ❌   | 完全跳过（伪代码/特殊环境）        |
/// | ` ```compile_fail ` | ✅  | ❌   | 预期编译失败                     |
/// | ` ```text `     | ❌      | ❌   | 纯文本，非代码                    |
/// | `# 行内容`      | ✅      | ✅   | 隐藏行，不在文档中显示            |
pub fn _doc_summary() {}

fn main() {
    println!("=== Lesson 064: 文档测试 ===\n");

    // ---- 演示所有函数的使用 ----
    println!("--- 基本函数 ---");
    println!("add(2, 3) = {}", add(2, 3));

    match divide(10.0, 3.0) {
        Ok(v) => println!("divide(10.0, 3.0) = {:.4}", v),
        Err(e) => println!("错误: {}", e),
    }

    println!("greet(\"Rustacean\") = {}", greet("Rustacean"));
    println!("fetch_data(\"https://example.com\") = {}", fetch_data("https://example.com"));

    println!("\n--- 斐波那契数列 ---");
    for i in 0..=10 {
        print!("fib({}) = {}  ", i, fibonacci(i));
        if (i + 1) % 4 == 0 {
            println!();
        }
    }
    println!();

    println!("\n--- 栈操作 ---");
    let mut stack = Stack::new();
    println!("创建空栈，is_empty = {}", stack.is_empty());

    for val in [10, 20, 30, 40, 50] {
        stack.push(val);
        println!("push({})，栈大小 = {}", val, stack.len());
    }

    println!("peek = {:?}", stack.peek());

    while let Some(val) = stack.pop() {
        println!("pop() = {}", val);
    }
    println!("栈已清空，is_empty = {}", stack.is_empty());

    // ---- 文档测试说明 ----
    println!("\n--- 文档测试要点 ---");
    println!("1. 文档测试只对 lib crate 有效（需要 src/lib.rs）");
    println!("2. 运行文档测试：cargo test --doc");
    println!("3. 生成文档：cargo doc --open");
    println!("4. 文档测试中使用 crate 名引用函数（如 my_crate::add）");
    println!("5. 用 # 隐藏行来简化文档展示");
    println!("6. 用 should_panic/no_run/ignore/compile_fail 控制测试行为");
    println!("7. 文档测试是保证文档与代码同步的最佳实践！");
}

// 单元测试模块（验证函数功能）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_divide() {
        assert!((divide(10.0, 3.0).unwrap() - 3.3333).abs() < 0.001);
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Rust"), "你好，Rust！");
    }

    #[test]
    #[should_panic(expected = "名字不能为空")]
    fn test_greet_empty() {
        greet("");
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
        let fibs: Vec<u64> = (0..8).map(fibonacci).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), Some(&3));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }
}
