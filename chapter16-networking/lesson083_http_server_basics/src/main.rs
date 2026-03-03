// ============================================================
// Lesson 083: HTTP 服务基础
// ============================================================
// 本课学习用 Rust 标准库手写一个简单的 HTTP 服务器：
// - 解析 HTTP 请求（方法、路径、头部）
// - 构建 HTTP 响应（状态行、头部、body）
// - 简单的路由分发
// - 返回 HTML 和 JSON 响应
//
// 运行方式：取消 main 中 run_server() 的注释
// 然后在浏览器中访问:
//   http://127.0.0.1:8080/
//   http://127.0.0.1:8080/about
//   http://127.0.0.1:8080/api/users
//   http://127.0.0.1:8080/api/echo (POST)
// ============================================================

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// ================================
// 1. HTTP 请求解析
// ================================

/// HTTP 请求方法
#[derive(Debug, Clone, PartialEq)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Unknown(String),
}

impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => HttpMethod::Get,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            "HEAD" => HttpMethod::Head,
            "OPTIONS" => HttpMethod::Options,
            other => HttpMethod::Unknown(other.to_string()),
        }
    }
}

/// 解析后的 HTTP 请求
#[derive(Debug)]
struct HttpRequest {
    /// 请求方法（GET, POST 等）
    method: HttpMethod,
    /// 请求路径（如 /api/users）
    path: String,
    /// 查询字符串参数
    query_params: HashMap<String, String>,
    /// HTTP 版本（如 HTTP/1.1）
    version: String,
    /// 请求头
    headers: HashMap<String, String>,
    /// 请求体
    body: String,
}

impl HttpRequest {
    /// 从 TcpStream 解析 HTTP 请求
    ///
    /// HTTP 请求格式：
    /// ```text
    /// GET /path?key=value HTTP/1.1\r\n
    /// Host: example.com\r\n
    /// Content-Type: text/plain\r\n
    /// \r\n
    /// body content here
    /// ```
    fn parse(stream: &TcpStream) -> Result<Self, String> {
        let mut reader = BufReader::new(stream);

        // --- 解析请求行 ---
        // 请求行格式: METHOD PATH HTTP/VERSION
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|e| format!("读取请求行失败: {}", e))?;

        let request_line = request_line.trim().to_string();
        if request_line.is_empty() {
            return Err("空请求".to_string());
        }

        let parts: Vec<&str> = request_line.splitn(3, ' ').collect();
        if parts.len() != 3 {
            return Err(format!("无效的请求行: {}", request_line));
        }

        let method = HttpMethod::from(parts[0]);
        let version = parts[2].to_string();

        // 解析路径和查询参数
        // 例如: /api/users?page=1&limit=10
        let (path, query_params) = Self::parse_path(parts[1]);

        // --- 解析请求头 ---
        // 每行一个头部，格式: Key: Value
        // 空行表示头部结束
        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader
                .read_line(&mut line)
                .map_err(|e| format!("读取头部失败: {}", e))?;

            let line = line.trim().to_string();
            if line.is_empty() {
                break; // 空行，头部结束
            }

            // 按第一个冒号分割
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_lowercase();
                let value = line[pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // --- 解析请求体 ---
        // 根据 Content-Length 读取指定长度的 body
        let body = if let Some(content_length) = headers.get("content-length") {
            let length: usize = content_length
                .parse()
                .map_err(|_| "无效的 Content-Length".to_string())?;

            if length > 0 {
                let mut body_buf = vec![0u8; length];
                reader
                    .read_exact(&mut body_buf)
                    .map_err(|e| format!("读取请求体失败: {}", e))?;
                String::from_utf8_lossy(&body_buf).to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        Ok(HttpRequest {
            method,
            path,
            query_params,
            version,
            headers,
            body,
        })
    }

    /// 解析路径和查询参数
    fn parse_path(raw_path: &str) -> (String, HashMap<String, String>) {
        let mut params = HashMap::new();

        if let Some(pos) = raw_path.find('?') {
            let path = raw_path[..pos].to_string();
            let query = &raw_path[pos + 1..];

            // 解析 key=value&key2=value2
            for pair in query.split('&') {
                if let Some(eq_pos) = pair.find('=') {
                    let key = pair[..eq_pos].to_string();
                    let value = pair[eq_pos + 1..].to_string();
                    params.insert(key, value);
                }
            }

            (path, params)
        } else {
            (raw_path.to_string(), params)
        }
    }
}

// ================================
// 2. HTTP 响应构建
// ================================

/// HTTP 状态码
#[derive(Debug, Clone)]
enum StatusCode {
    Ok,                  // 200
    Created,             // 201
    BadRequest,          // 400
    NotFound,            // 404
    MethodNotAllowed,    // 405
    InternalServerError, // 500
}

impl StatusCode {
    /// 返回状态行字符串
    fn as_str(&self) -> &str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::Created => "201 Created",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::NotFound => "404 Not Found",
            StatusCode::MethodNotAllowed => "405 Method Not Allowed",
            StatusCode::InternalServerError => "500 Internal Server Error",
        }
    }
}

/// HTTP 响应构建器
struct HttpResponse {
    status: StatusCode,
    headers: Vec<(String, String)>,
    body: String,
}

impl HttpResponse {
    /// 创建一个新的响应
    fn new(status: StatusCode) -> Self {
        HttpResponse {
            status,
            headers: Vec::new(),
            body: String::new(),
        }
    }

    /// 添加响应头
    fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    /// 设置响应体
    fn body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    /// 返回 HTML 响应
    fn html(body: &str) -> Self {
        HttpResponse::new(StatusCode::Ok)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(body)
    }

    /// 返回 JSON 响应
    fn json(data: &str) -> Self {
        HttpResponse::new(StatusCode::Ok)
            .header("Content-Type", "application/json; charset=utf-8")
            .body(data)
    }

    /// 返回 404 页面
    fn not_found() -> Self {
        let body = r#"<!DOCTYPE html>
<html>
<head><title>404 Not Found</title></head>
<body>
<h1>404 - 页面未找到</h1>
<p>您请求的页面不存在。</p>
<p><a href="/">返回首页</a></p>
</body>
</html>"#;
        HttpResponse::new(StatusCode::NotFound)
            .header("Content-Type", "text/html; charset=utf-8")
            .body(body)
    }

    /// 构建完整的 HTTP 响应字节
    ///
    /// HTTP 响应格式：
    /// ```text
    /// HTTP/1.1 200 OK\r\n
    /// Content-Type: text/html\r\n
    /// Content-Length: 13\r\n
    /// \r\n
    /// Hello, World!
    /// ```
    fn build(&self) -> Vec<u8> {
        let mut response = String::new();

        // 状态行
        response.push_str(&format!("HTTP/1.1 {}\r\n", self.status.as_str()));

        // 自动添加 Content-Length
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));

        // 添加 Server 头
        response.push_str("Server: RustHttpServer/1.0\r\n");

        // 添加 Connection 头
        response.push_str("Connection: close\r\n");

        // 自定义头部
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        // 空行分隔头部和体
        response.push_str("\r\n");

        // 响应体
        response.push_str(&self.body);

        response.into_bytes()
    }
}

// ================================
// 3. 路由分发
// ================================

/// 路由处理函数类型
type RouteHandler = fn(&HttpRequest) -> HttpResponse;

/// 路由器
struct Router {
    routes: Vec<(HttpMethod, String, RouteHandler)>,
}

impl Router {
    fn new() -> Self {
        Router { routes: Vec::new() }
    }

    /// 注册 GET 路由
    fn get(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes
            .push((HttpMethod::Get, path.to_string(), handler));
        self
    }

    /// 注册 POST 路由
    fn post(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes
            .push((HttpMethod::Post, path.to_string(), handler));
        self
    }

    /// 匹配路由并返回处理函数
    fn find_handler(&self, method: &HttpMethod, path: &str) -> Option<&RouteHandler> {
        self.routes
            .iter()
            .find(|(m, p, _)| m == method && p == path)
            .map(|(_, _, handler)| handler)
    }
}

// ================================
// 4. 路由处理函数
// ================================

/// 首页
fn handle_index(_req: &HttpRequest) -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Rust HTTP 服务器</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        h1 { color: #b7410e; }
        a { color: #0066cc; margin-right: 15px; }
        code { background: #f4f4f4; padding: 2px 6px; border-radius: 3px; }
        .nav { margin: 20px 0; }
    </style>
</head>
<body>
    <h1>🦀 Rust HTTP 服务器</h1>
    <p>这是一个用 Rust 标准库手写的 HTTP 服务器！</p>
    <div class="nav">
        <h3>可用页面：</h3>
        <ul>
            <li><a href="/">首页</a> - 当前页面</li>
            <li><a href="/about">关于</a> - 关于本项目</li>
            <li><a href="/api/users">用户列表 API</a> - 返回 JSON</li>
            <li><a href="/api/echo">Echo API</a> - POST 请求回显</li>
        </ul>
    </div>
    <p>服务器时间由 Rust 驱动 ⚡</p>
</body>
</html>"#;
    HttpResponse::html(html)
}

/// 关于页面
fn handle_about(_req: &HttpRequest) -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>关于 - Rust HTTP 服务器</title>
    <style>
        body { font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }
        h1 { color: #b7410e; }
    </style>
</head>
<body>
    <h1>关于本项目</h1>
    <p>这是 Learning Rust 课程的 Lesson 083 示例。</p>
    <h2>技术栈</h2>
    <ul>
        <li>语言: Rust</li>
        <li>网络: std::net (标准库)</li>
        <li>并发: std::thread (多线程)</li>
        <li>无外部依赖</li>
    </ul>
    <h2>功能特点</h2>
    <ul>
        <li>手动解析 HTTP 请求</li>
        <li>构建 HTTP 响应</li>
        <li>简单的路由分发</li>
        <li>支持 HTML 和 JSON 响应</li>
        <li>多线程处理并发请求</li>
    </ul>
    <p><a href="/">← 返回首页</a></p>
</body>
</html>"#;
    HttpResponse::html(html)
}

/// 用户列表 API（返回 JSON）
fn handle_users(req: &HttpRequest) -> HttpResponse {
    // 支持查询参数 ?page=1&limit=10
    let page = req
        .query_params
        .get("page")
        .and_then(|p| p.parse::<u32>().ok())
        .unwrap_or(1);
    let limit = req
        .query_params
        .get("limit")
        .and_then(|l| l.parse::<u32>().ok())
        .unwrap_or(10);

    // 手工构建 JSON（不依赖 serde）
    let json = format!(
        r#"{{
  "page": {},
  "limit": {},
  "total": 3,
  "users": [
    {{"id": 1, "name": "张三", "email": "zhangsan@example.com"}},
    {{"id": 2, "name": "李四", "email": "lisi@example.com"}},
    {{"id": 3, "name": "王五", "email": "wangwu@example.com"}}
  ]
}}"#,
        page, limit
    );

    HttpResponse::json(&json)
}

/// Echo API（回显 POST 请求体）
fn handle_echo(req: &HttpRequest) -> HttpResponse {
    let content_type = req
        .headers
        .get("content-type")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());

    let json = format!(
        r#"{{
  "method": "{:?}",
  "path": "{}",
  "content_type": "{}",
  "body_length": {},
  "body": "{}",
  "headers_count": {}
}}"#,
        req.method,
        req.path,
        content_type,
        req.body.len(),
        req.body.replace('"', "\\\""), // 转义 JSON 中的引号
        req.headers.len()
    );

    HttpResponse::json(&json)
}

/// 处理健康检查
fn handle_health(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::json(r#"{"status": "ok", "service": "rust-http-server"}"#)
}

// ================================
// 5. 服务器核心逻辑
// ================================

/// 处理单个 HTTP 连接
fn handle_connection(mut stream: TcpStream, router: &Router) {
    let peer = stream.peer_addr().unwrap_or_else(|_| "unknown".parse().unwrap());

    // 设置读取超时，防止慢客户端阻塞
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .ok();

    // 解析请求
    let response = match HttpRequest::parse(&stream) {
        Ok(request) => {
            println!(
                "[{}] {:?} {} ({})",
                peer, request.method, request.path, request.version
            );

            // 路由匹配
            match router.find_handler(&request.method, &request.path) {
                Some(handler) => handler(&request),
                None => {
                    // 检查路径是否存在但方法不对
                    let path_exists = router
                        .routes
                        .iter()
                        .any(|(_, p, _)| p == &request.path);

                    if path_exists {
                        HttpResponse::new(StatusCode::MethodNotAllowed)
                            .header("Content-Type", "text/plain; charset=utf-8")
                            .header("Allow", "GET, POST")
                            .body("405 Method Not Allowed")
                    } else {
                        HttpResponse::not_found()
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("[{}] 解析请求失败: {}", peer, e);
            HttpResponse::new(StatusCode::BadRequest)
                .header("Content-Type", "text/plain; charset=utf-8")
                .body(&format!("400 Bad Request: {}", e))
        }
    };

    // 发送响应
    let response_bytes = response.build();
    if let Err(e) = stream.write_all(&response_bytes) {
        eprintln!("[{}] 发送响应失败: {}", peer, e);
    }
    let _ = stream.flush();
}

/// 启动 HTTP 服务器
///
/// 绑定到指定地址并开始处理请求。
/// 此函数会阻塞当前线程。
fn run_server(addr: &str) {
    // 创建路由表
    let router = Router::new()
        .get("/", handle_index)
        .get("/about", handle_about)
        .get("/api/users", handle_users)
        .get("/api/echo", handle_echo)
        .post("/api/echo", handle_echo)
        .get("/health", handle_health);

    // 绑定 TCP 监听器
    let listener = TcpListener::bind(addr).expect(&format!("无法绑定到 {}", addr));
    println!("🦀 Rust HTTP 服务器启动!");
    println!("📡 监听地址: http://{}", addr);
    println!("📌 可用路由:");
    println!("   GET  /          - 首页");
    println!("   GET  /about     - 关于");
    println!("   GET  /api/users - 用户列表 (JSON)");
    println!("   POST /api/echo  - 回显请求体 (JSON)");
    println!("   GET  /health    - 健康检查");
    println!("按 Ctrl+C 停止服务器\n");

    // 使用 leak 让 router 获得 'static 生命周期
    // 这在服务器场景中是安全的，因为 router 存活到程序结束
    let router: &'static Router = Box::leak(Box::new(router));

    // 接受并处理连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 为每个连接创建线程
                thread::spawn(move || {
                    handle_connection(stream, router);
                });
            }
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
            }
        }
    }
}

// ================================
// 6. 演示请求解析和响应构建
// ================================

/// 演示 HTTP 请求解析（不需要网络连接）
fn demo_request_parsing() {
    println!("=== HTTP 请求解析演示 ===\n");

    // 模拟一个 HTTP 请求字符串
    let raw_request = "GET /api/users?page=2&limit=5 HTTP/1.1\r\n\
                        Host: localhost:8080\r\n\
                        User-Agent: Mozilla/5.0\r\n\
                        Accept: application/json\r\n\
                        \r\n";

    println!("原始请求:\n{}", raw_request);

    // 用 TcpStream 模拟连接来解析
    // 这里我们直接演示解析逻辑
    let path = "/api/users?page=2&limit=5";
    let (parsed_path, params) = HttpRequest::parse_path(path);

    println!("解析结果:");
    println!("  路径: {}", parsed_path);
    println!("  查询参数:");
    for (key, value) in &params {
        println!("    {} = {}", key, value);
    }

    // 演示方法解析
    println!("\nHTTP 方法解析:");
    let methods = vec!["GET", "POST", "PUT", "DELETE", "HEAD", "PATCH"];
    for m in methods {
        let method = HttpMethod::from(m);
        println!("  {} -> {:?}", m, method);
    }
}

/// 演示 HTTP 响应构建
fn demo_response_building() {
    println!("=== HTTP 响应构建演示 ===\n");

    // 构建 HTML 响应
    let html_response = HttpResponse::html("<h1>Hello, World!</h1>");
    let bytes = html_response.build();
    println!("--- HTML 响应 ---");
    println!("{}", String::from_utf8_lossy(&bytes));

    // 构建 JSON 响应
    let json_response = HttpResponse::json(r#"{"message": "hello", "code": 200}"#);
    let bytes = json_response.build();
    println!("--- JSON 响应 ---");
    println!("{}", String::from_utf8_lossy(&bytes));

    // 构建自定义响应
    let custom_response = HttpResponse::new(StatusCode::Created)
        .header("Content-Type", "application/json")
        .header("Location", "/api/users/4")
        .header("X-Request-Id", "req-12345")
        .body(r#"{"id": 4, "name": "新用户", "created": true}"#);
    let bytes = custom_response.build();
    println!("--- 自定义响应 (201 Created) ---");
    println!("{}", String::from_utf8_lossy(&bytes));

    // 构建 404 响应
    let not_found = HttpResponse::not_found();
    let bytes = not_found.build();
    println!("--- 404 响应 ---");
    // 只打印头部
    let response_str = String::from_utf8_lossy(&bytes);
    let header_end = response_str.find("\r\n\r\n").unwrap_or(response_str.len());
    println!("{}", &response_str[..header_end]);
    println!("... (body 省略)");
}

/// 演示路由器
fn demo_router() {
    println!("=== 路由器演示 ===\n");

    let router = Router::new()
        .get("/", handle_index)
        .get("/about", handle_about)
        .get("/api/users", handle_users)
        .post("/api/echo", handle_echo)
        .get("/health", handle_health);

    // 测试路由匹配
    let test_cases = vec![
        (HttpMethod::Get, "/"),
        (HttpMethod::Get, "/about"),
        (HttpMethod::Get, "/api/users"),
        (HttpMethod::Post, "/api/echo"),
        (HttpMethod::Get, "/nonexistent"),
        (HttpMethod::Post, "/"), // 路径存在但方法不对
    ];

    for (method, path) in test_cases {
        let found = router.find_handler(&method, path).is_some();
        let status = if found { "✓ 匹配" } else { "✗ 未匹配" };
        println!("  {:?} {} -> {}", method, path, status);
    }
}

fn main() {
    println!("===================================================");
    println!("  Lesson 083: HTTP 服务基础");
    println!("===================================================\n");

    // --- 可直接运行的演示（不阻塞）---

    // 1. 演示请求解析
    demo_request_parsing();
    println!("\n---------------------------------------------------\n");

    // 2. 演示响应构建
    demo_response_building();
    println!("\n---------------------------------------------------\n");

    // 3. 演示路由器
    demo_router();

    // --- 启动服务器（阻塞式，取消注释可运行）---
    println!("\n===================================================");
    println!("要启动 HTTP 服务器，请取消下面一行的注释：");
    println!("  run_server(\"127.0.0.1:8080\");");
    println!("然后在浏览器中访问 http://127.0.0.1:8080/");
    println!("===================================================");

    // 取消注释以启动服务器：
    // run_server("127.0.0.1:8080");
}
