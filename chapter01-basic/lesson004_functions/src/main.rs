/// # Lesson 004 - 函数
///
/// 本课学习 Rust 中函数的定义和使用。
///
/// ## 学习目标
/// - 掌握函数定义语法（fn）
/// - 理解参数与返回值
/// - 区分表达式（expression）与语句（statement）
/// - 学会通过元组返回多个值
/// - 掌握提前返回（return）
/// - 了解函数作为值传递
/// - 了解嵌套函数
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson004_functions
/// ```

// =============================================================
// Lesson 004: 函数
// =============================================================

fn main() {
    println!("=== Lesson 004: 函数 ===\n");

    // ---------------------------------------------------------
    // 1. 基本函数定义
    // ---------------------------------------------------------
    println!("--- 1. 基本函数定义 ---");

    // 调用一个无参数无返回值的函数
    greet();

    // Rust 中函数可以在 main 之前或之后定义（不像 C 语言需要前置声明）
    say_hello("Rust");
    say_hello("世界");

    // ---------------------------------------------------------
    // 2. 参数与返回值
    // ---------------------------------------------------------
    println!("\n--- 2. 参数与返回值 ---");

    // 带参数的函数
    print_value(42);

    // 带多个参数的函数（每个参数都必须标注类型）
    print_sum(3, 7);

    // 带返回值的函数
    let result = add(10, 20);
    println!("add(10, 20) = {}", result);

    // 返回值可以直接用在表达式中
    println!("add(100, 200) + 50 = {}", add(100, 200) + 50);

    // 不同类型的参数
    let area = rectangle_area(5.0, 3.0);
    println!("长方形面积 (5.0 × 3.0) = {}", area);

    let circle = circle_area(10.0);
    println!("圆的面积 (r=10.0) = {:.2}", circle);

    // ---------------------------------------------------------
    // 3. 表达式 vs 语句
    // ---------------------------------------------------------
    println!("\n--- 3. 表达式 vs 语句 ---");

    // 语句（Statement）：执行操作但不返回值，以分号结尾
    // 表达式（Expression）：计算并返回一个值，没有分号

    // let 是语句，不返回值
    let _x = 5; // 语句

    // 以下不合法（let 语句不返回值）：
    // let x = (let y = 6); // ❌

    // 花括号块是表达式
    let y = {
        let x = 3;
        x + 1 // 注意：没有分号！这是一个表达式，返回 4
    };
    println!("块表达式的值: {}", y);

    // 如果加了分号，就变成语句，返回 ()
    let z: () = {
        let _x = 3;
        // x + 1; // 加分号变成语句，块返回 ()
    };
    println!("加分号后返回单元类型: {:?}", z);

    // 函数体中最后一个表达式是返回值
    let five = return_five();
    println!("return_five() = {}", five);

    // if 也是表达式
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if 表达式: {}", number);

    // ---------------------------------------------------------
    // 4. 多返回值（通过元组）
    // ---------------------------------------------------------
    println!("\n--- 4. 多返回值（元组） ---");

    let (sum, product) = sum_and_product(3, 4);
    println!("sum_and_product(3, 4): 和={}, 积={}", sum, product);

    let (min, max, avg) = statistics(10.0, 20.0, 30.0);
    println!("statistics(10, 20, 30): 最小={}, 最大={}, 平均={}", min, max, avg);

    // 解构赋值时忽略部分值
    let (_, max_only, _) = statistics(1.0, 2.0, 3.0);
    println!("只取最大值: {}", max_only);

    // 返回带有含义的元组
    let (quotient, remainder) = div_rem(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);

    // ---------------------------------------------------------
    // 5. 提前返回（return）
    // ---------------------------------------------------------
    println!("\n--- 5. 提前返回（return） ---");

    // return 可以在函数任意位置提前返回
    println!("check_positive(5) = {}", check_positive(5));
    println!("check_positive(-3) = {}", check_positive(-3));
    println!("check_positive(0) = {}", check_positive(0));

    // 在复杂逻辑中使用 return
    let grade = get_grade(85);
    println!("85 分的等级: {}", grade);
    let grade = get_grade(92);
    println!("92 分的等级: {}", grade);
    let grade = get_grade(45);
    println!("45 分的等级: {}", grade);

    // 找到第一个偶数的索引
    let numbers = [1, 3, 5, 8, 11];
    match find_first_even(&numbers) {
        Some(index) => println!("第一个偶数在索引 {} 处", index),
        None => println!("没有找到偶数"),
    }

    // ---------------------------------------------------------
    // 6. 函数作为值
    // ---------------------------------------------------------
    println!("\n--- 6. 函数作为值 ---");

    // 函数指针：可以将函数赋值给变量
    let operation: fn(i32, i32) -> i32 = add;
    println!("operation(3, 4) = {}", operation(3, 4));

    // 切换函数
    let operation: fn(i32, i32) -> i32 = multiply;
    println!("operation(3, 4) = {}", operation(3, 4));

    // 将函数作为参数传递
    println!("apply(5, 3, add) = {}", apply(5, 3, add));
    println!("apply(5, 3, multiply) = {}", apply(5, 3, multiply));
    println!("apply(5, 3, subtract) = {}", apply(5, 3, subtract));

    // 使用函数数组
    let operations: [fn(i32, i32) -> i32; 3] = [add, subtract, multiply];
    let names = ["加法", "减法", "乘法"];
    let (a, b) = (10, 3);
    for (op, name) in operations.iter().zip(names.iter()) {
        println!("{}({}, {}) = {}", name, a, b, op(a, b));
    }

    // 返回函数
    let op = choose_operation('+');
    println!("choose_operation('+')(10, 5) = {}", op(10, 5));
    let op = choose_operation('*');
    println!("choose_operation('*')(10, 5) = {}", op(10, 5));

    // ---------------------------------------------------------
    // 7. 嵌套函数
    // ---------------------------------------------------------
    println!("\n--- 7. 嵌套函数 ---");

    // Rust 允许在函数内部定义函数
    fn inner_greeting(name: &str) {
        // 嵌套函数不能访问外部函数的局部变量
        // （如果需要捕获环境，使用闭包 —— 后续课程会学）
        println!("  来自嵌套函数的问候: 你好, {}!", name);
    }

    inner_greeting("内部函数");

    // 嵌套函数实现辅助逻辑
    fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }

    let temp_c = 100.0;
    let temp_f = celsius_to_fahrenheit(temp_c);
    println!("{}°C = {}°F", temp_c, temp_f);
    println!("{}°F = {:.1}°C", temp_f, fahrenheit_to_celsius(temp_f));

    // 嵌套函数中的递归
    fn factorial(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    for n in 0..=10 {
        println!("{}! = {}", n, factorial(n));
    }

    // ---------------------------------------------------------
    // 8. 发散函数（永不返回的函数）—— 了解即可
    // ---------------------------------------------------------
    println!("\n--- 8. 发散函数（补充） ---");

    // 返回类型为 ! 的函数永不返回（如 panic!、loop 无限循环等）
    // fn never_returns() -> ! {
    //     panic!("这个函数永不返回");
    // }

    // ! 类型可以被转换为任意类型，所以可以在 match 等场景使用
    let _value: i32 = if true {
        42
    } else {
        // panic! 返回 !，可以匹配任何类型
        panic!("不会执行到这里")
    };
    println!("发散函数的返回类型是 !（never type）");
    println!("常见的发散函数: panic!(), todo!(), unimplemented!(), loop {{}}");

    // ---------------------------------------------------------
    // 9. 小结
    // ---------------------------------------------------------
    println!("\n--- 小结 ---");
    println!("✅ fn 定义函数，参数必须标注类型");
    println!("✅ 函数体最后一个表达式（无分号）作为返回值");
    println!("✅ 表达式有值，语句没有值");
    println!("✅ 用元组返回多个值");
    println!("✅ return 可以提前返回");
    println!("✅ 函数可以作为值传递（函数指针 fn）");
    println!("✅ 支持嵌套函数定义");

    println!("\n🎉 恭喜！你已经完成了第四课！");
}

// =============================================================
// 以下是 main 外部定义的函数
// =============================================================

/// 无参数无返回值的函数
fn greet() {
    println!("你好！这是一个无参数函数。");
}

/// 带一个参数的函数
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

/// 打印一个值
fn print_value(value: i32) {
    println!("值是: {}", value);
}

/// 打印两个数的和
fn print_sum(a: i32, b: i32) {
    println!("{} + {} = {}", a, b, a + b);
}

/// 返回两个数的和
/// 注意：函数体最后一个表达式（没有分号）就是返回值
fn add(a: i32, b: i32) -> i32 {
    a + b // 没有分号 —— 这是一个表达式，作为返回值
}

/// 乘法
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 减法
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 返回五
fn return_five() -> i32 {
    5 // 直接返回表达式的值
}

/// 计算长方形面积
fn rectangle_area(width: f64, height: f64) -> f64 {
    width * height
}

/// 计算圆的面积
fn circle_area(radius: f64) -> f64 {
    std::f64::consts::PI * radius * radius
}

/// 返回多个值：和与积
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

/// 返回三个统计值：最小值、最大值、平均值
fn statistics(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let min = a.min(b).min(c);
    let max = a.max(b).max(c);
    let avg = (a + b + c) / 3.0;
    (min, max, avg)
}

/// 返回商和余数
fn div_rem(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

/// 检查正负：使用 return 提前返回
fn check_positive(n: i32) -> &'static str {
    if n > 0 {
        return "正数";
    }
    if n < 0 {
        return "负数";
    }
    "零" // 最后一个表达式，不需要 return
}

/// 根据分数返回等级
fn get_grade(score: u32) -> &'static str {
    if score >= 90 {
        return "A（优秀）";
    }
    if score >= 80 {
        return "B（良好）";
    }
    if score >= 70 {
        return "C（中等）";
    }
    if score >= 60 {
        return "D（及格）";
    }
    "F（不及格）"
}

/// 找到第一个偶数的索引
fn find_first_even(numbers: &[i32]) -> Option<usize> {
    for (index, &num) in numbers.iter().enumerate() {
        if num % 2 == 0 {
            return Some(index);
        }
    }
    None
}

/// 接受函数作为参数
fn apply(a: i32, b: i32, f: fn(i32, i32) -> i32) -> i32 {
    f(a, b)
}

/// 返回函数指针
fn choose_operation(op: char) -> fn(i32, i32) -> i32 {
    match op {
        '+' => add,
        '-' => subtract,
        '*' => multiply,
        _ => add, // 默认返回加法
    }
}
