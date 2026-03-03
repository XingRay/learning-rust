/// # Lesson 010 - 切片 (Slices)
///
/// 切片是对集合中一段连续元素的引用，它不拥有数据的所有权。
///
/// ## 学习目标
/// - 掌握字符串切片 `&str`
/// - 掌握数组切片 `&[T]`
/// - 学会将切片作为函数参数
/// - 熟练使用切片的范围语法
/// - 理解 `String` 到 `&str` 的转换
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson010_slices
/// ```

// =============================================================
// Lesson 010: 切片 (Slices)
// =============================================================

fn main() {
    // ---------------------------------------------------------
    // 1. 字符串切片 &str
    // ---------------------------------------------------------
    // 字符串切片是对 String 中一部分内容的引用
    // 类型为 &str，它不拥有数据的所有权
    println!("=== 1. 字符串切片 &str ===");

    let s = String::from("Hello, World!");

    // 使用范围语法创建切片 [start..end]
    // start 是起始字节索引（包含），end 是结束字节索引（不包含）
    let hello = &s[0..5];   // "Hello"
    let world = &s[7..12];  // "World"
    println!("完整字符串: {}", s);
    println!("切片1: {}", hello);
    println!("切片2: {}", world);

    // 切片的可视化：
    //
    //   s: [ptr | len:13 | cap:13] ---> 堆上: "Hello, World!"
    //
    //   hello: [ptr(指向H) | len:5]       ──┐
    //   world: [ptr(指向W) | len:5]       ──┤ 都指向 s 的堆内存
    //                                       ──┘

    // ---------------------------------------------------------
    // 2. 切片的范围语法
    // ---------------------------------------------------------
    println!("\n=== 2. 切片的范围语法 ===");

    let text = String::from("abcdefgh");

    // [n..m] - 从索引 n 到索引 m（不包含 m）
    let slice1 = &text[2..5]; // "cde"
    println!("[2..5] = \"{}\"", slice1);

    // [..n] - 从开头到索引 n（不包含 n），等价于 [0..n]
    let slice2 = &text[..3]; // "abc"
    println!("[..3]  = \"{}\"", slice2);

    // [n..] - 从索引 n 到末尾，等价于 [n..len]
    let slice3 = &text[5..]; // "fgh"
    println!("[5..]  = \"{}\"", slice3);

    // [..] - 整个字符串的切片，等价于 [0..len]
    let slice4 = &text[..]; // "abcdefgh"
    println!("[..]   = \"{}\"", slice4);

    // [n..=m] - 包含结束索引的范围（闭区间）
    let slice5 = &text[2..=5]; // "cdef"（包含索引 5）
    println!("[2..=5] = \"{}\"", slice5);

    // ⚠️ 注意：字符串切片的索引必须落在有效的 UTF-8 字符边界上
    let chinese = String::from("你好世界");
    // 中文字符在 UTF-8 中占 3 个字节
    let ni_hao = &chinese[0..6]; // "你好"（每个中文字符 3 字节）
    println!("中文切片: {}", ni_hao);
    // let bad_slice = &chinese[0..1]; // ❌ 运行时 panic！不在字符边界上

    // ---------------------------------------------------------
    // 3. 字符串字面量就是切片
    // ---------------------------------------------------------
    println!("\n=== 3. 字符串字面量就是切片 ===");

    // 字符串字面量的类型就是 &str
    let literal: &str = "I am a string slice";
    // 它直接指向编译到二进制文件中的字符串数据
    // 所以字符串字面量是不可变的
    println!("字面量: {}", literal);

    // &str vs String:
    //   String: 可变、堆分配、拥有所有权、可增长
    //   &str:   不可变、可能指向堆/栈/静态区、是引用、固定大小
    println!("&str 是一个「胖指针」：包含指针和长度");

    // ---------------------------------------------------------
    // 4. 从 String 到 &str
    // ---------------------------------------------------------
    println!("\n=== 4. 从 String 到 &str ===");

    let owned_string = String::from("Hello, Rust!");

    // 方法 1: 使用 & 和切片语法
    let slice_a: &str = &owned_string[..];
    println!("方法1 (切片语法): {}", slice_a);

    // 方法 2: 使用 & 自动解引用（Deref coercion）
    let slice_b: &str = &owned_string;
    println!("方法2 (自动解引用): {}", slice_b);

    // 方法 3: 使用 as_str() 方法
    let slice_c: &str = owned_string.as_str();
    println!("方法3 (as_str): {}", slice_c);

    // 从 &str 到 String
    let back_to_string = slice_a.to_string();
    let also_string = String::from(slice_a);
    println!("&str -> String: {} / {}", back_to_string, also_string);

    // ---------------------------------------------------------
    // 5. 切片作为函数参数
    // ---------------------------------------------------------
    // 最佳实践：函数参数用 &str 而不是 &String
    // 这样既可以接收 &String（通过 Deref 自动转换），也可以接收 &str
    println!("\n=== 5. 切片作为函数参数 ===");

    let my_string = String::from("Hello World");

    // first_word 接受 &str，但可以传入 &String（自动解引用）
    let word = first_word(&my_string);
    println!("第一个单词: {}", word);

    // 也可以直接传入字符串字面量
    let word2 = first_word("Good Morning Rust");
    println!("第一个单词: {}", word2);

    // 也可以传入 String 的切片
    let word3 = first_word(&my_string[6..]);
    println!("第一个单词: {}", word3);

    // ---------------------------------------------------------
    // 6. 数组切片 &[T]
    // ---------------------------------------------------------
    // 切片不仅适用于字符串，也适用于数组和 Vec
    println!("\n=== 6. 数组切片 &[T] ===");

    // 从数组创建切片
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    let slice_arr: &[i32] = &arr[2..6]; // [3, 4, 5, 6]
    println!("数组: {:?}", arr);
    println!("数组切片 [2..6]: {:?}", slice_arr);

    // 从 Vec 创建切片
    let vec = vec![10, 20, 30, 40, 50];
    let slice_vec: &[i32] = &vec[1..4]; // [20, 30, 40]
    println!("Vec: {:?}", vec);
    println!("Vec 切片 [1..4]: {:?}", slice_vec);

    // 整个数组/Vec 的切片
    let full_slice = &arr[..];
    println!("完整切片: {:?}", full_slice);

    // ---------------------------------------------------------
    // 7. 可变切片 &mut [T]
    // ---------------------------------------------------------
    println!("\n=== 7. 可变切片 &mut [T] ===");

    let mut numbers = [1, 2, 3, 4, 5];
    println!("修改前: {:?}", numbers);

    // 获取可变切片
    let slice_mut = &mut numbers[1..4];
    // 修改切片中的元素
    slice_mut[0] = 20; // numbers[1] = 20
    slice_mut[1] = 30; // numbers[2] = 30
    slice_mut[2] = 40; // numbers[3] = 40
    println!("可变切片修改后: {:?}", slice_mut);

    // 注意：slice_mut 的作用域到这里结束
    println!("原始数组也被修改了: {:?}", numbers);

    // ---------------------------------------------------------
    // 8. 切片作为函数参数的最佳实践
    // ---------------------------------------------------------
    println!("\n=== 8. 切片作为函数参数 ===");

    // 使用 &[T] 作为参数，既可以接收数组切片，也可以接收 Vec 切片
    let array = [10, 20, 30, 40, 50];
    let vector = vec![100, 200, 300];

    println!("数组之和: {}", sum(&array));       // 自动转为 &[i32]
    println!("Vec 之和: {}", sum(&vector));      // 自动转为 &[i32]
    println!("部分之和: {}", sum(&array[1..4])); // 切片 [20, 30, 40]

    // 可变切片作为参数
    let mut data = vec![5, 3, 8, 1, 9, 2, 7];
    println!("排序前: {:?}", data);
    sort_slice(&mut data);
    println!("排序后: {:?}", data);

    // ---------------------------------------------------------
    // 9. 切片的常用方法
    // ---------------------------------------------------------
    println!("\n=== 9. 切片的常用方法 ===");

    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // len() - 获取长度
    println!("长度: {}", nums.len());

    // is_empty() - 是否为空
    println!("是否为空: {}", nums.is_empty());

    // first() / last() - 获取首尾元素
    println!("第一个: {:?}", nums.first());
    println!("最后一个: {:?}", nums.last());

    // contains() - 是否包含某元素
    println!("包含 5? {}", nums.contains(&5));
    println!("包含 11? {}", nums.contains(&11));

    // iter() - 获取迭代器
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("每个元素翻倍: {:?}", doubled);

    // windows() - 滑动窗口
    print!("大小为3的滑动窗口: ");
    for window in nums[..5].windows(3) {
        print!("{:?} ", window);
    }
    println!();

    // chunks() - 分块
    print!("每3个一组: ");
    for chunk in nums.chunks(3) {
        print!("{:?} ", chunk);
    }
    println!();

    // split_at() - 在指定位置分割
    let (left, right) = nums.split_at(5);
    println!("左半: {:?}, 右半: {:?}", left, right);

    // ---------------------------------------------------------
    // 10. 综合示例
    // ---------------------------------------------------------
    println!("\n=== 10. 综合示例 ===");

    // 示例：从文本中提取所有单词
    let sentence = "Rust is a systems programming language";
    let words = extract_words(sentence);
    println!("句子: \"{}\"", sentence);
    println!("单词列表: {:?}", words);
    println!("单词数量: {}", words.len());

    // 示例：找到切片中的最大最小值
    let values = [38, 15, 72, 4, 91, 56, 23];
    if let (Some(min), Some(max)) = (find_min(&values), find_max(&values)) {
        println!("数组 {:?} 中，最小值={}, 最大值={}", values, min, max);
    }

    println!("\n🎉 恭喜！你已经掌握了切片的核心用法！");
    println!("💡 提示：切片是 Rust 中非常高效的数据视图方式，");
    println!("   下一课我们将学习「生命周期」，理解引用的有效范围。");
}

/// 返回字符串中的第一个单词（使用 &str 作为参数和返回值）
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s // 如果没有空格，整个字符串就是一个单词
}

/// 计算切片中所有元素的和
fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &val in slice {
        total += val;
    }
    total
    // 也可以用迭代器写法: slice.iter().sum()
}

/// 使用冒泡排序对可变切片排序
fn sort_slice(slice: &mut [i32]) {
    let len = slice.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

/// 从文本中提取所有单词
fn extract_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

/// 找到切片中的最小值
fn find_min(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        return None;
    }
    let mut min = slice[0];
    for &val in &slice[1..] {
        if val < min {
            min = val;
        }
    }
    Some(min)
}

/// 找到切片中的最大值
fn find_max(slice: &[i32]) -> Option<i32> {
    if slice.is_empty() {
        return None;
    }
    let mut max = slice[0];
    for &val in &slice[1..] {
        if val > max {
            max = val;
        }
    }
    Some(max)
}
