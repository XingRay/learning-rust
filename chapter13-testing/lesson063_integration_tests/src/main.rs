// ============================================================
// Lesson 063: 集成测试 (Integration Tests)
// ============================================================
// 本课学习 Rust 中的集成测试，包括：
// - tests/ 目录结构和组织方式
// - 集成测试如何从外部测试你的库
// - 公共模块（辅助函数）的组织
// - cargo test --test 指定运行特定集成测试文件
// - 测试组织策略
//
// 【重要说明】
// 集成测试在 Rust 项目中有严格的目录结构要求：
//
// 项目结构示例：
// my_project/
// ├── Cargo.toml
// ├── src/
// │   ├── lib.rs          # 库代码（集成测试只能测试 lib crate）
// │   └── main.rs         # 可选的二进制入口
// ├── tests/              # 集成测试目录（与 src 同级）
// │   ├── test_basic.rs   # 每个文件是一个独立的测试 crate
// │   ├── test_advanced.rs
// │   └── common/         # 公共辅助模块（不会被当作测试文件）
// │       └── mod.rs      # 辅助函数定义
// └── benches/            # 基准测试目录
//
// 【关键点】
// 1. tests/ 目录中的每个 .rs 文件都是一个独立的 crate
// 2. 集成测试只能测试 lib crate 的公共 API（pub 项）
// 3. 二进制 crate（只有 main.rs）不能直接做集成测试
// 4. tests/ 下的子目录中的文件不会被当作独立测试文件
//    （所以公共辅助代码放在 tests/common/mod.rs 中）
//
// 本课因为是单文件示例（binary crate），无法真正创建 tests/ 目录，
// 但我们会模拟集成测试的写法，并用详细注释说明实际项目中的做法。
// ============================================================

// ============================================================
// 模拟一个"库"的公共 API
// 在真实项目中，这些代码应该在 src/lib.rs 中
// 集成测试只能访问 pub 标记的函数和类型
// ============================================================

/// 一个简单的计算器模块
pub mod calculator {
    /// 加法
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// 减法
    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    /// 乘法
    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    /// 安全除法
    pub fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("除数不能为零".to_string())
        } else {
            Ok(a / b)
        }
    }

    /// 私有辅助函数 —— 集成测试无法访问此函数
    /// 只有单元测试（在同文件中的 #[cfg(test)] 模块）能访问
    fn _internal_helper() -> &'static str {
        "我是私有函数"
    }
}

/// 一个用户管理模块
pub mod user {
    #[derive(Debug, Clone, PartialEq)]
    pub struct User {
        pub name: String,
        pub email: String,
        pub age: u32,
    }

    impl User {
        pub fn new(name: &str, email: &str, age: u32) -> Result<User, String> {
            if name.is_empty() {
                return Err("用户名不能为空".to_string());
            }
            if !email.contains('@') {
                return Err("邮箱格式不正确".to_string());
            }
            if age > 150 {
                return Err("年龄不合理".to_string());
            }
            Ok(User {
                name: name.to_string(),
                email: email.to_string(),
                age,
            })
        }

        pub fn is_adult(&self) -> bool {
            self.age >= 18
        }

        pub fn greeting(&self) -> String {
            format!("你好，{}！", self.name)
        }
    }

    /// 用户仓库（模拟数据存储）
    pub struct UserRepository {
        users: Vec<User>,
    }

    impl UserRepository {
        pub fn new() -> Self {
            UserRepository { users: Vec::new() }
        }

        pub fn add_user(&mut self, user: User) {
            self.users.push(user);
        }

        pub fn find_by_name(&self, name: &str) -> Option<&User> {
            self.users.iter().find(|u| u.name == name)
        }

        pub fn count(&self) -> usize {
            self.users.len()
        }

        pub fn adults(&self) -> Vec<&User> {
            self.users.iter().filter(|u| u.is_adult()).collect()
        }
    }
}

fn main() {
    println!("=== Lesson 063: 集成测试 ===\n");

    // ---- 演示公共 API 的使用 ----
    println!("--- 计算器模块 ---");
    println!("add(5, 3) = {}", calculator::add(5, 3));
    println!("subtract(10, 4) = {}", calculator::subtract(10, 4));
    println!("multiply(6, 7) = {}", calculator::multiply(6, 7));
    match calculator::divide(10.0, 3.0) {
        Ok(v) => println!("divide(10.0, 3.0) = {:.4}", v),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n--- 用户模块 ---");
    let alice = user::User::new("Alice", "alice@example.com", 25).unwrap();
    println!("用户: {:?}", alice);
    println!("是否成年: {}", alice.is_adult());
    println!("{}", alice.greeting());

    let mut repo = user::UserRepository::new();
    repo.add_user(alice);
    repo.add_user(user::User::new("Bob", "bob@example.com", 15).unwrap());
    println!("用户总数: {}", repo.count());
    println!("成年用户数: {}", repo.adults().len());

    // ---- 集成测试目录结构说明 ----
    println!("\n--- 集成测试目录结构说明 ---");
    println!("在真实项目中，集成测试这样组织：");
    println!();
    println!("my_project/");
    println!("├── Cargo.toml");
    println!("├── src/");
    println!("│   └── lib.rs               # 库的公共 API");
    println!("├── tests/                    # 集成测试目录");
    println!("│   ├── calculator_tests.rs   # 计算器的集成测试");
    println!("│   ├── user_tests.rs         # 用户模块的集成测试");
    println!("│   └── common/              # 公共辅助模块");
    println!("│       └── mod.rs           # 辅助函数");
    println!();

    println!("--- cargo test 集成测试相关命令 ---");
    println!("运行所有测试（单元+集成）:     cargo test");
    println!("只运行集成测试:               cargo test --test '*'");
    println!("运行特定集成测试文件:          cargo test --test calculator_tests");
    println!("运行集成测试中的特定函数:      cargo test --test user_tests test_create_user");
}

// ============================================================
// 模拟集成测试的写法
// ============================================================
// 在真实项目中，以下代码会放在 tests/ 目录下的独立文件中。
// 例如 tests/calculator_tests.rs:
//
// ```rust
// // tests/calculator_tests.rs
// // 注意：集成测试中不需要 #[cfg(test)]，因为 tests/ 目录
// // 本身就只在 cargo test 时编译
//
// use my_project::calculator;  // 导入库的公共 API
//
// mod common;  // 导入公共辅助模块
//
// #[test]
// fn test_add() {
//     assert_eq!(calculator::add(2, 3), 5);
// }
// ```
//
// tests/common/mod.rs:
// ```rust
// // 公共辅助函数
// // 注意：使用 common/mod.rs 而不是 common.rs
// // 这样 Cargo 不会把它当作独立的测试文件
//
// pub fn setup() {
//     // 初始化测试环境...
// }
//
// pub fn create_test_user() -> my_project::user::User {
//     my_project::user::User::new("TestUser", "test@example.com", 25).unwrap()
// }
// ```
// ============================================================

// 因为这是 binary crate，我们用 #[cfg(test)] 模拟集成测试风格
#[cfg(test)]
mod integration_style_tests {
    use super::calculator;
    use super::user;

    // ============================================================
    // 模拟 tests/common/mod.rs 的辅助模块
    // ============================================================
    mod common {
        use super::super::user;

        /// 设置测试环境（模拟）
        pub fn setup() {
            // 在真实项目中，这里可能会：
            // - 初始化日志系统
            // - 清理测试数据库
            // - 设置环境变量
            // println!("测试环境已初始化");  // cargo test 默认会捕获输出
        }

        /// 创建测试用的用户仓库，预填充一些数据
        pub fn create_test_repo() -> user::UserRepository {
            let mut repo = user::UserRepository::new();
            repo.add_user(user::User::new("Alice", "alice@test.com", 30).unwrap());
            repo.add_user(user::User::new("Bob", "bob@test.com", 16).unwrap());
            repo.add_user(user::User::new("Charlie", "charlie@test.com", 25).unwrap());
            repo
        }
    }

    // ============================================================
    // 模拟 tests/calculator_tests.rs
    // 集成测试测试的是模块之间的协作和公共 API
    // ============================================================

    #[test]
    fn test_calculator_add_and_subtract() {
        // 集成测试关注的是公共 API 的行为，而不是内部实现
        let sum = calculator::add(10, 5);
        let diff = calculator::subtract(sum, 3);
        assert_eq!(diff, 12);
    }

    #[test]
    fn test_calculator_multiply_and_divide() {
        let product = calculator::multiply(6, 7);
        assert_eq!(product, 42);

        let quotient = calculator::divide(product as f64, 7.0).unwrap();
        assert_eq!(quotient, 6.0);
    }

    #[test]
    fn test_calculator_divide_by_zero() {
        let result = calculator::divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "除数不能为零");
    }

    #[test]
    fn test_calculator_expression() {
        // 模拟一个复杂表达式: (10 + 5) * 2 - 3
        let step1 = calculator::add(10, 5);       // 15
        let step2 = calculator::multiply(step1, 2); // 30
        let result = calculator::subtract(step2, 3); // 27
        assert_eq!(result, 27);
    }

    // ============================================================
    // 模拟 tests/user_tests.rs
    // ============================================================

    #[test]
    fn test_create_valid_user() {
        common::setup();
        let user = user::User::new("张三", "zhangsan@example.com", 28);
        assert!(user.is_ok());
        let user = user.unwrap();
        assert_eq!(user.name, "张三");
        assert_eq!(user.email, "zhangsan@example.com");
        assert_eq!(user.age, 28);
    }

    #[test]
    fn test_create_user_empty_name() {
        let result = user::User::new("", "test@example.com", 25);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户名不能为空");
    }

    #[test]
    fn test_create_user_invalid_email() {
        let result = user::User::new("Test", "invalid-email", 25);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "邮箱格式不正确");
    }

    #[test]
    fn test_create_user_unreasonable_age() {
        let result = user::User::new("Test", "test@example.com", 200);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "年龄不合理");
    }

    #[test]
    fn test_user_is_adult() {
        let adult = user::User::new("成年人", "a@b.com", 18).unwrap();
        assert!(adult.is_adult());

        let minor = user::User::new("未成年人", "a@b.com", 17).unwrap();
        assert!(!minor.is_adult());
    }

    // ============================================================
    // 使用公共辅助函数的集成测试
    // ============================================================

    #[test]
    fn test_user_repository_with_setup() {
        common::setup();
        let repo = common::create_test_repo();

        // 验证预填充的数据
        assert_eq!(repo.count(), 3);
        assert!(repo.find_by_name("Alice").is_some());
        assert!(repo.find_by_name("Unknown").is_none());
    }

    #[test]
    fn test_user_repository_adults() {
        let repo = common::create_test_repo();
        let adults = repo.adults();

        // Alice(30) 和 Charlie(25) 是成年人，Bob(16) 不是
        assert_eq!(adults.len(), 2);
    }

    #[test]
    fn test_user_repository_add_and_find() {
        let mut repo = user::UserRepository::new();
        assert_eq!(repo.count(), 0);

        let user = user::User::new("新用户", "new@test.com", 22).unwrap();
        repo.add_user(user.clone());

        assert_eq!(repo.count(), 1);
        let found = repo.find_by_name("新用户").unwrap();
        assert_eq!(found, &user);
    }

    #[test]
    fn test_user_greeting() {
        let user = user::User::new("世界", "world@test.com", 20).unwrap();
        assert_eq!(user.greeting(), "你好，世界！");
    }

    // ============================================================
    // 测试组织策略说明
    // ============================================================
    //
    // 最佳实践：
    //
    // 1. 单元测试 vs 集成测试
    //    - 单元测试：在 src/ 中的 #[cfg(test)] 模块，测试内部逻辑
    //    - 集成测试：在 tests/ 目录，只测试公共 API
    //
    // 2. 测试文件组织
    //    - 按功能模块划分测试文件：tests/calculator_tests.rs, tests/user_tests.rs
    //    - 公共辅助代码放在 tests/common/mod.rs
    //    - 不要在 tests/ 根目录放辅助用的 .rs 文件（会被当作测试运行）
    //
    // 3. 二进制 crate 的测试策略
    //    - 如果项目只有 main.rs，不能写集成测试
    //    - 推荐做法：将逻辑抽取到 lib.rs，main.rs 只做入口
    //    - 这样可以对 lib.rs 做集成测试
    //
    // 4. 测试粒度
    //    - 单元测试：细粒度，测试每个函数的各种边界情况
    //    - 集成测试：粗粒度，测试模块间的交互和完整工作流
    //
    // 5. 命令行技巧
    //    - `cargo test`：运行全部测试
    //    - `cargo test --lib`：只运行 lib.rs 中的单元测试
    //    - `cargo test --test calculator_tests`：只运行指定的集成测试文件
    //    - `cargo test --doc`：只运行文档测试
    //    - `cargo test -- --test-threads=1`：单线程运行（避免并行冲突）
    // ============================================================
}
