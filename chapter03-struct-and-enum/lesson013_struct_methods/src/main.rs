/// # Lesson 013 - 结构体方法
///
/// 方法（method）是定义在结构体上下文中的函数，让数据和行为紧密结合。
///
/// ## 学习目标
/// - 理解 impl 块的用法
/// - 掌握 &self、&mut self、self 三种方法接收者
/// - 学会定义关联函数（没有 self 参数的函数）
/// - 了解多个 impl 块的用法
/// - 掌握方法链（method chaining）模式
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson013_struct_methods
/// ```

// =============================================================
// Lesson 013: 结构体方法 - Struct Methods
// =============================================================

// ---------------------------------------------------------
// 定义一个矩形结构体，作为演示的基础
// ---------------------------------------------------------
#[derive(Debug, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}

// ---------------------------------------------------------
// 1. impl 块 - 为结构体定义方法
// ---------------------------------------------------------
// 所有在 impl 块中定义的函数都叫做 "关联函数"（associated function）
// 其中第一个参数是 self 的叫做 "方法"（method）
impl Rectangle {
    // ---------------------------------------------------------
    // 关联函数（Associated Function）- 没有 self 参数
    // ---------------------------------------------------------
    // 关联函数通过 :: 语法调用，如 Rectangle::new(10.0, 20.0)
    // 类似于其他语言中的 "静态方法" 或 "构造函数"
    // 最常见的关联函数是 new，用于创建实例
    fn new(width: f64, height: f64) -> Self {
        // Self 是 impl 块所对应类型的别名，这里等同于 Rectangle
        Self { width, height }
    }

    // 另一个关联函数：创建正方形
    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // ---------------------------------------------------------
    // &self 方法 - 不可变借用
    // ---------------------------------------------------------
    // &self 是 self: &Self 的语法糖
    // 方法只读取数据，不修改也不消耗结构体
    // 这是最常用的方法形式

    /// 计算矩形的面积
    fn area(&self) -> f64 {
        self.width * self.height
    }

    /// 计算矩形的周长
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    /// 判断是否为正方形
    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    /// 判断当前矩形是否能完全包含另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    /// 获取宽度（getter 方法）
    /// Rust 允许方法名与字段名同名，这是一种常见的 getter 模式
    fn width(&self) -> f64 {
        self.width
    }

    // ---------------------------------------------------------
    // &mut self 方法 - 可变借用
    // ---------------------------------------------------------
    // &mut self 是 self: &mut Self 的语法糖
    // 方法可以修改结构体的字段

    /// 设置宽度
    fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    /// 设置高度
    fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    /// 按比例缩放矩形
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    // ---------------------------------------------------------
    // self 方法 - 获取所有权
    // ---------------------------------------------------------
    // self 是 self: Self 的语法糖
    // 方法会消耗（移动）原来的结构体
    // 调用后原来的变量不能再使用
    // 通常用于将结构体转换为另一种类型

    /// 将矩形旋转 90 度（宽高互换），消耗原矩形，返回新矩形
    fn rotate(self) -> Rectangle {
        Rectangle {
            width: self.height,
            height: self.width,
        }
    }

    /// 将矩形转换为描述字符串，消耗原矩形
    fn into_description(self) -> String {
        format!(
            "矩形(宽={}, 高={}, 面积={})",
            self.width,
            self.height,
            self.area()
        )
    }
}

// ---------------------------------------------------------
// 2. 多个 impl 块
// ---------------------------------------------------------
// Rust 允许为同一个结构体定义多个 impl 块
// 所有 impl 块中的方法效果相同，主要用于代码组织
// 在使用泛型和 trait 时，多个 impl 块会特别有用
impl Rectangle {
    /// 打印矩形信息
    fn display_info(&self) {
        println!(
            "Rectangle {{ width: {}, height: {}, area: {}, perimeter: {} }}",
            self.width,
            self.height,
            self.area(),
            self.perimeter()
        );
    }
}

// ---------------------------------------------------------
// 3. 方法链（Method Chaining）
// ---------------------------------------------------------
// 通过让 &mut self 方法返回 &mut Self，可以实现方法链
#[derive(Debug)]
struct QueryBuilder {
    table: String,
    conditions: Vec<String>,
    limit: Option<usize>,
    order_by: Option<String>,
}

impl QueryBuilder {
    /// 关联函数：创建新的查询构建器
    fn new(table: &str) -> Self {
        Self {
            table: table.to_string(),
            conditions: Vec::new(),
            limit: None,
            order_by: None,
        }
    }

    /// 添加查询条件 - 返回 &mut Self 以支持链式调用
    fn where_clause(&mut self, condition: &str) -> &mut Self {
        self.conditions.push(condition.to_string());
        self // 返回自身的可变引用
    }

    /// 设置查询数量限制
    fn limit(&mut self, count: usize) -> &mut Self {
        self.limit = Some(count);
        self
    }

    /// 设置排序字段
    fn order_by(&mut self, field: &str) -> &mut Self {
        self.order_by = Some(field.to_string());
        self
    }

    /// 构建最终的 SQL 查询字符串
    fn build(&self) -> String {
        let mut query = format!("SELECT * FROM {}", self.table);

        if !self.conditions.is_empty() {
            query.push_str(" WHERE ");
            query.push_str(&self.conditions.join(" AND "));
        }

        if let Some(ref order) = self.order_by {
            query.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        query
    }
}

// ---------------------------------------------------------
// 4. 所有权语义下方法链的另一种模式（Builder Pattern - 消耗式）
// ---------------------------------------------------------
#[derive(Debug)]
struct Config {
    debug: bool,
    verbose: bool,
    max_retries: u32,
    timeout_secs: u64,
}

impl Config {
    fn new() -> Self {
        Self {
            debug: false,
            verbose: false,
            max_retries: 3,
            timeout_secs: 30,
        }
    }

    // 消耗式构建器：每个方法接收 self 并返回 Self
    // 这种方式转移所有权，适合构建不可变对象
    fn debug(mut self, enable: bool) -> Self {
        self.debug = enable;
        self
    }

    fn verbose(mut self, enable: bool) -> Self {
        self.verbose = enable;
        self
    }

    fn max_retries(mut self, count: u32) -> Self {
        self.max_retries = count;
        self
    }

    fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }
}

// ---------------------------------------------------------
// 5. 带有多种方法接收者的综合示例
// ---------------------------------------------------------
#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    /// 关联函数：创建新账户
    fn new(owner: &str, initial_balance: f64) -> Self {
        Self {
            owner: owner.to_string(),
            balance: initial_balance,
        }
    }

    /// &self：查询余额
    fn balance(&self) -> f64 {
        self.balance
    }

    /// &self：显示账户信息
    fn display(&self) {
        println!("账户 [{}]: 余额 ¥{:.2}", self.owner, self.balance);
    }

    /// &mut self：存款
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("存款 ¥{:.2} 成功", amount);
        } else {
            println!("存款金额必须大于 0");
        }
    }

    /// &mut self：取款
    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && self.balance >= amount {
            self.balance -= amount;
            println!("取款 ¥{:.2} 成功", amount);
            true
        } else {
            println!("取款失败：余额不足或金额无效");
            false
        }
    }

    /// self：关闭账户，消耗账户并返回余额
    fn close(self) -> f64 {
        println!("账户 [{}] 已关闭，退还余额 ¥{:.2}", self.owner, self.balance);
        self.balance
        // 调用后 self 被消耗，BankAccount 实例不再可用
    }
}

fn main() {
    println!("=== Lesson 013: 结构体方法 ===\n");

    // ---------------------------------------------------------
    // 1. 关联函数创建实例
    // ---------------------------------------------------------
    println!("--- 1. 关联函数创建实例 ---");

    // 使用 :: 语法调用关联函数
    let rect1 = Rectangle::new(10.0, 20.0);
    let square = Rectangle::square(15.0);

    println!("矩形: {:?}", rect1);
    println!("正方形: {:?}", square);

    // 类似的标准库示例：
    // String::new()      - 创建空字符串
    // String::from("hi") - 从字面量创建字符串
    // Vec::new()         - 创建空向量

    // ---------------------------------------------------------
    // 2. &self 方法 - 只读访问
    // ---------------------------------------------------------
    println!("\n--- 2. &self 方法 ---");

    println!("面积: {}", rect1.area());
    println!("周长: {}", rect1.perimeter());
    println!("是正方形? {}", rect1.is_square());
    println!("正方形是正方形? {}", square.is_square());

    // 方法名与字段名同名（getter）
    println!("宽度（通过方法）: {}", rect1.width());

    // 方法也可以接收其他参数
    let rect2 = Rectangle::new(5.0, 10.0);
    println!(
        "rect1 能包含 rect2 吗? {}",
        rect1.can_hold(&rect2) // 传递引用
    );
    println!("rect2 能包含 rect1 吗? {}", rect2.can_hold(&rect1));

    // ---------------------------------------------------------
    // 3. &mut self 方法 - 修改数据
    // ---------------------------------------------------------
    println!("\n--- 3. &mut self 方法 ---");

    let mut rect3 = Rectangle::new(10.0, 5.0);
    println!("缩放前: {:?}", rect3);

    rect3.scale(2.0);
    println!("放大 2 倍后: {:?}", rect3);

    rect3.set_width(100.0);
    println!("设置宽度为 100 后: {:?}", rect3);

    rect3.set_height(50.0);
    println!("设置高度为 50 后: {:?}", rect3);

    // ---------------------------------------------------------
    // 4. self 方法 - 消耗所有权
    // ---------------------------------------------------------
    println!("\n--- 4. self 方法 ---");

    let rect4 = Rectangle::new(30.0, 10.0);
    println!("旋转前: {:?}", rect4);

    let rect4_rotated = rect4.rotate(); // rect4 被移动，不能再使用
    println!("旋转后: {:?}", rect4_rotated);
    // println!("{:?}", rect4); // ❌ 编译错误！rect4 已被移动

    let rect5 = Rectangle::new(8.0, 6.0);
    let description = rect5.into_description(); // rect5 被消耗
    println!("描述: {}", description);
    // println!("{:?}", rect5); // ❌ 编译错误！rect5 已被消耗

    // ---------------------------------------------------------
    // 5. 自动引用和解引用
    // ---------------------------------------------------------
    println!("\n--- 5. 自动引用和解引用 ---");

    // Rust 有自动引用（automatic referencing）功能
    // 当调用方法时，Rust 会自动添加 &、&mut 或 * 来匹配方法签名
    // 所以以下两种写法等价：
    let rect = Rectangle::new(5.0, 3.0);

    let a1 = rect.area();       // Rust 自动添加 &，等价于下面的写法
    let a2 = (&rect).area();    // 手动添加 &
    println!("自动引用: {}, 手动引用: {}", a1, a2);

    // 这就是为什么 Rust 不需要 -> 运算符（C/C++ 中用于指针调用方法）
    // Rust 的方法调用会自动解引用

    // ---------------------------------------------------------
    // 6. 方法链（借用式）
    // ---------------------------------------------------------
    println!("\n--- 6. 方法链（借用式） ---");

    let sql = QueryBuilder::new("users")
        .where_clause("age > 18")
        .where_clause("active = true")
        .order_by("created_at DESC")
        .limit(10)
        .build();

    println!("生成的 SQL: {}", sql);

    // 没有任何条件的查询
    let simple_sql = QueryBuilder::new("products").limit(5).build();
    println!("简单查询: {}", simple_sql);

    // ---------------------------------------------------------
    // 7. 方法链（消耗式 - Builder Pattern）
    // ---------------------------------------------------------
    println!("\n--- 7. 方法链（消耗式 Builder Pattern） ---");

    let config = Config::new()
        .debug(true)
        .verbose(false)
        .max_retries(5)
        .timeout(60);

    println!("配置: {:?}", config);

    // 使用默认配置
    let default_config = Config::new();
    println!("默认配置: {:?}", default_config);

    // ---------------------------------------------------------
    // 8. 综合示例：银行账户
    // ---------------------------------------------------------
    println!("\n--- 8. 综合示例：银行账户 ---");

    let mut account = BankAccount::new("张三", 1000.0);
    account.display(); // &self

    account.deposit(500.0); // &mut self
    account.display();

    account.withdraw(200.0); // &mut self
    println!("当前余额: ¥{:.2}", account.balance()); // &self

    account.withdraw(2000.0); // 余额不足

    let remaining = account.close(); // self - 消耗账户
    println!("退还金额: ¥{:.2}", remaining);
    // account.display(); // ❌ 编译错误！account 已被 close() 消耗

    // ---------------------------------------------------------
    // 9. 多个 impl 块
    // ---------------------------------------------------------
    println!("\n--- 9. 多个 impl 块 ---");

    // display_info 定义在第二个 impl 块中，但使用方式完全相同
    let rect = Rectangle::new(12.0, 8.0);
    rect.display_info(); // 来自第二个 impl 块

    // 在实际项目中，多个 impl 块常用于：
    // - 按功能分组方法
    // - 为不同的 trait 实现分别定义 impl 块
    // - 使用条件编译（#[cfg]）为不同平台提供不同实现

    println!("\n🎉 恭喜！你已经完成了结构体方法的学习！");
}
