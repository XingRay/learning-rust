// ============================================================
// Lesson 103: 实战项目 - Web API 服务器
// ============================================================
//
// 使用纯标准库构建一个完整的 HTTP API 服务器。
//
// 功能：
//   - 基于 std::net::TcpListener 的 HTTP 服务器
//   - 路由分发（GET / POST / PUT / DELETE）
//   - 手动构建 JSON 字符串（不依赖 serde）
//   - 内存数据存储（Arc<Mutex<Vec<Item>>>）
//   - CRUD 接口：增删改查
//   - 多线程处理请求
//
// API 路由：
//   GET    /api/items          获取所有项目
//   GET    /api/items/{id}     获取单个项目
//   POST   /api/items          创建项目
//   PUT    /api/items/{id}     更新项目
//   DELETE /api/items/{id}     删除项目
//   GET    /health             健康检查

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================
// 1. 数据模型
// ============================================================

/// 数据项
#[derive(Debug, Clone)]
struct Item {
    id: u64,
    name: String,
    description: String,
    completed: bool,
}

impl Item {
    /// 手动序列化为 JSON 字符串
    fn to_json(&self) -> String {
        format!(
            r#"{{"id":{},"name":"{}","description":"{}","completed":{}}}"#,
            self.id,
            escape_json(&self.name),
            escape_json(&self.description),
            self.completed,
        )
    }
}

/// 内存数据存储
struct Store {
    items: Vec<Item>,
    next_id: u64,
}

impl Store {
    fn new() -> Self {
        Store {
            items: Vec::new(),
            next_id: 1,
        }
    }

    /// 获取所有项目
    fn get_all(&self) -> Vec<Item> {
        self.items.clone()
    }

    /// 按 ID 获取
    fn get_by_id(&self, id: u64) -> Option<Item> {
        self.items.iter().find(|item| item.id == id).cloned()
    }

    /// 创建新项目
    fn create(&mut self, name: String, description: String) -> Item {
        let item = Item {
            id: self.next_id,
            name,
            description,
            completed: false,
        };
        self.next_id += 1;
        self.items.push(item.clone());
        item
    }

    /// 更新项目
    fn update(&mut self, id: u64, name: Option<String>, description: Option<String>, completed: Option<bool>) -> Option<Item> {
        if let Some(item) = self.items.iter_mut().find(|item| item.id == id) {
            if let Some(n) = name {
                item.name = n;
            }
            if let Some(d) = description {
                item.description = d;
            }
            if let Some(c) = completed {
                item.completed = c;
            }
            Some(item.clone())
        } else {
            None
        }
    }

    /// 删除项目
    fn delete(&mut self, id: u64) -> bool {
        let len_before = self.items.len();
        self.items.retain(|item| item.id != id);
        self.items.len() != len_before
    }
}

// ============================================================
// 2. JSON 辅助函数
// ============================================================

/// 转义 JSON 字符串中的特殊字符
fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// 将 Item 列表序列化为 JSON 数组
fn items_to_json(items: &[Item]) -> String {
    let json_items: Vec<String> = items.iter().map(|item| item.to_json()).collect();
    format!("[{}]", json_items.join(","))
}

/// 从 JSON 字符串中简单提取字段值（非常简化的解析器）
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\"", key);
    let pos = json.find(&search)?;
    let after_key = &json[pos + search.len()..];

    // 跳过 : 和空格
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let trimmed = after_colon.trim_start();

    if trimmed.starts_with('"') {
        // 字符串值
        let content = &trimmed[1..];
        let mut result = String::new();
        let mut chars = content.chars();
        while let Some(ch) = chars.next() {
            if ch == '\\' {
                if let Some(escaped) = chars.next() {
                    match escaped {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        _ => {
                            result.push('\\');
                            result.push(escaped);
                        }
                    }
                }
            } else if ch == '"' {
                break;
            } else {
                result.push(ch);
            }
        }
        Some(result)
    } else {
        None
    }
}

/// 从 JSON 字符串中提取布尔值
fn extract_json_bool(json: &str, key: &str) -> Option<bool> {
    let search = format!("\"{}\"", key);
    let pos = json.find(&search)?;
    let after_key = &json[pos + search.len()..];
    let after_colon = after_key.trim_start().strip_prefix(':')?;
    let trimmed = after_colon.trim_start();

    if trimmed.starts_with("true") {
        Some(true)
    } else if trimmed.starts_with("false") {
        Some(false)
    } else {
        None
    }
}

// ============================================================
// 3. HTTP 请求/响应
// ============================================================

/// 解析后的 HTTP 请求
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

/// HTTP 响应
struct HttpResponse {
    status_code: u16,
    status_text: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpResponse {
    fn new(status_code: u16, status_text: &str) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.to_string(),
            headers: vec![
                ("Content-Type".to_string(), "application/json; charset=utf-8".to_string()),
                ("Access-Control-Allow-Origin".to_string(), "*".to_string()),
            ],
            body: String::new(),
        }
    }

    fn json(status_code: u16, status_text: &str, body: &str) -> Self {
        let mut resp = HttpResponse::new(status_code, status_text);
        resp.body = body.to_string();
        resp
    }

    fn ok(body: &str) -> Self {
        Self::json(200, "OK", body)
    }

    fn created(body: &str) -> Self {
        Self::json(201, "Created", body)
    }

    fn not_found(msg: &str) -> Self {
        Self::json(404, "Not Found", &format!(r#"{{"error":"{}"}}"#, escape_json(msg)))
    }

    fn bad_request(msg: &str) -> Self {
        Self::json(400, "Bad Request", &format!(r#"{{"error":"{}"}}"#, escape_json(msg)))
    }

    fn method_not_allowed() -> Self {
        Self::json(405, "Method Not Allowed", r#"{"error":"Method Not Allowed"}"#)
    }

    fn no_content() -> Self {
        HttpResponse::new(204, "No Content")
    }

    /// 序列化为 HTTP 响应字符串
    fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);

        // 添加 Content-Length
        let body_bytes = self.body.as_bytes();
        response.push_str(&format!("Content-Length: {}\r\n", body_bytes.len()));

        // 添加其他 headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();
        bytes.extend_from_slice(body_bytes);
        bytes
    }
}

/// 解析 HTTP 请求
fn parse_request(reader: &mut BufReader<&mut std::net::TcpStream>) -> Option<HttpRequest> {
    // 读取请求行
    let mut request_line = String::new();
    if reader.read_line(&mut request_line).ok()? == 0 {
        return None;
    }

    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() < 2 {
        return None;
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();

    // 读取 headers
    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).ok()? == 0 {
            break;
        }
        let line = line.trim().to_string();
        if line.is_empty() {
            break; // headers 结束
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(
                key.trim().to_lowercase(),
                value.trim().to_string(),
            );
        }
    }

    // 读取 body（如果有 Content-Length）
    let mut body = String::new();
    if let Some(length_str) = headers.get("content-length") {
        if let Ok(length) = length_str.parse::<usize>() {
            if length > 0 {
                let mut buf = vec![0u8; length];
                use std::io::Read;
                if reader.read_exact(&mut buf).is_ok() {
                    body = String::from_utf8_lossy(&buf).to_string();
                }
            }
        }
    }

    Some(HttpRequest {
        method,
        path,
        headers,
        body,
    })
}

// ============================================================
// 4. 路由处理
// ============================================================

/// 从路径中提取 ID: /api/items/123 -> Some(123)
fn extract_id(path: &str, prefix: &str) -> Option<u64> {
    let remaining = path.strip_prefix(prefix)?;
    let id_str = remaining.trim_matches('/');
    id_str.parse().ok()
}

/// 路由分发，处理请求并返回响应
fn handle_request(req: &HttpRequest, store: &Arc<Mutex<Store>>) -> HttpResponse {
    let path = req.path.as_str();
    let method = req.method.as_str();

    // 打印请求日志
    println!("  [{} {}] body_len={}", method, path, req.body.len());

    // 路由匹配
    match (method, path) {
        // 健康检查
        ("GET", "/health") => {
            HttpResponse::ok(r#"{"status":"ok","message":"Server is running"}"#)
        }

        // 获取所有项目
        ("GET", "/api/items") => {
            let store = store.lock().unwrap();
            let items = store.get_all();
            let json = items_to_json(&items);
            HttpResponse::ok(&json)
        }

        // 创建新项目
        ("POST", "/api/items") => {
            let name = extract_json_string(&req.body, "name");
            let description = extract_json_string(&req.body, "description");

            match (name, description) {
                (Some(name), Some(desc)) => {
                    let mut store = store.lock().unwrap();
                    let item = store.create(name, desc);
                    HttpResponse::created(&item.to_json())
                }
                _ => HttpResponse::bad_request("需要 name 和 description 字段"),
            }
        }

        // 按 ID 匹配的路由
        _ if path.starts_with("/api/items/") => {
            match extract_id(path, "/api/items/") {
                Some(id) => match method {
                    // 获取单个项目
                    "GET" => {
                        let store = store.lock().unwrap();
                        match store.get_by_id(id) {
                            Some(item) => HttpResponse::ok(&item.to_json()),
                            None => HttpResponse::not_found(&format!("Item {} not found", id)),
                        }
                    }
                    // 更新项目
                    "PUT" => {
                        let name = extract_json_string(&req.body, "name");
                        let description = extract_json_string(&req.body, "description");
                        let completed = extract_json_bool(&req.body, "completed");

                        let mut store = store.lock().unwrap();
                        match store.update(id, name, description, completed) {
                            Some(item) => HttpResponse::ok(&item.to_json()),
                            None => HttpResponse::not_found(&format!("Item {} not found", id)),
                        }
                    }
                    // 删除项目
                    "DELETE" => {
                        let mut store = store.lock().unwrap();
                        if store.delete(id) {
                            HttpResponse::no_content()
                        } else {
                            HttpResponse::not_found(&format!("Item {} not found", id))
                        }
                    }
                    _ => HttpResponse::method_not_allowed(),
                },
                None => HttpResponse::bad_request("Invalid item ID"),
            }
        }

        // 未匹配的路由
        _ => HttpResponse::not_found("Route not found"),
    }
}

// ============================================================
// 5. 服务器启动
// ============================================================

/// 启动 HTTP 服务器
fn start_server(addr: &str) {
    let listener = TcpListener::bind(addr).unwrap_or_else(|e| {
        eprintln!("无法绑定地址 {}: {}", addr, e);
        std::process::exit(1);
    });

    println!("🚀 HTTP API 服务器已启动: http://{}", addr);
    println!();
    println!("可用的 API 端点:");
    println!("  GET    /health             - 健康检查");
    println!("  GET    /api/items          - 获取所有项目");
    println!("  GET    /api/items/{{id}}     - 获取单个项目");
    println!("  POST   /api/items          - 创建项目");
    println!("  PUT    /api/items/{{id}}     - 更新项目");
    println!("  DELETE /api/items/{{id}}     - 删除项目");
    println!();
    println!("测试命令:");
    println!("  curl http://{}/health", addr);
    println!("  curl http://{}/api/items", addr);
    println!("  curl -X POST -d '{{\"name\":\"Task 1\",\"description\":\"My first task\"}}' http://{}/api/items", addr);
    println!("  curl -X PUT -d '{{\"completed\":true}}' http://{}/api/items/1", addr);
    println!("  curl -X DELETE http://{}/api/items/1", addr);
    println!();
    println!("按 Ctrl+C 停止服务器");
    println!("{}", "=".repeat(60));

    // 共享的数据存储
    let store = Arc::new(Mutex::new(Store::new()));

    // 预置一些示例数据
    {
        let mut s = store.lock().unwrap();
        s.create("学习 Rust".to_string(), "完成 Rust 编程语言教程".to_string());
        s.create("写博客".to_string(), "分享 Rust 学习心得".to_string());
        s.create("做项目".to_string(), "用 Rust 实现一个 Web 服务器".to_string());
        println!("已预置 {} 条示例数据", s.get_all().len());
    }

    println!();

    // 接受连接
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let store = Arc::clone(&store);

                // 每个请求启动一个新线程处理
                thread::spawn(move || {
                    let peer = stream.peer_addr()
                        .map(|a| a.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());

                    // 解析请求
                    let response = {
                        let mut reader = BufReader::new(&mut stream);
                        match parse_request(&mut reader) {
                            Some(req) => {
                                println!("[{}] {} {}", peer, req.method, req.path);
                                handle_request(&req, &store)
                            }
                            None => {
                                HttpResponse::bad_request("Invalid HTTP request")
                            }
                        }
                    };

                    // 发送响应
                    let response_bytes = response.to_bytes();
                    if let Err(e) = stream.write_all(&response_bytes) {
                        eprintln!("[{}] 写响应失败: {}", peer, e);
                    }
                    let _ = stream.flush();
                });
            }
            Err(e) => {
                eprintln!("接受连接错误: {}", e);
            }
        }
    }
}

// ============================================================
// 6. 演示模式（不阻塞）
// ============================================================

/// 在演示模式下，模拟请求处理来展示功能
fn demo_mode() {
    println!("--- 演示模式（模拟请求处理）---\n");

    let store = Arc::new(Mutex::new(Store::new()));

    // 模拟 POST 创建
    println!(">> POST /api/items - 创建项目");
    let req = HttpRequest {
        method: "POST".to_string(),
        path: "/api/items".to_string(),
        headers: HashMap::new(),
        body: r#"{"name":"学习 Rust","description":"完成所有课程"}"#.to_string(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 再创建一个
    println!(">> POST /api/items - 创建第二个项目");
    let req = HttpRequest {
        method: "POST".to_string(),
        path: "/api/items".to_string(),
        headers: HashMap::new(),
        body: r#"{"name":"写测试","description":"为项目编写单元测试"}"#.to_string(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 获取所有
    println!(">> GET /api/items - 获取所有项目");
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/api/items".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 获取单个
    println!(">> GET /api/items/1 - 获取单个项目");
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/api/items/1".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 更新
    println!(">> PUT /api/items/1 - 更新项目");
    let req = HttpRequest {
        method: "PUT".to_string(),
        path: "/api/items/1".to_string(),
        headers: HashMap::new(),
        body: r#"{"name":"学习 Rust（进行中）","completed":true}"#.to_string(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 获取所有（查看更新后的结果）
    println!(">> GET /api/items - 更新后获取所有项目");
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/api/items".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 删除
    println!(">> DELETE /api/items/2 - 删除项目");
    let req = HttpRequest {
        method: "DELETE".to_string(),
        path: "/api/items/2".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: (no content)\n", resp.status_code);

    // 获取所有（查看删除后的结果）
    println!(">> GET /api/items - 删除后获取所有项目");
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/api/items".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 获取不存在的项目
    println!(">> GET /api/items/999 - 获取不存在的项目");
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/api/items/999".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 健康检查
    println!(">> GET /health - 健康检查");
    let req = HttpRequest {
        method: "GET".to_string(),
        path: "/health".to_string(),
        headers: HashMap::new(),
        body: String::new(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);

    // 错误请求
    println!(">> POST /api/items - 缺少必要字段");
    let req = HttpRequest {
        method: "POST".to_string(),
        path: "/api/items".to_string(),
        headers: HashMap::new(),
        body: r#"{"name":"只有名字"}"#.to_string(),
    };
    let resp = handle_request(&req, &store);
    println!("  响应 [{}]: {}\n", resp.status_code, resp.body);
}

// ============================================================
// main 函数
// ============================================================

fn main() {
    println!("=== Lesson 103: 实战项目 - Web API 服务器 ===\n");

    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 && args[1] == "serve" {
        // 启动真实的 HTTP 服务器
        let addr = if args.len() > 2 {
            args[2].as_str()
        } else {
            "127.0.0.1:8080"
        };
        start_server(addr);
    } else {
        // 演示模式
        println!("提示: 运行 `cargo run -- serve` 启动真实的 HTTP 服务器");
        println!("      运行 `cargo run -- serve 0.0.0.0:3000` 指定绑定地址");
        println!();

        demo_mode();

        println!("=== Web API 服务器学习完成！===");
        println!();
        println!("架构总结:");
        println!("┌──────────────────┬──────────────────────────────────────┐");
        println!("│ 组件             │ 说明                                 │");
        println!("├──────────────────┼──────────────────────────────────────┤");
        println!("│ TcpListener      │ 监听 TCP 连接                        │");
        println!("│ HTTP 解析        │ 手动解析请求行/头/体                  │");
        println!("│ 路由分发         │ 根据 method + path 匹配处理函数       │");
        println!("│ JSON 序列化      │ 手动拼接 JSON 字符串                  │");
        println!("│ 数据存储         │ Arc<Mutex<Store>> 线程安全共享        │");
        println!("│ 多线程           │ 每个请求 thread::spawn               │");
        println!("└──────────────────┴──────────────────────────────────────┘");
    }
}
