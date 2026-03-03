/// # Lesson 024 - Vec 动态数组
///
/// Vec<T> 是 Rust 中最常用的集合类型，它在堆上分配内存，
/// 可以动态地增长和缩小。
///
/// ## 学习目标
/// - 掌握 Vec 的创建方式：Vec::new() 和 vec![] 宏
/// - 掌握常用操作：push/pop/insert/remove
/// - 理解索引访问与 get 方法的区别
/// - 掌握 Vec 的遍历方式
/// - 理解 Vec 的内存布局（len 与 capacity）
/// - 学会用 Vec<枚举> 存储多种类型
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson024_vector
/// ```

fn main() {
    // =============================================================
    // 1. 创建 Vec
    // =============================================================
    println!("===== 1. 创建 Vec =====");

    // 方式一：Vec::new() 创建空向量，需要类型注解或后续 push 推断类型
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("Vec::new() 创建: {:?}", v1);

    // 方式二：vec![] 宏，更简洁，自动推断类型
    let v2 = vec![10, 20, 30];
    println!("vec![] 宏创建: {:?}", v2);

    // 创建包含相同元素的 Vec
    let v3 = vec![0; 5]; // 5 个 0
    println!("vec![0; 5] 创建: {:?}", v3);

    // 使用 Vec::with_capacity 预分配容量，避免频繁重新分配
    let mut v4 = Vec::with_capacity(10);
    v4.push(100);
    println!(
        "with_capacity(10): len = {}, capacity = {}",
        v4.len(),
        v4.capacity()
    );

    // =============================================================
    // 2. 添加与删除元素：push / pop / insert / remove
    // =============================================================
    println!("\n===== 2. push / pop / insert / remove =====");

    let mut fruits = vec![
        String::from("苹果"),
        String::from("香蕉"),
        String::from("橙子"),
    ];
    println!("初始: {:?}", fruits);

    // push: 在末尾添加元素
    fruits.push(String::from("葡萄"));
    println!("push 后: {:?}", fruits);

    // pop: 移除并返回最后一个元素，返回 Option<T>
    let last = fruits.pop();
    println!("pop 返回: {:?}, 剩余: {:?}", last, fruits);

    // insert: 在指定位置插入元素（后面的元素会右移）
    fruits.insert(1, String::from("芒果"));
    println!("insert(1, 芒果) 后: {:?}", fruits);

    // remove: 移除指定位置的元素并返回（后面的元素会左移）
    let removed = fruits.remove(2);
    println!("remove(2) 返回: {}, 剩余: {:?}", removed, fruits);

    // extend: 批量添加元素
    let more = vec![String::from("草莓"), String::from("蓝莓")];
    fruits.extend(more);
    println!("extend 后: {:?}", fruits);

    // retain: 只保留满足条件的元素
    fruits.retain(|f| f.len() > 6); // UTF-8 中文字符占 3 字节
    println!("retain(len > 6) 后: {:?}", fruits);

    // clear: 清空所有元素
    fruits.clear();
    println!("clear 后: {:?}, len = {}", fruits, fruits.len());

    // =============================================================
    // 3. 索引访问与 get 方法
    // =============================================================
    println!("\n===== 3. 索引访问与 get 方法 =====");

    let colors = vec!["红", "绿", "蓝", "黄", "紫"];

    // 方式一：直接索引访问（越界会 panic）
    let first = colors[0];
    let third = colors[2];
    println!("colors[0] = {}, colors[2] = {}", first, third);

    // 方式二：get 方法，返回 Option<&T>，越界时返回 None，更安全
    match colors.get(1) {
        Some(color) => println!("colors.get(1) = {}", color),
        None => println!("索引 1 不存在"),
    }

    match colors.get(100) {
        Some(color) => println!("colors.get(100) = {}", color),
        None => println!("colors.get(100) = None，安全地处理了越界！"),
    }

    // first / last 方法
    println!("first = {:?}, last = {:?}", colors.first(), colors.last());

    // contains 检查是否包含某元素
    println!("包含 '蓝': {}", colors.contains(&"蓝"));
    println!("包含 '黑': {}", colors.contains(&"黑"));

    // =============================================================
    // 4. 遍历 Vec
    // =============================================================
    println!("\n===== 4. 遍历 Vec =====");

    let scores = vec![85, 92, 78, 96, 88];

    // 方式一：不可变引用遍历（最常用）
    print!("不可变遍历: ");
    for &score in &scores {
        print!("{} ", score);
    }
    println!();

    // 方式二：可变引用遍历，可以修改元素
    let mut scores_mut = vec![85, 92, 78, 96, 88];
    for score in &mut scores_mut {
        *score += 10; // 每个分数加 10 分
    }
    println!("加 10 分后: {:?}", scores_mut);

    // 方式三：带索引遍历
    for (index, &score) in scores.iter().enumerate() {
        println!("  第 {} 名: {} 分", index + 1, score);
    }

    // 方式四：into_iter 消费 Vec（遍历后 Vec 不再可用）
    let names = vec!["Alice", "Bob", "Charlie"];
    for name in names {
        // names 被 into_iter 消费
        print!("{} ", name);
    }
    println!();
    // println!("{:?}", names); // 编译错误！names 已被消费

    // =============================================================
    // 5. Vec 的内存布局：len 与 capacity
    // =============================================================
    println!("\n===== 5. Vec 内存布局（len / capacity） =====");

    // Vec 在栈上存储三个值：指针（指向堆上数据）、长度、容量
    // - len: 当前元素数量
    // - capacity: 已分配的堆内存能容纳的元素数量
    // 当 len == capacity 时，再 push 会触发重新分配（通常容量翻倍）

    let mut v = Vec::new();
    println!(
        "空 Vec:       len = {}, capacity = {}",
        v.len(),
        v.capacity()
    );

    for i in 0..10 {
        v.push(i);
        println!(
            "  push({})  => len = {}, capacity = {}",
            i,
            v.len(),
            v.capacity()
        );
    }

    // shrink_to_fit: 释放多余容量
    v.shrink_to_fit();
    println!(
        "shrink_to_fit: len = {}, capacity = {}",
        v.len(),
        v.capacity()
    );

    // truncate: 截断到指定长度
    v.truncate(5);
    println!("truncate(5):   len = {}, capacity = {}", v.len(), v.capacity());
    println!("内容: {:?}", v);

    // =============================================================
    // 6. 切片操作
    // =============================================================
    println!("\n===== 6. 切片操作 =====");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Vec 可以自动解引用为切片 &[T]
    let slice = &numbers[2..5]; // 获取索引 2、3、4 的元素
    println!("numbers[2..5] = {:?}", slice);

    let first_three = &numbers[..3]; // 前三个
    println!("numbers[..3]  = {:?}", first_three);

    let last_three = &numbers[7..]; // 最后三个
    println!("numbers[7..]  = {:?}", last_three);

    // windows: 滑动窗口
    print!("windows(3): ");
    for window in numbers.windows(3) {
        print!("{:?} ", window);
    }
    println!();

    // chunks: 分块
    print!("chunks(3):  ");
    for chunk in numbers.chunks(3) {
        print!("{:?} ", chunk);
    }
    println!();

    // =============================================================
    // 7. 排序与搜索
    // =============================================================
    println!("\n===== 7. 排序与搜索 =====");

    let mut nums = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("排序前: {:?}", nums);

    nums.sort();
    println!("sort 后: {:?}", nums);

    nums.dedup(); // 去除连续重复元素（需要先排序）
    println!("dedup 后: {:?}", nums);

    // sort_by: 自定义排序（降序）
    nums.sort_by(|a, b| b.cmp(a));
    println!("降序排序: {:?}", nums);

    // 浮点数排序（f64 没有实现 Ord，需要用 sort_by）
    let mut floats = vec![3.14, 1.0, 2.71, 0.5];
    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("浮点数排序: {:?}", floats);

    // =============================================================
    // 8. Vec<枚举> 存储多种类型
    // =============================================================
    println!("\n===== 8. Vec<枚举> 存储多种类型 =====");

    // Vec 要求所有元素类型相同，但我们可以用枚举来"包装"不同类型

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
        Bool(bool),
    }

    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("你好")),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Bool(true),
    ];

    // 遍历并匹配
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("  整数: {}", i),
            SpreadsheetCell::Float(f) => println!("  浮点: {:.2}", f),
            SpreadsheetCell::Text(s) => println!("  文本: {}", s),
            SpreadsheetCell::Bool(b) => println!("  布尔: {}", b),
        }
    }

    // =============================================================
    // 9. 常用方法速查
    // =============================================================
    println!("\n===== 9. 常用方法速查 =====");

    let data = vec![10, 20, 30, 40, 50];

    println!("is_empty: {}", data.is_empty());
    println!("len: {}", data.len());
    println!("contains(&30): {}", data.contains(&30));
    println!("iter().sum(): {}", data.iter().sum::<i32>());
    println!("iter().min(): {:?}", data.iter().min());
    println!("iter().max(): {:?}", data.iter().max());

    // 从 Vec 转换
    let joined: String = vec!["Hello", "World"].join(", ");
    println!("join: {}", joined);

    // iter + collect 创建新 Vec
    let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    println!("\n🎉 恭喜！你已经完成了 Vec 动态数组的学习！");
}
