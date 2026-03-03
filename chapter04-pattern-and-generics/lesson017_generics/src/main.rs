/// # Lesson 017 - 泛型 (Generics)
///
/// 泛型是 Rust 中实现代码复用的核心机制之一。
///
/// ## 学习目标
/// - 理解泛型的概念和用途
/// - 掌握泛型函数的定义和使用
/// - 掌握泛型结构体和泛型枚举
/// - 学会为泛型类型实现方法
/// - 理解单态化 (monomorphization) 和零成本抽象
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson017_generics
/// ```

// =============================================================
// Lesson 017: 泛型 (Generics) - 编写通用、可复用的代码
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 为什么需要泛型？—— 从重复代码说起
    // ---------------------------------------------------------
    // 假设我们需要找到一个 i32 列表中的最大值
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in &list[1..] {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 如果还需要找 f64 列表中的最大值，就得再写一遍几乎一样的代码
    fn largest_f64(list: &[f64]) -> &f64 {
        let mut largest = &list[0];
        for item in &list[1..] {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("i32 最大值: {}", largest_i32(&numbers));

    let floats = vec![1.1, 5.5, 3.3, 2.2];
    println!("f64 最大值: {}", largest_f64(&floats));

    // 这两个函数的逻辑完全相同，只是类型不同。泛型可以帮我们消除这种重复！

    // ---------------------------------------------------------
    // 2. 泛型函数 fn largest<T>
    // ---------------------------------------------------------
    // 使用泛型参数 T 代替具体类型
    // T: PartialOrd 是一个 trait bound（特征约束），
    // 表示 T 必须支持比较操作（> < ==）
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in &list[1..] {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 现在一个函数就能处理多种类型！
    let numbers = vec![34, 50, 25, 100, 65];
    println!("\n泛型 largest - i32 最大值: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("泛型 largest - char 最大值: {}", largest(&chars));

    let words = vec!["hello", "world", "rust", "泛型"];
    println!("泛型 largest - &str 最大值: {}", largest(&words));

    // 多个泛型参数的函数
    fn print_pair<T: std::fmt::Debug, U: std::fmt::Debug>(first: T, second: U) {
        println!("配对: ({:?}, {:?})", first, second);
    }

    print_pair(42, "hello");
    print_pair(3.14, true);
    print_pair("Rust", vec![1, 2, 3]);

    // ---------------------------------------------------------
    // 3. 泛型结构体 Point<T>
    // ---------------------------------------------------------
    println!("\n--- 泛型结构体 ---");

    // 单个泛型参数：x 和 y 必须是同一类型
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("整数点: {:?}", integer_point);
    println!("浮点点: {:?}", float_point);

    // 注意：下面这行会报错，因为 x 和 y 必须是同一类型
    // let mixed_point = Point { x: 5, y: 4.0 }; // 编译错误!

    // 如果需要不同类型，使用多个泛型参数
    #[derive(Debug)]
    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    let mixed = MixedPoint { x: 5, y: 4.0 };
    println!("混合点: {:?}", mixed);

    let string_bool = MixedPoint {
        x: "hello",
        y: true,
    };
    println!("字符串-布尔点: {:?}", string_bool);

    // ---------------------------------------------------------
    // 4. 泛型枚举 - Option<T> 和 Result<T, E>
    // ---------------------------------------------------------
    println!("\n--- 泛型枚举 ---");

    // Rust 标准库中最重要的两个泛型枚举：
    //
    // Option<T> 的定义（简化版）：
    // enum Option<T> {
    //     Some(T),  // 包含一个 T 类型的值
    //     None,     // 没有值
    // }
    //
    // Result<T, E> 的定义（简化版）：
    // enum Result<T, E> {
    //     Ok(T),    // 操作成功，包含 T 类型的结果
    //     Err(E),   // 操作失败，包含 E 类型的错误
    // }

    // Option<T> 示例
    let some_number: Option<i32> = Some(42);
    let some_string: Option<&str> = Some("hello");
    let no_value: Option<i32> = None;
    println!("Option<i32>: {:?}", some_number);
    println!("Option<&str>: {:?}", some_string);
    println!("Option<i32> None: {:?}", no_value);

    // Result<T, E> 示例
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零！".to_string())
        } else {
            Ok(a / b)
        }
    }

    println!("10 / 3 = {:?}", divide(10.0, 3.0));
    println!("10 / 0 = {:?}", divide(10.0, 0.0));

    // 自定义泛型枚举
    #[derive(Debug)]
    enum Container<T> {
        Empty,
        Single(T),
        Pair(T, T),
    }

    let empty: Container<i32> = Container::Empty;
    let single = Container::Single("hello");
    let pair = Container::Pair(1, 2);
    println!("容器: {:?}, {:?}, {:?}", empty, single, pair);

    // ---------------------------------------------------------
    // 5. 泛型方法 impl<T>
    // ---------------------------------------------------------
    println!("\n--- 泛型方法 ---");

    // 为泛型结构体实现方法
    // 注意：impl 后面也要声明 <T>，告诉编译器这是泛型实现
    impl<T> Point<T> {
        // 返回 x 字段的引用
        fn x(&self) -> &T {
            &self.x
        }

        // 返回 y 字段的引用
        fn y(&self) -> &T {
            &self.y
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x() = {}", p.x());
    println!("p.y() = {}", p.y());

    // 方法中也可以引入额外的泛型参数
    impl<T, U> MixedPoint<T, U> {
        // 这个方法将 self 的 x 和 other 的 y 组合成新的 MixedPoint
        // 方法本身引入了新的泛型参数 V, W
        fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
            MixedPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint {
        x: "Hello",
        y: 'c',
    };
    let p3 = p1.mixup(p2);
    println!("混合后: p3.x = {}, p3.y = {}", p3.x, p3.y);

    // ---------------------------------------------------------
    // 6. 为特定类型实现方法
    // ---------------------------------------------------------
    println!("\n--- 特定类型的方法实现 ---");

    // 只为 Point<f64> 实现的方法（其他类型的 Point 没有此方法）
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let float_p = Point { x: 3.0_f64, y: 4.0_f64 };
    println!(
        "点 ({}, {}) 到原点的距离: {}",
        float_p.x,
        float_p.y,
        float_p.distance_from_origin()
    );

    // 下面这行会报错，因为 Point<i32> 没有 distance_from_origin 方法
    // let int_p = Point { x: 3, y: 4 };
    // int_p.distance_from_origin(); // 编译错误!

    // 带有 trait bound 的方法实现
    // 只有 T 实现了 Display + PartialOrd 的 Point 才拥有 describe 方法
    impl<T: std::fmt::Display + PartialOrd> Point<T> {
        fn describe(&self) -> String {
            if self.x >= self.y {
                format!("点({}, {}): x >= y", self.x, self.y)
            } else {
                format!("点({}, {}): x < y", self.x, self.y)
            }
        }
    }

    let p1 = Point { x: 10, y: 5 };
    let p2 = Point { x: 1.0, y: 9.9 };
    println!("{}", p1.describe());
    println!("{}", p2.describe());

    // ---------------------------------------------------------
    // 7. 泛型在更多场景中的应用
    // ---------------------------------------------------------
    println!("\n--- 泛型的实用示例 ---");

    // 泛型栈（后进先出）
    #[derive(Debug)]
    struct Stack<T> {
        elements: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Self {
            Stack {
                elements: Vec::new(),
            }
        }

        fn push(&mut self, item: T) {
            self.elements.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.elements.pop()
        }

        fn peek(&self) -> Option<&T> {
            self.elements.last()
        }

        fn is_empty(&self) -> bool {
            self.elements.is_empty()
        }

        fn size(&self) -> usize {
            self.elements.len()
        }
    }

    let mut int_stack: Stack<i32> = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    println!("栈顶元素: {:?}", int_stack.peek());
    println!("弹出: {:?}", int_stack.pop());
    println!("弹出后栈顶: {:?}", int_stack.peek());
    println!("栈大小: {}", int_stack.size());

    let mut str_stack: Stack<&str> = Stack::new();
    str_stack.push("hello");
    str_stack.push("world");
    println!("字符串栈顶: {:?}", str_stack.peek());

    // ---------------------------------------------------------
    // 8. 单态化 (Monomorphization) - 泛型的零成本抽象
    // ---------------------------------------------------------
    println!("\n--- 单态化与性能 ---");

    // Rust 的泛型在编译时会进行「单态化」(monomorphization)：
    // 编译器会为每个具体使用的类型生成专门的代码。
    //
    // 例如，当我们写：
    //   let integer = Some(5);     // Option<i32>
    //   let float = Some(5.0);    // Option<f64>
    //
    // 编译器实际上会生成类似这样的代码：
    //
    //   enum Option_i32 {
    //       Some(i32),
    //       None,
    //   }
    //
    //   enum Option_f64 {
    //       Some(f64),
    //       None,
    //   }
    //
    // 所以：
    // ✅ 泛型不会带来任何运行时开销
    // ✅ 泛型代码的性能与手动编写特定类型的代码完全相同
    // ❌ 但可能会增加编译后的二进制文件大小（因为生成了多份代码）

    // 验证单态化效果
    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    // 以下调用会让编译器生成 add_i32、add_f64、add_i64 三个版本
    let sum_i32 = add(1_i32, 2_i32);
    let sum_f64 = add(1.5_f64, 2.5_f64);
    let sum_i64 = add(100_i64, 200_i64);
    println!("i32 加法: {}", sum_i32);
    println!("f64 加法: {}", sum_f64);
    println!("i64 加法: {}", sum_i64);

    // ---------------------------------------------------------
    // 9. const 泛型（常量泛型）—— 补充知识
    // ---------------------------------------------------------
    println!("\n--- const 泛型 ---");

    // Rust 还支持 const 泛型，让数组长度也能参数化
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: &[T; N]) {
        println!("长度为 {} 的数组: {:?}", N, arr);
    }

    display_array(&[1, 2, 3]);
    display_array(&[1, 2, 3, 4, 5]);
    display_array(&["hello", "world"]);

    // ---------------------------------------------------------
    // 10. 总结
    // ---------------------------------------------------------
    println!("\n--- 总结 ---");
    println!("📌 泛型让我们编写通用的、可复用的代码");
    println!("📌 泛型函数: fn name<T>(param: T) -> T");
    println!("📌 泛型结构体: struct Name<T> {{ field: T }}");
    println!("📌 泛型枚举: enum Name<T> {{ Variant(T) }}");
    println!("📌 泛型方法: impl<T> Name<T> {{ fn method(&self) }}");
    println!("📌 特定类型方法: impl Name<f64> {{ fn specific(&self) }}");
    println!("📌 单态化保证泛型是零成本抽象，无运行时开销");

    println!("\n🎉 恭喜！你已经掌握了 Rust 泛型的核心概念！");
}
