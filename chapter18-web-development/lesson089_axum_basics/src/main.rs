// ============================================================
// Lesson 089: Axum 基础
// ============================================================
// Axum 是由 Tokio 团队开发的 Web 框架，具有以下特点：
//   - 基于 Tower 生态系统，与 Tower 中间件无缝集成
//   - 无宏路由，使用函数式 API 定义路由
//   - 类型安全的 extractor 系统
//   - 优秀的性能和人体工程学设计
//
// 本课将学习：
//   1. Router 路由定义
//   2. Handler 函数编写
//   3. Path / Query / Json extractor
//   4. Json 响应
//   5. 状态共享 State

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

// ============================================================
// 1. 数据模型定义
// ============================================================

/// 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<u32>,
    name: String,
    email: String,
}

/// 查询参数模型
#[derive(Debug, Deserialize)]
struct ListParams {
    page: Option<u32>,
    limit: Option<u32>,
    search: Option<String>,
}

/// 统一的 API 响应格式
#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    code: u32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    /// 成功响应
    fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    /// 带自定义消息的成功响应
    fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            code: 200,
            message: message.into(),
            data: Some(data),
        }
    }
}

/// 无数据的响应
#[derive(Serialize)]
struct EmptyResponse {
    code: u32,
    message: String,
}

impl EmptyResponse {
    fn new(code: u32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

// ============================================================
// 2. 应用状态
// ============================================================

/// 应用共享状态
/// 使用 Arc<RwLock<...>> 实现线程安全的状态共享
/// Axum 使用 State extractor 来注入共享状态
#[derive(Clone)]
struct AppState {
    /// 用户存储：模拟数据库
    users: Arc<RwLock<HashMap<u32, User>>>,
    /// 应用名称
    app_name: String,
    /// 自增 ID 计数器
    next_id: Arc<RwLock<u32>>,
}

impl AppState {
    fn new() -> Self {
        let mut users = HashMap::new();
        // 预设一些示例数据
        users.insert(1, User {
            id: Some(1),
            name: "张三".to_string(),
            email: "zhangsan@example.com".to_string(),
        });
        users.insert(2, User {
            id: Some(2),
            name: "李四".to_string(),
            email: "lisi@example.com".to_string(),
        });

        Self {
            users: Arc::new(RwLock::new(users)),
            app_name: "Axum 学习项目".to_string(),
            next_id: Arc::new(RwLock::new(3)),
        }
    }
}

// ============================================================
// 3. Handler 函数
// ============================================================
// Axum 的 handler 是普通的异步函数，参数通过 extractor 自动注入。
// 函数签名决定了 Axum 如何解析请求。

/// 最简单的 handler —— 返回纯文本
async fn root() -> &'static str {
    "欢迎使用 Axum Web 框架！"
}

/// 返回 JSON 响应
/// Axum 的 Json 包装器会自动设置 Content-Type: application/json
async fn app_info(State(state): State<AppState>) -> impl IntoResponse {
    // State extractor：从应用状态中提取数据
    Json(serde_json::json!({
        "app_name": state.app_name,
        "version": "1.0.0",
        "framework": "axum",
    }))
}

/// Path extractor：从 URL 路径中提取参数
/// 路由定义为 /users/:id
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let users = state.users.read().unwrap();

    match users.get(&id) {
        Some(user) => (
            StatusCode::OK,
            Json(ApiResponse::success(user.clone())),
        ).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(EmptyResponse::new(404, format!("用户 {} 不存在", id))),
        ).into_response(),
    }
}

/// Query extractor：从查询字符串提取参数
/// 例如 GET /users?page=1&limit=10&search=张
async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let users = state.users.read().unwrap();

    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);

    // 过滤和分页
    let mut user_list: Vec<&User> = users.values().collect();

    // 如果有搜索关键字，按名字过滤
    if let Some(ref search) = params.search {
        user_list.retain(|u| u.name.contains(search));
    }

    // 简单的分页逻辑
    let total = user_list.len();
    let start = ((page - 1) * limit) as usize;
    let end = (start + limit as usize).min(total);
    let page_items: Vec<&User> = if start < total {
        user_list[start..end].to_vec()
    } else {
        vec![]
    };

    Json(serde_json::json!({
        "code": 200,
        "data": {
            "items": page_items,
            "total": total,
            "page": page,
            "limit": limit,
        }
    }))
}

/// Json extractor：从请求体解析 JSON
/// POST /users，Body 为 JSON 格式的用户数据
async fn create_user(
    State(state): State<AppState>,
    Json(mut user): Json<User>,
) -> impl IntoResponse {
    // 分配新 ID
    let mut next_id = state.next_id.write().unwrap();
    let id = *next_id;
    *next_id += 1;
    user.id = Some(id);

    // 存储用户
    let mut users = state.users.write().unwrap();
    users.insert(id, user.clone());

    // 返回 201 Created
    (
        StatusCode::CREATED,
        Json(ApiResponse::success_with_message(user, "用户创建成功")),
    )
}

/// 组合多个 extractor：Path + Json
/// PUT /users/:id
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(mut input): Json<User>,
) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();

    if users.contains_key(&id) {
        input.id = Some(id);
        users.insert(id, input.clone());
        (
            StatusCode::OK,
            Json(ApiResponse::success_with_message(input, "用户更新成功")),
        ).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(EmptyResponse::new(404, format!("用户 {} 不存在", id))),
        ).into_response()
    }
}

/// DELETE /users/:id
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();

    if users.remove(&id).is_some() {
        (
            StatusCode::OK,
            Json(EmptyResponse::new(200, format!("用户 {} 已删除", id))),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(EmptyResponse::new(404, format!("用户 {} 不存在", id))),
        )
    }
}

/// 演示多个 Path 参数的提取
/// GET /greet/:name/:greeting
async fn greet(Path((name, greeting)): Path<(String, String)>) -> impl IntoResponse {
    Json(serde_json::json!({
        "message": format!("{}, {}!", greeting, name),
    }))
}

/// 健康检查
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": "2024-01-01T00:00:00Z",
    }))
}

// ============================================================
// 4. 路由定义与服务器启动
// ============================================================

/// 创建用户相关路由
/// 将路由拆分为独立函数，便于模块化管理
fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user).put(update_user).delete(delete_user))
}

/// 创建完整的应用路由
fn create_app(state: AppState) -> Router {
    Router::new()
        // 基础路由
        .route("/", get(root))
        .route("/info", get(app_info))
        .route("/health", get(health_check))
        // 嵌套路由：所有 /users 前缀的路由
        .nest("/users", user_routes())
        // 多参数路由
        .route("/greet/{name}/{greeting}", get(greet))
        // 注入共享状态
        .with_state(state)
}

#[tokio::main]
async fn main() {
    println!("=== Lesson 089: Axum 基础 ===\n");

    // 创建共享状态
    let state = AppState::new();

    // 构建应用
    let app = create_app(state);

    println!("服务器启动中...");
    println!("访问 http://127.0.0.1:3000 查看效果\n");
    println!("可用的路由：");
    println!("  GET    /                      -> 欢迎页面");
    println!("  GET    /info                   -> 应用信息（State 示例）");
    println!("  GET    /health                 -> 健康检查");
    println!("  GET    /users?page=1&limit=10  -> 用户列表（Query 参数）");
    println!("  GET    /users/:id              -> 获取用户（Path 参数）");
    println!("  POST   /users                  -> 创建用户（Json Body）");
    println!("  PUT    /users/:id              -> 更新用户（Path + Json）");
    println!("  DELETE /users/:id              -> 删除用户");
    println!("  GET    /greet/:name/:greeting  -> 问候（多 Path 参数）");
    println!("\n按 Ctrl+C 停止服务器");

    // 启动服务器
    // Axum 使用 tokio 的 TcpListener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// ============================================================
// 知识点总结：
// ============================================================
//
// 1. Router 路由定义：
//    - Router::new().route(path, method_handler)   基础路由
//    - .nest(prefix, router)                        嵌套路由（路由分组）
//    - get(handler) / post(handler) / put / delete  HTTP 方法
//    - get(h1).post(h2)                             同路径多方法
//
// 2. Handler 函数：
//    - 普通 async 函数，参数为 extractor
//    - 返回 impl IntoResponse
//    - (StatusCode, Json(...)) 元组可以同时设置状态码和响应体
//
// 3. Extractor 提取器：
//    - Path<T>：从 URL 路径参数提取 (/:id)
//    - Query<T>：从查询字符串提取 (?key=value)
//    - Json<T>：从请求体解析 JSON
//    - State<T>：从应用状态提取共享数据
//
// 4. State 状态共享：
//    - 定义一个 #[derive(Clone)] 的结构体
//    - 内部使用 Arc<RwLock<T>> 实现线程安全的可变共享
//    - 通过 .with_state(state) 注入到 Router
//    - Handler 中用 State(state): State<AppState> 提取
//
// 5. 响应构建：
//    - Json(data) 返回 JSON
//    - &str / String 返回纯文本
//    - (StatusCode, body) 元组设置状态码
//    - impl IntoResponse 灵活返回不同类型
