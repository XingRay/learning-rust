// ============================================================
// Lesson 088: Actix-web 基础
// ============================================================
// 本课介绍 Rust 中最流行的 Web 框架之一 ── Actix-web。
// 我们将学习：
//   1. HttpServer 与 App 的创建和配置
//   2. 路由定义：web::get()、web::post() 等
//   3. Handler 函数的编写
//   4. Extractor：Path、Query、Json 提取请求数据
//   5. HttpResponse 的构建与返回

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

// ============================================================
// 1. 数据模型定义
// ============================================================

/// 用户信息 —— 用于 JSON 请求和响应
#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

/// 查询参数 —— 用于演示 Query extractor
#[derive(Debug, Deserialize)]
struct Pagination {
    page: Option<u32>,
    per_page: Option<u32>,
}

/// 通用 API 响应体
#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    success: bool,
    data: T,
}

// ============================================================
// 2. Handler 函数
// ============================================================

/// 最简单的 handler：返回纯文本
/// 任何返回 impl Responder 的异步函数都可以作为 handler
async fn hello() -> impl Responder {
    // HttpResponse::Ok() 生成 200 状态码
    HttpResponse::Ok().body("你好，欢迎来到 Actix-web 世界！")
}

/// 使用 HttpRequest 获取请求信息
async fn request_info(req: HttpRequest) -> impl Responder {
    let method = req.method().to_string();
    let path = req.path().to_string();
    let version = format!("{:?}", req.version());

    // 构建 JSON 响应
    HttpResponse::Ok().json(serde_json::json!({
        "method": method,
        "path": path,
        "version": version,
    }))
}

/// Path extractor：从 URL 路径中提取参数
/// 路由定义为 /users/{id}，这里的 id 会被自动解析
async fn get_user_by_id(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();

    // 模拟根据 ID 返回用户
    let user = User {
        name: format!("用户_{}", user_id),
        age: 20 + user_id,
    };

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: user,
    })
}

/// Path extractor：提取多个路径参数
/// 路由定义为 /users/{name}/{age}
async fn get_user_by_name_age(path: web::Path<(String, u32)>) -> impl Responder {
    let (name, age) = path.into_inner();

    let user = User { name, age };

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: user,
    })
}

/// Query extractor：从查询字符串中提取参数
/// 例如 GET /list?page=1&per_page=10
async fn list_items(query: web::Query<Pagination>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(10);

    HttpResponse::Ok().json(serde_json::json!({
        "message": "获取列表成功",
        "page": page,
        "per_page": per_page,
        "items": ["项目A", "项目B", "项目C"],
    }))
}

/// Json extractor：从请求体中解析 JSON 数据
/// 客户端发送 POST 请求，Body 为 JSON 格式
async fn create_user(user: web::Json<User>) -> impl Responder {
    println!("收到创建用户请求: {:?}", user);

    // user.into_inner() 取出内部的 User 值
    let new_user = user.into_inner();

    // 返回 201 Created 状态码
    HttpResponse::Created().json(ApiResponse {
        success: true,
        data: serde_json::json!({
            "message": "用户创建成功",
            "user": {
                "name": new_user.name,
                "age": new_user.age,
            }
        }),
    })
}

/// 组合多个 extractor
/// Path + Json 同时使用
async fn update_user(
    path: web::Path<u32>,
    user: web::Json<User>,
) -> impl Responder {
    let user_id = path.into_inner();
    let user_data = user.into_inner();

    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("用户 {} 已更新", user_id),
        "updated_data": {
            "name": user_data.name,
            "age": user_data.age,
        }
    }))
}

/// 演示不同的 HttpResponse 构建方式
async fn response_demo() -> HttpResponse {
    // HttpResponse 提供了多种便捷方法对应不同的 HTTP 状态码：
    // - HttpResponse::Ok()           -> 200
    // - HttpResponse::Created()      -> 201
    // - HttpResponse::BadRequest()   -> 400
    // - HttpResponse::NotFound()     -> 404
    // - HttpResponse::InternalServerError() -> 500

    HttpResponse::Ok()
        .insert_header(("X-Custom-Header", "Actix-Web-Demo"))  // 自定义响应头
        .insert_header(("X-Request-Id", "12345"))
        .content_type("application/json")                       // 设置 Content-Type
        .json(serde_json::json!({
            "message": "这是一个自定义响应头的示例",
            "framework": "actix-web",
        }))
}

/// 删除用户（演示 DELETE 方法）
async fn delete_user(path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();

    // 返回 204 No Content 也是常见的删除响应
    // 这里为了演示，返回 200 + 消息
    HttpResponse::Ok().json(serde_json::json!({
        "message": format!("用户 {} 已删除", user_id),
    }))
}

/// 健康检查端点
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": "1.0.0",
    }))
}

// ============================================================
// 3. 路由配置函数
// ============================================================

/// 将路由配置提取为独立函数，便于模块化管理
/// 在大型项目中，通常按功能模块拆分路由
fn configure_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // GET /users/{id} -> 获取单个用户
            .route("/{id}", web::get().to(get_user_by_id))
            // GET /users/{name}/{age} -> 通过名字和年龄查找
            .route("/{name}/{age}", web::get().to(get_user_by_name_age))
            // POST /users -> 创建用户
            .route("", web::post().to(create_user))
            // PUT /users/{id} -> 更新用户
            .route("/{id}", web::put().to(update_user))
            // DELETE /users/{id} -> 删除用户
            .route("/{id}", web::delete().to(delete_user)),
    );
}

// ============================================================
// 4. 启动服务器
// ============================================================

/// 使用 #[actix_web::main] 宏启动异步运行时
/// 这等价于使用 tokio::main 并手动创建 System
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== Lesson 088: Actix-web 基础 ===\n");

    println!("服务器启动中...");
    println!("访问 http://127.0.0.1:8080 查看效果\n");
    println!("可用的路由：");
    println!("  GET  /                  -> 欢迎页面");
    println!("  GET  /info              -> 请求信息");
    println!("  GET  /health            -> 健康检查");
    println!("  GET  /response-demo     -> 响应构建示例");
    println!("  GET  /list?page=1&per_page=10 -> 列表（Query 参数）");
    println!("  GET  /users/{{id}}        -> 获取用户（Path 参数）");
    println!("  GET  /users/{{name}}/{{age}} -> 获取用户（多 Path 参数）");
    println!("  POST /users             -> 创建用户（JSON Body）");
    println!("  PUT  /users/{{id}}        -> 更新用户（Path + JSON）");
    println!("  DELETE /users/{{id}}      -> 删除用户");
    println!("\n按 Ctrl+C 停止服务器");

    // HttpServer::new 接收一个闭包，该闭包返回 App 实例
    // 每个工作线程都会调用这个闭包创建独立的 App 实例
    HttpServer::new(|| {
        App::new()
            // 基础路由：直接在 App 上注册
            .route("/", web::get().to(hello))
            .route("/info", web::get().to(request_info))
            .route("/health", web::get().to(health_check))
            .route("/response-demo", web::get().to(response_demo))
            .route("/list", web::get().to(list_items))
            // 使用 configure 方法加载模块化路由
            .configure(configure_user_routes)
    })
    // 绑定地址和端口
    .bind("127.0.0.1:8080")?
    // workers 设置工作线程数量（默认等于 CPU 核心数）
    .workers(2)
    // 启动服务器
    .run()
    .await
}

// ============================================================
// 知识点总结：
// ============================================================
// 1. HttpServer::new(|| App::new()) 是 Actix-web 的标准启动方式
//    - 闭包会被每个工作线程调用，所以状态需要是 Send + Sync 的
//
// 2. 路由定义：
//    - .route(path, method.to(handler))    直接定义
//    - .configure(fn)                       模块化配置
//    - web::scope(prefix)                   路由分组，添加公共前缀
//
// 3. Handler 函数：
//    - 必须是 async 函数
//    - 返回 impl Responder 或具体类型如 HttpResponse
//    - 参数通过 Extractor 自动注入
//
// 4. Extractor（提取器）：
//    - web::Path<T>   从 URL 路径提取参数（如 /users/{id}）
//    - web::Query<T>  从查询字符串提取参数（如 ?page=1）
//    - web::Json<T>   从请求体提取 JSON 数据
//    - HttpRequest     获取原始请求对象
//
// 5. HttpResponse 构建：
//    - HttpResponse::Ok()        200
//    - HttpResponse::Created()   201
//    - .json(data)              JSON 响应
//    - .body(text)              纯文本响应
//    - .insert_header(...)      添加响应头
