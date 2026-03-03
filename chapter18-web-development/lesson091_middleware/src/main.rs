// ============================================================
// Lesson 091: 中间件（Middleware）
// ============================================================
// 中间件是 Web 开发中的核心概念，它在请求到达 handler 之前
// 和响应返回客户端之前执行通用逻辑。
//
// 本课将学习：
//   1. Tower 中间件概念与架构
//   2. 日志中间件（tracing）
//   3. CORS 跨域配置
//   4. 自定义中间件（请求计时）
//   5. 中间件的执行顺序

use axum::{
    extract::{Json, Request, State},
    http::{HeaderMap, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

// ============================================================
// 1. 应用状态
// ============================================================

/// 应用共享状态，包含请求统计信息
#[derive(Clone)]
struct AppState {
    /// 总请求计数
    request_count: Arc<AtomicU64>,
}

impl AppState {
    fn new() -> Self {
        Self {
            request_count: Arc::new(AtomicU64::new(0)),
        }
    }
}

/// API 响应模型
#[derive(Serialize)]
struct ApiResponse<T: Serialize> {
    code: u16,
    message: String,
    data: T,
}

// ============================================================
// 2. 自定义中间件：请求计时
// ============================================================
// Axum 允许使用 `axum::middleware::from_fn` 从普通异步函数创建中间件。
// 中间件函数接收请求，调用 next.run(request) 传递给下一层，
// 然后可以对响应进行处理。

/// 请求计时中间件
/// 记录每个请求的处理耗时，并添加到响应头中
async fn timing_middleware(request: Request, next: Next) -> Response {
    // --- 请求阶段：在 handler 执行之前 ---
    let method = request.method().clone();
    let uri = request.uri().to_string();
    let start = Instant::now();

    tracing::info!("→ 收到请求: {} {}", method, uri);

    // 调用 next.run() 将请求传递给下一个中间件或 handler
    let response = next.run(request).await;

    // --- 响应阶段：在 handler 执行之后 ---
    let duration = start.elapsed();
    let status = response.status();

    tracing::info!(
        "← 响应完成: {} {} -> {} (耗时: {:?})",
        method,
        uri,
        status,
        duration,
    );

    // 可以修改响应，例如添加自定义头
    // 注意：这里我们需要重新构建响应来添加头
    let (mut parts, body) = response.into_parts();
    parts.headers.insert(
        "X-Response-Time",
        format!("{}ms", duration.as_millis()).parse().unwrap(),
    );

    Response::from_parts(parts, body)
}

// ============================================================
// 3. 自定义中间件：请求计数
// ============================================================

/// 请求计数中间件
/// 使用 State 来记录总请求数
async fn request_counter_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    // 原子操作递增请求计数
    let count = state.request_count.fetch_add(1, Ordering::SeqCst) + 1;
    tracing::info!("📊 这是第 {} 个请求", count);

    let mut response = next.run(request).await;

    // 在响应头中添加请求计数
    response.headers_mut().insert(
        "X-Request-Count",
        count.to_string().parse().unwrap(),
    );

    response
}

// ============================================================
// 4. 自定义中间件：简单的认证检查
// ============================================================

/// 认证中间件
/// 检查请求头中是否包含有效的 API Key
async fn auth_middleware(headers: HeaderMap, request: Request, next: Next) -> Response {
    // 检查 Authorization 头
    match headers.get("Authorization") {
        Some(value) => {
            let auth_str = value.to_str().unwrap_or("");
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                // 简化的 token 验证（实际项目应使用 JWT 等）
                if token == "secret-token-123" {
                    tracing::info!("🔓 认证成功");
                    next.run(request).await
                } else {
                    tracing::warn!("🔒 无效的 Token");
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(serde_json::json!({
                            "error": "无效的认证令牌",
                        })),
                    )
                        .into_response()
                }
            } else {
                tracing::warn!("🔒 认证格式错误");
                (
                    StatusCode::UNAUTHORIZED,
                    Json(serde_json::json!({
                        "error": "认证格式应为 Bearer <token>",
                    })),
                )
                    .into_response()
            }
        }
        None => {
            tracing::warn!("🔒 缺少认证头");
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "缺少 Authorization 头",
                })),
            )
                .into_response()
        }
    }
}

// ============================================================
// 5. Handler 函数
// ============================================================

/// 公开路由：首页
async fn index() -> impl IntoResponse {
    Json(serde_json::json!({
        "message": "欢迎使用中间件示例 API",
        "endpoints": {
            "public": ["/", "/health", "/stats"],
            "protected": ["/api/profile", "/api/data"],
        }
    }))
}

/// 公开路由：健康检查
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "middleware-demo",
    }))
}

/// 公开路由：查看请求统计
async fn stats(State(state): State<AppState>) -> impl IntoResponse {
    let count = state.request_count.load(Ordering::SeqCst);
    Json(ApiResponse {
        code: 200,
        message: "success".to_string(),
        data: serde_json::json!({
            "total_requests": count,
        }),
    })
}

/// 受保护路由：用户 profile（需要认证）
async fn profile() -> impl IntoResponse {
    Json(serde_json::json!({
        "user": "张三",
        "role": "admin",
        "message": "你已经通过了认证中间件！",
    }))
}

/// 受保护路由：获取数据（需要认证）
async fn protected_data() -> impl IntoResponse {
    Json(serde_json::json!({
        "data": ["机密数据1", "机密数据2", "机密数据3"],
        "message": "这些数据受认证保护",
    }))
}

// ============================================================
// 6. CORS 配置
// ============================================================

/// 创建 CORS 中间件层
/// CORS（跨域资源共享）控制哪些域名可以访问 API
fn create_cors_layer() -> CorsLayer {
    CorsLayer::new()
        // 允许的来源（Origin）
        // Any 允许所有来源，生产环境应指定具体域名
        .allow_origin(Any)
        // 允许的 HTTP 方法
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        // 允许的请求头
        .allow_headers(Any)
        // 允许携带凭据（cookies 等）
        // 注意：当 allow_origin 为 Any 时，不能设置 allow_credentials(true)
        // .allow_credentials(true)
        // 预检请求的缓存时间
        .max_age(std::time::Duration::from_secs(3600))
}

// ============================================================
// 7. 路由与中间件组装
// ============================================================

/// 创建受保护的 API 路由（需要认证）
fn protected_routes() -> Router<AppState> {
    Router::new()
        .route("/profile", get(profile))
        .route("/data", get(protected_data))
        // 这些路由会经过认证中间件
        .layer(middleware::from_fn(auth_middleware))
}

/// 创建完整的应用
fn create_app() -> Router {
    let state = AppState::new();

    // ============================================================
    // 中间件执行顺序说明：
    // ============================================================
    // 中间件按照添加的 **反序** 执行（洋葱模型）：
    //
    //   请求 → [最后添加的中间件] → [...] → [最先添加的中间件] → Handler
    //   响应 ← [最后添加的中间件] ← [...] ← [最先添加的中间件] ← Handler
    //
    // 在下面的配置中，执行顺序为：
    //   请求 → CORS → 计时 → 请求计数 → (认证，仅受保护路由) → Handler
    //

    Router::new()
        // --- 公开路由（不需要认证）---
        .route("/", get(index))
        .route("/health", get(health_check))
        .route("/stats", get(stats))
        // --- 受保护路由（需要认证）---
        .nest("/api", protected_routes())
        // --- 全局中间件（按添加顺序的反序执行）---
        // 请求计数中间件（需要 State，使用 from_fn_with_state）
        .layer(middleware::from_fn_with_state(
            state.clone(),
            request_counter_middleware,
        ))
        // 请求计时中间件
        .layer(middleware::from_fn(timing_middleware))
        // Tower HTTP 追踪中间件（日志记录）
        .layer(TraceLayer::new_for_http())
        // CORS 中间件
        .layer(create_cors_layer())
        // 注入应用状态
        .with_state(state)
}

// ============================================================
// 8. 服务器启动
// ============================================================

#[tokio::main]
async fn main() {
    // 初始化 tracing 日志订阅器
    // 这会将日志输出到标准输出
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    println!("=== Lesson 091: 中间件 ===\n");

    let app = create_app();

    println!("中间件示例服务器启动中...");
    println!("访问 http://127.0.0.1:3000 查看效果\n");
    println!("中间件执行流程（洋葱模型）：");
    println!("  请求 → CORS → TraceLayer → 计时 → 请求计数 → [认证] → Handler");
    println!("  响应 ← CORS ← TraceLayer ← 计时 ← 请求计数 ← [认证] ← Handler\n");
    println!("公开路由：");
    println!("  GET /           -> 首页");
    println!("  GET /health     -> 健康检查");
    println!("  GET /stats      -> 请求统计\n");
    println!("受保护路由（需要 Authorization: Bearer secret-token-123）：");
    println!("  GET /api/profile -> 用户信息");
    println!("  GET /api/data    -> 受保护数据\n");
    println!("按 Ctrl+C 停止服务器");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// ============================================================
// 知识点总结：
// ============================================================
//
// 1. Tower 中间件架构：
//    - Tower 是 Rust 生态中通用的中间件框架
//    - Service trait 是核心抽象：接收 Request，返回 Future<Response>
//    - Layer trait 用于包装 Service，添加中间件行为
//    - Axum 完全基于 Tower 构建，可以使用所有 Tower 中间件
//
// 2. 自定义中间件（from_fn）：
//    - axum::middleware::from_fn(async fn) 从异步函数创建中间件
//    - 函数签名：async fn(request: Request, next: Next) -> Response
//    - 调用 next.run(request) 传递给下一层
//    - 可以在调用前后执行逻辑（请求前处理 / 响应后处理）
//
// 3. 带状态的中间件：
//    - from_fn_with_state(state, async fn) 可以注入状态
//    - 函数签名增加 State(s): State<S> 参数
//
// 4. CORS 配置：
//    - CorsLayer 来自 tower-http crate
//    - .allow_origin()  控制允许的源
//    - .allow_methods() 控制允许的 HTTP 方法
//    - .allow_headers() 控制允许的请求头
//    - .max_age()       预检请求缓存时间
//
// 5. 日志中间件（Tracing）：
//    - tracing_subscriber 初始化日志收集器
//    - TraceLayer::new_for_http() 自动记录 HTTP 请求日志
//    - tracing::info! / warn! / error! 手动记录日志
//
// 6. 中间件执行顺序（洋葱模型）：
//    - .layer() 添加的中间件按 **反序** 处理请求
//    - 最后添加的 layer 最先接触请求
//    - 响应阶段则反过来
//    - 类似于 "洋葱" 一层层包裹
