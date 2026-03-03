/// # Lesson 029 - 自定义迭代器
///
/// 通过为自定义类型实现 Iterator trait，
/// 可以让你的类型融入 Rust 的迭代器生态系统，
/// 获得所有迭代器适配器和消费者的能力。
///
/// ## 学习目标
/// - 为自定义类型实现 Iterator trait
/// - 实现经典的 Fibonacci 迭代器
/// - 理解 IntoIterator trait 的作用
/// - 学习 DoubleEndedIterator 双端迭代器
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson029_custom_iterator
/// ```

fn main() {
    // =============================================================
    // 1. 为自定义类型实现 Iterator
    // =============================================================
    println!("===== 1. 自定义计数器迭代器 =====");

    // 定义一个简单的计数器
    struct Counter {
        current: u32,
        max: u32,
    }

    impl Counter {
        fn new(max: u32) -> Counter {
            Counter { current: 0, max }
        }
    }

    // 为 Counter 实现 Iterator trait
    // 只需要实现 next() 方法，其他方法（map, filter, fold 等）会自动获得！
    impl Iterator for Counter {
        type Item = u32; // 关联类型：迭代器产出 u32

        fn next(&mut self) -> Option<Self::Item> {
            if self.current < self.max {
                self.current += 1;
                Some(self.current) // 返回 1, 2, 3, ..., max
            } else {
                None // 迭代结束
            }
        }
    }

    // 使用我们的自定义迭代器
    let counter = Counter::new(5);
    print!("Counter(5): ");
    for n in counter {
        print!("{} ", n);
    }
    println!();

    // 因为实现了 Iterator，自动获得了所有适配器方法！
    let sum: u32 = Counter::new(10).sum();
    println!("Counter(10).sum() = {}", sum);

    let squares: Vec<u32> = Counter::new(5).map(|x| x * x).collect();
    println!("Counter(5).map(x²) = {:?}", squares);

    let evens: Vec<u32> = Counter::new(10).filter(|x| x % 2 == 0).collect();
    println!("Counter(10).filter(偶数) = {:?}", evens);

    // 组合两个自定义迭代器
    let paired: Vec<(u32, u32)> = Counter::new(5).zip(Counter::new(5).skip(1)).collect();
    println!("Counter.zip(Counter.skip(1)) = {:?}", paired);

    // =============================================================
    // 2. 经典示例：Fibonacci 迭代器
    // =============================================================
    println!("\n===== 2. Fibonacci 迭代器 =====");

    struct Fibonacci {
        a: u64,
        b: u64,
    }

    impl Fibonacci {
        fn new() -> Fibonacci {
            Fibonacci { a: 0, b: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.a;
            let new_b = self.a + self.b;
            self.a = self.b;
            self.b = new_b;
            Some(result) // Fibonacci 是无限迭代器，永远返回 Some
        }
    }

    // 取前 15 个 Fibonacci 数（无限迭代器必须用 take 限制）
    let fibs: Vec<u64> = Fibonacci::new().take(15).collect();
    println!("前 15 个 Fibonacci: {:?}", fibs);

    // 找到第一个超过 1000 的 Fibonacci 数
    let big = Fibonacci::new().find(|&x| x > 1000);
    println!("第一个 > 1000 的 Fibonacci: {:?}", big);

    // Fibonacci 数中的偶数，取前 8 个
    let even_fibs: Vec<u64> = Fibonacci::new()
        .filter(|x| x % 2 == 0)
        .take(8)
        .collect();
    println!("Fibonacci 偶数 (前 8 个): {:?}", even_fibs);

    // 前 20 个 Fibonacci 数的和
    let fib_sum: u64 = Fibonacci::new().take(20).sum();
    println!("前 20 个 Fibonacci 之和: {}", fib_sum);

    // =============================================================
    // 3. 更复杂的自定义迭代器：范围步进器
    // =============================================================
    println!("\n===== 3. 范围步进迭代器 =====");

    // 一个支持自定义步长的范围迭代器
    struct StepRange {
        current: i32,
        end: i32,
        step: i32,
    }

    impl StepRange {
        fn new(start: i32, end: i32, step: i32) -> StepRange {
            assert!(step != 0, "步长不能为 0");
            StepRange {
                current: start,
                end,
                step,
            }
        }
    }

    impl Iterator for StepRange {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if (self.step > 0 && self.current < self.end)
                || (self.step < 0 && self.current > self.end)
            {
                let result = self.current;
                self.current += self.step;
                Some(result)
            } else {
                None
            }
        }
    }

    // 正向步进
    let forward: Vec<i32> = StepRange::new(0, 20, 3).collect();
    println!("StepRange(0, 20, 3): {:?}", forward);

    // 反向步进
    let backward: Vec<i32> = StepRange::new(10, -5, -2).collect();
    println!("StepRange(10, -5, -2): {:?}", backward);

    // =============================================================
    // 4. IntoIterator trait
    // =============================================================
    println!("\n===== 4. IntoIterator trait =====");

    // IntoIterator trait 让类型可以用在 for 循环中
    // trait IntoIterator {
    //     type Item;
    //     type IntoIter: Iterator<Item = Self::Item>;
    //     fn into_iter(self) -> Self::IntoIter;
    // }
    //
    // 当你写 `for x in something` 时，
    // 编译器会调用 something.into_iter()

    // 为自定义集合实现 IntoIterator
    struct Classroom {
        students: Vec<String>,
    }

    impl Classroom {
        fn new(names: Vec<&str>) -> Classroom {
            Classroom {
                students: names.into_iter().map(String::from).collect(),
            }
        }
    }

    // 为 Classroom 实现 IntoIterator（消费版本）
    impl IntoIterator for Classroom {
        type Item = String;
        type IntoIter = std::vec::IntoIter<String>;

        fn into_iter(self) -> Self::IntoIter {
            self.students.into_iter()
        }
    }

    // 为 &Classroom 实现 IntoIterator（借用版本）
    impl<'a> IntoIterator for &'a Classroom {
        type Item = &'a String;
        type IntoIter = std::slice::Iter<'a, String>;

        fn into_iter(self) -> Self::IntoIter {
            self.students.iter()
        }
    }

    let class = Classroom::new(vec!["Alice", "Bob", "Charlie", "Diana"]);

    // 借用遍历（class 仍然可用）
    println!("教室里的学生（借用遍历）:");
    for student in &class {
        println!("  {}", student);
    }

    // 消费遍历（class 被消费）
    println!("教室里的学生（消费遍历）:");
    for student in class {
        println!("  {} (已获取所有权)", student);
    }
    // class 不再可用

    // =============================================================
    // 5. 迭代器返回引用的情况
    // =============================================================
    println!("\n===== 5. 迭代器返回引用 =====");

    // 一个迭代自身内容的结构体
    struct Sentence {
        text: String,
    }

    impl Sentence {
        fn new(text: &str) -> Sentence {
            Sentence {
                text: String::from(text),
            }
        }

        // 返回一个单词迭代器（借用 self）
        fn words(&self) -> std::str::SplitWhitespace<'_> {
            self.text.split_whitespace()
        }
    }

    let sentence = Sentence::new("Rust is a systems programming language");
    println!("单词:");
    for (i, word) in sentence.words().enumerate() {
        println!("  [{}] {}", i, word);
    }

    // sentence 仍然可用
    println!("原句: {}", sentence.text);
    println!("单词数: {}", sentence.words().count());

    // =============================================================
    // 6. DoubleEndedIterator —— 双端迭代器
    // =============================================================
    println!("\n===== 6. DoubleEndedIterator 双端迭代器 =====");

    // DoubleEndedIterator 允许从两端消费元素
    // trait DoubleEndedIterator: Iterator {
    //     fn next_back(&mut self) -> Option<Self::Item>;
    // }

    // Vec 的迭代器实现了 DoubleEndedIterator
    let colors = vec!["红", "橙", "黄", "绿", "蓝"];

    // rev(): 反转迭代器（需要 DoubleEndedIterator）
    print!("反转遍历: ");
    for color in colors.iter().rev() {
        print!("{} ", color);
    }
    println!();

    // 手动从两端交替消费
    let nums = vec![1, 2, 3, 4, 5, 6];
    let mut iter = nums.iter();
    println!("\n从两端交替消费:");
    println!("  前端: {:?}", iter.next());      // Some(&1)
    println!("  后端: {:?}", iter.next_back()); // Some(&6)
    println!("  前端: {:?}", iter.next());      // Some(&2)
    println!("  后端: {:?}", iter.next_back()); // Some(&5)
    println!("  前端: {:?}", iter.next());      // Some(&3)
    println!("  后端: {:?}", iter.next_back()); // Some(&4)
    println!("  前端: {:?}", iter.next());      // None

    // 为自定义类型实现 DoubleEndedIterator
    println!("\n--- 自定义双端迭代器 ---");

    struct Countdown {
        front: i32,
        back: i32,
    }

    impl Countdown {
        fn new(start: i32, end: i32) -> Countdown {
            Countdown {
                front: start,
                back: end,
            }
        }
    }

    impl Iterator for Countdown {
        type Item = i32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.front <= self.back {
                let val = self.front;
                self.front += 1;
                Some(val)
            } else {
                None
            }
        }
    }

    impl DoubleEndedIterator for Countdown {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.back >= self.front {
                let val = self.back;
                self.back -= 1;
                Some(val)
            } else {
                None
            }
        }
    }

    // 正向
    let forward: Vec<i32> = Countdown::new(1, 8).collect();
    println!("正向: {:?}", forward);

    // 反向
    let reversed: Vec<i32> = Countdown::new(1, 8).rev().collect();
    println!("反向: {:?}", reversed);

    // 取最后 3 个（rev + take + rev）
    let last_three: Vec<i32> = Countdown::new(1, 8).rev().take(3).collect();
    println!("最后 3 个: {:?}", last_three);

    // =============================================================
    // 7. size_hint 与 ExactSizeIterator
    // =============================================================
    println!("\n===== 7. size_hint 与 ExactSizeIterator =====");

    // size_hint 为 collect 等方法提供预分配优化
    let v = vec![1, 2, 3, 4, 5];
    let iter = v.iter();
    println!("Vec iter size_hint: {:?}", iter.size_hint()); // (5, Some(5))

    // 我们的 Fibonacci 是无限迭代器
    let fib = Fibonacci::new();
    println!("Fibonacci size_hint: {:?}", fib.size_hint()); // (0, None)

    // 为 Countdown 实现 ExactSizeIterator
    impl ExactSizeIterator for Countdown {
        fn len(&self) -> usize {
            if self.front > self.back {
                0
            } else {
                (self.back - self.front + 1) as usize
            }
        }
    }

    let countdown = Countdown::new(1, 10);
    println!("Countdown len: {}", countdown.len());
    println!("Countdown size_hint: {:?}", countdown.size_hint());

    // =============================================================
    // 8. 综合实战：自定义矩阵行迭代器
    // =============================================================
    println!("\n===== 8. 综合实战：矩阵迭代器 =====");

    struct Matrix {
        data: Vec<Vec<i32>>,
        rows: usize,
        cols: usize,
    }

    impl Matrix {
        fn new(data: Vec<Vec<i32>>) -> Matrix {
            let rows = data.len();
            let cols = if rows > 0 { data[0].len() } else { 0 };
            Matrix { data, rows, cols }
        }

        // 返回行迭代器
        fn rows(&self) -> RowIter<'_> {
            RowIter {
                matrix: self,
                current_row: 0,
            }
        }

        // 返回所有元素的迭代器（按行优先顺序）
        fn elements(&self) -> impl Iterator<Item = &i32> {
            self.data.iter().flatten()
        }
    }

    // 行迭代器
    struct RowIter<'a> {
        matrix: &'a Matrix,
        current_row: usize,
    }

    impl<'a> Iterator for RowIter<'a> {
        type Item = &'a Vec<i32>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_row < self.matrix.rows {
                let row = &self.matrix.data[self.current_row];
                self.current_row += 1;
                Some(row)
            } else {
                None
            }
        }
    }

    let matrix = Matrix::new(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ]);

    // 遍历行
    println!("矩阵 ({}x{}):", matrix.rows, matrix.cols);
    for (i, row) in matrix.rows().enumerate() {
        println!("  第 {} 行: {:?}", i, row);
    }

    // 使用迭代器方法
    let all_elements: Vec<&i32> = matrix.elements().collect();
    println!("所有元素: {:?}", all_elements);

    let sum: i32 = matrix.elements().sum();
    println!("元素总和: {}", sum);

    let max = matrix.elements().max();
    println!("最大元素: {:?}", max);

    // 每行的和
    let row_sums: Vec<i32> = matrix.rows().map(|row| row.iter().sum()).collect();
    println!("每行之和: {:?}", row_sums);

    // =============================================================
    // 9. 迭代器设计模式总结
    // =============================================================
    println!("\n===== 9. 迭代器设计模式总结 =====");

    println!("实现自定义迭代器的步骤:");
    println!("  1. 定义一个结构体，保存迭代状态");
    println!("  2. 实现 Iterator trait，定义 Item 类型和 next() 方法");
    println!("  3. (可选) 实现 DoubleEndedIterator 支持反向迭代");
    println!("  4. (可选) 实现 ExactSizeIterator 提供精确长度");
    println!("  5. (可选) 为你的集合类型实现 IntoIterator");
    println!();
    println!("常见的自定义迭代器模式:");
    println!("  - 有限序列：Counter, StepRange");
    println!("  - 无限序列：Fibonacci（必须用 take 限制）");
    println!("  - 集合迭代：Matrix RowIter（借用集合内容）");
    println!("  - 转换迭代：将一个迭代器包装为另一个");

    println!("\n🎉 恭喜！你已经完成了自定义迭代器的学习！");
    println!("至此，Chapter 06 集合与迭代器全部完成！");
}
