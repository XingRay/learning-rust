/// # Lesson 026 - HashSet
///
/// HashSet<T> 是一个不包含重复元素的集合，
/// 基于 HashMap<T, ()> 实现，提供 O(1) 的平均查找性能。
///
/// ## 学习目标
/// - 掌握 HashSet 的创建与基本操作
/// - 理解 insert / contains / remove 的使用
/// - 掌握集合运算：交集、并集、差集、对称差集
/// - 完成去重实战示例
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson026_hashset
/// ```

use std::collections::HashSet;

fn main() {
    // =============================================================
    // 1. 创建 HashSet
    // =============================================================
    println!("===== 1. 创建 HashSet =====");

    // 方式一：HashSet::new() 创建空集合
    let mut fruits: HashSet<String> = HashSet::new();
    fruits.insert(String::from("苹果"));
    fruits.insert(String::from("香蕉"));
    fruits.insert(String::from("橙子"));
    println!("HashSet::new(): {:?}", fruits);

    // 方式二：从数组创建
    let colors = HashSet::from(["红", "绿", "蓝", "黄"]);
    println!("HashSet::from(): {:?}", colors);

    // 方式三：从迭代器 collect
    let numbers: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    println!("collect(): {:?}", numbers);

    // 方式四：从迭代器 collect，自动去重
    let with_dupes: HashSet<i32> = vec![1, 2, 2, 3, 3, 3, 4].into_iter().collect();
    println!("去重 collect: {:?} (原始含重复)", with_dupes);

    // =============================================================
    // 2. 插入：insert
    // =============================================================
    println!("\n===== 2. insert 插入 =====");

    let mut set = HashSet::new();

    // insert 返回 bool：true 表示成功插入，false 表示已存在
    let inserted1 = set.insert("Rust");
    println!("插入 'Rust': {} (新元素)", inserted1);

    let inserted2 = set.insert("Rust");
    println!("再插入 'Rust': {} (已存在，未插入)", inserted2);

    set.insert("Python");
    set.insert("Go");
    println!("当前集合: {:?}", set);
    println!("元素数量: {}", set.len());

    // =============================================================
    // 3. 查询：contains
    // =============================================================
    println!("\n===== 3. contains 查询 =====");

    let languages = HashSet::from(["Rust", "Python", "Go", "Java", "C++"]);

    println!("包含 'Rust': {}", languages.contains("Rust"));
    println!("包含 'Ruby': {}", languages.contains("Ruby"));

    // get 方法：返回对集合中元素的引用
    match languages.get("Python") {
        Some(lang) => println!("找到: {}", lang),
        None => println!("未找到"),
    }

    // is_empty 和 len
    println!("is_empty: {}", languages.is_empty());
    println!("len: {}", languages.len());

    // =============================================================
    // 4. 删除：remove / take
    // =============================================================
    println!("\n===== 4. remove 删除 =====");

    let mut skills = HashSet::from(["编程", "阅读", "运动", "音乐", "绘画"]);
    println!("删除前: {:?}", skills);

    // remove 返回 bool
    let removed = skills.remove("运动");
    println!("remove('运动'): {}", removed);

    let not_found = skills.remove("游泳");
    println!("remove('游泳'): {} (不存在)", not_found);

    println!("删除后: {:?}", skills);

    // retain: 保留满足条件的元素
    let mut nums: HashSet<i32> = (1..=10).collect();
    println!("\n原始集合: {:?}", nums);
    nums.retain(|&x| x % 2 == 0); // 只保留偶数
    println!("retain 偶数: {:?}", nums);

    // clear: 清空
    nums.clear();
    println!("clear 后: {:?}, len = {}", nums, nums.len());

    // =============================================================
    // 5. 集合运算 —— 这是 HashSet 最强大的功能！
    // =============================================================
    println!("\n===== 5. 集合运算 =====");

    let set_a: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set_b: HashSet<i32> = vec![3, 4, 5, 6, 7].into_iter().collect();

    println!("集合 A: {:?}", set_a);
    println!("集合 B: {:?}", set_b);

    // 交集（intersection）：两个集合都有的元素
    let intersection: HashSet<&i32> = set_a.intersection(&set_b).collect();
    println!("\nA ∩ B 交集: {:?}", intersection);

    // 并集（union）：两个集合的所有元素（去重）
    let union: HashSet<&i32> = set_a.union(&set_b).collect();
    println!("A ∪ B 并集: {:?}", union);

    // 差集（difference）：在 A 中但不在 B 中的元素
    let diff_a: HashSet<&i32> = set_a.difference(&set_b).collect();
    println!("A - B 差集: {:?}", diff_a);

    let diff_b: HashSet<&i32> = set_b.difference(&set_a).collect();
    println!("B - A 差集: {:?}", diff_b);

    // 对称差集（symmetric_difference）：只在其中一个集合中的元素
    let sym_diff: HashSet<&i32> = set_a.symmetric_difference(&set_b).collect();
    println!("A △ B 对称差集: {:?}", sym_diff);

    // =============================================================
    // 6. 子集与超集判断
    // =============================================================
    println!("\n===== 6. 子集与超集 =====");

    let superset: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let subset: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
    let disjoint: HashSet<i32> = vec![6, 7, 8].into_iter().collect();

    // is_subset: 是否是子集
    println!("{:?} 是 {:?} 的子集: {}", subset, superset, subset.is_subset(&superset));

    // is_superset: 是否是超集
    println!(
        "{:?} 是 {:?} 的超集: {}",
        superset,
        subset,
        superset.is_superset(&subset)
    );

    // is_disjoint: 是否没有交集（不相交）
    println!(
        "{:?} 与 {:?} 不相交: {}",
        superset,
        disjoint,
        superset.is_disjoint(&disjoint)
    );
    println!(
        "{:?} 与 {:?} 不相交: {}",
        superset,
        subset,
        superset.is_disjoint(&subset)
    );

    // =============================================================
    // 7. 遍历 HashSet
    // =============================================================
    println!("\n===== 7. 遍历 HashSet =====");

    let animals = HashSet::from(["猫", "狗", "鸟", "鱼", "兔"]);

    // for 循环遍历（顺序不确定）
    print!("遍历: ");
    for animal in &animals {
        print!("{} ", animal);
    }
    println!();

    // 迭代器方法
    let count = animals.iter().count();
    println!("元素数量: {}", count);

    // 转换为排序后的 Vec
    let mut sorted: Vec<&&str> = animals.iter().collect();
    sorted.sort();
    println!("排序后: {:?}", sorted);

    // =============================================================
    // 8. 实战：去重示例
    // =============================================================
    println!("\n===== 8. 实战：去重 =====");

    // 示例 1：简单数组去重
    let data = vec![5, 3, 1, 4, 2, 3, 5, 1, 4, 2, 1, 3, 5];
    println!("原始数据: {:?}", data);

    // 方法一：收集到 HashSet（无序）
    let unique: HashSet<i32> = data.iter().cloned().collect();
    println!("HashSet 去重（无序）: {:?}", unique);

    // 方法二：保持原始顺序去重
    let mut seen = HashSet::new();
    let unique_ordered: Vec<i32> = data
        .iter()
        .filter(|x| seen.insert(**x)) // insert 返回 true 表示是新元素
        .cloned()
        .collect();
    println!("保序去重: {:?}", unique_ordered);

    // 示例 2：找出两个列表的共同元素
    println!("\n--- 找共同好友 ---");
    let alice_friends = HashSet::from(["Bob", "Charlie", "Dave", "Eve"]);
    let bob_friends = HashSet::from(["Charlie", "Eve", "Frank", "Grace"]);

    println!("Alice 的朋友: {:?}", alice_friends);
    println!("Bob 的朋友: {:?}", bob_friends);

    let common: Vec<&&str> = alice_friends.intersection(&bob_friends).collect();
    println!("共同好友: {:?}", common);

    let all_people: Vec<&&str> = alice_friends.union(&bob_friends).collect();
    println!("所有人: {:?}", all_people);

    let only_alice: Vec<&&str> = alice_friends.difference(&bob_friends).collect();
    println!("只是 Alice 的朋友: {:?}", only_alice);

    // 示例 3：检测文本中的重复单词
    println!("\n--- 检测重复单词 ---");
    let sentence = "the cat sat on the mat and the cat";
    let words: Vec<&str> = sentence.split_whitespace().collect();

    let mut seen_words = HashSet::new();
    let mut duplicates = HashSet::new();

    for word in &words {
        if !seen_words.insert(*word) {
            duplicates.insert(*word);
        }
    }

    println!("句子: \"{}\"", sentence);
    println!("重复的单词: {:?}", duplicates);

    // =============================================================
    // 9. HashSet 与 HashMap 的关系
    // =============================================================
    println!("\n===== 9. HashSet 与 HashMap 的关系 =====");
    println!("HashSet<T> 本质上就是 HashMap<T, ()>");
    println!("它只关心 key 的存在性，不需要关联的值");
    println!("因此所有 HashMap 的 key 相关性能特征都适用于 HashSet");

    // 自定义类型需要实现 Hash + Eq trait 才能放入 HashSet
    // 大多数基础类型（数字、字符串、bool 等）已经实现了
    // 可以通过 #[derive(Hash, Eq, PartialEq)] 自动派生

    #[derive(Debug, Hash, Eq, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let mut point_set = HashSet::new();
    point_set.insert(Point { x: 1, y: 2 });
    point_set.insert(Point { x: 3, y: 4 });
    point_set.insert(Point { x: 1, y: 2 }); // 重复，不会插入

    println!("Point 集合: {:?}", point_set);
    println!("包含 (1,2): {}", point_set.contains(&Point { x: 1, y: 2 }));

    println!("\n🎉 恭喜！你已经完成了 HashSet 的学习！");
}
