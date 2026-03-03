/// # Lesson 077 - JSON 序列化
///
/// 本课学习使用 serde 和 serde_json 进行 JSON 序列化与反序列化。
///
/// ## 学习目标
/// - 理解 serde 框架的基本概念
/// - 掌握 #[derive(Serialize, Deserialize)] 的用法
/// - 学会 serde_json::to_string / from_str 的基本转换
/// - 使用 to_string_pretty 进行美化输出
/// - 了解 Value 类型进行动态 JSON 解析
/// - 掌握嵌套结构体的序列化
/// - 学会使用 #[serde(rename/skip/default)] 等属性
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson077_serde_json
/// ```

// =============================================================
// Lesson 077: JSON 序列化（serde_json）
// =============================================================

use serde::{Deserialize, Serialize};
use serde_json::Value;

fn main() {
    println!("===== Lesson 077: JSON 序列化 =====\n");

    // ---------------------------------------------------------
    // 1. 基本序列化与反序列化
    // ---------------------------------------------------------
    println!("--- 1. 基本序列化与反序列化 ---");

    // 使用 derive 宏自动实现 Serialize 和 Deserialize
    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        name: String,
        age: u32,
        email: String,
    }

    let user = User {
        name: "张三".to_string(),
        age: 28,
        email: "zhangsan@example.com".to_string(),
    };

    // 序列化: Rust 结构体 -> JSON 字符串
    let json_str = serde_json::to_string(&user).unwrap();
    println!("  序列化 (to_string):");
    println!("    {}", json_str);

    // 反序列化: JSON 字符串 -> Rust 结构体
    let parsed: User = serde_json::from_str(&json_str).unwrap();
    println!("  反序列化 (from_str):");
    println!("    {:?}", parsed);
    println!();

    // ---------------------------------------------------------
    // 2. 美化输出 to_string_pretty
    // ---------------------------------------------------------
    println!("--- 2. 美化输出 ---");

    let pretty = serde_json::to_string_pretty(&user).unwrap();
    println!("  to_string_pretty:");
    for line in pretty.lines() {
        println!("    {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 3. 常见数据类型的序列化
    // ---------------------------------------------------------
    println!("--- 3. 常见数据类型 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct DataTypes {
        integer: i64,
        float: f64,
        boolean: bool,
        text: String,
        nothing: Option<String>,    // None -> null
        something: Option<String>,  // Some -> 值
        numbers: Vec<i32>,          // Vec -> JSON 数组
    }

    let data = DataTypes {
        integer: 42,
        float: 3.14,
        boolean: true,
        text: "hello".to_string(),
        nothing: None,
        something: Some("world".to_string()),
        numbers: vec![1, 2, 3, 4, 5],
    };

    let json = serde_json::to_string_pretty(&data).unwrap();
    println!("  各种类型的 JSON 表示:");
    for line in json.lines() {
        println!("    {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 4. 嵌套结构体序列化
    // ---------------------------------------------------------
    println!("--- 4. 嵌套结构体 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct Address {
        city: String,
        street: String,
        zip: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Company {
        name: String,
        address: Address,
        employees: Vec<Employee>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Employee {
        name: String,
        role: String,
    }

    let company = Company {
        name: "Rust 科技".to_string(),
        address: Address {
            city: "北京".to_string(),
            street: "中关村大街1号".to_string(),
            zip: "100080".to_string(),
        },
        employees: vec![
            Employee {
                name: "李四".to_string(),
                role: "工程师".to_string(),
            },
            Employee {
                name: "王五".to_string(),
                role: "产品经理".to_string(),
            },
        ],
    };

    let json = serde_json::to_string_pretty(&company).unwrap();
    println!("  嵌套结构体 JSON:");
    for line in json.lines() {
        println!("    {}", line);
    }

    // 反序列化嵌套 JSON
    let parsed_company: Company = serde_json::from_str(&json).unwrap();
    println!("  反序列化后:");
    println!("    公司: {}", parsed_company.name);
    println!("    城市: {}", parsed_company.address.city);
    println!("    员工数: {}", parsed_company.employees.len());
    println!();

    // ---------------------------------------------------------
    // 5. Value 动态解析
    // ---------------------------------------------------------
    println!("--- 5. Value 动态解析 ---");

    // 当不知道 JSON 结构或结构不固定时，使用 Value 类型
    let json_str = r#"{
        "name": "动态解析",
        "version": 2,
        "features": ["fast", "safe", "concurrent"],
        "metadata": {
            "author": "Rust Team",
            "year": 2024
        }
    }"#;

    let value: Value = serde_json::from_str(json_str).unwrap();

    // 使用索引访问（返回 &Value，不存在返回 Value::Null）
    println!("  动态访问 JSON 字段:");
    println!("    name:     {}", value["name"]);
    println!("    version:  {}", value["version"]);
    println!("    features: {}", value["features"]);
    println!("    author:   {}", value["metadata"]["author"]);
    println!("    不存在:    {}", value["nonexistent"]); // 输出 null

    // 类型转换方法
    if let Some(name) = value["name"].as_str() {
        println!("    name 字符串值: {}", name);
    }
    if let Some(ver) = value["version"].as_i64() {
        println!("    version 整数值: {}", ver);
    }
    if let Some(features) = value["features"].as_array() {
        println!("    features 数组长度: {}", features.len());
    }
    println!();

    // ---------------------------------------------------------
    // 6. 使用 json! 宏构造 Value
    // ---------------------------------------------------------
    println!("--- 6. json! 宏 ---");

    // json! 宏可以直接用类似 JSON 的语法构造 Value
    let value = serde_json::json!({
        "name": "serde_json",
        "version": 1,
        "tags": ["serialization", "json"],
        "active": true,
        "config": {
            "debug": false,
            "level": 3
        }
    });

    println!("  json! 宏构造:");
    let pretty = serde_json::to_string_pretty(&value).unwrap();
    for line in pretty.lines() {
        println!("    {}", line);
    }

    // json! 中也可以嵌入变量
    let name = "Rust";
    let version = 2021;
    let dynamic = serde_json::json!({
        "language": name,
        "edition": version,
        "features": ["ownership", "borrowing"]
    });
    println!("  嵌入变量: {}", dynamic);
    println!();

    // ---------------------------------------------------------
    // 7. #[serde(rename)] —— 字段重命名
    // ---------------------------------------------------------
    println!("--- 7. #[serde(rename)] 字段重命名 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct ApiResponse {
        // 将 Rust 的 snake_case 字段映射为 JSON 的 camelCase
        #[serde(rename = "statusCode")]
        status_code: u16,

        #[serde(rename = "errorMessage")]
        error_message: Option<String>,

        // 序列化和反序列化使用不同的名字
        #[serde(rename(serialize = "responseData", deserialize = "responseData"))]
        data: String,
    }

    let resp = ApiResponse {
        status_code: 200,
        error_message: None,
        data: "success".to_string(),
    };

    let json = serde_json::to_string_pretty(&resp).unwrap();
    println!("  rename 后的 JSON:");
    for line in json.lines() {
        println!("    {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 8. #[serde(rename_all)] —— 批量重命名策略
    // ---------------------------------------------------------
    println!("--- 8. #[serde(rename_all)] ---");

    // 在结构体级别统一重命名策略
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Config {
        max_connections: u32,
        connection_timeout: u64,
        enable_logging: bool,
        log_file_path: String,
    }

    let config = Config {
        max_connections: 100,
        connection_timeout: 30,
        enable_logging: true,
        log_file_path: "/var/log/app.log".to_string(),
    };

    let json = serde_json::to_string_pretty(&config).unwrap();
    println!("  rename_all = \"camelCase\":");
    for line in json.lines() {
        println!("    {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 9. #[serde(skip)] —— 跳过字段
    // ---------------------------------------------------------
    println!("--- 9. #[serde(skip)] 跳过字段 ---");

    #[derive(Debug, Serialize, Deserialize)]
    #[allow(dead_code)] // 部分字段被 serde(skip) 跳过，不会被直接读取
    struct UserProfile {
        username: String,

        // skip: 序列化和反序列化时都跳过此字段
        #[serde(skip)]
        password_hash: String,

        // skip_serializing: 只在序列化时跳过
        #[serde(skip_serializing)]
        internal_id: u64,

        // skip_serializing_if: 条件跳过（常用于 Option）
        #[serde(skip_serializing_if = "Option::is_none")]
        nickname: Option<String>,

        bio: String,
    }

    let profile = UserProfile {
        username: "rustacean".to_string(),
        password_hash: "secret_hash_123".to_string(),
        internal_id: 999,
        nickname: None, // 会被跳过，因为 is_none
        bio: "I love Rust!".to_string(),
    };

    let json = serde_json::to_string_pretty(&profile).unwrap();
    println!("  跳过敏感字段后的 JSON:");
    for line in json.lines() {
        println!("    {}", line);
    }
    println!("  注意: password_hash 和 nickname(None) 没有出现在 JSON 中");
    println!();

    // ---------------------------------------------------------
    // 10. #[serde(default)] —— 默认值
    // ---------------------------------------------------------
    println!("--- 10. #[serde(default)] 默认值 ---");

    #[derive(Debug, Serialize, Deserialize)]
    struct Settings {
        name: String,

        // 反序列化时如果缺少此字段，使用类型的 Default 值
        #[serde(default)]
        debug: bool, // 默认 false

        #[serde(default)]
        port: u16, // 默认 0

        // 使用自定义默认值函数
        #[serde(default = "default_host")]
        host: String,

        #[serde(default = "default_max_retries")]
        max_retries: u32,
    }

    fn default_host() -> String {
        "localhost".to_string()
    }

    fn default_max_retries() -> u32 {
        3
    }

    // 只提供部分字段的 JSON
    let partial_json = r#"{"name": "my_app"}"#;
    let settings: Settings = serde_json::from_str(partial_json).unwrap();

    println!("  输入 JSON: {}", partial_json);
    println!("  反序列化结果: {:?}", settings);
    println!("    name:        {}", settings.name);
    println!("    debug:       {} (默认值)", settings.debug);
    println!("    port:        {} (默认值)", settings.port);
    println!("    host:        {} (自定义默认值)", settings.host);
    println!("    max_retries: {} (自定义默认值)", settings.max_retries);
    println!();

    // ---------------------------------------------------------
    // 11. 枚举的序列化
    // ---------------------------------------------------------
    println!("--- 11. 枚举序列化 ---");

    // 枚举有多种序列化表示方式
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")] // 内部标签模式
    enum Shape {
        Circle { radius: f64 },
        Rectangle { width: f64, height: f64 },
        Triangle { base: f64, height: f64 },
    }

    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 20.0,
        },
        Shape::Triangle {
            base: 6.0,
            height: 8.0,
        },
    ];

    let json = serde_json::to_string_pretty(&shapes).unwrap();
    println!("  枚举数组 (tag = \"type\"):");
    for line in json.lines() {
        println!("    {}", line);
    }

    // 反序列化回来
    let parsed: Vec<Shape> = serde_json::from_str(&json).unwrap();
    for shape in &parsed {
        match shape {
            Shape::Circle { radius } => println!("  圆形: 半径={}", radius),
            Shape::Rectangle { width, height } => {
                println!("  矩形: {}x{}", width, height)
            }
            Shape::Triangle { base, height } => {
                println!("  三角形: 底={}, 高={}", base, height)
            }
        }
    }
    println!();

    // ---------------------------------------------------------
    // 12. HashMap 与 JSON 对象
    // ---------------------------------------------------------
    println!("--- 12. HashMap 与 JSON ---");

    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 95);
    scores.insert("Bob".to_string(), 87);
    scores.insert("Charlie".to_string(), 92);

    let json = serde_json::to_string_pretty(&scores).unwrap();
    println!("  HashMap -> JSON:");
    for line in json.lines() {
        println!("    {}", line);
    }

    // JSON -> HashMap
    let parsed: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
    println!("  JSON -> HashMap: {:?}", parsed);
    println!();

    // ---------------------------------------------------------
    // 13. 错误处理
    // ---------------------------------------------------------
    println!("--- 13. 反序列化错误处理 ---");

    // 类型不匹配
    let bad_json = r#"{"name": 123, "age": "not_a_number", "email": "test"}"#;
    let result: Result<User, _> = serde_json::from_str(bad_json);
    match result {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  类型不匹配错误: {}", e),
    }

    // 无效 JSON 语法
    let invalid = "{ invalid json }";
    let result: Result<Value, _> = serde_json::from_str(invalid);
    match result {
        Ok(_) => println!("  不应该成功"),
        Err(e) => println!("  无效 JSON 错误: {}", e),
    }

    println!("\n🎉 恭喜！你已完成 Lesson 077 —— JSON 序列化！");
}
