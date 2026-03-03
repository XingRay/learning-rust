/// # Lesson 027 - 迭代器基础
///
/// 迭代器是 Rust 中处理序列数据的核心抽象。
/// Rust 的迭代器是零成本抽象（zero-cost abstraction），
/// 编译后的性能与手写循环一样高效。
///
/// ## 学习目标
/// - 理解 Iterator trait 和 next() 方法
/// - 掌握 iter() / iter_mut() / into_iter() 的区别
/// - 理解 for 循环与迭代器的关系
/// - 学会手动调用 next() 方法
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson027_iterator_basics
/// ```

fn main() {
    // =============================================================
    // 1. Iterator trait 核心概念
    // =============================================================
    println!("===== 1. Iterator trait 核心概念 =====");

    // Iterator trait 的定义（简化版）：
    //
    // trait Iterator {
    //     type Item;                    // 关联类型：迭代器产出的元素类型
    //     fn next(&mut self) -> Option<Self::Item>;  // 核心方法
    //     // ... 还有很多默认方法（map, filter, fold 等）
    // }
    //
    // next() 返回 Some(item) 表示还有下一个元素
    // next() 返回 None 表示迭代结束

    println!("Iterator trait 的核心是 next() 方法");
    println!("next() 返回 Option<Item>:");
    println!("  Some(item) => 还有元素");
    println!("  None       => 迭代结束");

    // =============================================================
    // 2. 手动调用 next()
    // =============================================================
    println!("\n===== 2. 手动调用 next() =====");

    let numbers = vec![10, 20, 30];

    // 创建迭代器（注意：迭代器本身是惰性的，创建时不做任何事）
    let mut iter = numbers.iter();

    // 手动调用 next()
    println!("next() = {:?}", iter.next()); // Some(&10)
    println!("next() = {:?}", iter.next()); // Some(&20)
    println!("next() = {:?}", iter.next()); // Some(&30)
    println!("next() = {:?}", iter.next()); // None（已耗尽）
    println!("next() = {:?}", iter.next()); // None（继续返回 None）

    // 使用 while let 手动迭代
    println!("\n使用 while let 遍历:");
    let fruits = vec!["苹果", "香蕉", "橙子"];
    let mut fruit_iter = fruits.iter();
    while let Some(fruit) = fruit_iter.next() {
        println!("  {}", fruit);
    }

    // =============================================================
    // 3. iter() / iter_mut() / into_iter() 的区别
    // =============================================================
    println!("\n===== 3. iter() / iter_mut() / into_iter() =====");

    // --- iter(): 产生不可变引用 &T ---
    println!("--- iter(): 不可变引用 &T ---");
    let names = vec![String::from("Alice"), String::from("Bob"), String::from("Charlie")];

    for name in names.iter() {
        // name 的类型是 &String
        println!("  {} (长度: {})", name, name.len());
    }
    // names 仍然可用，因为 iter() 只借用了元素
    println!("  names 仍然可用: {:?}", names);

    // --- iter_mut(): 产生可变引用 &mut T ---
    println!("\n--- iter_mut(): 可变引用 &mut T ---");
    let mut scores = vec![80, 85, 90, 75, 95];
    println!("  加分前: {:?}", scores);

    for score in scores.iter_mut() {
        // score 的类型是 &mut i32
        *score += 5; // 每个分数加 5
    }
    println!("  加分后: {:?}", scores);

    // --- into_iter(): 获取所有权 T ---
    println!("\n--- into_iter(): 获取所有权 T ---");
    let cities = vec![
        String::from("北京"),
        String::from("上海"),
        String::from("广州"),
    ];

    for city in cities.into_iter() {
        // city 的类型是 String（拥有所有权）
        println!("  城市: {} (我拥有这个 String)", city);
    }
    // cities 不再可用，因为 into_iter() 消费了整个 Vec
    // println!("{:?}", cities); // 编译错误！

    // =============================================================
    // 4. 三种迭代方式的对比总结
    // =============================================================
    println!("\n===== 4. 三种迭代方式对比 =====");

    println!("┌─────────────┬──────────────┬──────────────────┐");
    println!("│ 方法        │ 产出类型     │ 原集合是否可用   │");
    println!("├─────────────┼──────────────┼──────────────────┤");
    println!("│ iter()      │ &T           │ 可用（只借用）   │");
    println!("│ iter_mut()  │ &mut T       │ 可用（可变借用） │");
    println!("│ into_iter() │ T            │ 不可用（被消费） │");
    println!("└─────────────┴──────────────┴──────────────────┘");

    // =============================================================
    // 5. for 循环与迭代器的关系
    // =============================================================
    println!("\n===== 5. for 循环与迭代器 =====");

    // for 循环实际上是迭代器的语法糖！
    // 以下两种写法完全等价：

    let data = vec![1, 2, 3, 4, 5];

    // 写法一：for 循环（语法糖）
    print!("for 循环:    ");
    for x in &data {
        print!("{} ", x);
    }
    println!();

    // 写法二：等价的迭代器展开（编译器实际做的事）
    print!("迭代器展开:  ");
    let mut iter = (&data).into_iter(); // &data 调用 into_iter 等价于 data.iter()
    loop {
        match iter.next() {
            Some(x) => print!("{} ", x),
            None => break,
        }
    }
    println!();

    // for x in v       等价于  for x in v.into_iter()  —— 消费 v
    // for x in &v      等价于  for x in v.iter()       —— 借用 v
    // for x in &mut v  等价于  for x in v.iter_mut()   —— 可变借用 v

    println!("\nfor 循环的语法糖对应关系:");
    println!("  for x in v       <=> v.into_iter()  (消费)");
    println!("  for x in &v      <=> v.iter()       (不可变借用)");
    println!("  for x in &mut v  <=> v.iter_mut()   (可变借用)");

    // =============================================================
    // 6. 其他类型的迭代器
    // =============================================================
    println!("\n===== 6. 其他类型的迭代器 =====");

    // 字符串的字符迭代器
    println!("--- 字符串迭代 ---");
    let greeting = "你好Rust!";
    print!("chars(): ");
    for ch in greeting.chars() {
        print!("'{}' ", ch);
    }
    println!();

    print!("bytes(): ");
    for byte in greeting.bytes() {
        print!("{:#04x} ", byte);
    }
    println!();

    // 范围（Range）迭代器
    println!("\n--- 范围迭代器 ---");
    print!("0..5:    ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    print!("0..=5:   ");
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();

    print!("(0..10).step_by(2): ");
    for i in (0..10).step_by(2) {
        print!("{} ", i);
    }
    println!();

    // 数组的迭代器
    println!("\n--- 数组迭代 ---");
    let arr = [100, 200, 300];
    print!("数组 iter: ");
    for x in arr.iter() {
        print!("{} ", x);
    }
    println!();

    // =============================================================
    // 7. 迭代器是惰性的（Lazy）
    // =============================================================
    println!("\n===== 7. 迭代器的惰性求值 =====");

    let v = vec![1, 2, 3, 4, 5];

    // 仅仅创建迭代器，不会执行任何操作
    let _iter = v.iter().map(|x| {
        println!("  处理: {}", x); // 如果不消费，这行永远不会执行
        x * 2
    });
    println!("创建了 map 迭代器，但还没有执行任何操作！");
    println!("(上面没有打印 '处理: ...'，因为迭代器是惰性的)");

    // 只有消费迭代器时，才会真正执行
    println!("\n现在消费迭代器:");
    let result: Vec<i32> = v
        .iter()
        .map(|x| {
            println!("  处理: {}", x);
            x * 2
        })
        .collect(); // collect() 是消费者，触发实际计算
    println!("结果: {:?}", result);

    // =============================================================
    // 8. 常用的消费者方法（consuming adaptors）
    // =============================================================
    println!("\n===== 8. 消费者方法 =====");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // sum: 求和
    let total: i32 = numbers.iter().sum();
    println!("sum: {}", total);

    // count: 计数
    let count = numbers.iter().count();
    println!("count: {}", count);

    // min / max
    println!("min: {:?}", numbers.iter().min());
    println!("max: {:?}", numbers.iter().max());

    // collect: 收集到集合中
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // last: 获取最后一个元素（消费整个迭代器）
    println!("last: {:?}", numbers.iter().last());

    // nth: 获取第 n 个元素（0-indexed）
    println!("nth(3): {:?}", numbers.iter().nth(3)); // 第 4 个元素

    // =============================================================
    // 9. 实战：手动实现简单迭代
    // =============================================================
    println!("\n===== 9. 实战：用迭代器处理数据 =====");

    // 场景：统计学生成绩
    let student_scores = vec![
        ("Alice", 92),
        ("Bob", 78),
        ("Charlie", 95),
        ("Diana", 88),
        ("Eve", 65),
    ];

    println!("学生成绩:");
    for (name, score) in student_scores.iter() {
        let grade = match score {
            90..=100 => "优秀",
            80..=89 => "良好",
            70..=79 => "中等",
            60..=69 => "及格",
            _ => "不及格",
        };
        println!("  {} : {} 分 ({})", name, score, grade);
    }

    // 使用迭代器计算平均分
    let total: i32 = student_scores.iter().map(|(_, score)| score).sum();
    let avg = total as f64 / student_scores.len() as f64;
    println!("\n平均分: {:.1}", avg);

    // 找出最高分
    let best = student_scores
        .iter()
        .max_by_key(|(_, score)| *score);
    if let Some((name, score)) = best {
        println!("最高分: {} ({} 分)", name, score);
    }

    // 及格的学生
    let passed: Vec<&str> = student_scores
        .iter()
        .filter(|(_, score)| *score >= 70)
        .map(|(name, _)| *name)
        .collect();
    println!("及格的学生: {:?}", passed);

    println!("\n🎉 恭喜！你已经完成了迭代器基础的学习！");
}
