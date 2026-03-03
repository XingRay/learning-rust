// ============================================================
// Lesson 098: 建造者模式 (Builder Pattern)
// ============================================================
//
// 建造者模式是一种创建型设计模式，它允许你分步骤构建复杂对象。
// 当一个对象有很多可选参数时，Builder Pattern 比直接用构造函数更清晰。
//
// 在 Rust 中，Builder Pattern 非常常见，因为 Rust 没有函数重载和默认参数。
//
// 提示：在实际项目中，可以使用 `derive_builder` crate 自动生成 Builder 代码：
//   #[derive(Builder)]
//   struct Server { ... }
// 这会自动为每个字段生成 setter 方法和 build() 方法。

use std::fmt;

// ============================================================
// 1. 基本的 Builder Pattern
// ============================================================

/// 一个 HTTP 服务器配置结构体
/// 有些字段是必选的（host, port），有些是可选的
#[derive(Debug, Clone)]
struct ServerConfig {
    // 必选字段
    host: String,
    port: u16,
    // 可选字段
    max_connections: usize,
    timeout_seconds: u64,
    tls_enabled: bool,
    log_level: String,
    workers: usize,
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Server [{}:{} | max_conn={} | timeout={}s | tls={} | log={} | workers={}]",
            self.host,
            self.port,
            self.max_connections,
            self.timeout_seconds,
            self.tls_enabled,
            self.log_level,
            self.workers
        )
    }
}

/// ServerConfig 的建造者
/// 必选字段用 Option 包裹，可选字段给默认值
struct ServerConfigBuilder {
    // 必选字段 —— 用 Option 表示尚未设置
    host: Option<String>,
    port: Option<u16>,
    // 可选字段 —— 直接给默认值
    max_connections: usize,
    timeout_seconds: u64,
    tls_enabled: bool,
    log_level: String,
    workers: usize,
}

/// 自定义错误类型，用于 build() 失败时返回
#[derive(Debug)]
enum BuilderError {
    MissingField(&'static str),
    InvalidValue(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuilderError::MissingField(field) => write!(f, "缺少必选字段: {}", field),
            BuilderError::InvalidValue(msg) => write!(f, "无效值: {}", msg),
        }
    }
}

impl ServerConfigBuilder {
    /// 创建一个新的 Builder，所有可选字段使用默认值
    fn new() -> Self {
        ServerConfigBuilder {
            host: None,
            port: None,
            max_connections: 100,     // 默认最大连接数
            timeout_seconds: 30,      // 默认超时 30 秒
            tls_enabled: false,       // 默认不启用 TLS
            log_level: "info".to_string(), // 默认日志级别
            workers: 4,               // 默认 4 个工作线程
        }
    }

    /// 设置主机地址（必选）
    /// 返回 &mut Self 实现链式调用
    fn host(&mut self, host: &str) -> &mut Self {
        self.host = Some(host.to_string());
        self
    }

    /// 设置端口号（必选）
    fn port(&mut self, port: u16) -> &mut Self {
        self.port = Some(port);
        self
    }

    /// 设置最大连接数（可选）
    fn max_connections(&mut self, max: usize) -> &mut Self {
        self.max_connections = max;
        self
    }

    /// 设置超时时间（可选）
    fn timeout(&mut self, seconds: u64) -> &mut Self {
        self.timeout_seconds = seconds;
        self
    }

    /// 启用 TLS（可选）
    fn tls(&mut self, enabled: bool) -> &mut Self {
        self.tls_enabled = enabled;
        self
    }

    /// 设置日志级别（可选）
    fn log_level(&mut self, level: &str) -> &mut Self {
        self.log_level = level.to_string();
        self
    }

    /// 设置工作线程数（可选）
    fn workers(&mut self, count: usize) -> &mut Self {
        self.workers = count;
        self
    }

    /// 构建最终对象，验证必选字段是否已设置
    /// 返回 Result，如果缺少必选字段则返回错误
    fn build(&self) -> Result<ServerConfig, BuilderError> {
        // 检查必选字段
        let host = self
            .host
            .clone()
            .ok_or(BuilderError::MissingField("host"))?;
        let port = self
            .port
            .ok_or(BuilderError::MissingField("port"))?;

        // 额外的验证逻辑
        if host.is_empty() {
            return Err(BuilderError::InvalidValue(
                "host 不能为空字符串".to_string(),
            ));
        }
        if self.workers == 0 {
            return Err(BuilderError::InvalidValue(
                "workers 必须大于 0".to_string(),
            ));
        }

        Ok(ServerConfig {
            host,
            port,
            max_connections: self.max_connections,
            timeout_seconds: self.timeout_seconds,
            tls_enabled: self.tls_enabled,
            log_level: self.log_level.clone(),
            workers: self.workers,
        })
    }
}

// ============================================================
// 2. 消费型 Builder（使用所有权转移实现链式调用）
// ============================================================

/// 另一种风格：每个方法消费 self 并返回 Self
/// 这种方式可以在一行内完成所有配置
#[derive(Debug)]
struct Email {
    from: String,
    to: Vec<String>,
    subject: String,
    body: String,
    cc: Vec<String>,
    attachments: Vec<String>,
}

struct EmailBuilder {
    from: Option<String>,
    to: Vec<String>,
    subject: Option<String>,
    body: String,
    cc: Vec<String>,
    attachments: Vec<String>,
}

impl EmailBuilder {
    fn new() -> Self {
        EmailBuilder {
            from: None,
            to: Vec::new(),
            subject: None,
            body: String::new(),
            cc: Vec::new(),
            attachments: Vec::new(),
        }
    }

    /// 消费型链式调用 —— 参数是 self（不是 &mut self）
    fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    fn to(mut self, to: &str) -> Self {
        self.to.push(to.to_string());
        self
    }

    fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_string());
        self
    }

    fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    fn cc(mut self, cc: &str) -> Self {
        self.cc.push(cc.to_string());
        self
    }

    fn attachment(mut self, file: &str) -> Self {
        self.attachments.push(file.to_string());
        self
    }

    fn build(self) -> Result<Email, BuilderError> {
        let from = self
            .from
            .ok_or(BuilderError::MissingField("from"))?;
        if self.to.is_empty() {
            return Err(BuilderError::MissingField("to"));
        }
        let subject = self
            .subject
            .ok_or(BuilderError::MissingField("subject"))?;

        Ok(Email {
            from,
            to: self.to,
            subject,
            body: self.body,
            cc: self.cc,
            attachments: self.attachments,
        })
    }
}

// ============================================================
// 3. 带有 Option 字段的目标结构体
// ============================================================

/// 有时候目标结构体本身就有 Option 字段
#[derive(Debug)]
struct UserProfile {
    username: String,          // 必选
    email: String,             // 必选
    display_name: Option<String>,  // 可选
    bio: Option<String>,           // 可选
    avatar_url: Option<String>,    // 可选
    age: Option<u8>,               // 可选
}

struct UserProfileBuilder {
    username: Option<String>,
    email: Option<String>,
    display_name: Option<String>,
    bio: Option<String>,
    avatar_url: Option<String>,
    age: Option<u8>,
}

impl UserProfileBuilder {
    fn new() -> Self {
        UserProfileBuilder {
            username: None,
            email: None,
            display_name: None,
            bio: None,
            avatar_url: None,
            age: None,
        }
    }

    fn username(mut self, name: &str) -> Self {
        self.username = Some(name.to_string());
        self
    }

    fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    fn display_name(mut self, name: &str) -> Self {
        self.display_name = Some(name.to_string());
        self
    }

    fn bio(mut self, bio: &str) -> Self {
        self.bio = Some(bio.to_string());
        self
    }

    fn avatar_url(mut self, url: &str) -> Self {
        self.avatar_url = Some(url.to_string());
        self
    }

    fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }

    fn build(self) -> Result<UserProfile, BuilderError> {
        let username = self
            .username
            .ok_or(BuilderError::MissingField("username"))?;
        let email = self
            .email
            .ok_or(BuilderError::MissingField("email"))?;

        // 可选字段直接传递 Option 值
        Ok(UserProfile {
            username,
            email,
            display_name: self.display_name,
            bio: self.bio,
            avatar_url: self.avatar_url,
            age: self.age,
        })
    }
}

// ============================================================
// main 函数 —— 演示所有用法
// ============================================================

fn main() {
    println!("=== Lesson 098: 建造者模式 (Builder Pattern) ===\n");

    // ---------------------------------------------------------
    // 1. 使用 &mut Self 风格的 Builder
    // ---------------------------------------------------------
    println!("--- 1. ServerConfig Builder（&mut Self 风格）---");

    // 完整配置
    let config = ServerConfigBuilder::new()
        .host("127.0.0.1")
        .port(8080)
        .max_connections(500)
        .timeout(60)
        .tls(true)
        .log_level("debug")
        .workers(8)
        .build()
        .expect("构建 ServerConfig 失败");

    println!("完整配置: {}", config);

    // 只设置必选字段，其余用默认值
    let minimal_config = ServerConfigBuilder::new()
        .host("0.0.0.0")
        .port(3000)
        .build()
        .expect("构建 ServerConfig 失败");

    println!("最小配置: {}", minimal_config);

    // 缺少必选字段，build() 返回 Err
    let result = ServerConfigBuilder::new()
        .port(8080)
        .build();

    match result {
        Ok(_) => println!("不应该到达这里"),
        Err(e) => println!("预期的错误: {}", e),
    }

    // 无效值验证
    let result = ServerConfigBuilder::new()
        .host("localhost")
        .port(443)
        .workers(0) // 无效！
        .build();

    match result {
        Ok(_) => println!("不应该到达这里"),
        Err(e) => println!("验证错误: {}", e),
    }

    println!();

    // ---------------------------------------------------------
    // 2. 消费型 Builder（self 风格，一行完成）
    // ---------------------------------------------------------
    println!("--- 2. Email Builder（消费型，self 风格）---");

    let email = EmailBuilder::new()
        .from("alice@example.com")
        .to("bob@example.com")
        .to("charlie@example.com")   // 可以多次调用 to() 添加多个收件人
        .subject("会议通知")
        .body("明天下午三点开会，请准时参加。")
        .cc("dave@example.com")
        .attachment("meeting_agenda.pdf")
        .build()
        .expect("构建 Email 失败");

    println!("邮件: {:?}", email);

    // 缺少必选字段
    let result = EmailBuilder::new()
        .from("alice@example.com")
        .body("你好")
        .build(); // 缺少 to 和 subject

    match result {
        Ok(_) => println!("不应该到达这里"),
        Err(e) => println!("预期的错误: {}", e),
    }

    println!();

    // ---------------------------------------------------------
    // 3. 带 Option 字段的 Builder
    // ---------------------------------------------------------
    println!("--- 3. UserProfile Builder（Option 字段处理）---");

    // 完整的用户资料
    let full_profile = UserProfileBuilder::new()
        .username("rustacean")
        .email("rust@example.com")
        .display_name("Rust 爱好者")
        .bio("热爱 Rust 编程语言")
        .avatar_url("https://example.com/avatar.png")
        .age(25)
        .build()
        .expect("构建 UserProfile 失败");

    println!("完整资料: {:?}", full_profile);

    // 只有必选字段的用户资料
    let minimal_profile = UserProfileBuilder::new()
        .username("newbie")
        .email("new@example.com")
        .build()
        .expect("构建 UserProfile 失败");

    println!("最小资料: {:?}", minimal_profile);
    println!(
        "  display_name 是否有值: {}",
        minimal_profile.display_name.is_some()
    );
    println!("  bio 是否有值: {}", minimal_profile.bio.is_some());

    println!();

    // ---------------------------------------------------------
    // 4. 总结：两种 Builder 风格对比
    // ---------------------------------------------------------
    println!("--- 4. 两种 Builder 风格对比 ---");
    println!("┌──────────────────┬───────────────────────────────────┐");
    println!("│ 风格             │ 特点                              │");
    println!("├──────────────────┼───────────────────────────────────┤");
    println!("│ &mut Self 返回   │ Builder 可复用，可多次 build()     │");
    println!("│ self 消费        │ 一行完成，Builder 用后即弃         │");
    println!("└──────────────────┴───────────────────────────────────┘");

    // &mut Self 风格的复用示例
    let mut builder = ServerConfigBuilder::new();
    builder.host("localhost").port(8080);

    let config1 = builder.build().unwrap();
    // builder 还能继续使用！
    builder.port(9090);
    let config2 = builder.build().unwrap();

    println!("config1 端口: {}", config1.port);
    println!("config2 端口: {}", config2.port);

    println!("\n=== Builder Pattern 学习完成！===");

    // 注意：在实际项目中，推荐使用 derive_builder crate：
    //
    // ```toml
    // [dependencies]
    // derive_builder = "0.12"
    // ```
    //
    // ```rust
    // use derive_builder::Builder;
    //
    // #[derive(Builder)]
    // #[builder(setter(into))]
    // struct ServerConfig {
    //     host: String,
    //     port: u16,
    //     #[builder(default = "100")]
    //     max_connections: usize,
    //     #[builder(default)]
    //     tls_enabled: bool,
    // }
    // ```
    //
    // 这会自动生成 ServerConfigBuilder，大幅减少样板代码。
}
