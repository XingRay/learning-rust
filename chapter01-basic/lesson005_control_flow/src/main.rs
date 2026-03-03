/// # Lesson 005 - 控制流
///
/// 本课学习 Rust 中的控制流语句。
///
/// ## 学习目标
/// - 掌握 if/else if/else 条件判断
/// - 理解 if 作为表达式
/// - 掌握 loop、while、for 三种循环
/// - 学会使用循环标签（label）
/// - 掌握 break 带值返回和 continue
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson005_control_flow
/// ```

// =============================================================
// Lesson 005: 控制流
// =============================================================

fn main() {
    println!("=== Lesson 005: 控制流 ===\n");

    // ---------------------------------------------------------
    // 1. if / else if / else
    // ---------------------------------------------------------
    println!("--- 1. if / else if / else ---");

    let number = 7;

    // 基本 if/else
    if number > 0 {
        println!("{} 是正数", number);
    } else {
        println!("{} 不是正数", number);
    }

    // if / else if / else 链
    let score = 85;
    if score >= 90 {
        println!("成绩: A（优秀）");
    } else if score >= 80 {
        println!("成绩: B（良好）");
    } else if score >= 70 {
        println!("成绩: C（中等）");
    } else if score >= 60 {
        println!("成绩: D（及格）");
    } else {
        println!("成绩: F（不及格）");
    }

    // 注意：Rust 中 if 的条件必须是 bool 类型
    // if 1 { ... }     // ❌ 错误: 不会自动将数字转为布尔
    // if number { ... } // ❌ 错误: 必须是 bool

    // 多个条件组合
    let age = 25;
    let has_id = true;
    if age >= 18 && has_id {
        println!("允许进入（年龄满足且有证件）");
    }

    let temperature = -5;
    if temperature > 35 || temperature < 0 {
        println!("极端温度: {}°C", temperature);
    }

    // ---------------------------------------------------------
    // 2. if 作为表达式
    // ---------------------------------------------------------
    println!("\n--- 2. if 作为表达式 ---");

    // 在 Rust 中，if 是一个表达式，可以有返回值
    let condition = true;
    let value = if condition { 5 } else { 10 };
    println!("condition={}, value={}", condition, value);

    // 类似三元运算符（Rust 没有 ?: 运算符，用 if 代替）
    let x = 10;
    let abs_x = if x >= 0 { x } else { -x };
    println!("|{}| = {}", x, abs_x);

    // if 表达式的所有分支必须返回相同类型
    let number = 42;
    let description = if number % 2 == 0 {
        "偶数"
    } else {
        "奇数"
    };
    println!("{} 是{}", number, description);

    // if 表达式可以用在更复杂的地方
    let speed = 120;
    println!(
        "速度 {} km/h - {}",
        speed,
        if speed > 120 {
            "超速！"
        } else if speed > 100 {
            "偏快"
        } else {
            "正常"
        }
    );

    // 在 let 绑定中使用 if 表达式进行条件初始化
    let config_mode = "production";
    let max_connections = if config_mode == "production" { 1000 } else { 10 };
    println!("模式: {}, 最大连接数: {}", config_mode, max_connections);

    // ---------------------------------------------------------
    // 3. loop 无限循环
    // ---------------------------------------------------------
    println!("\n--- 3. loop 无限循环 ---");

    // loop 创建无限循环，用 break 退出
    let mut count = 0;
    loop {
        count += 1;
        if count >= 5 {
            println!("loop 循环了 {} 次后退出", count);
            break;
        }
    }

    // loop 倒计时
    let mut countdown = 3;
    loop {
        if countdown == 0 {
            println!("发射！🚀");
            break;
        }
        println!("倒计时: {}", countdown);
        countdown -= 1;
    }

    // ---------------------------------------------------------
    // 4. break 带值返回
    // ---------------------------------------------------------
    println!("\n--- 4. break 带值返回 ---");

    // loop 是表达式，break 可以返回值
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // 返回 20
        }
    };
    println!("loop 返回值: {}", result);

    // 实际用例：重试操作直到成功
    let mut attempt = 0;
    let data = loop {
        attempt += 1;
        // 模拟操作：第 3 次成功
        if attempt >= 3 {
            break format!("第 {} 次尝试成功", attempt);
        }
        println!("第 {} 次尝试失败，重试...", attempt);
    };
    println!("{}", data);

    // ---------------------------------------------------------
    // 5. loop 标签（嵌套循环控制）
    // ---------------------------------------------------------
    println!("\n--- 5. loop 标签 ---");

    // 使用标签（'label）来控制嵌套循环
    // 标签以单引号开头
    let mut found = false;
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i + j == 6 {
                println!("在 ({}, {}) 找到 i+j=6，跳出外层循环", i, j);
                found = true;
                break 'outer; // 直接跳出外层循环
            }
        }
    }
    println!("是否找到: {}", found);

    // 标签与 continue 配合
    println!("\n跳过外层循环的特定迭代:");
    'outer_loop: for i in 1..=4 {
        for j in 1..=4 {
            if i == 2 && j == 2 {
                println!("  跳过 i={} 的剩余迭代", i);
                continue 'outer_loop; // 跳到外层循环的下一次迭代
            }
            print!("({},{}) ", i, j);
        }
        println!(); // 换行
    }
    println!();

    // loop 也可以加标签
    let mut x = 0;
    let result = 'search: loop {
        x += 1;
        let mut y = 0;
        loop {
            y += 1;
            if x * y > 20 {
                break 'search (x, y); // 从外层 loop 返回值
            }
            if y > 10 {
                break; // 只跳出内层 loop
            }
        }
    };
    println!("找到 x*y > 20: ({}, {}), 乘积={}", result.0, result.1, result.0 * result.1);

    // ---------------------------------------------------------
    // 6. while 循环
    // ---------------------------------------------------------
    println!("\n--- 6. while 循环 ---");

    // while 循环：条件为 true 时继续
    let mut n = 1;
    while n <= 5 {
        print!("{} ", n);
        n += 1;
    }
    println!("(while 循环结束)");

    // 用 while 实现猜数字逻辑
    let target = 7;
    let mut guess = 1;
    while guess != target {
        guess += 1;
    }
    println!("猜了 {} 次找到目标 {}", guess, target);

    // while 与数组
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < arr.len() {
        print!("arr[{}]={} ", index, arr[index]);
        index += 1;
    }
    println!();

    // while 的条件可以很复杂
    let mut total = 0;
    let mut i = 1;
    while i <= 100 && total < 200 {
        total += i;
        i += 1;
    }
    println!("累加到 {}，总和={} (>= 200 或 i > 100 时停止)", i - 1, total);

    // ---------------------------------------------------------
    // 7. for 循环与范围（Range）
    // ---------------------------------------------------------
    println!("\n--- 7. for 循环与 Range ---");

    // for 循环是 Rust 中最常用的循环
    // 遍历范围（Range）
    print!("1..5 (不含5): ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    print!("1..=5 (包含5): ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // 反向迭代
    print!("反向 (5..=1): ");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // 遍历数组
    let fruits = ["苹果", "香蕉", "橙子", "葡萄"];
    print!("水果: ");
    for fruit in &fruits {
        print!("{} ", fruit);
    }
    println!();

    // 带索引的遍历
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}. {}", index + 1, fruit);
    }

    // 遍历字符串的字符
    print!("遍历字符: ");
    for ch in "你好Rust".chars() {
        print!("'{}' ", ch);
    }
    println!();

    // for 循环求和
    let mut sum = 0;
    for n in 1..=100 {
        sum += n;
    }
    println!("1+2+...+100 = {}", sum);

    // 步长遍历（使用 step_by）
    print!("偶数 (0..=10 step 2): ");
    for i in (0..=10).step_by(2) {
        print!("{} ", i);
    }
    println!();

    // ---------------------------------------------------------
    // 8. continue
    // ---------------------------------------------------------
    println!("\n--- 8. continue ---");

    // continue 跳过本次迭代，进入下一次
    print!("跳过 3 的倍数: ");
    for i in 1..=15 {
        if i % 3 == 0 {
            continue; // 跳过 3 的倍数
        }
        print!("{} ", i);
    }
    println!();

    // 只打印偶数
    print!("偶数: ");
    for i in 1..=10 {
        if i % 2 != 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!();

    // ---------------------------------------------------------
    // 9. 嵌套循环与标签（综合示例）
    // ---------------------------------------------------------
    println!("\n--- 9. 嵌套循环综合示例 ---");

    // 九九乘法表
    println!("九九乘法表:");
    for i in 1..=9 {
        for j in 1..=i {
            print!("{}×{}={:<4}", j, i, i * j);
        }
        println!();
    }

    // 寻找满足条件的数对
    println!("\n寻找 a² + b² = c² (c < 20):");
    'pythagorean: for c in 1..20 {
        for a in 1..c {
            for b in a..c {
                if a * a + b * b == c * c {
                    println!("  {}² + {}² = {}² ({} + {} = {})", a, b, c, a * a, b * b, c * c);
                    if c > 15 {
                        println!("  c > 15，停止搜索");
                        break 'pythagorean;
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------
    // 10. 实战示例：FizzBuzz
    // ---------------------------------------------------------
    println!("\n--- 10. 实战: FizzBuzz ---");

    for i in 1..=20 {
        let result = if i % 15 == 0 {
            String::from("FizzBuzz")
        } else if i % 3 == 0 {
            String::from("Fizz")
        } else if i % 5 == 0 {
            String::from("Buzz")
        } else {
            i.to_string()
        };
        print!("{:>8} ", result);
        if i % 5 == 0 {
            println!();
        }
    }

    // ---------------------------------------------------------
    // 11. 小结
    // ---------------------------------------------------------
    println!("\n--- 小结 ---");
    println!("✅ if/else if/else 条件判断，条件必须是 bool");
    println!("✅ if 是表达式，可以返回值（类似三元运算符）");
    println!("✅ loop 无限循环，break 退出，可以带值返回");
    println!("✅ while 条件循环");
    println!("✅ for 迭代循环，配合 Range 和迭代器使用");
    println!("✅ break 和 continue 控制循环流");
    println!("✅ 标签（'label）控制嵌套循环的 break/continue");

    println!("\n🎉 恭喜！你已经完成了第五课！");
}
