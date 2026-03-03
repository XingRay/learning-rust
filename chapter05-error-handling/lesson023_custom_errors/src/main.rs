/// # Lesson 023 - 自定义错误
///
/// 本课介绍如何在 Rust 中创建自定义错误类型，实现完善的错误处理体系。
///
/// ## 学习目标
/// - 学习使用 enum 定义自定义错误类型
/// - 实现 `std::fmt::Display` trait 提供用户友好的错误信息
/// - 实现 `std::error::Error` trait 融入 Rust 错误生态
/// - 使用 `From` trait 实现错误类型的自动转换
/// - 了解 `thiserror` 和 `anyhow` 库的作用（不引入依赖）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson023_custom_errors
/// ```

// =============================================================
// Lesson 023: 自定义错误
// =============================================================

use std::error::Error;
use std::fmt;
use std::io;
use std::num::ParseIntError;

fn main() {
    println!("===== Lesson 023: 自定义错误 =====\n");

    // ---------------------------------------------------------
    // 1. 为什么需要自定义错误
    // ---------------------------------------------------------
    println!("--- 1. 为什么需要自定义错误 ---");
    println!("在实际项目中，我们需要自定义错误的原因：");
    println!("  1. 统一不同来源的错误类型（IO、解析、网络等）");
    println!("  2. 提供更有意义的错误信息给调用者");
    println!("  3. 允许调用者根据错误种类做不同处理");
    println!("  4. 隐藏内部实现细节（只暴露应用层面的错误）");
    println!();

    // ---------------------------------------------------------
    // 2. 使用 enum 定义自定义错误类型
    // ---------------------------------------------------------
    // 自定义错误通常使用 enum 来定义，每个变体表示一种错误情况。
    println!("--- 2. 使用 enum 定义自定义错误类型 ---");

    // 一个用户管理系统的错误类型
    #[derive(Debug)]
    enum UserError {
        // 用户未找到
        NotFound(String),
        // 无效的输入
        InvalidInput {
            field: String,
            reason: String,
        },
        // 权限不足
        PermissionDenied,
        // IO 错误（包裹了底层的 io::Error）
        IoError(io::Error),
        // 解析错误（包裹了底层的 ParseIntError）
        ParseError(ParseIntError),
    }

    // ---------------------------------------------------------
    // 3. 实现 std::fmt::Display
    // ---------------------------------------------------------
    // Display trait 提供用户友好的错误信息。
    // 这是对外展示的信息，应该简洁明了。
    println!("--- 3. 实现 std::fmt::Display ---");

    impl fmt::Display for UserError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                UserError::NotFound(name) => {
                    write!(f, "用户 '{}' 未找到", name)
                }
                UserError::InvalidInput { field, reason } => {
                    write!(f, "字段 '{}' 无效: {}", field, reason)
                }
                UserError::PermissionDenied => {
                    write!(f, "权限不足，拒绝访问")
                }
                UserError::IoError(e) => {
                    write!(f, "IO 错误: {}", e)
                }
                UserError::ParseError(e) => {
                    write!(f, "解析错误: {}", e)
                }
            }
        }
    }

    // 演示 Display 输出
    let errors = vec![
        UserError::NotFound("张三".to_string()),
        UserError::InvalidInput {
            field: "email".to_string(),
            reason: "格式不正确".to_string(),
        },
        UserError::PermissionDenied,
    ];

    for err in &errors {
        println!("  Display: {}", err);   // 调用 Display trait
        println!("  Debug:   {:?}", err);  // 调用 Debug trait
        println!();
    }

    // ---------------------------------------------------------
    // 4. 实现 std::error::Error
    // ---------------------------------------------------------
    // Error trait 是 Rust 错误处理生态的核心。
    // 实现了 Error trait 的类型可以：
    //   - 被 Box<dyn Error> 持有
    //   - 用 ? 操作符传播
    //   - 提供错误源（source）用于错误链追踪
    //
    // Error trait 要求：
    //   - 必须实现 Debug + Display
    //   - 可选：实现 source() 方法返回底层错误
    println!("--- 4. 实现 std::error::Error ---");

    impl Error for UserError {
        // source() 返回导致此错误的底层错误（如果有的话）
        // 这对于构建错误链非常有用
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                UserError::IoError(e) => Some(e),     // 底层是 io::Error
                UserError::ParseError(e) => Some(e),   // 底层是 ParseIntError
                _ => None,                              // 其他错误没有底层错误源
            }
        }
    }

    // 演示错误链
    let io_err = io::Error::new(io::ErrorKind::NotFound, "文件不存在");
    let user_err = UserError::IoError(io_err);
    println!("错误: {}", user_err);
    if let Some(source) = user_err.source() {
        println!("底层错误源: {}", source);
    }

    println!();

    // ---------------------------------------------------------
    // 5. From trait 转换
    // ---------------------------------------------------------
    // 实现 From trait 后，? 操作符可以自动将底层错误转换为自定义错误。
    println!("--- 5. From trait 转换 ---");

    // 从 io::Error 转换
    impl From<io::Error> for UserError {
        fn from(e: io::Error) -> Self {
            UserError::IoError(e)
        }
    }

    // 从 ParseIntError 转换
    impl From<ParseIntError> for UserError {
        fn from(e: ParseIntError) -> Self {
            UserError::ParseError(e)
        }
    }

    // 现在可以使用 ? 操作符自动转换了
    fn load_user_age(data: &str) -> Result<u32, UserError> {
        // parse::<u32>() 返回 Result<u32, ParseIntError>
        // ? 会自动通过 From<ParseIntError> for UserError 进行转换
        let age: u32 = data.trim().parse()?;

        if age > 150 {
            return Err(UserError::InvalidInput {
                field: "age".to_string(),
                reason: format!("年龄 {} 不合理", age),
            });
        }

        Ok(age)
    }

    let test_ages = vec!["25", "abc", "200", "30"];
    for age_str in test_ages {
        match load_user_age(age_str) {
            Ok(age) => println!("  \"{}\" -> 年龄: {}", age_str, age),
            Err(e) => println!("  \"{}\" -> 错误: {}", age_str, e),
        }
    }

    println!();

    // ---------------------------------------------------------
    // 6. 完整实战示例：用户注册系统
    // ---------------------------------------------------------
    println!("--- 6. 完整实战示例：用户注册系统 ---");

    // 定义用户
    #[derive(Debug)]
    #[allow(dead_code)]
    struct User {
        name: String,
        age: u32,
        email: String,
    }

    // 注册系统错误
    #[derive(Debug)]
    enum RegistrationError {
        EmptyName,
        InvalidAge(String),
        InvalidEmail(String),
        DuplicateUser(String),
        StorageError(io::Error),
    }

    impl fmt::Display for RegistrationError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                RegistrationError::EmptyName => write!(f, "用户名不能为空"),
                RegistrationError::InvalidAge(reason) => {
                    write!(f, "年龄无效: {}", reason)
                }
                RegistrationError::InvalidEmail(reason) => {
                    write!(f, "邮箱无效: {}", reason)
                }
                RegistrationError::DuplicateUser(name) => {
                    write!(f, "用户 '{}' 已存在", name)
                }
                RegistrationError::StorageError(e) => {
                    write!(f, "存储错误: {}", e)
                }
            }
        }
    }

    impl Error for RegistrationError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                RegistrationError::StorageError(e) => Some(e),
                _ => None,
            }
        }
    }

    impl From<io::Error> for RegistrationError {
        fn from(e: io::Error) -> Self {
            RegistrationError::StorageError(e)
        }
    }

    // 验证函数
    fn validate_name(name: &str) -> Result<String, RegistrationError> {
        let name = name.trim().to_string();
        if name.is_empty() {
            return Err(RegistrationError::EmptyName);
        }
        Ok(name)
    }

    fn validate_age(age_str: &str) -> Result<u32, RegistrationError> {
        let age: u32 = age_str
            .trim()
            .parse()
            .map_err(|_| RegistrationError::InvalidAge(format!("'{}' 不是有效数字", age_str)))?;
        if age == 0 || age > 150 {
            return Err(RegistrationError::InvalidAge(format!(
                "年龄 {} 超出合理范围 (1-150)",
                age
            )));
        }
        Ok(age)
    }

    fn validate_email(email: &str) -> Result<String, RegistrationError> {
        let email = email.trim().to_string();
        if !email.contains('@') {
            return Err(RegistrationError::InvalidEmail(
                "缺少 @ 符号".to_string(),
            ));
        }
        if !email.contains('.') {
            return Err(RegistrationError::InvalidEmail(
                "缺少域名后缀".to_string(),
            ));
        }
        Ok(email)
    }

    // 模拟已存在的用户
    fn check_duplicate(name: &str) -> Result<(), RegistrationError> {
        let existing_users = vec!["admin", "root", "test"];
        if existing_users.contains(&name.to_lowercase().as_str()) {
            return Err(RegistrationError::DuplicateUser(name.to_string()));
        }
        Ok(())
    }

    // 注册函数 —— 使用 ? 串联所有验证
    fn register_user(
        name: &str,
        age_str: &str,
        email: &str,
    ) -> Result<User, RegistrationError> {
        let name = validate_name(name)?;     // ? 传播错误
        let age = validate_age(age_str)?;    // ? 传播错误
        let email = validate_email(email)?;  // ? 传播错误
        check_duplicate(&name)?;             // ? 传播错误

        Ok(User { name, age, email })
    }

    // 测试各种注册场景
    let test_cases = vec![
        ("张三", "25", "zhangsan@example.com"),
        ("", "25", "test@example.com"),
        ("李四", "abc", "lisi@example.com"),
        ("王五", "25", "invalid-email"),
        ("admin", "30", "admin@example.com"),
        ("赵六", "200", "zhao@example.com"),
    ];

    for (name, age, email) in test_cases {
        match register_user(name, age, email) {
            Ok(user) => println!("  注册成功: {:?}", user),
            Err(e) => println!("  注册失败 (name=\"{}\", age=\"{}\", email=\"{}\"): {}",
                name, age, email, e),
        }
    }

    println!();

    // ---------------------------------------------------------
    // 7. 错误链遍历
    // ---------------------------------------------------------
    println!("--- 7. 错误链遍历 ---");

    // 打印完整的错误链
    fn print_error_chain(err: &dyn Error) {
        println!("  错误: {}", err);
        let mut source = err.source();
        let mut depth = 1;
        while let Some(cause) = source {
            println!("  {}原因 {}: {}", "  ".repeat(depth), depth, cause);
            source = cause.source();
            depth += 1;
        }
    }

    // 构造一个有错误链的错误
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "磁盘只读");
    let reg_err = RegistrationError::StorageError(io_err);
    print_error_chain(&reg_err);

    println!();

    // ---------------------------------------------------------
    // 8. thiserror 和 anyhow（概念介绍）
    // ---------------------------------------------------------
    // 以下介绍两个非常流行的 Rust 错误处理库。
    // 我们只做概念说明，不引入依赖。
    println!("--- 8. thiserror 和 anyhow（概念介绍） ---");

    println!("【thiserror】—— 用于库开发");
    println!("  - 通过 derive 宏自动实现 Display 和 Error trait");
    println!("  - 减少手写 impl 的样板代码");
    println!("  - 用法示例（伪代码）：");
    println!("    // 在 Cargo.toml 中添加: thiserror = \"1\"");
    println!("    use thiserror::Error;");
    println!();
    println!("    #[derive(Error, Debug)]");
    println!("    enum MyError {{");
    println!("        #[error(\"IO 错误: {{0}}\")]");
    println!("        Io(#[from] std::io::Error),");
    println!();
    println!("        #[error(\"解析错误: {{0}}\")]");
    println!("        Parse(#[from] std::num::ParseIntError),");
    println!();
    println!("        #[error(\"自定义错误: {{msg}}\")]");
    println!("        Custom {{ msg: String }},");
    println!("    }}");
    println!();
    println!("  以上宏展开后等价于我们手写的 Display + Error + From 实现。");
    println!();

    println!("【anyhow】—— 用于应用程序开发");
    println!("  - 提供 anyhow::Error 作为通用错误类型（类似 Box<dyn Error>）");
    println!("  - 提供 anyhow::Result<T> 类型别名");
    println!("  - 支持 context() 方法添加上下文信息");
    println!("  - 用法示例（伪代码）：");
    println!("    // 在 Cargo.toml 中添加: anyhow = \"1\"");
    println!("    use anyhow::{{Context, Result}};");
    println!();
    println!("    fn read_config() -> Result<Config> {{");
    println!("        let content = std::fs::read_to_string(\"config.toml\")");
    println!("            .context(\"无法读取配置文件\")?;");
    println!("        let config: Config = toml::from_str(&content)");
    println!("            .context(\"配置文件格式错误\")?;");
    println!("        Ok(config)");
    println!("    }}");
    println!();

    println!("【选择建议】");
    println!("  - 写库 (library)  -> 使用 thiserror（让用户能精确匹配错误）");
    println!("  - 写应用 (binary) -> 使用 anyhow（简洁，关注错误上下文）");
    println!("  - 学习 / 小项目   -> 手写实现（理解原理）或 Box<dyn Error>");
    println!();

    // ---------------------------------------------------------
    // 9. 手动模拟 thiserror 的效果
    // ---------------------------------------------------------
    // 为了展示 thiserror 的效果，我们手动实现一个类似的例子
    println!("--- 9. 手动模拟 thiserror 的效果 ---");

    // 假设这是 thiserror 会为我们生成的代码
    #[derive(Debug)]
    #[allow(dead_code)]
    enum DatabaseError {
        ConnectionFailed(String),
        QueryFailed { query: String, reason: String },
        RecordNotFound(u64),
        IoError(io::Error),
    }

    impl fmt::Display for DatabaseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                // 等价于 #[error("连接失败: {0}")]
                DatabaseError::ConnectionFailed(addr) => {
                    write!(f, "连接失败: {}", addr)
                }
                // 等价于 #[error("查询 '{query}' 失败: {reason}")]
                DatabaseError::QueryFailed { query, reason } => {
                    write!(f, "查询 '{}' 失败: {}", query, reason)
                }
                // 等价于 #[error("记录 {0} 未找到")]
                DatabaseError::RecordNotFound(id) => {
                    write!(f, "记录 {} 未找到", id)
                }
                // 等价于 #[error("IO 错误: {0}")]
                DatabaseError::IoError(e) => {
                    write!(f, "IO 错误: {}", e)
                }
            }
        }
    }

    impl Error for DatabaseError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                // 等价于 #[source] 或 #[from]
                DatabaseError::IoError(e) => Some(e),
                _ => None,
            }
        }
    }

    // 等价于 #[from]
    impl From<io::Error> for DatabaseError {
        fn from(e: io::Error) -> Self {
            DatabaseError::IoError(e)
        }
    }

    // 使用自定义错误
    fn query_user_by_id(id: u64) -> Result<String, DatabaseError> {
        // 模拟数据库查询
        match id {
            1 => Ok("Alice".to_string()),
            2 => Ok("Bob".to_string()),
            _ => Err(DatabaseError::RecordNotFound(id)),
        }
    }

    fn get_user_report(id: u64) -> Result<String, DatabaseError> {
        let name = query_user_by_id(id)?;
        Ok(format!("用户报告: ID={}, 姓名={}", id, name))
    }

    for id in [1, 2, 99] {
        match get_user_report(id) {
            Ok(report) => println!("  {}", report),
            Err(e) => println!("  查询 ID={} 失败: {}", id, e),
        }
    }

    println!();

    // ---------------------------------------------------------
    // 10. 最佳实践总结
    // ---------------------------------------------------------
    println!("--- 10. 最佳实践总结 ---");
    println!("1. 为每个模块/库定义专属的错误 enum");
    println!("2. 总是实现 Display（用户友好）和 Debug（开发调试）");
    println!("3. 实现 Error trait 以融入 Rust 错误生态");
    println!("4. 为底层错误实现 From，让 ? 操作符自动转换");
    println!("5. 使用 source() 构建错误链，方便追踪根因");
    println!("6. 库代码使用具体错误类型（或 thiserror）");
    println!("7. 应用代码可以使用 anyhow 或 Box<dyn Error>");
    println!("8. 避免在库代码中使用 panic，总是返回 Result");

    println!("\n🎉 恭喜！你已经完成了 Lesson 023 —— 自定义错误！");
    println!("   至此，Chapter 05 错误处理的全部课程学习完毕！");
}
