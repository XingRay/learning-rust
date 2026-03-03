// ============================================================
// Lesson 082: HTTP 客户端 (reqwest)
// ============================================================
// 本课学习使用 reqwest 库进行 HTTP 通信：
// - reqwest::blocking::get 同步 GET 请求
// - reqwest::blocking::Client 构建器模式
// - GET / POST 请求
// - JSON 响应解析（配合 serde）
// - 请求头设置
// - 错误处理
//
// 依赖：
//   reqwest = { version = "0.12", features = ["blocking", "json"] }
//   serde = { version = "1", features = ["derive"] }
//   serde_json = "1"
//
// 注意：本课示例需要网络连接。如果无法联网，
// 代码仍可编译，运行时会输出错误信息。
// ============================================================

use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, CONTENT_TYPE, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ================================
// 1. 最简单的 GET 请求
// ================================
fn simple_get() {
    println!("=== 简单 GET 请求 ===\n");

    // reqwest::blocking::get 是最简单的请求方式
    // 它返回 Result<Response>
    match reqwest::blocking::get("https://httpbin.org/get") {
        Ok(response) => {
            // 检查状态码
            println!("状态码: {}", response.status());
            println!("是否成功(2xx): {}", response.status().is_success());

            // 获取响应头
            println!("Content-Type: {:?}", response.headers().get("content-type"));

            // 获取响应体为文本
            match response.text() {
                Ok(body) => {
                    // 只打印前 300 个字符
                    let preview: String = body.chars().take(300).collect();
                    println!("响应体预览:\n{}", preview);
                }
                Err(e) => eprintln!("读取响应体失败: {}", e),
            }
        }
        Err(e) => {
            eprintln!("请求失败: {}", e);
            eprintln!("提示：请确保网络连接正常");
        }
    }
}

// ================================
// 2. 使用 Client 构建器
// ================================
fn client_builder_demo() {
    println!("=== Client 构建器 ===\n");

    // Client::builder() 提供丰富的配置选项
    let client = Client::builder()
        // 设置超时（连接 + 读取总时间）
        .timeout(Duration::from_secs(10))
        // 设置连接超时
        .connect_timeout(Duration::from_secs(5))
        // 设置 User-Agent
        .user_agent("rust-learning/1.0")
        // 是否跟随重定向（默认 true）
        .redirect(reqwest::redirect::Policy::limited(5))
        // 构建客户端
        .build()
        .expect("构建 Client 失败");

    println!("Client 已创建（超时: 10秒，最大重定向: 5次）");

    // 使用构建好的 client 发送请求
    match client.get("https://httpbin.org/get").send() {
        Ok(resp) => {
            println!("请求成功! 状态: {}", resp.status());
        }
        Err(e) => {
            eprintln!("请求失败: {}", e);
        }
    }
}

// ================================
// 3. GET 请求带查询参数
// ================================
fn get_with_query_params() {
    println!("=== GET 请求带查询参数 ===\n");

    let client = Client::new();

    // 方式 1: query 方法添加查询参数
    match client
        .get("https://httpbin.org/get")
        .query(&[("name", "Rust"), ("version", "1.75")])
        .send()
    {
        Ok(resp) => {
            println!("方式1 - URL 参数:");
            println!("  最终 URL: {}", resp.url());
            println!("  状态: {}", resp.status());
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }

    // 方式 2: 使用 HashMap 作为查询参数
    let mut params = HashMap::new();
    params.insert("search", "rust programming");
    params.insert("page", "1");

    match client
        .get("https://httpbin.org/get")
        .query(&params)
        .send()
    {
        Ok(resp) => {
            println!("\n方式2 - HashMap 参数:");
            println!("  最终 URL: {}", resp.url());
            println!("  状态: {}", resp.status());
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }
}

// ================================
// 4. POST 请求
// ================================
fn post_requests() {
    println!("=== POST 请求 ===\n");

    let client = Client::new();

    // 方式 1: 发送 JSON body
    // 使用 serde_json::json! 宏构建 JSON
    let json_body = serde_json::json!({
        "username": "rustacean",
        "email": "rust@example.com",
        "level": 42
    });

    match client
        .post("https://httpbin.org/post")
        .json(&json_body)
        .send()
    {
        Ok(resp) => {
            println!("JSON POST - 状态: {}", resp.status());
            if let Ok(body) = resp.text() {
                let preview: String = body.chars().take(300).collect();
                println!("响应预览:\n{}\n", preview);
            }
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }

    // 方式 2: 发送表单数据 (application/x-www-form-urlencoded)
    let mut form_data = HashMap::new();
    form_data.insert("username", "rustacean");
    form_data.insert("password", "secret123");

    match client
        .post("https://httpbin.org/post")
        .form(&form_data)
        .send()
    {
        Ok(resp) => {
            println!("表单 POST - 状态: {}", resp.status());
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }

    // 方式 3: 发送纯文本 body
    match client
        .post("https://httpbin.org/post")
        .body("raw text body content")
        .send()
    {
        Ok(resp) => {
            println!("文本 POST - 状态: {}", resp.status());
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }
}

// ================================
// 5. JSON 响应解析（使用 serde）
// ================================

// 定义与 JSON 响应对应的结构体
// httpbin.org/get 返回的 JSON 结构
#[derive(Debug, Deserialize)]
struct HttpBinResponse {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    origin: String,
    url: String,
}

// 用于 POST 演示的自定义类型
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// httpbin.org/post 返回的 JSON 结构（部分字段）
#[derive(Debug, Deserialize)]
struct HttpBinPostResponse {
    // json 字段包含我们发送的 JSON body
    json: Option<User>,
    url: String,
}

fn json_parsing_demo() {
    println!("=== JSON 解析 ===\n");

    let client = Client::new();

    // --- 解析 GET 响应 ---
    println!("--- 解析 GET 响应 ---");
    match client
        .get("https://httpbin.org/get")
        .query(&[("key", "value")])
        .send()
    {
        Ok(resp) => {
            // .json() 方法自动反序列化 JSON 到指定类型
            match resp.json::<HttpBinResponse>() {
                Ok(data) => {
                    println!("解析成功!");
                    println!("  来源 IP: {}", data.origin);
                    println!("  请求 URL: {}", data.url);
                    println!("  查询参数: {:?}", data.args);
                    println!("  请求头数量: {}", data.headers.len());
                }
                Err(e) => eprintln!("JSON 解析失败: {}", e),
            }
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }

    // --- 发送和接收自定义类型 ---
    println!("\n--- 发送和接收自定义类型 ---");
    let user = User {
        name: "张三".to_string(),
        age: 30,
        email: "zhangsan@example.com".to_string(),
    };

    println!("发送: {:?}", user);

    match client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
    {
        Ok(resp) => {
            match resp.json::<HttpBinPostResponse>() {
                Ok(data) => {
                    println!("解析成功!");
                    if let Some(returned_user) = data.json {
                        println!("  返回的用户: {:?}", returned_user);
                        println!("  姓名: {}", returned_user.name);
                        println!("  年龄: {}", returned_user.age);
                    }
                }
                Err(e) => eprintln!("JSON 解析失败: {}", e),
            }
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }

    // --- 使用 serde_json::Value 处理动态 JSON ---
    println!("\n--- 动态 JSON 解析 ---");
    match client.get("https://httpbin.org/get").send() {
        Ok(resp) => {
            // serde_json::Value 可以处理任意 JSON 结构
            match resp.json::<serde_json::Value>() {
                Ok(value) => {
                    println!("类型: {}", if value.is_object() { "对象" } else { "其他" });

                    // 使用索引操作符访问字段
                    if let Some(origin) = value.get("origin") {
                        println!("origin: {}", origin);
                    }

                    // 遍历 headers 对象
                    if let Some(headers) = value.get("headers").and_then(|h| h.as_object()) {
                        println!("部分请求头:");
                        for (key, val) in headers.iter().take(3) {
                            println!("  {}: {}", key, val);
                        }
                    }
                }
                Err(e) => eprintln!("解析失败: {}", e),
            }
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }
}

// ================================
// 6. 自定义请求头
// ================================
fn custom_headers_demo() {
    println!("=== 自定义请求头 ===\n");

    let client = Client::new();

    // 方式 1: 逐个添加请求头
    match client
        .get("https://httpbin.org/headers")
        .header(USER_AGENT, "MyRustApp/1.0")
        .header(ACCEPT, "application/json")
        .header("X-Custom-Header", "custom-value")
        .header("Authorization", "Bearer my-token-123")
        .send()
    {
        Ok(resp) => {
            println!("方式1 - 逐个添加:");
            println!("  状态: {}", resp.status());
            if let Ok(body) = resp.text() {
                let preview: String = body.chars().take(400).collect();
                println!("  响应:\n{}", preview);
            }
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }

    // 方式 2: 使用 HeaderMap 批量设置
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("RustLearning/1.0"),
    );
    // 自定义头部
    headers.insert(
        reqwest::header::HeaderName::from_static("x-request-id"),
        HeaderValue::from_static("req-12345"),
    );

    match client
        .get("https://httpbin.org/headers")
        .headers(headers)
        .send()
    {
        Ok(resp) => {
            println!("\n方式2 - HeaderMap:");
            println!("  状态: {}", resp.status());
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }
}

// ================================
// 7. 响应状态码处理
// ================================
fn status_code_handling() {
    println!("=== 状态码处理 ===\n");

    let client = Client::new();

    // 请求不同状态码的端点
    let urls = vec![
        ("200 OK", "https://httpbin.org/status/200"),
        ("404 Not Found", "https://httpbin.org/status/404"),
        ("500 Server Error", "https://httpbin.org/status/500"),
    ];

    for (desc, url) in urls {
        match client.get(url).send() {
            Ok(resp) => {
                let status = resp.status();
                println!(
                    "{}: 状态码 = {}, 成功 = {}, 客户端错误 = {}, 服务器错误 = {}",
                    desc,
                    status.as_u16(),
                    status.is_success(),
                    status.is_client_error(),
                    status.is_server_error()
                );
            }
            Err(e) => eprintln!("{}: 请求失败 - {}", desc, e),
        }
    }

    // 使用 error_for_status() 将 4xx/5xx 转换为错误
    println!("\n--- error_for_status() 演示 ---");
    match client.get("https://httpbin.org/status/404").send() {
        Ok(resp) => {
            // error_for_status() 会在 4xx/5xx 时返回 Err
            match resp.error_for_status() {
                Ok(_) => println!("请求成功"),
                Err(e) => {
                    println!("状态码错误: {}", e);
                    // 可以从错误中提取状态码
                    if let Some(status) = e.status() {
                        println!("错误状态码: {}", status);
                    }
                }
            }
        }
        Err(e) => eprintln!("请求失败: {}", e),
    }
}

// ================================
// 8. 完整的错误处理示例
// ================================
fn comprehensive_error_handling() {
    println!("=== 完整错误处理 ===\n");

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();

    // 封装一个健壮的请求函数
    fn fetch_data(client: &Client, url: &str) -> Result<String, String> {
        let response = client
            .get(url)
            .send()
            .map_err(|e| {
                if e.is_timeout() {
                    format!("请求超时: {}", e)
                } else if e.is_connect() {
                    format!("连接失败: {}", e)
                } else if e.is_redirect() {
                    format!("重定向错误: {}", e)
                } else {
                    format!("请求错误: {}", e)
                }
            })?;

        // 检查状态码
        let status = response.status();
        if !status.is_success() {
            return Err(format!("HTTP 错误: {} {}", status.as_u16(), status.canonical_reason().unwrap_or("Unknown")));
        }

        // 读取响应体
        response.text().map_err(|e| format!("读取响应体失败: {}", e))
    }

    // 测试正常请求
    match fetch_data(&client, "https://httpbin.org/get") {
        Ok(body) => {
            let preview: String = body.chars().take(100).collect();
            println!("成功: {}...", preview);
        }
        Err(e) => println!("失败: {}", e),
    }

    // 测试错误 URL
    match fetch_data(&client, "https://httpbin.org/status/500") {
        Ok(_) => println!("不应该到这里"),
        Err(e) => println!("预期的失败: {}", e),
    }

    // 测试不存在的域名
    match fetch_data(&client, "https://this-domain-does-not-exist-12345.com") {
        Ok(_) => println!("不应该到这里"),
        Err(e) => println!("预期的失败: {}", e),
    }
}

fn main() {
    println!("===================================================");
    println!("  Lesson 082: HTTP 客户端 (reqwest)");
    println!("===================================================");
    println!("注意：本课示例需要网络连接。");
    println!("如果无法联网，代码仍可编译，运行时会显示错误提示。\n");

    // 1. 简单 GET
    simple_get();
    println!("\n---------------------------------------------------\n");

    // 2. Client 构建器
    client_builder_demo();
    println!("\n---------------------------------------------------\n");

    // 3. 带查询参数的 GET
    get_with_query_params();
    println!("\n---------------------------------------------------\n");

    // 4. POST 请求
    post_requests();
    println!("\n---------------------------------------------------\n");

    // 5. JSON 解析
    json_parsing_demo();
    println!("\n---------------------------------------------------\n");

    // 6. 自定义请求头
    custom_headers_demo();
    println!("\n---------------------------------------------------\n");

    // 7. 状态码处理
    status_code_handling();
    println!("\n---------------------------------------------------\n");

    // 8. 完整错误处理
    comprehensive_error_handling();
}
