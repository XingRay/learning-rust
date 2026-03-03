/// # Lesson 038 - Fn 系列 Trait
///
/// Rust 中有三种闭包相关的 trait：Fn、FnMut、FnOnce。
/// 它们决定了闭包如何使用捕获的变量。
///
/// ## 学习目标
/// - 理解 `Fn`：以不可变引用方式使用捕获的变量
/// - 理解 `FnMut`：以可变引用方式使用捕获的变量
/// - 理解 `FnOnce`：获取捕获变量的所有权（只能调用一次）
/// - 掌握三者的继承层次关系
/// - 学会在实践中选择正确的 trait bound
///
/// ## 运行方式
/// 在项目根目录下执行:
/// ```bash
/// cargo run -p lesson038_fn_traits
/// ```

// =============================================================
// Lesson 038: Fn 系列 Trait - The Fn Trait Family
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. Fn - 不可变借用捕获
    // ---------------------------------------------------------
    // Fn trait 表示闭包通过不可变引用（&self）使用捕获的变量
    // 可以被多次调用，且不会修改捕获的变量
    println!("=== 1. Fn - 不可变借用捕获 ===\n");

    let greeting = String::from("你好");

    // 这个闭包只是读取 greeting，所以实现了 Fn
    let say_hello = |name: &str| {
        println!("  {}, {}!", greeting, name);
    };

    // Fn 闭包可以多次调用
    say_hello("Alice");
    say_hello("Bob");
    say_hello("Charlie");

    // greeting 仍然可以被使用（因为只是不可变借用）
    println!("greeting 仍然存在: {}", greeting);

    // 使用 Fn trait bound 的函数
    fn call_multiple_times(f: &dyn Fn(i32) -> i32, values: &[i32]) {
        for &v in values {
            println!("  f({}) = {}", v, f(v));
        }
    }

    let factor = 3;
    let multiply = |x: i32| -> i32 { x * factor };

    call_multiple_times(&multiply, &[1, 2, 3, 4, 5]);
    println!();

    // ---------------------------------------------------------
    // 2. FnMut - 可变借用捕获
    // ---------------------------------------------------------
    // FnMut trait 表示闭包通过可变引用（&mut self）使用捕获的变量
    // 可以被多次调用，但每次调用可能修改捕获的变量
    println!("=== 2. FnMut - 可变借用捕获 ===\n");

    let mut total = 0;

    // 这个闭包修改了 total，所以需要 FnMut
    let mut accumulate = |x: i32| {
        total += x;
        total
    };

    println!("累加 10: {}", accumulate(10));
    println!("累加 20: {}", accumulate(20));
    println!("累加 30: {}", accumulate(30));
    // accumulate 的借用到此结束
    println!("最终总和: {}", total);

    // 使用 FnMut 的函数
    fn apply_to_each<F>(values: &[i32], mut f: F)
    where
        F: FnMut(i32),
    {
        for &v in values {
            f(v);
        }
    }

    let mut results = Vec::new();
    apply_to_each(&[10, 20, 30], |x| {
        results.push(x * 2);
    });
    println!("结果: {:?}", results);

    // 构建字符串的例子
    let mut log = String::new();
    let mut logger = |msg: &str| {
        log.push_str(msg);
        log.push('\n');
    };

    logger("开始处理");
    logger("处理数据中...");
    logger("处理完成");

    // logger 的借用结束后可以使用 log
    drop(logger); // 显式结束借用
    println!("日志内容:\n{}", log);

    // ---------------------------------------------------------
    // 3. FnOnce - 消耗捕获的变量
    // ---------------------------------------------------------
    // FnOnce trait 表示闭包通过值（self）使用捕获的变量
    // 只能被调用一次，因为调用时可能消耗捕获的值
    println!("=== 3. FnOnce - 消耗捕获的变量 ===\n");

    let name = String::from("重要数据");

    // 这个闭包消耗了 name（drop 会获取所有权）
    let consume = || {
        let moved_name = name; // 移动所有权到闭包内部
        println!("  消耗了: {}", moved_name);
        // moved_name 在这里被丢弃
    };

    consume();
    // consume(); // 错误！FnOnce 只能调用一次
    // println!("{}", name); // 错误！name 已被移动

    // 使用 FnOnce 的函数
    fn execute_once<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        let result = f();
        println!("  执行结果: {}", result);
    }

    let data = vec![1, 2, 3, 4, 5];
    execute_once(|| {
        // 消耗 data，将其转为字符串
        let s = format!("数据: {:?}", data);
        // data 的所有权被移入闭包，可以自由使用
        drop(data); // 显式丢弃 data
        s
    });

    // 另一个 FnOnce 的例子：发送数据
    fn send_data<F>(sender: F)
    where
        F: FnOnce(String),
    {
        let payload = String::from("重要消息内容");
        sender(payload); // 将 payload 发送给闭包
    }

    send_data(|msg| {
        println!("  收到消息: {}", msg);
        // msg 的所有权被转移到这里，可以自由处理
    });
    println!();

    // ---------------------------------------------------------
    // 4. 三者的关系与层次
    // ---------------------------------------------------------
    // Fn、FnMut、FnOnce 存在继承关系：
    //
    //   FnOnce
    //     ↑ （FnMut 是 FnOnce 的子 trait）
    //   FnMut
    //     ↑ （Fn 是 FnMut 的子 trait）
    //    Fn
    //
    // 也就是说：
    // - 实现了 Fn 的闭包，自动实现 FnMut 和 FnOnce
    // - 实现了 FnMut 的闭包，自动实现 FnOnce
    // - 所有闭包都至少实现了 FnOnce
    println!("=== 4. 三者的关系与层次 ===\n");

    // 演示：Fn 闭包可以用在任何要求 FnMut 或 FnOnce 的地方
    let simple_closure = |x: i32| x * 2; // 这是一个 Fn 闭包

    // 可以作为 Fn 使用
    fn needs_fn(f: impl Fn(i32) -> i32) -> i32 {
        f(5) + f(10) // 多次调用
    }

    // 可以作为 FnMut 使用
    fn needs_fn_mut(mut f: impl FnMut(i32) -> i32) -> i32 {
        f(5) + f(10)
    }

    // 可以作为 FnOnce 使用
    fn needs_fn_once(f: impl FnOnce(i32) -> i32) -> i32 {
        f(5) // 只调用一次
    }

    println!("Fn 闭包作为 Fn:     {}", needs_fn(simple_closure));
    println!("Fn 闭包作为 FnMut:  {}", needs_fn_mut(simple_closure));
    println!("Fn 闭包作为 FnOnce: {}", needs_fn_once(simple_closure));

    // FnMut 闭包不能作为 Fn 使用
    // 下面演示 FnMut 闭包
    let mut counter_val = 0;
    let mut fn_mut_closure = |x: i32| {
        counter_val += x;
        counter_val
    };

    // fn_mut_closure 不能传给 needs_fn，因为它修改了捕获的变量
    // println!("{}", needs_fn(fn_mut_closure)); // 编译错误！
    println!("FnMut 闭包作为 FnMut: {}", needs_fn_mut(&mut fn_mut_closure));

    // 打印层次关系
    println!("\n  层次关系图:");
    println!("  ┌──────────────────────────────────────┐");
    println!("  │            FnOnce (self)              │ ← 所有闭包都实现");
    println!("  │  ┌────────────────────────────────┐   │");
    println!("  │  │        FnMut (&mut self)       │   │ ← 不消耗捕获值的闭包");
    println!("  │  │  ┌──────────────────────────┐  │   │");
    println!("  │  │  │       Fn (&self)         │  │   │ ← 不修改捕获值的闭包");
    println!("  │  │  └──────────────────────────┘  │   │");
    println!("  │  └────────────────────────────────┘   │");
    println!("  └──────────────────────────────────────┘");
    println!();

    // ---------------------------------------------------------
    // 5. 如何选择正确的 trait bound
    // ---------------------------------------------------------
    println!("=== 5. 如何选择正确的 trait bound ===\n");

    // 规则：尽可能使用最宽松的约束（给调用者更大的灵活性）
    // - 如果函数只调用闭包一次 → 使用 FnOnce（最宽松，接受所有闭包）
    // - 如果函数需要多次调用闭包，且闭包可能修改状态 → 使用 FnMut
    // - 如果函数需要多次调用闭包，且不允许修改状态 → 使用 Fn（最严格）

    // 示例 1: 只调用一次 - 使用 FnOnce
    fn run_once<F: FnOnce() -> String>(f: F) -> String {
        f()
    }

    let data = String::from("hello");
    let result = run_once(move || format!("处理: {}", data));
    println!("run_once 结果: {}", result);

    // 示例 2: 可能多次调用，允许修改 - 使用 FnMut
    fn retry<F: FnMut() -> bool>(mut f: F, max_attempts: u32) -> bool {
        for attempt in 1..=max_attempts {
            println!("  第 {} 次尝试...", attempt);
            if f() {
                println!("  成功！");
                return true;
            }
        }
        println!("  所有尝试都失败了");
        false
    }

    let mut attempt_count = 0;
    retry(
        || {
            attempt_count += 1;
            attempt_count >= 3 // 第3次才成功
        },
        5,
    );

    // 示例 3: 多次调用，不允许修改 - 使用 Fn
    fn apply_to_list<F: Fn(i32) -> i32>(f: F, list: &[i32]) -> Vec<i32> {
        list.iter().map(|&x| f(x)).collect()
    }

    let results = apply_to_list(|x| x * x, &[1, 2, 3, 4, 5]);
    println!("apply_to_list 平方: {:?}", results);

    // 标准库中的例子
    println!("\n--- 标准库中的 trait bound 选择 ---");
    println!("  Iterator::map       使用 FnMut  - 可能需要修改状态");
    println!("  Iterator::filter    使用 FnMut  - 但通常传入 Fn");
    println!("  Iterator::for_each  使用 FnMut  - 对每个元素执行操作");
    println!("  Option::map         使用 FnOnce - 最多调用一次");
    println!("  Option::unwrap_or_else 使用 FnOnce - 最多调用一次");
    println!("  thread::spawn       使用 FnOnce - 线程函数只执行一次");
    println!();

    // ---------------------------------------------------------
    // 6. 综合示例：可配置的数据管道
    // ---------------------------------------------------------
    println!("=== 6. 综合示例：可配置的数据管道 ===\n");

    struct Pipeline {
        steps: Vec<Box<dyn Fn(i32) -> i32>>,
    }

    impl Pipeline {
        fn new() -> Self {
            Pipeline { steps: Vec::new() }
        }

        fn add_step(mut self, step: impl Fn(i32) -> i32 + 'static) -> Self {
            self.steps.push(Box::new(step));
            self
        }

        fn execute(&self, input: i32) -> i32 {
            let mut result = input;
            for (i, step) in self.steps.iter().enumerate() {
                let new_result = step(result);
                println!("  步骤 {}: {} -> {}", i + 1, result, new_result);
                result = new_result;
            }
            result
        }
    }

    let pipeline = Pipeline::new()
        .add_step(|x| x + 10)   // 步骤1: 加10
        .add_step(|x| x * 2)    // 步骤2: 乘2
        .add_step(|x| x - 5)    // 步骤3: 减5
        .add_step(|x| x / 3);   // 步骤4: 除3

    let result = pipeline.execute(5);
    println!("管道结果: {}\n", result);

    // 带有捕获的管道步骤
    let multiplier = 100;
    let pipeline2 = Pipeline::new()
        .add_step(|x| x + 1)
        .add_step(move |x| x * multiplier);

    let result2 = pipeline2.execute(3);
    println!("管道2结果: {}", result2);

    // ---------------------------------------------------------
    // 7. 编译器如何自动推断闭包的 trait
    // ---------------------------------------------------------
    println!("\n=== 7. 编译器如何推断闭包实现的 trait ===\n");

    // 编译器根据闭包体内的行为自动决定实现哪些 trait：
    //
    // 1. 如果闭包不捕获任何变量，或只以 &T 方式使用捕获变量
    //    → 实现 Fn + FnMut + FnOnce
    //
    // 2. 如果闭包以 &mut T 方式使用捕获变量
    //    → 实现 FnMut + FnOnce（不实现 Fn）
    //
    // 3. 如果闭包将捕获变量移出（消耗）
    //    → 只实现 FnOnce

    // 示例对照
    let val = String::from("hello");

    // Fn：只读取 val
    let fn_closure = || println!("  读取: {}", val);
    fn_closure();
    fn_closure(); // ✓ 可以多次调用

    // FnMut 的例子（需要新的作用域避免借用冲突）
    {
        let mut data = vec![1, 2, 3];
        let mut fn_mut_closure = || {
            data.push(4); // 修改 data
        };
        fn_mut_closure();
        fn_mut_closure(); // ✓ 可以多次调用
        println!("  修改后: {:?}", data);
    }

    // FnOnce：消耗捕获的变量
    let data_to_consume = vec![10, 20, 30];
    let fn_once_closure = || {
        drop(data_to_consume); // 消耗 data_to_consume
        println!("  数据已被消耗");
    };
    fn_once_closure();
    // fn_once_closure(); // ✗ 不能再次调用

    println!("\n🎉 恭喜！你已经完成了 Fn 系列 Trait 的学习！");
}
