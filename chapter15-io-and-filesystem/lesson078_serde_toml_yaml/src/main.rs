/// # Lesson 078 - TOML 与 YAML
///
/// 本课学习使用 serde 和 toml crate 进行 TOML 格式的序列化与反序列化。
/// YAML 部分用注释说明（serde_yaml 用法类似），不引入额外依赖。
///
/// ## 学习目标
/// - 了解 TOML 格式及其应用场景（如 Cargo.toml）
/// - 掌握 toml::from_str 和 toml::to_string 的用法
/// - 学会读写 TOML 配置文件
/// - 理解 TOML 的嵌套表（nested table）
/// - 理解 TOML 的数组表（array of tables）
/// - 了解 YAML 格式的基本概念和 serde_yaml 的用法（注释说明）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson078_serde_toml_yaml
/// ```

// =============================================================
// Lesson 078: TOML 与 YAML（配置文件格式）
// =============================================================

use serde::{Deserialize, Serialize};

fn main() {
    println!("===== Lesson 078: TOML 与 YAML =====\n");

    // =========================================================
    //  第一部分：TOML 格式
    // =========================================================

    // ---------------------------------------------------------
    // 1. TOML 格式简介
    // ---------------------------------------------------------
    println!("--- 1. TOML 格式简介 ---");
    println!("  TOML = Tom's Obvious, Minimal Language");
    println!("  特点：");
    println!("    - 可读性强，语法简洁");
    println!("    - Rust 生态中广泛使用（Cargo.toml）");
    println!("    - 支持：字符串、整数、浮点、布尔、日期时间");
    println!("    - 支持：数组、表（类似对象）、内联表");
    println!();

    // ---------------------------------------------------------
    // 2. 基本序列化与反序列化
    // ---------------------------------------------------------
    println!("--- 2. TOML 基本序列化与反序列化 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct AppConfig {
        name: String,
        version: String,
        debug: bool,
        port: u16,
    }

    let config = AppConfig {
        name: "my_app".to_string(),
        version: "1.0.0".to_string(),
        debug: true,
        port: 8080,
    };

    // 序列化: Rust 结构体 -> TOML 字符串
    let toml_str = toml::to_string(&config).unwrap();
    println!("  序列化为 TOML:");
    for line in toml_str.lines() {
        println!("    {}", line);
    }
    println!();

    // 反序列化: TOML 字符串 -> Rust 结构体
    let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
    println!("  反序列化结果: {:?}", parsed);
    println!();

    // ---------------------------------------------------------
    // 3. 美化输出 to_string_pretty
    // ---------------------------------------------------------
    println!("--- 3. TOML 美化输出 ---");

    // toml::to_string_pretty 生成更易读的格式
    let pretty = toml::to_string_pretty(&config).unwrap();
    println!("  to_string_pretty:");
    for line in pretty.lines() {
        println!("    {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 4. 嵌套表（Nested Tables）
    // ---------------------------------------------------------
    println!("--- 4. 嵌套表 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct ServerConfig {
        title: String,
        server: Server,
        database: Database,
        logging: Logging,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Server {
        host: String,
        port: u16,
        workers: u32,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Database {
        url: String,
        max_connections: u32,
        timeout_secs: u64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Logging {
        level: String,
        file: String,
    }

    let server_config = ServerConfig {
        title: "我的服务器配置".to_string(),
        server: Server {
            host: "0.0.0.0".to_string(),
            port: 3000,
            workers: 4,
        },
        database: Database {
            url: "postgres://localhost/mydb".to_string(),
            max_connections: 20,
            timeout_secs: 30,
        },
        logging: Logging {
            level: "info".to_string(),
            file: "/var/log/app.log".to_string(),
        },
    };

    let toml_str = toml::to_string_pretty(&server_config).unwrap();
    println!("  嵌套表的 TOML 输出:");
    for line in toml_str.lines() {
        println!("    {}", line);
    }
    println!();

    // 从 TOML 字符串反序列化
    let parsed: ServerConfig = toml::from_str(&toml_str).unwrap();
    println!("  反序列化验证:");
    println!("    title:    {}", parsed.title);
    println!("    server:   {}:{}", parsed.server.host, parsed.server.port);
    println!("    database: {}", parsed.database.url);
    println!("    logging:  {} -> {}", parsed.logging.level, parsed.logging.file);
    println!();

    // ---------------------------------------------------------
    // 5. 从 TOML 文本解析（模拟配置文件）
    // ---------------------------------------------------------
    println!("--- 5. 解析 TOML 配置文件 ---");

    // 模拟一个 TOML 配置文件的内容
    let config_content = r#"
        # 这是一个 TOML 配置文件示例
        title = "项目配置"

        [server]
        host = "127.0.0.1"
        port = 8080
        workers = 8

        [database]
        url = "sqlite:///data/app.db"
        max_connections = 10
        timeout_secs = 60

        [logging]
        level = "debug"
        file = "app.log"
    "#;

    let parsed: ServerConfig = toml::from_str(config_content).unwrap();
    println!("  从 TOML 文本解析:");
    println!("    标题:   {}", parsed.title);
    println!("    服务器: {}:{} ({}个worker)", parsed.server.host, parsed.server.port, parsed.server.workers);
    println!("    数据库: {} (最大{}连接)", parsed.database.url, parsed.database.max_connections);
    println!();

    // ---------------------------------------------------------
    // 6. 数组表（Array of Tables）
    // ---------------------------------------------------------
    println!("--- 6. 数组表 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct Project {
        name: String,
        #[serde(default)]
        authors: Vec<String>,
        #[serde(default)]
        dependencies: Vec<Dependency>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Dependency {
        name: String,
        version: String,
        #[serde(default)]
        optional: bool,
    }

    // 从 TOML 解析数组表
    // [[dependencies]] 是 TOML 的数组表语法
    let project_toml = r#"
        name = "awesome-project"
        authors = ["Alice", "Bob", "Charlie"]

        [[dependencies]]
        name = "serde"
        version = "1.0"
        optional = false

        [[dependencies]]
        name = "toml"
        version = "0.8"
        optional = false

        [[dependencies]]
        name = "serde_yaml"
        version = "0.9"
        optional = true
    "#;

    let project: Project = toml::from_str(project_toml).unwrap();
    println!("  项目: {}", project.name);
    println!("  作者: {:?}", project.authors);
    println!("  依赖:");
    for dep in &project.dependencies {
        let opt = if dep.optional { " (可选)" } else { "" };
        println!("    {} v{}{}", dep.name, dep.version, opt);
    }
    println!();

    // 序列化回 TOML
    let back_to_toml = toml::to_string_pretty(&project).unwrap();
    println!("  序列化回 TOML:");
    for line in back_to_toml.lines() {
        println!("    {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 7. TOML 中的各种数据类型
    // ---------------------------------------------------------
    println!("--- 7. TOML 数据类型 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct TomlTypes {
        // 字符串
        basic_string: String,

        // 整数
        integer: i64,
        hex: i64,

        // 浮点
        float: f64,

        // 布尔
        flag: bool,

        // 数组
        ports: Vec<u16>,
        tags: Vec<String>,
    }

    let types_toml = r#"
        basic_string = "Hello, TOML!"
        integer = 42
        hex = 0xFF
        float = 3.14
        flag = true
        ports = [8080, 8081, 8082]
        tags = ["rust", "toml", "config"]
    "#;

    let types: TomlTypes = toml::from_str(types_toml).unwrap();
    println!("  字符串:  {}", types.basic_string);
    println!("  整数:    {}", types.integer);
    println!("  十六进制: {} (0xFF)", types.hex);
    println!("  浮点:    {}", types.float);
    println!("  布尔:    {}", types.flag);
    println!("  端口:    {:?}", types.ports);
    println!("  标签:    {:?}", types.tags);
    println!();

    // ---------------------------------------------------------
    // 8. 使用 HashMap 动态解析 TOML
    // ---------------------------------------------------------
    println!("--- 8. 动态解析 TOML ---");

    use std::collections::HashMap;

    let dynamic_toml = r#"
        name = "dynamic"
        version = "0.1.0"
        edition = "2021"
    "#;

    // 解析为 HashMap
    let map: HashMap<String, String> = toml::from_str(dynamic_toml).unwrap();
    println!("  HashMap 解析:");
    for (key, value) in &map {
        println!("    {} = {}", key, value);
    }
    println!();

    // 解析为 toml::Value（类似 serde_json::Value）
    let value: toml::Value = toml::from_str(dynamic_toml).unwrap();
    println!("  toml::Value 解析:");
    if let Some(name) = value.get("name") {
        println!("    name = {}", name);
    }
    if let Some(ver) = value.get("version") {
        println!("    version = {}", ver);
    }
    println!();

    // ---------------------------------------------------------
    // 9. Option 和 Default 在 TOML 中的应用
    // ---------------------------------------------------------
    println!("--- 9. Option 与 Default ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct OptionalConfig {
        name: String,

        #[serde(default = "default_port")]
        port: u16,

        #[serde(default)]
        debug: bool,

        description: Option<String>,
    }

    fn default_port() -> u16 {
        3000
    }

    // 只提供必要字段
    let minimal = r#"name = "minimal_app""#;
    let config: OptionalConfig = toml::from_str(minimal).unwrap();

    println!("  最小配置: {:?}", config);
    println!("    name:        {}", config.name);
    println!("    port:        {} (默认值)", config.port);
    println!("    debug:       {} (默认值)", config.debug);
    println!("    description: {:?} (可选, 未提供)", config.description);
    println!();

    // ---------------------------------------------------------
    // 10. 配置文件读写完整示例
    // ---------------------------------------------------------
    println!("--- 10. 配置文件读写示例 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct FullConfig {
        app: AppSection,
        features: Features,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AppSection {
        name: String,
        version: String,
        description: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Features {
        enable_cache: bool,
        cache_ttl: u64,
        allowed_origins: Vec<String>,
    }

    let full_config = FullConfig {
        app: AppSection {
            name: "web-service".to_string(),
            version: "2.1.0".to_string(),
            description: "一个 Web 服务".to_string(),
        },
        features: Features {
            enable_cache: true,
            cache_ttl: 3600,
            allowed_origins: vec![
                "https://example.com".to_string(),
                "https://api.example.com".to_string(),
            ],
        },
    };

    // 序列化为 TOML（通常会写入文件）
    let toml_content = toml::to_string_pretty(&full_config).unwrap();
    println!("  完整配置文件内容:");
    for line in toml_content.lines() {
        println!("    {}", line);
    }

    // 模拟写入和读取文件
    // 实际使用中：
    //   写入: fs::write("config.toml", &toml_content)?;
    //   读取: let content = fs::read_to_string("config.toml")?;
    //         let config: FullConfig = toml::from_str(&content)?;

    let read_back: FullConfig = toml::from_str(&toml_content).unwrap();
    println!("  读取验证:");
    println!("    app.name: {}", read_back.app.name);
    println!("    features.cache_ttl: {}", read_back.features.cache_ttl);
    println!("    origins: {:?}", read_back.features.allowed_origins);
    println!();

    // ---------------------------------------------------------
    // 11. TOML 错误处理
    // ---------------------------------------------------------
    println!("--- 11. TOML 错误处理 ---");

    // 无效的 TOML 语法
    let invalid_toml = r#"
        name = "test"
        [invalid
        broken = true
    "#;
    match toml::from_str::<toml::Value>(invalid_toml) {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  语法错误: {}", e),
    }

    // 类型不匹配
    let wrong_type = r#"name = 123"#; // name 期望字符串
    match toml::from_str::<AppConfig>(wrong_type) {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  类型不匹配: {}", e),
    }

    // 缺少必要字段
    let missing_field = r#"name = "test""#;
    match toml::from_str::<AppConfig>(missing_field) {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  缺少字段: {}", e),
    }
    println!();

    // =========================================================
    //  第二部分：YAML 格式（注释说明）
    // =========================================================

    // ---------------------------------------------------------
    // 12. YAML 格式简介与 serde_yaml 用法
    // ---------------------------------------------------------
    println!("--- 12. YAML 格式说明（注释） ---");
    println!("  YAML = YAML Ain't Markup Language");
    println!("  特点：");
    println!("    - 使用缩进表示层级关系（类似 Python）");
    println!("    - 广泛用于 Kubernetes、Docker Compose、CI/CD 配置");
    println!("    - Rust 中使用 serde_yaml crate");
    println!();

    // YAML 的基本语法示例（以注释形式展示）：
    //
    // ```yaml
    // # 这是 YAML 注释
    // name: my_app
    // version: "1.0.0"
    // debug: true
    // port: 8080
    //
    // # 嵌套结构（用缩进）
    // server:
    //   host: "0.0.0.0"
    //   port: 3000
    //   workers: 4
    //
    // # 数组
    // features:
    //   - fast
    //   - safe
    //   - concurrent
    //
    // # 数组对象
    // dependencies:
    //   - name: serde
    //     version: "1.0"
    //   - name: toml
    //     version: "0.8"
    // ```

    // serde_yaml 的用法与 serde_json/toml 几乎相同：
    //
    // ```rust
    // // Cargo.toml 中添加:
    // // [dependencies]
    // // serde = { version = "1", features = ["derive"] }
    // // serde_yaml = "0.9"
    //
    // use serde::{Serialize, Deserialize};
    //
    // #[derive(Debug, Serialize, Deserialize)]
    // struct Config {
    //     name: String,
    //     port: u16,
    // }
    //
    // // 序列化为 YAML
    // let config = Config { name: "app".to_string(), port: 8080 };
    // let yaml_str = serde_yaml::to_string(&config).unwrap();
    // println!("{}", yaml_str);
    // // 输出:
    // // name: app
    // // port: 8080
    //
    // // 反序列化
    // let parsed: Config = serde_yaml::from_str(&yaml_str).unwrap();
    //
    // // 从文件读取
    // let file = std::fs::File::open("config.yaml").unwrap();
    // let config: Config = serde_yaml::from_reader(file).unwrap();
    //
    // // 写入文件
    // let file = std::fs::File::create("config.yaml").unwrap();
    // serde_yaml::to_writer(file, &config).unwrap();
    //
    // // 动态解析（类似 serde_json::Value）
    // let value: serde_yaml::Value = serde_yaml::from_str(&yaml_str).unwrap();
    // ```

    println!("  serde_yaml 核心 API（与 serde_json/toml 一致）：");
    println!("    serde_yaml::to_string(&value)     -> String");
    println!("    serde_yaml::from_str::<T>(s)       -> Result<T>");
    println!("    serde_yaml::from_reader(reader)    -> Result<T>");
    println!("    serde_yaml::to_writer(writer, &v)  -> Result<()>");
    println!();

    // ---------------------------------------------------------
    // 13. 三种格式对比
    // ---------------------------------------------------------
    println!("--- 13. JSON / TOML / YAML 对比 ---");

    // 用同一个结构体展示三种格式的区别

    #[derive(Debug, Serialize, Deserialize)]
    struct CompareConfig {
        name: String,
        port: u16,
        tags: Vec<String>,
    }

    let compare = CompareConfig {
        name: "example".to_string(),
        port: 8080,
        tags: vec!["web".to_string(), "api".to_string()],
    };

    // JSON 格式
    println!("  ┌─ JSON 格式:");
    let json = serde_json::to_string_pretty(&compare).unwrap();
    for line in json.lines() {
        println!("  │   {}", line);
    }

    // TOML 格式
    println!("  ├─ TOML 格式:");
    let toml_out = toml::to_string_pretty(&compare).unwrap();
    for line in toml_out.lines() {
        println!("  │   {}", line);
    }

    // YAML 格式（手动展示）
    println!("  └─ YAML 格式（示意）:");
    println!("  │   name: example");
    println!("  │   port: 8080");
    println!("  │   tags:");
    println!("  │     - web");
    println!("  │     - api");
    println!();

    println!("  格式选择建议:");
    println!("    JSON  - API 交互、前后端通信、通用数据交换");
    println!("    TOML  - Rust 项目配置（Cargo.toml）、简单配置文件");
    println!("    YAML  - Kubernetes/Docker 配置、CI/CD 配置、复杂层级配置");

    println!("\n🎉 恭喜！你已完成 Lesson 078 —— TOML 与 YAML！");
}
