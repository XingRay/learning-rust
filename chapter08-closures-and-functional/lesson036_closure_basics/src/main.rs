/// # Lesson 036 - 闭包基础
///
/// 闭包（Closure）是可以捕获其所在环境中变量的匿名函数。
///
/// ## 学习目标
/// - 掌握闭包的基本语法 `|x| x + 1`
/// - 理解闭包的类型推断机制
/// - 理解闭包捕获环境变量的三种方式：不可变借用、可变借用、移动
/// - 掌握 `move` 关键字的使用
/// - 对比闭包与普通函数的区别
///
/// ## 运行方式
/// 在项目根目录下执行:
/// ```bash
/// cargo run -p lesson036_closure_basics
/// ```

// =============================================================
// Lesson 036: 闭包基础 - Closure Basics
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 闭包的基本语法
    // ---------------------------------------------------------
    // 闭包使用 |参数| 表达式 的语法，类似于匿名函数
    println!("=== 1. 闭包的基本语法 ===\n");

    // 最简单的闭包：接受一个参数，返回参数加 1
    let add_one = |x| x + 1;
    println!("add_one(5) = {}", add_one(5));

    // 带类型注解的闭包（通常不需要，因为编译器可以推断）
    let add_two = |x: i32| -> i32 { x + 2 };
    println!("add_two(5) = {}", add_two(5));

    // 多个参数的闭包
    let add = |a, b| a + b;
    println!("add(3, 4) = {}", add(3, 4));

    // 无参数的闭包
    let greet = || println!("你好，闭包！");
    greet();

    // 多行闭包体使用花括号
    let calculate = |x: i32, y: i32| {
        let sum = x + y;
        let product = x * y;
        println!("  {} + {} = {}", x, y, sum);
        println!("  {} * {} = {}", x, y, product);
        sum + product // 最后一个表达式作为返回值
    };
    let result = calculate(3, 4);
    println!("calculate(3, 4) 返回: {}\n", result);

    // ---------------------------------------------------------
    // 2. 闭包的类型推断
    // ---------------------------------------------------------
    // 闭包的参数和返回类型通常由编译器根据使用上下文推断
    println!("=== 2. 闭包的类型推断 ===\n");

    // 编译器根据第一次调用推断类型
    let identity = |x| x;

    // 第一次调用决定了闭包的类型为 i32 -> i32
    let val = identity(42);
    println!("identity(42) = {}", val);

    // 注意：如果之后尝试用不同类型调用，将会编译错误
    // let s = identity("hello"); // 错误！类型已经被推断为 i32

    // 对比：函数签名必须显式标注类型
    fn add_one_fn(x: i32) -> i32 {
        x + 1
    }
    println!("add_one_fn(10) = {}", add_one_fn(10));

    // 闭包可以根据上下文推断不同的类型
    let double_int = |x: i32| x * 2;
    let double_float = |x: f64| x * 2.0;
    println!("double_int(5) = {}", double_int(5));
    println!("double_float(5.5) = {}\n", double_float(5.5));

    // ---------------------------------------------------------
    // 3. 捕获环境变量 - 不可变借用
    // ---------------------------------------------------------
    // 闭包可以引用定义它的作用域中的变量
    // 最常见的方式是不可变借用
    println!("=== 3. 捕获环境变量 - 不可变借用 ===\n");

    let name = String::from("Rust");
    let age = 10;

    // 闭包通过不可变引用捕获 name 和 age
    let print_info = || {
        println!("  语言: {}, 年龄: {} 岁", name, age);
    };

    // 原变量仍可使用（因为只是不可变借用）
    println!("外部访问 name: {}", name);
    print_info(); // 调用闭包
    println!("外部仍可访问 name: {}", name);

    // 多次调用闭包也没问题
    print_info();

    // 不可变借用允许同时存在多个引用
    let another_ref = &name;
    println!("另一个引用: {}\n", another_ref);

    // ---------------------------------------------------------
    // 4. 捕获环境变量 - 可变借用
    // ---------------------------------------------------------
    // 闭包也可以通过可变引用捕获变量
    println!("=== 4. 捕获环境变量 - 可变借用 ===\n");

    let mut count = 0;

    // 闭包通过可变引用捕获 count
    // 注意：闭包本身也需要是 mut 的
    let mut increment = || {
        count += 1;
        println!("  count = {}", count);
    };

    // 在闭包存活期间，不能再借用 count
    // println!("{}", count); // 错误！可变借用已被闭包持有

    increment();
    increment();
    increment();

    // 闭包不再使用后，可以再次访问 count
    // （increment 的最后一次使用在上面，之后闭包的借用就结束了）
    println!("最终 count = {}\n", count);

    // 另一个可变借用的例子：向 Vec 添加元素
    let mut names = vec!["Alice".to_string()];

    let mut add_name = |name: &str| {
        names.push(name.to_string());
        println!("  当前列表: {:?}", names);
    };

    add_name("Bob");
    add_name("Charlie");

    // 闭包 add_name 的借用在这里结束
    println!("最终列表: {:?}\n", names);

    // ---------------------------------------------------------
    // 5. 捕获环境变量 - 移动（所有权转移）
    // ---------------------------------------------------------
    // 当闭包消耗了捕获的变量时，变量的所有权会转移到闭包中
    println!("=== 5. 捕获环境变量 - 移动 ===\n");

    let data = vec![1, 2, 3, 4, 5];

    // 这个闭包获取了 data 的所有权，因为 drop 会消耗它
    let consume_data = || {
        let moved_data = data; // data 的所有权被移动到闭包内部变量
        println!("  闭包内部使用: {:?}", moved_data);
        // moved_data 在这里被丢弃
    };

    // data 的所有权已转移给闭包，外部不能再使用
    // println!("{:?}", data); // 错误！data 已被移动

    consume_data();
    // consume_data(); // 错误！闭包只能调用一次，因为它消耗了捕获的值
    println!();

    // ---------------------------------------------------------
    // 6. move 关键字
    // ---------------------------------------------------------
    // move 关键字强制闭包获取所有权，即使闭包体中只是读取变量
    println!("=== 6. move 关键字 ===\n");

    let message = String::from("Hello from main");

    // 使用 move 关键字强制闭包获取 message 的所有权
    let print_message = move || {
        println!("  闭包中: {}", message);
    };

    // message 的所有权已转移给闭包
    // println!("{}", message); // 错误！message 已被 move 到闭包中

    print_message();
    print_message(); // 可以多次调用，因为闭包内部只是借用

    // move 闭包在多线程中尤其重要
    // 因为新线程可能比创建它的线程活得更久
    // 所以必须获取变量的所有权
    let data_for_thread = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        // 如果不加 move，编译器会报错
        // 因为 data_for_thread 可能在主线程中被释放
        println!("  线程中: {:?}", data_for_thread);
    });
    handle.join().unwrap();

    // move 对于 Copy 类型的行为
    let number = 42; // i32 实现了 Copy trait
    let print_number = move || {
        println!("  闭包中的数字: {}", number);
    };
    // Copy 类型在 move 后原变量仍可使用（因为是复制而非移动）
    println!("外部的数字: {}", number);
    print_number();
    println!();

    // ---------------------------------------------------------
    // 7. 闭包 vs 函数的对比
    // ---------------------------------------------------------
    println!("=== 7. 闭包 vs 函数 ===\n");

    // 函数：不能捕获环境变量
    fn multiply_by_two(x: i32) -> i32 {
        // let _ = some_outer_variable; // 错误！函数不能访问外部变量
        x * 2
    }

    // 闭包：可以捕获环境变量
    let factor = 3;
    let multiply_by_factor = |x| x * factor;

    println!("函数 multiply_by_two(5) = {}", multiply_by_two(5));
    println!("闭包 multiply_by_factor(5) = {}", multiply_by_factor(5));

    // 函数必须声明参数类型，闭包可以推断
    fn square(x: i32) -> i32 {
        x * x
    }
    let square_closure = |x: i32| x * x;
    println!("函数 square(4) = {}", square(4));
    println!("闭包 square_closure(4) = {}", square_closure(4));

    // 函数可以作为函数指针传递
    let fn_ptr: fn(i32) -> i32 = multiply_by_two;
    println!("函数指针调用: {}", fn_ptr(10));

    // 闭包不能直接作为函数指针（如果它捕获了环境变量）
    // let fn_ptr2: fn(i32) -> i32 = multiply_by_factor; // 错误！

    // 但不捕获环境的闭包可以转为函数指针
    let no_capture_closure = |x: i32| x + 100;
    let fn_ptr3: fn(i32) -> i32 = no_capture_closure;
    println!("不捕获环境的闭包作为函数指针: {}", fn_ptr3(5));

    // ---------------------------------------------------------
    // 8. 综合示例：使用闭包进行数据处理
    // ---------------------------------------------------------
    println!("\n=== 8. 综合示例 ===\n");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 使用闭包过滤偶数
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // 使用闭包进行映射
    let doubled: Vec<i32> = numbers.iter().map(|&x| x * 2).collect();
    println!("翻倍: {:?}", doubled);

    // 捕获外部变量的闭包用于 filter
    let threshold = 5;
    let above_threshold: Vec<&i32> = numbers.iter().filter(|&&x| x > threshold).collect();
    println!("大于 {} 的数: {:?}", threshold, above_threshold);

    // 链式调用多个闭包
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)    // 过滤偶数
        .map(|&x| x * x)              // 平方
        .sum();                         // 求和
    println!("偶数的平方和: {}", result);

    println!("\n🎉 恭喜！你已经完成了闭包基础的学习！");
}
