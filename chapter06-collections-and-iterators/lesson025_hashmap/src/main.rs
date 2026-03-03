/// # Lesson 025 - HashMap
///
/// HashMap<K, V> 是 Rust 中最常用的键值对集合，
/// 基于哈希表实现，提供 O(1) 的平均查找/插入/删除性能。
///
/// ## 学习目标
/// - 掌握 HashMap 的创建与基本操作
/// - 理解 insert / get / remove 的使用
/// - 掌握 entry API（or_insert / or_insert_with）
/// - 理解 HashMap 的所有权规则
/// - 完成词频统计实战示例
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson025_hashmap
/// ```

use std::collections::HashMap;

fn main() {
    // =============================================================
    // 1. 创建 HashMap
    // =============================================================
    println!("===== 1. 创建 HashMap =====");

    // 方式一：HashMap::new() 创建空 HashMap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"), 87);
    scores.insert(String::from("Charlie"), 92);
    println!("scores: {:?}", scores);

    // 方式二：从元组数组收集（collect）
    let teams = vec![
        (String::from("红队"), 100),
        (String::from("蓝队"), 85),
        (String::from("绿队"), 92),
    ];
    let team_scores: HashMap<String, i32> = teams.into_iter().collect();
    println!("team_scores: {:?}", team_scores);

    // 方式三：使用 HashMap::from（Rust 标准方式）
    let capitals = HashMap::from([
        ("中国", "北京"),
        ("日本", "东京"),
        ("韩国", "首尔"),
        ("美国", "华盛顿"),
    ]);
    println!("capitals: {:?}", capitals);

    // =============================================================
    // 2. 插入与更新：insert
    // =============================================================
    println!("\n===== 2. insert 插入与更新 =====");

    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 5);
    println!("初始: {:?}", map);

    // insert 相同 key 会覆盖旧值
    let old_value = map.insert("apple", 10);
    println!("覆盖 apple: 旧值 = {:?}, 新 map: {:?}", old_value, map);

    // 插入新 key 返回 None
    let new_insert = map.insert("cherry", 7);
    println!("新插入 cherry: 返回 {:?}, map: {:?}", new_insert, map);

    // =============================================================
    // 3. 查询：get / get_mut / contains_key
    // =============================================================
    println!("\n===== 3. get / contains_key 查询 =====");

    let mut student_ages = HashMap::from([
        (String::from("小明"), 18),
        (String::from("小红"), 20),
        (String::from("小李"), 22),
    ]);

    // get 返回 Option<&V>
    match student_ages.get("小明") {
        Some(age) => println!("小明的年龄: {}", age),
        None => println!("找不到小明"),
    }

    // 使用 if let 简化
    if let Some(age) = student_ages.get("小红") {
        println!("小红的年龄: {}", age);
    }

    // 查询不存在的 key
    println!("小王的年龄: {:?}", student_ages.get("小王")); // None

    // contains_key 检查 key 是否存在
    println!("包含 '小李': {}", student_ages.contains_key("小李"));
    println!("包含 '小王': {}", student_ages.contains_key("小王"));

    // get_mut 获取可变引用，修改值
    if let Some(age) = student_ages.get_mut("小李") {
        *age += 1; // 小李过生日了！
        println!("小李生日后的年龄: {}", age);
    }

    // =============================================================
    // 4. 删除：remove
    // =============================================================
    println!("\n===== 4. remove 删除 =====");

    let mut inventory = HashMap::from([
        ("苹果", 50),
        ("香蕉", 30),
        ("橙子", 20),
    ]);
    println!("删除前: {:?}", inventory);

    // remove 返回 Option<V>
    let removed = inventory.remove("香蕉");
    println!("remove('香蕉'): {:?}", removed);
    println!("删除后: {:?}", inventory);

    // 删除不存在的 key
    let not_found = inventory.remove("西瓜");
    println!("remove('西瓜'): {:?}", not_found);

    // =============================================================
    // 5. Entry API —— 高效的"查找或插入"
    // =============================================================
    println!("\n===== 5. Entry API =====");

    // entry() 返回一个 Entry 枚举，表示"这个 key 对应的位置"
    // 避免了先 get 再 insert 的两次哈希计算

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    // or_insert: 如果 key 不存在，插入默认值并返回可变引用
    word_count.entry("hello").or_insert(0);
    word_count.entry("world").or_insert(0);
    word_count.entry("hello").or_insert(999); // 已存在，不会覆盖
    println!("or_insert 示例: {:?}", word_count);

    // or_insert 返回可变引用，可以直接修改
    *word_count.entry("hello").or_insert(0) += 1;
    *word_count.entry("hello").or_insert(0) += 1;
    *word_count.entry("world").or_insert(0) += 1;
    println!("累加后: {:?}", word_count);

    // or_insert_with: 用闭包计算默认值（惰性求值）
    let mut cache: HashMap<&str, String> = HashMap::new();
    let key = "greeting";
    let value = cache
        .entry(key)
        .or_insert_with(|| {
            println!("  (闭包被调用，计算默认值...)");
            String::from("Hello, Rust!")
        });
    println!("第一次: {}", value);

    // 第二次不会调用闭包
    let value2 = cache
        .entry(key)
        .or_insert_with(|| {
            println!("  (这不会被打印)");
            String::from("不会用到")
        });
    println!("第二次: {}", value2);

    // and_modify: 对已存在的值进行修改
    let mut scores_map: HashMap<&str, Vec<i32>> = HashMap::new();
    scores_map
        .entry("Alice")
        .and_modify(|v| v.push(95))
        .or_insert_with(|| vec![95]);
    scores_map
        .entry("Alice")
        .and_modify(|v| v.push(88))
        .or_insert_with(|| vec![88]);
    scores_map
        .entry("Bob")
        .and_modify(|v| v.push(76))
        .or_insert_with(|| vec![76]);
    println!("and_modify 示例: {:?}", scores_map);

    // =============================================================
    // 6. 遍历 HashMap
    // =============================================================
    println!("\n===== 6. 遍历 HashMap =====");

    let config = HashMap::from([
        ("host", "127.0.0.1"),
        ("port", "8080"),
        ("debug", "true"),
    ]);

    // 方式一：遍历键值对
    println!("遍历键值对:");
    for (key, value) in &config {
        println!("  {} = {}", key, value);
    }

    // 方式二：只遍历 keys
    print!("所有 keys: ");
    for key in config.keys() {
        print!("{} ", key);
    }
    println!();

    // 方式三：只遍历 values
    print!("所有 values: ");
    for value in config.values() {
        print!("{} ", value);
    }
    println!();

    // 注意：HashMap 的遍历顺序是不确定的！

    // =============================================================
    // 7. 所有权规则
    // =============================================================
    println!("\n===== 7. 所有权规则 =====");

    // 对于实现了 Copy trait 的类型（如 i32），值会被复制
    let mut map_copy = HashMap::new();
    let x = 10;
    let y = 20;
    map_copy.insert(x, y);
    println!("i32 插入后仍可用: x = {}, y = {}", x, y); // x, y 仍可用

    // 对于拥有所有权的类型（如 String），值会被移动
    let mut map_move = HashMap::new();
    let key = String::from("name");
    let val = String::from("Rust");
    map_move.insert(key, val);
    // println!("{}", key); // 编译错误！key 的所有权已被移动到 map 中
    // println!("{}", val); // 编译错误！val 的所有权已被移动到 map 中
    println!("String 插入后所有权被转移，原变量不再可用");
    println!("map_move: {:?}", map_move);

    // 如果插入引用，引用的生命周期必须至少和 HashMap 一样长
    let mut map_ref = HashMap::new();
    let name = String::from("Alice");
    map_ref.insert(&name, 100); // 插入引用
    println!("引用仍可用: name = {}, map: {:?}", name, map_ref);

    // =============================================================
    // 8. 实战：词频统计
    // =============================================================
    println!("\n===== 8. 实战：词频统计 =====");

    let text = "the quick brown fox jumps over the lazy dog the fox";

    let mut word_freq: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // entry + or_insert 是词频统计的经典模式
        let count = word_freq.entry(word).or_insert(0);
        *count += 1;
    }

    println!("原文: \"{}\"", text);
    println!("词频统计:");

    // 按词频降序排列输出
    let mut freq_vec: Vec<(&&str, &i32)> = word_freq.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (word, count) in freq_vec {
        println!("  {:>8} : {} 次", word, count);
    }

    // =============================================================
    // 9. 常用方法速查
    // =============================================================
    println!("\n===== 9. 常用方法速查 =====");

    let demo = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    println!("len: {}", demo.len());
    println!("is_empty: {}", demo.is_empty());
    println!("contains_key('b'): {}", demo.contains_key("b"));
    println!("keys: {:?}", demo.keys().collect::<Vec<_>>());
    println!("values: {:?}", demo.values().collect::<Vec<_>>());

    // iter().filter 过滤
    let big: HashMap<&&str, &i32> = demo.iter().filter(|(_, &v)| v > 1).collect();
    println!("值大于 1 的: {:?}", big);

    println!("\n🎉 恭喜！你已经完成了 HashMap 的学习！");
}
