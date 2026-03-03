/// # Lesson 037 - 闭包作为参数
///
/// 闭包可以作为函数参数传递，这是 Rust 中实现高阶函数的核心方式。
///
/// ## 学习目标
/// - 使用 `impl Fn(i32) -> i32` 作为参数类型
/// - 使用泛型闭包参数 `fn apply<F: Fn()>(f: F)`
/// - 理解函数指针 `fn()` 与闭包的区别
/// - 学会从函数中返回闭包 `-> impl Fn()`
/// - 将闭包存储在结构体中
///
/// ## 运行方式
/// 在项目根目录下执行:
/// ```bash
/// cargo run -p lesson037_closure_as_param
/// ```

// =============================================================
// Lesson 037: 闭包作为参数 - Closures as Parameters
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 使用 impl Fn 作为参数类型
    // ---------------------------------------------------------
    // `impl Fn(i32) -> i32` 表示接受任何实现了 Fn(i32) -> i32 的类型
    // 这是最简洁的写法，适合大多数场景
    println!("=== 1. 使用 impl Fn 作为参数类型 ===\n");

    // 定义一个接受闭包作为参数的函数
    fn apply_to_five(f: impl Fn(i32) -> i32) -> i32 {
        f(5)
    }

    // 传入不同的闭包
    let result1 = apply_to_five(|x| x + 1);
    let result2 = apply_to_five(|x| x * 2);
    let result3 = apply_to_five(|x| x * x);

    println!("f(5) = x + 1 -> {}", result1);
    println!("f(5) = x * 2 -> {}", result2);
    println!("f(5) = x * x -> {}", result3);

    // 也可以传入捕获环境变量的闭包
    let offset = 100;
    let result4 = apply_to_five(|x| x + offset);
    println!("f(5) = x + {} -> {}", offset, result4);

    // 使用 impl Fn 接受无参数的闭包
    fn run_action(action: impl Fn()) {
        println!("  执行动作前...");
        action();
        println!("  执行动作后...");
    }

    run_action(|| println!("  >> 我是一个闭包动作！"));
    println!();

    // ---------------------------------------------------------
    // 2. 泛型闭包参数
    // ---------------------------------------------------------
    // 使用泛型参数 + trait bound 是另一种常见写法
    // 与 impl Fn 相比，泛型写法更灵活（可以在多个地方使用同一个类型 F）
    println!("=== 2. 泛型闭包参数 ===\n");

    // 使用泛型 + trait bound
    fn apply_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(f(x))
    }

    let double = |x| x * 2;
    println!("apply_twice(double, 3) = {}", apply_twice(double, 3)); // 3 -> 6 -> 12

    let add_ten = |x| x + 10;
    println!("apply_twice(add_ten, 5) = {}", apply_twice(add_ten, 5)); // 5 -> 15 -> 25

    // 使用 where 子句（更清晰的写法）
    fn apply_and_print<F>(f: F, value: i32)
    where
        F: Fn(i32) -> i32,
    {
        let result = f(value);
        println!("  f({}) = {}", value, result);
    }

    apply_and_print(|x| x * 3, 7);
    apply_and_print(|x| x - 1, 10);

    // 接受多个闭包参数
    fn combine<F, G>(f: F, g: G, x: i32) -> i32
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
    {
        g(f(x))
    }

    let result = combine(|x| x + 1, |x| x * 2, 5);
    println!("combine(+1, *2, 5) = {} (先加1得6，再乘2得12)", result);

    let result = combine(|x| x * 2, |x| x + 1, 5);
    println!("combine(*2, +1, 5) = {} (先乘2得10，再加1得11)", result);
    println!();

    // ---------------------------------------------------------
    // 3. 函数指针 fn()
    // ---------------------------------------------------------
    // fn() 是函数指针类型，它只能指向不捕获环境的函数或闭包
    println!("=== 3. 函数指针 fn() ===\n");

    // 普通函数可以作为函数指针
    fn square(x: i32) -> i32 {
        x * x
    }

    // fn(i32) -> i32 是函数指针类型
    let f: fn(i32) -> i32 = square;
    println!("函数指针 square(4) = {}", f(4));

    // 不捕获环境的闭包也可以转为函数指针
    let f2: fn(i32) -> i32 = |x| x + 100;
    println!("闭包函数指针 f2(4) = {}", f2(4));

    // 函数指针实现了所有三种 Fn trait（Fn、FnMut、FnOnce）
    // 所以函数指针可以传给任何接受闭包的函数
    fn apply_fn(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }
    println!("传入普通函数: {}", apply_fn(square, 5));

    // 函数指针用于接受函数的场景
    fn do_math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
        op(a, b)
    }

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    println!("do_math(add, 3, 4) = {}", do_math(add, 3, 4));
    println!("do_math(multiply, 3, 4) = {}", do_math(multiply, 3, 4));

    // 函数指针 vs 闭包 trait 的区别
    // fn() - 具体类型，大小已知，不能捕获环境
    // impl Fn() - 泛型参数，编译时单态化，可以捕获环境
    // dyn Fn() - trait 对象，运行时动态分发，可以捕获环境

    // 函数指针在 C FFI 中也很有用
    println!();

    // ---------------------------------------------------------
    // 4. 返回闭包
    // ---------------------------------------------------------
    // 函数可以返回闭包，使用 impl Fn 或 Box<dyn Fn>
    println!("=== 4. 返回闭包 ===\n");

    // 使用 impl Fn 返回闭包（最常见的方式）
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n // 必须使用 move，因为 n 会在函数返回后失效
    }

    let add_5 = make_adder(5);
    let add_10 = make_adder(10);
    println!("add_5(3) = {}", add_5(3));
    println!("add_10(3) = {}", add_10(3));

    // 返回带有状态的闭包
    fn make_counter() -> impl FnMut() -> i32 {
        let mut count = 0;
        move || {
            count += 1;
            count
        }
    }

    let mut counter = make_counter();
    println!("counter() = {}", counter());
    println!("counter() = {}", counter());
    println!("counter() = {}", counter());

    // 创建一个乘法器工厂
    fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
        move |x| x * factor
    }

    let double = make_multiplier(2);
    let triple = make_multiplier(3);
    println!("double(7) = {}", double(7));
    println!("triple(7) = {}", triple(7));

    // 使用 Box<dyn Fn> 在需要动态分发时返回不同类型的闭包
    fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
        match op {
            "add" => Box::new(|a, b| a + b),
            "sub" => Box::new(|a, b| a - b),
            "mul" => Box::new(|a, b| a * b),
            _ => Box::new(|a, b| a + b), // 默认加法
        }
    }

    let op_add = make_operation("add");
    let op_mul = make_operation("mul");
    println!("add(10, 3) = {}", op_add(10, 3));
    println!("mul(10, 3) = {}", op_mul(10, 3));
    println!();

    // ---------------------------------------------------------
    // 5. 闭包存储在结构体中
    // ---------------------------------------------------------
    // 闭包可以作为结构体的字段，但需要使用泛型或 trait 对象
    println!("=== 5. 闭包存储在结构体中 ===\n");

    // 方式一：使用泛型（编译时确定类型，零开销）
    struct Transformer<F>
    where
        F: Fn(i32) -> i32,
    {
        name: String,
        transform: F,
    }

    impl<F> Transformer<F>
    where
        F: Fn(i32) -> i32,
    {
        fn new(name: &str, transform: F) -> Self {
            Transformer {
                name: name.to_string(),
                transform,
            }
        }

        fn apply(&self, value: i32) -> i32 {
            (self.transform)(value)
        }
    }

    let doubler = Transformer::new("翻倍器", |x| x * 2);
    let squarer = Transformer::new("平方器", |x| x * x);

    println!("{}(5) = {}", doubler.name, doubler.apply(5));
    println!("{}(5) = {}", squarer.name, squarer.apply(5));

    // 方式二：使用 Box<dyn Fn>（运行时动态分发，可以存储不同闭包）
    struct Callback {
        name: String,
        handler: Box<dyn Fn(String) -> String>,
    }

    impl Callback {
        fn new(name: &str, handler: Box<dyn Fn(String) -> String>) -> Self {
            Callback {
                name: name.to_string(),
                handler,
            }
        }

        fn execute(&self, input: String) -> String {
            (self.handler)(input)
        }
    }

    let upper_cb = Callback::new(
        "转大写",
        Box::new(|s: String| s.to_uppercase()),
    );
    let exclaim_cb = Callback::new(
        "加感叹号",
        Box::new(|s: String| format!("{}!!!", s)),
    );

    println!("{}(\"hello\") = {}", upper_cb.name, upper_cb.execute("hello".to_string()));
    println!("{}(\"wow\") = {}", exclaim_cb.name, exclaim_cb.execute("wow".to_string()));

    // 方式三：使用 Vec 存储多个回调（必须使用 Box<dyn Fn>）
    let callbacks: Vec<Callback> = vec![upper_cb, exclaim_cb];

    println!("\n--- 批量执行回调 ---");
    for cb in &callbacks {
        let result = cb.execute("rust".to_string());
        println!("  {}: {}", cb.name, result);
    }

    // ---------------------------------------------------------
    // 6. 实际应用：事件处理系统
    // ---------------------------------------------------------
    println!("\n=== 6. 实际应用：事件处理系统 ===\n");

    struct EventEmitter {
        listeners: Vec<Box<dyn Fn(&str)>>,
    }

    impl EventEmitter {
        fn new() -> Self {
            EventEmitter {
                listeners: Vec::new(),
            }
        }

        fn on(&mut self, listener: impl Fn(&str) + 'static) {
            self.listeners.push(Box::new(listener));
        }

        fn emit(&self, event: &str) {
            for listener in &self.listeners {
                listener(event);
            }
        }
    }

    let mut emitter = EventEmitter::new();

    // 注册多个事件监听器
    emitter.on(|event| println!("  监听器1: 收到事件 '{}'", event));
    emitter.on(|event| println!("  监听器2: 记录事件 '{}'", event));

    let prefix = String::from("LOG");
    emitter.on(move |event| println!("  监听器3: [{}] {}", prefix, event));

    // 触发事件
    emitter.emit("用户登录");
    println!("---");
    emitter.emit("数据更新");

    println!("\n🎉 恭喜！你已经完成了闭包作为参数的学习！");
}
