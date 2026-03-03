#![allow(dead_code)]
// ============================================================
// Lesson 071: Workspace (工作空间)
// ============================================================
// Workspace 是 Cargo 提供的管理多个相关 crate 的机制。
// 它允许多个 package 共享同一个 Cargo.lock 和输出目录（target/）。
//
// 本课讲解 workspace 的概念、配置和使用方式。
// ============================================================

// ============================================================
// 1. Workspace 概念和优势
// ============================================================
//
// 什么是 Workspace？
// - 一组共享 Cargo.lock 和 target/ 目录的 packages
// - 通过根目录的 Cargo.toml 中的 [workspace] 定义
// - 类似于其他语言的 monorepo 概念
//
// Workspace 的优势：
// (1) 共享依赖版本：所有成员 crate 使用相同版本的第三方依赖，避免版本冲突
// (2) 统一编译缓存：共享 target/ 目录，减少重复编译
// (3) 方便交叉引用：成员 crate 之间可以通过 path 依赖互相使用
// (4) 统一管理：一个 `cargo build` 即可编译所有成员
// (5) 原子性修改：可以在一次提交中修改多个 crate

// ============================================================
// 2. [workspace] 配置说明
// ============================================================
//
// workspace 的 Cargo.toml 有两种方式：
//
// 方式一：虚拟工作空间（Virtual Workspace）
// 根目录只有 [workspace]，没有 [package]：
//
// ```toml
// # /my_project/Cargo.toml
// [workspace]
// resolver = "2"                    # 推荐使用 resolver 2
// members = [
//     "crates/core",                # 核心库
//     "crates/utils",               # 工具库
//     "crates/cli",                 # 命令行工具
//     "crates/server",              # 服务器
// ]
// ```
//
// 方式二：根 package + workspace
// 根目录既是 package 又是 workspace：
//
// ```toml
// # /my_project/Cargo.toml
// [package]
// name = "my_project"
// version = "0.1.0"
// edition = "2021"
//
// [workspace]
// members = [
//     "sub_crate_a",
//     "sub_crate_b",
// ]
// ```
//
// 排除特定目录：
// ```toml
// [workspace]
// members = ["crates/*"]            # 支持 glob 模式
// exclude = ["crates/experimental"] # 排除特定目录
// ```

// ============================================================
// 3. 共享配置（workspace.dependencies）
// ============================================================
//
// Rust 1.64+ 支持在 workspace 级别定义共享依赖：
//
// ```toml
// # 根 Cargo.toml
// [workspace]
// members = ["crate_a", "crate_b"]
//
// [workspace.dependencies]
// serde = { version = "1.0", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }
// log = "0.4"
// ```
//
// ```toml
// # crate_a/Cargo.toml
// [package]
// name = "crate_a"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// serde = { workspace = true }      # 引用 workspace 中定义的版本
// tokio = { workspace = true }
// ```
//
// 这样所有 crate 使用统一的依赖版本，避免版本冲突。

// ============================================================
// 4. 代码演示：模拟 workspace 中多个 crate 的协作
// ============================================================

/// 模拟 workspace 中的 "core" crate —— 核心类型和逻辑
mod core_crate {
    /// 用户模型
    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u64,
        pub name: String,
        pub email: String,
    }

    impl User {
        pub fn new(id: u64, name: &str, email: &str) -> Self {
            User {
                id,
                name: name.to_string(),
                email: email.to_string(),
            }
        }

        pub fn display_name(&self) -> String {
            format!("{} (ID: {})", self.name, self.id)
        }
    }

    /// 订单模型
    #[derive(Debug, Clone)]
    pub struct Order {
        pub id: u64,
        pub user_id: u64,
        pub items: Vec<OrderItem>,
    }

    #[derive(Debug, Clone)]
    pub struct OrderItem {
        pub product: String,
        pub quantity: u32,
        pub price: f64,
    }

    impl Order {
        pub fn new(id: u64, user_id: u64) -> Self {
            Order {
                id,
                user_id,
                items: Vec::new(),
            }
        }

        pub fn add_item(&mut self, product: &str, quantity: u32, price: f64) {
            self.items.push(OrderItem {
                product: product.to_string(),
                quantity,
                price,
            });
        }

        pub fn total(&self) -> f64 {
            self.items
                .iter()
                .map(|item| item.price * item.quantity as f64)
                .sum()
        }
    }
}

/// 模拟 workspace 中的 "utils" crate —— 工具函数
/// 在实际 workspace 中，这个 crate 的 Cargo.toml 会写：
/// [dependencies]
/// core_crate = { path = "../core" }
mod utils_crate {
    use super::core_crate::{Order, User};

    /// 格式化用户信息
    pub fn format_user(user: &User) -> String {
        format!(
            "┌─ 用户信息 ─────────────\n│ ID:    {}\n│ 姓名:  {}\n│ 邮箱:  {}\n└────────────────────────",
            user.id, user.name, user.email
        )
    }

    /// 格式化订单信息
    pub fn format_order(order: &Order) -> String {
        let mut result = format!(
            "┌─ 订单 #{} (用户ID: {}) ──\n",
            order.id, order.user_id
        );
        for item in &order.items {
            result.push_str(&format!(
                "│ {} x{} = ¥{:.2}\n",
                item.product,
                item.quantity,
                item.price * item.quantity as f64
            ));
        }
        result.push_str(&format!("│ 总计: ¥{:.2}\n", order.total()));
        result.push_str("└────────────────────────");
        result
    }

    /// 验证邮箱格式（简单版本）
    pub fn validate_email(email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}

/// 模拟 workspace 中的 "service" crate —— 业务逻辑层
/// 在实际 workspace 中，依赖 core 和 utils：
/// [dependencies]
/// core_crate = { path = "../core" }
/// utils_crate = { path = "../utils" }
mod service_crate {
    use super::core_crate::{Order, User};
    use super::utils_crate;

    /// 简单的用户服务
    pub struct UserService {
        users: Vec<User>,
    }

    impl UserService {
        pub fn new() -> Self {
            UserService { users: Vec::new() }
        }

        pub fn register(&mut self, name: &str, email: &str) -> Result<&User, String> {
            // 验证邮箱
            if !utils_crate::validate_email(email) {
                return Err(format!("无效的邮箱地址: {}", email));
            }

            // 检查重复
            if self.users.iter().any(|u| u.email == email) {
                return Err(format!("邮箱已注册: {}", email));
            }

            let id = self.users.len() as u64 + 1;
            let user = User::new(id, name, email);
            self.users.push(user);
            Ok(self.users.last().unwrap())
        }

        pub fn find_user(&self, id: u64) -> Option<&User> {
            self.users.iter().find(|u| u.id == id)
        }

        pub fn list_users(&self) -> &[User] {
            &self.users
        }
    }

    /// 简单的订单服务
    pub struct OrderService {
        orders: Vec<Order>,
    }

    impl OrderService {
        pub fn new() -> Self {
            OrderService {
                orders: Vec::new(),
            }
        }

        pub fn create_order(&mut self, user_id: u64) -> &mut Order {
            let id = self.orders.len() as u64 + 1;
            let order = Order::new(id, user_id);
            self.orders.push(order);
            self.orders.last_mut().unwrap()
        }

        pub fn get_order(&self, id: u64) -> Option<&Order> {
            self.orders.iter().find(|o| o.id == id)
        }

        pub fn user_orders(&self, user_id: u64) -> Vec<&Order> {
            self.orders.iter().filter(|o| o.user_id == user_id).collect()
        }
    }
}

fn main() {
    println!("=== Lesson 071: Workspace ===\n");

    // ---- 1. Workspace 概念 ----
    println!("--- 1. Workspace 概念 ---");
    println!("Workspace = 多个 crate 共享 Cargo.lock + target/");
    println!("优势:");
    println!("  • 统一依赖版本，避免冲突");
    println!("  • 共享编译缓存，加快构建");
    println!("  • crate 间通过 path 依赖互相引用");
    println!("  • 一个命令管理所有 crate");
    println!();

    // ---- 2. 模拟 workspace 中多 crate 协作 ----
    println!("--- 2. 多 Crate 协作演示 ---");
    println!("模拟 workspace 结构:");
    println!("  my_workspace/");
    println!("  ├── Cargo.toml          (workspace 根配置)");
    println!("  ├── crates/");
    println!("  │   ├── core/           (核心模型)");
    println!("  │   ├── utils/          (工具函数)");
    println!("  │   └── service/        (业务逻辑)");
    println!("  └── Cargo.lock          (共享锁文件)");
    println!();

    // ---- 3. 使用 core crate ----
    println!("--- 3. Core Crate: 创建用户和订单 ---");
    let user = core_crate::User::new(1, "张三", "zhangsan@example.com");
    println!("用户: {}", user.display_name());

    let mut order = core_crate::Order::new(1, user.id);
    order.add_item("Rust 编程之道", 1, 89.0);
    order.add_item("Rust 标准库 Cookbook", 2, 59.0);
    println!("订单总额: ¥{:.2}", order.total());
    println!();

    // ---- 4. 使用 utils crate ----
    println!("--- 4. Utils Crate: 格式化输出 ---");
    println!("{}", utils_crate::format_user(&user));
    println!();
    println!("{}", utils_crate::format_order(&order));
    println!();

    // ---- 5. 使用 service crate ----
    println!("--- 5. Service Crate: 业务逻辑 ---");
    let mut user_service = service_crate::UserService::new();
    let mut order_service = service_crate::OrderService::new();

    // 注册用户
    match user_service.register("李四", "lisi@example.com") {
        Ok(u) => println!("注册成功: {}", u.display_name()),
        Err(e) => println!("注册失败: {}", e),
    }
    match user_service.register("王五", "wangwu@example.com") {
        Ok(u) => println!("注册成功: {}", u.display_name()),
        Err(e) => println!("注册失败: {}", e),
    }
    // 测试无效邮箱
    match user_service.register("赵六", "invalid-email") {
        Ok(u) => println!("注册成功: {}", u.display_name()),
        Err(e) => println!("注册失败: {}", e),
    }
    // 测试重复邮箱
    match user_service.register("李四2", "lisi@example.com") {
        Ok(u) => println!("注册成功: {}", u.display_name()),
        Err(e) => println!("注册失败: {}", e),
    }
    println!();

    // 创建订单
    {
        let o = order_service.create_order(1);
        o.add_item("键盘", 1, 299.0);
        o.add_item("鼠标", 1, 149.0);
    }
    {
        let o = order_service.create_order(1);
        o.add_item("显示器", 1, 1999.0);
    }
    {
        let o = order_service.create_order(2);
        o.add_item("耳机", 2, 199.0);
    }

    // 查询订单
    if let Some(user) = user_service.find_user(1) {
        let orders = order_service.user_orders(user.id);
        println!("{} 的订单:", user.display_name());
        for o in orders {
            println!("{}", utils_crate::format_order(o));
        }
    }
    println!();

    // 所有用户列表
    println!("所有注册用户:");
    for u in user_service.list_users() {
        println!("  - {}: {}", u.display_name(), u.email);
    }
    println!();

    // ---- 6. 成员间依赖（path 依赖）----
    println!("--- 6. 成员间 path 依赖 ---");
    println!("在 workspace 中，crate 之间通过 path 依赖引用:");
    println!();
    println!("  # service/Cargo.toml");
    println!("  [dependencies]");
    println!("  core = {{ path = \"../core\" }}");
    println!("  utils = {{ path = \"../utils\" }}");
    println!();
    println!("  # 代码中直接 use:");
    println!("  use core::User;");
    println!("  use utils::format_user;");
    println!();

    // ---- 7. Cargo.lock 共享 ----
    println!("--- 7. 共享 Cargo.lock ---");
    println!("所有 workspace 成员共享同一个 Cargo.lock 文件");
    println!("确保所有 crate 使用完全相同版本的第三方依赖");
    println!("这避免了'在我的机器上能编译'的问题");
    println!();

    // ---- 8. Cargo 命令与 -p 参数 ----
    println!("--- 8. Cargo 命令与 -p 参数 ---");
    println!("在 workspace 根目录运行 cargo 命令:");
    println!("  cargo build              → 构建所有成员");
    println!("  cargo build -p core      → 只构建 core crate");
    println!("  cargo test               → 测试所有成员");
    println!("  cargo test -p utils      → 只测试 utils crate");
    println!("  cargo run -p cli         → 运行 cli crate");
    println!("  cargo check -p service   → 检查 service crate");
    println!();

    // 结合本项目说明
    println!("本项目 (learning-rust) 本身就是一个 workspace!");
    println!("根 Cargo.toml 使用 [workspace] + members 管理所有 lesson:");
    println!("  [workspace]");
    println!("  resolver = \"2\"");
    println!("  members = [");
    println!("      \"chapter01-basic/lesson001_helloworld\",");
    println!("      \"chapter14-cargo-and-modules/lesson071_workspace\",");
    println!("      // ... 更多 lesson");
    println!("  ]");
    println!();
    println!("可以使用 -p 参数单独操作某个 lesson:");
    println!("  cargo run -p lesson071_workspace");
    println!("  cargo check -p lesson067_module_system");
    println!();

    // ---- 总结 ----
    println!("=== 总结 ===");
    println!("1. Workspace 管理多个相关的 crate，共享 Cargo.lock 和 target/");
    println!("2. 在根 Cargo.toml 的 [workspace] 中用 members 列出成员");
    println!("3. 成员间用 path 依赖互相引用");
    println!("4. workspace.dependencies 统一管理依赖版本");
    println!("5. 使用 cargo -p <crate> 操作特定成员");
}
