/// # Lesson 028 - 迭代器适配器
///
/// 迭代器适配器是 Rust 函数式编程的核心工具，
/// 它们将一个迭代器转换为另一个迭代器，支持链式调用。
///
/// ## 学习目标
/// - 掌握常用适配器：map/filter/enumerate/zip/chain/take/skip/peekable
/// - 掌握 collect 到不同集合类型
/// - 掌握折叠操作：fold/reduce
/// - 掌握聚合操作：sum/min/max/count
/// - 掌握查找操作：any/all/find/position
/// - 理解惰性求值原理
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson028_iterator_adapters
/// ```

use std::collections::{HashMap, HashSet};

fn main() {
    // =============================================================
    // 1. map —— 对每个元素进行转换
    // =============================================================
    println!("===== 1. map 转换 =====");

    let numbers = vec![1, 2, 3, 4, 5];

    // map: 将每个元素应用函数，产生新的迭代器
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("原始: {:?}", numbers);
    println!("平方: {:?}", squares);

    // map 可以改变类型
    let strings: Vec<String> = numbers.iter().map(|x| format!("第{}号", x)).collect();
    println!("转字符串: {:?}", strings);

    // =============================================================
    // 2. filter —— 过滤元素
    // =============================================================
    println!("\n===== 2. filter 过滤 =====");

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter: 保留闭包返回 true 的元素
    let evens: Vec<&i32> = values.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);

    // filter_map: filter + map 的组合，闭包返回 Option
    let text_numbers = vec!["1", "abc", "3", "def", "5"];
    let parsed: Vec<i32> = text_numbers
        .iter()
        .filter_map(|s| s.parse::<i32>().ok()) // parse 失败返回 None，自动过滤
        .collect();
    println!("filter_map 解析数字: {:?}", parsed);

    // =============================================================
    // 3. enumerate —— 带索引遍历
    // =============================================================
    println!("\n===== 3. enumerate 带索引 =====");

    let fruits = vec!["苹果", "香蕉", "橙子", "葡萄"];

    for (index, fruit) in fruits.iter().enumerate() {
        println!("  [{}] {}", index, fruit);
    }

    // 找出特定元素的索引
    let target = "橙子";
    if let Some((idx, _)) = fruits.iter().enumerate().find(|(_, &f)| f == target) {
        println!("'{}' 的索引是 {}", target, idx);
    }

    // =============================================================
    // 4. zip —— 将两个迭代器配对
    // =============================================================
    println!("\n===== 4. zip 配对 =====");

    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![95, 87, 92];

    // zip: 将两个迭代器元素一一配对
    let paired: Vec<(&&str, &i32)> = names.iter().zip(scores.iter()).collect();
    println!("配对: {:?}", paired);

    // zip 遍历
    println!("学生成绩:");
    for (name, score) in names.iter().zip(scores.iter()) {
        println!("  {} : {} 分", name, score);
    }

    // 长度不同时，以短的为准
    let a = vec![1, 2, 3, 4, 5];
    let b = vec!["a", "b", "c"];
    let zipped: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("不等长 zip: {:?} (以短的为准)", zipped);

    // unzip: zip 的反操作
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    let (nums, words): (Vec<i32>, Vec<&str>) = pairs.into_iter().unzip();
    println!("unzip: nums = {:?}, words = {:?}", nums, words);

    // =============================================================
    // 5. chain —— 连接两个迭代器
    // =============================================================
    println!("\n===== 5. chain 连接 =====");

    let first_half = vec![1, 2, 3];
    let second_half = vec![4, 5, 6];

    // chain: 先迭代第一个，再迭代第二个
    let combined: Vec<&i32> = first_half.iter().chain(second_half.iter()).collect();
    println!("chain: {:?}", combined);

    // 多个链接
    let a = [1, 2];
    let b = [3, 4];
    let c = [5, 6];
    let all: Vec<&i32> = a.iter().chain(b.iter()).chain(c.iter()).collect();
    println!("多重 chain: {:?}", all);

    // =============================================================
    // 6. take / skip —— 取前 N 个 / 跳过前 N 个
    // =============================================================
    println!("\n===== 6. take / skip =====");

    let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // take: 只取前 N 个元素
    let first_three: Vec<&i32> = data.iter().take(3).collect();
    println!("take(3): {:?}", first_three);

    // skip: 跳过前 N 个元素
    let after_skip: Vec<&i32> = data.iter().skip(7).collect();
    println!("skip(7): {:?}", after_skip);

    // 组合：分页效果（跳过前 3 个，取 4 个）
    let page: Vec<&i32> = data.iter().skip(3).take(4).collect();
    println!("skip(3).take(4) 分页: {:?}", page);

    // take_while: 取元素直到条件不满足
    let taken: Vec<&i32> = data.iter().take_while(|&&x| x < 50).collect();
    println!("take_while(< 50): {:?}", taken);

    // skip_while: 跳过元素直到条件不满足
    let skipped: Vec<&i32> = data.iter().skip_while(|&&x| x < 50).collect();
    println!("skip_while(< 50): {:?}", skipped);

    // =============================================================
    // 7. peekable —— 可预览的迭代器
    // =============================================================
    println!("\n===== 7. peekable 预览 =====");

    let values = vec![1, 2, 3, 4, 5];
    let mut peekable = values.iter().peekable();

    // peek: 查看下一个元素但不消费
    println!("peek: {:?}", peekable.peek()); // Some(&&1)
    println!("peek: {:?}", peekable.peek()); // Some(&&1) — 仍然是同一个！
    println!("next: {:?}", peekable.next()); // Some(&1) — 消费了
    println!("peek: {:?}", peekable.peek()); // Some(&&2) — 现在是下一个

    // 实用场景：根据下一个元素决定当前处理方式
    println!("\n用 peekable 分组连续数字:");
    let nums = vec![1, 1, 2, 2, 2, 3, 1, 1];
    let mut iter = nums.iter().peekable();
    while let Some(&current) = iter.next() {
        let mut count = 1;
        while iter.peek() == Some(&&current) {
            iter.next();
            count += 1;
        }
        println!("  {} 出现了 {} 次（连续）", current, count);
    }

    // =============================================================
    // 8. flat_map / flatten —— 展平嵌套迭代器
    // =============================================================
    println!("\n===== 8. flat_map / flatten =====");

    // flatten: 展平嵌套结构
    let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let flat: Vec<&i32> = nested.iter().flatten().collect();
    println!("flatten: {:?}", flat);

    // flat_map: map + flatten 的组合
    let sentences = vec!["hello world", "foo bar baz"];
    let words: Vec<&str> = sentences.iter().flat_map(|s| s.split_whitespace()).collect();
    println!("flat_map 分词: {:?}", words);

    // Option 也可以 flatten
    let options = vec![Some(1), None, Some(3), None, Some(5)];
    let values: Vec<&i32> = options.iter().flatten().collect();
    println!("flatten Option: {:?}", values);

    // =============================================================
    // 9. collect 到不同集合
    // =============================================================
    println!("\n===== 9. collect 到不同集合 =====");

    let data = vec![1, 2, 3, 2, 4, 3, 5];

    // 收集到 Vec
    let to_vec: Vec<i32> = data.iter().cloned().collect();
    println!("Vec: {:?}", to_vec);

    // 收集到 HashSet（自动去重）
    let to_set: HashSet<i32> = data.iter().cloned().collect();
    println!("HashSet: {:?}", to_set);

    // 收集到 HashMap
    let to_map: HashMap<i32, i32> = data
        .iter()
        .enumerate()
        .map(|(i, &v)| (v, i as i32))
        .collect();
    println!("HashMap: {:?}", to_map);

    // 收集到 String
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let word: String = chars.iter().collect();
    println!("String: {}", word);

    // collect 结合 Result，可以提前终止
    let string_nums = vec!["1", "2", "three", "4"];
    let parsed: Result<Vec<i32>, _> = string_nums.iter().map(|s| s.parse::<i32>()).collect();
    println!("Result collect: {:?} (遇到错误就停)", parsed);

    let valid_nums = vec!["1", "2", "3", "4"];
    let parsed: Result<Vec<i32>, _> = valid_nums.iter().map(|s| s.parse::<i32>()).collect();
    println!("Result collect: {:?} (全部成功)", parsed);

    // =============================================================
    // 10. fold / reduce —— 折叠操作
    // =============================================================
    println!("\n===== 10. fold / reduce =====");

    let numbers = vec![1, 2, 3, 4, 5];

    // fold: 需要初始值，对每个元素应用累积函数
    // fold(初始值, |累积器, 元素| 新的累积值)
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("fold 求和: {}", sum);

    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("fold 求积: {}", product);

    // fold 可以改变类型
    let sentence = numbers
        .iter()
        .fold(String::new(), |acc, &x| {
            if acc.is_empty() {
                format!("{}", x)
            } else {
                format!("{}, {}", acc, x)
            }
        });
    println!("fold 拼接: {}", sentence);

    // reduce: 类似 fold，但用第一个元素作为初始值
    let max = numbers.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("reduce 求最大值: {:?}", max);

    // 空迭代器的 reduce 返回 None
    let empty: Vec<i32> = vec![];
    let result = empty.iter().copied().reduce(|a, b| a + b);
    println!("空迭代器 reduce: {:?}", result);

    // =============================================================
    // 11. sum / min / max / count
    // =============================================================
    println!("\n===== 11. 聚合操作 =====");

    let scores = vec![85, 92, 78, 96, 88, 73, 95];

    let total: i32 = scores.iter().sum();
    println!("sum: {}", total);

    let count = scores.iter().count();
    println!("count: {}", count);

    let min = scores.iter().min();
    println!("min: {:?}", min);

    let max = scores.iter().max();
    println!("max: {:?}", max);

    let avg = total as f64 / count as f64;
    println!("average: {:.1}", avg);

    // min_by_key / max_by_key: 按某个键比较
    let students = vec![("Alice", 92), ("Bob", 78), ("Charlie", 95)];
    let best = students.iter().max_by_key(|(_, score)| *score);
    let worst = students.iter().min_by_key(|(_, score)| *score);
    println!("最高分: {:?}", best);
    println!("最低分: {:?}", worst);

    // =============================================================
    // 12. any / all / find / position
    // =============================================================
    println!("\n===== 12. any / all / find / position =====");

    let numbers = vec![2, 4, 6, 7, 8, 10];

    // any: 是否存在满足条件的元素
    let has_odd = numbers.iter().any(|&x| x % 2 != 0);
    println!("any 奇数: {}", has_odd);

    // all: 是否所有元素都满足条件
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("all 正数: {}", all_positive);

    let all_even = numbers.iter().all(|&x| x % 2 == 0);
    println!("all 偶数: {} (7 不是偶数)", all_even);

    // find: 找到第一个满足条件的元素
    let first_odd = numbers.iter().find(|&&x| x % 2 != 0);
    println!("find 第一个奇数: {:?}", first_odd);

    // find_map: find + map 的组合
    let texts = vec!["hello", "42", "world", "100"];
    let first_num = texts.iter().find_map(|s| s.parse::<i32>().ok());
    println!("find_map 第一个数字: {:?}", first_num);

    // position: 找到第一个满足条件的元素的索引
    let pos = numbers.iter().position(|&x| x == 7);
    println!("position(7): {:?}", pos);

    let pos_none = numbers.iter().position(|&x| x == 99);
    println!("position(99): {:?}", pos_none);

    // rposition: 从后往前找
    let data = vec![1, 2, 3, 2, 1];
    let last_two = data.iter().rposition(|&x| x == 2);
    println!("rposition(2): {:?}", last_two);

    // =============================================================
    // 13. 惰性求值演示
    // =============================================================
    println!("\n===== 13. 惰性求值深入理解 =====");

    // 迭代器适配器是惰性的，只有遇到消费者才会执行
    // 而且是按需逐个处理，不是一层一层处理

    println!("观察处理顺序（逐元素处理，不是逐层处理）:");
    let result: Vec<i32> = (1..=5)
        .map(|x| {
            println!("  map: {} -> {}", x, x * 2);
            x * 2
        })
        .filter(|x| {
            let keep = *x > 4;
            println!("  filter: {} -> {}", x, if keep { "保留" } else { "丢弃" });
            keep
        })
        .take(2) // 只取前 2 个通过 filter 的元素
        .collect();

    println!("最终结果: {:?}", result);
    println!("注意: take(2) 让迭代提前终止，4 和 5 没有被处理！");

    // =============================================================
    // 14. 链式调用实战
    // =============================================================
    println!("\n===== 14. 链式调用实战 =====");

    // 场景：处理日志数据
    let logs = vec![
        "2024-01-01 INFO: 服务启动",
        "2024-01-01 ERROR: 数据库连接失败",
        "2024-01-02 INFO: 用户登录",
        "2024-01-02 WARN: 内存使用率高",
        "2024-01-03 ERROR: 请求超时",
        "2024-01-03 INFO: 用户登出",
    ];

    // 找出所有错误日志并编号
    println!("错误日志:");
    let errors: Vec<String> = logs
        .iter()
        .filter(|log| log.contains("ERROR"))
        .enumerate()
        .map(|(i, log)| format!("  #{}: {}", i + 1, log))
        .collect();
    for error in &errors {
        println!("{}", error);
    }

    // 统计各级别日志数量
    let mut level_counts: HashMap<&str, usize> = HashMap::new();
    for log in &logs {
        let level = if log.contains("ERROR") {
            "ERROR"
        } else if log.contains("WARN") {
            "WARN"
        } else {
            "INFO"
        };
        *level_counts.entry(level).or_insert(0) += 1;
    }
    println!("\n日志级别统计: {:?}", level_counts);

    // =============================================================
    // 15. inspect —— 调试迭代器管道
    // =============================================================
    println!("\n===== 15. inspect 调试 =====");

    // inspect 不改变迭代器，但可以在中间"偷看"每个元素（常用于调试）
    let result: Vec<i32> = (1..=6)
        .inspect(|x| print!("原始:{} ", x))
        .filter(|x| x % 2 == 0)
        .inspect(|x| print!("过滤后:{} ", x))
        .map(|x| x * 10)
        .inspect(|x| print!("映射后:{} ", x))
        .collect();
    println!();
    println!("inspect 最终: {:?}", result);

    println!("\n🎉 恭喜！你已经完成了迭代器适配器的学习！");
}
