/// # Lesson 012 - 结构体基础
///
/// 结构体（struct）是 Rust 中自定义数据类型的核心方式之一。
///
/// ## 学习目标
/// - 理解结构体的定义与实例化
/// - 掌握字段初始化简写语法
/// - 掌握结构体更新语法（..）
/// - 了解元组结构体和单元结构体
/// - 学会使用 #[derive(Debug)] 打印结构体
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson012_struct_basics
/// ```

// =============================================================
// Lesson 012: 结构体基础 - Struct Basics
// =============================================================

// ---------------------------------------------------------
// 1. 定义结构体
// ---------------------------------------------------------
// 使用 struct 关键字定义结构体
// 每个字段都有名称和类型，字段之间用逗号分隔
// #[derive(Debug)] 允许我们使用 {:?} 或 {:#?} 打印结构体
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    age: u32,
    active: bool,
}

// ---------------------------------------------------------
// 2. 元组结构体（Tuple Struct）
// ---------------------------------------------------------
// 元组结构体有类型名，但字段没有名称，通过索引访问
// 常用于给元组一个有意义的类型名，或者创建新类型（newtype pattern）
#[derive(Debug)]
struct Color(u8, u8, u8); // RGB 颜色

#[derive(Debug)]
struct Point3D(f64, f64, f64); // 三维坐标点

// 即使两个元组结构体的字段类型完全相同，它们也是不同的类型
// Color 和另一个 (u8, u8, u8) 的元组结构体不能互相赋值

// ---------------------------------------------------------
// 3. 单元结构体（Unit Struct）
// ---------------------------------------------------------
// 没有任何字段的结构体叫做单元结构体
// 它类似于 ()（unit 类型），但有自己的类型名
// 常用于实现 trait 但不需要存储数据的场景
#[derive(Debug)]
struct AlwaysEqual;

// 另一个单元结构体的例子：标记类型
#[derive(Debug)]
struct Meter;

#[derive(Debug)]
struct Kilometer;

// ---------------------------------------------------------
// 辅助函数：演示字段初始化简写
// ---------------------------------------------------------
// 当函数参数名与结构体字段名相同时，可以使用简写语法
fn build_user(username: String, email: String) -> User {
    User {
        username, // 简写：等价于 username: username
        email,    // 简写：等价于 email: email
        age: 0,
        active: true,
    }
}

fn main() {
    println!("=== Lesson 012: 结构体基础 ===\n");

    // ---------------------------------------------------------
    // 1. 创建结构体实例
    // ---------------------------------------------------------
    println!("--- 1. 创建结构体实例 ---");

    // 创建 User 结构体的实例，必须为每个字段提供值
    // 字段的顺序不必与定义时一致
    let user1 = User {
        email: String::from("alice@example.com"),
        username: String::from("alice"),
        active: true,
        age: 25,
    };

    // 访问结构体的字段使用点号（.）
    println!("用户名: {}", user1.username);
    println!("邮箱: {}", user1.email);
    println!("年龄: {}", user1.age);
    println!("是否活跃: {}", user1.active);

    // ---------------------------------------------------------
    // 2. 可变结构体
    // ---------------------------------------------------------
    println!("\n--- 2. 可变结构体 ---");

    // 整个结构体实例必须是可变的，Rust 不允许只标记某个字段为可变
    let mut user2 = User {
        username: String::from("bob"),
        email: String::from("bob@example.com"),
        age: 30,
        active: true,
    };

    println!("修改前 - 邮箱: {}", user2.email);
    user2.email = String::from("bob_new@example.com");
    user2.age = 31;
    println!("修改后 - 邮箱: {}", user2.email);
    println!("修改后 - 年龄: {}", user2.age);

    // ---------------------------------------------------------
    // 3. 字段初始化简写
    // ---------------------------------------------------------
    println!("\n--- 3. 字段初始化简写 ---");

    let user3 = build_user(String::from("charlie"), String::from("charlie@example.com"));
    println!("通过函数创建: username={}, email={}", user3.username, user3.email);

    // 也可以在创建实例时直接使用简写
    let username = String::from("diana");
    let email = String::from("diana@example.com");
    let user4 = User {
        username, // 简写
        email,    // 简写
        age: 22,
        active: false,
    };
    println!("简写创建: username={}, active={}", user4.username, user4.active);

    // ---------------------------------------------------------
    // 4. 结构体更新语法（..）
    // ---------------------------------------------------------
    println!("\n--- 4. 结构体更新语法（..） ---");

    // 使用 .. 语法从已有的实例创建新实例
    // 未显式指定的字段将从指定的实例中获取值
    // 注意：.. 必须放在最后
    let user5 = User {
        username: String::from("eve"),
        email: String::from("eve@example.com"),
        ..user4 // 从 user4 获取剩余字段（age 和 active）
    };
    println!(
        "更新语法创建: username={}, age={}, active={}",
        user5.username, user5.age, user5.active
    );

    // ⚠️ 重要：结构体更新语法会「移动」数据
    // 因为 user4 的 String 字段被移动了吗？不，我们显式指定了所有 String 字段
    // 所以 user4 中只有 age(u32) 和 active(bool) 被复制了（它们实现了 Copy trait）
    // 如果我们没有显式指定 username 或 email，user4 就会被部分移动，
    // 之后就不能再整体使用 user4 了

    // 演示：只更新一个非 Copy 字段，其余字段从旧实例获取
    let base_user = User {
        username: String::from("base"),
        email: String::from("base@example.com"),
        age: 20,
        active: true,
    };

    let new_user = User {
        email: String::from("new@example.com"),
        ..base_user // username(String) 被移动了！
    };
    println!(
        "新用户: username={}, email={}",
        new_user.username, new_user.email
    );
    // 此时 base_user.username 已被移动，不能再访问 base_user.username
    // 但 base_user.age 和 base_user.active 仍可访问（它们是 Copy 类型）
    println!(
        "base_user 的 Copy 字段仍可用: age={}, active={}",
        base_user.age, base_user.active
    );
    // println!("{}", base_user.username); // ❌ 编译错误！username 已被移动

    // ---------------------------------------------------------
    // 5. 元组结构体
    // ---------------------------------------------------------
    println!("\n--- 5. 元组结构体 ---");

    let red = Color(255, 0, 0);
    let origin = Point3D(0.0, 0.0, 0.0);

    // 通过索引访问字段：.0, .1, .2 ...
    println!("红色: R={}, G={}, B={}", red.0, red.1, red.2);
    println!("原点: x={}, y={}, z={}", origin.0, origin.1, origin.2);

    // 元组结构体也可以解构
    let Color(r, g, b) = red;
    println!("解构后: r={}, g={}, b={}", r, g, b);

    let Point3D(x, y, z) = origin;
    println!("解构后: x={}, y={}, z={}", x, y, z);

    // 元组结构体的类型安全性：
    // 即使字段类型相同，不同的元组结构体也是不同的类型
    // let wrong: Color = Point3D(1.0, 2.0, 3.0); // ❌ 编译错误！类型不同

    // ---------------------------------------------------------
    // 6. 单元结构体
    // ---------------------------------------------------------
    println!("\n--- 6. 单元结构体 ---");

    let _equal = AlwaysEqual;
    println!("单元结构体 AlwaysEqual: {:?}", _equal);

    // 单元结构体没有字段，不占用内存空间（zero-sized type，ZST）
    println!(
        "AlwaysEqual 的大小: {} 字节",
        std::mem::size_of::<AlwaysEqual>()
    );
    println!("Meter 的大小: {} 字节", std::mem::size_of::<Meter>());

    // ---------------------------------------------------------
    // 7. 打印结构体 - #[derive(Debug)]
    // ---------------------------------------------------------
    println!("\n--- 7. 打印结构体 ---");

    let user = User {
        username: String::from("frank"),
        email: String::from("frank@example.com"),
        age: 28,
        active: true,
    };

    // 使用 {:?} 进行调试打印（单行格式）
    println!("Debug 打印（单行）: {:?}", user);

    // 使用 {:#?} 进行美化的调试打印（多行格式）
    println!("Debug 打印（多行）:");
    println!("{:#?}", user);

    // 使用 dbg! 宏打印调试信息（会输出文件名、行号和表达式的值）
    // dbg! 会获取所有权并返回，所以常用于表达式中
    let debug_color = Color(128, 64, 32);
    println!("dbg! 宏演示:");
    let _returned = dbg!(debug_color); // dbg! 会获取所有权

    // ---------------------------------------------------------
    // 8. 结构体中的所有权
    // ---------------------------------------------------------
    println!("\n--- 8. 结构体中的所有权 ---");

    // User 结构体拥有其所有字段的所有权
    // 使用 String 而不是 &str 是因为我们希望结构体拥有数据
    // 如果想在结构体中存储引用，需要使用生命周期（后续课程讲解）

    // 以下代码无法编译（如果取消注释），因为 &str 需要生命周期标注：
    // struct UserRef {
    //     username: &str,  // ❌ 缺少生命周期标注
    //     email: &str,     // ❌ 缺少生命周期标注
    // }

    // 正确做法：使用 String 拥有数据
    #[derive(Debug)]
    struct OwnedUser {
        username: String,
        email: String,
    }

    let owned = OwnedUser {
        username: String::from("grace"),
        email: String::from("grace@example.com"),
    };
    println!("拥有数据的结构体: {:?}", owned);

    // ---------------------------------------------------------
    // 9. 综合示例：矩形面积计算
    // ---------------------------------------------------------
    println!("\n--- 9. 综合示例：矩形面积计算 ---");

    #[derive(Debug)]
    struct Rectangle {
        width: f64,
        height: f64,
    }

    // 使用结构体让代码更有意义
    fn area(rect: &Rectangle) -> f64 {
        rect.width * rect.height
    }

    let rect = Rectangle {
        width: 30.0,
        height: 50.0,
    };

    println!("矩形: {:?}", rect);
    println!(
        "宽={}, 高={}, 面积={}",
        rect.width,
        rect.height,
        area(&rect)
    );

    // 使用 dbg! 宏查看计算过程
    let scale = 2.0;
    let scaled_rect = Rectangle {
        width: dbg!(rect.width * scale), // dbg! 返回值，所以可以嵌入表达式
        height: rect.height * scale,
    };
    println!("放大后的矩形: {:?}", scaled_rect);
    println!("放大后的面积: {}", area(&scaled_rect));

    println!("\n🎉 恭喜！你已经完成了结构体基础的学习！");
}
