// ============================================================
// Lesson 090: REST API 实战
// ============================================================
// 本课使用 Axum 框架构建一个完整的 RESTful API。
// 我们将实现一个「待办事项」管理系统，包含：
//   1. 完整的 CRUD 操作（增删改查）
//   2. 内存存储（Arc<RwLock<HashMap>>）
//   3. RESTful 路由设计
//   4. JSON 请求和响应
//   5. 错误处理和合适的 HTTP 状态码
//   6. UUID 作为资源唯一标识

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

// ============================================================
// 1. 数据模型
// ============================================================

/// 待办事项模型 —— 存储在数据库中的完整模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Todo {
    /// 唯一标识，使用 UUID v4
    id: String,
    /// 标题
    title: String,
    /// 详细描述
    description: Option<String>,
    /// 是否已完成
    completed: bool,
    /// 优先级: low, medium, high
    priority: Priority,
    /// 创建时间（简化表示）
    created_at: String,
}

/// 优先级枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Priority {
    Low,
    Medium,
    High,
}

/// 创建待办事项的请求体 —— 不包含 id 和 created_at（由服务端生成）
#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
    priority: Option<Priority>,
}

/// 更新待办事项的请求体 —— 所有字段可选
#[derive(Debug, Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
    priority: Option<Priority>,
}

/// 列表查询参数
#[derive(Debug, Deserialize)]
struct TodoQuery {
    /// 按完成状态过滤
    completed: Option<bool>,
    /// 按优先级过滤
    priority: Option<Priority>,
    /// 分页：页码
    page: Option<usize>,
    /// 分页：每页数量
    per_page: Option<usize>,
}

// ============================================================
// 2. 响应模型
// ============================================================

/// 统一的成功响应
#[derive(Serialize)]
struct SuccessResponse<T: Serialize> {
    success: bool,
    data: T,
}

/// 分页响应
#[derive(Serialize)]
struct PaginatedResponse<T: Serialize> {
    success: bool,
    data: Vec<T>,
    pagination: PaginationInfo,
}

/// 分页信息
#[derive(Serialize)]
struct PaginationInfo {
    total: usize,
    page: usize,
    per_page: usize,
    total_pages: usize,
}

/// 统一的错误响应
#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: ErrorDetail,
}

#[derive(Serialize)]
struct ErrorDetail {
    code: u16,
    message: String,
}

// ============================================================
// 3. 应用状态
// ============================================================

/// 应用共享状态 —— 使用内存存储
#[derive(Clone)]
struct AppState {
    /// 待办事项存储：HashMap<UUID字符串, Todo>
    todos: Arc<RwLock<HashMap<String, Todo>>>,
}

impl AppState {
    fn new() -> Self {
        let mut todos = HashMap::new();

        // 预设一些示例数据
        let todo1 = Todo {
            id: Uuid::new_v4().to_string(),
            title: "学习 Rust 基础".to_string(),
            description: Some("完成 Rust Book 前五章".to_string()),
            completed: true,
            priority: Priority::High,
            created_at: "2024-01-01T10:00:00Z".to_string(),
        };
        let todo2 = Todo {
            id: Uuid::new_v4().to_string(),
            title: "构建 REST API".to_string(),
            description: Some("使用 Axum 框架构建一个完整的 REST API".to_string()),
            completed: false,
            priority: Priority::Medium,
            created_at: "2024-01-02T14:30:00Z".to_string(),
        };
        let todo3 = Todo {
            id: Uuid::new_v4().to_string(),
            title: "编写单元测试".to_string(),
            description: None,
            completed: false,
            priority: Priority::Low,
            created_at: "2024-01-03T09:15:00Z".to_string(),
        };

        todos.insert(todo1.id.clone(), todo1);
        todos.insert(todo2.id.clone(), todo2);
        todos.insert(todo3.id.clone(), todo3);

        Self {
            todos: Arc::new(RwLock::new(todos)),
        }
    }
}

// ============================================================
// 4. 辅助函数
// ============================================================

/// 构建错误响应的辅助函数
fn error_response(status: StatusCode, message: impl Into<String>) -> impl IntoResponse {
    (
        status,
        Json(ErrorResponse {
            success: false,
            error: ErrorDetail {
                code: status.as_u16(),
                message: message.into(),
            },
        }),
    )
}

/// 获取当前时间的简化表示
fn now_string() -> String {
    // 实际项目中应使用 chrono 库
    "2024-01-15T12:00:00Z".to_string()
}

// ============================================================
// 5. Handler 函数 —— CRUD 操作
// ============================================================

/// GET /todos
/// 获取待办事项列表，支持过滤和分页
async fn list_todos(
    State(state): State<AppState>,
    Query(params): Query<TodoQuery>,
) -> impl IntoResponse {
    let todos = state.todos.read().unwrap();

    // 过滤
    let mut filtered: Vec<Todo> = todos
        .values()
        .filter(|todo| {
            // 按完成状态过滤
            if let Some(completed) = params.completed {
                if todo.completed != completed {
                    return false;
                }
            }
            // 按优先级过滤
            if let Some(ref priority) = params.priority {
                if &todo.priority != priority {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // 按创建时间排序
    filtered.sort_by(|a, b| a.created_at.cmp(&b.created_at));

    // 分页
    let total = filtered.len();
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).min(100); // 最多 100 条
    let total_pages = (total + per_page - 1) / per_page.max(1);

    let start = (page - 1) * per_page;
    let end = (start + per_page).min(total);

    let page_items = if start < total {
        filtered[start..end].to_vec()
    } else {
        vec![]
    };

    (
        StatusCode::OK,
        Json(PaginatedResponse {
            success: true,
            data: page_items,
            pagination: PaginationInfo {
                total,
                page,
                per_page,
                total_pages,
            },
        }),
    )
}

/// GET /todos/:id
/// 获取单个待办事项
async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let todos = state.todos.read().unwrap();

    match todos.get(&id) {
        Some(todo) => (
            StatusCode::OK,
            Json(SuccessResponse {
                success: true,
                data: todo.clone(),
            }),
        )
            .into_response(),
        None => error_response(
            StatusCode::NOT_FOUND,
            format!("待办事项 '{}' 不存在", id),
        )
            .into_response(),
    }
}

/// POST /todos
/// 创建新的待办事项
async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<CreateTodoRequest>,
) -> impl IntoResponse {
    // 验证输入
    if input.title.trim().is_empty() {
        return error_response(StatusCode::BAD_REQUEST, "标题不能为空").into_response();
    }

    if input.title.len() > 200 {
        return error_response(StatusCode::BAD_REQUEST, "标题长度不能超过 200 个字符")
            .into_response();
    }

    // 创建新的待办事项
    let todo = Todo {
        id: Uuid::new_v4().to_string(),
        title: input.title.trim().to_string(),
        description: input.description.map(|d| d.trim().to_string()),
        completed: false,
        priority: input.priority.unwrap_or(Priority::Medium),
        created_at: now_string(),
    };

    // 存储
    let mut todos = state.todos.write().unwrap();
    todos.insert(todo.id.clone(), todo.clone());

    // 返回 201 Created
    (
        StatusCode::CREATED,
        Json(SuccessResponse {
            success: true,
            data: todo,
        }),
    )
        .into_response()
}

/// PUT /todos/:id
/// 更新待办事项（部分更新）
async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<UpdateTodoRequest>,
) -> impl IntoResponse {
    let mut todos = state.todos.write().unwrap();

    match todos.get_mut(&id) {
        Some(todo) => {
            // 只更新提供了的字段（部分更新）
            if let Some(title) = input.title {
                if title.trim().is_empty() {
                    return error_response(StatusCode::BAD_REQUEST, "标题不能为空")
                        .into_response();
                }
                todo.title = title.trim().to_string();
            }

            if let Some(description) = input.description {
                todo.description = Some(description);
            }

            if let Some(completed) = input.completed {
                todo.completed = completed;
            }

            if let Some(priority) = input.priority {
                todo.priority = priority;
            }

            let updated_todo = todo.clone();

            (
                StatusCode::OK,
                Json(SuccessResponse {
                    success: true,
                    data: updated_todo,
                }),
            )
                .into_response()
        }
        None => error_response(
            StatusCode::NOT_FOUND,
            format!("待办事项 '{}' 不存在", id),
        )
            .into_response(),
    }
}

/// DELETE /todos/:id
/// 删除待办事项
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut todos = state.todos.write().unwrap();

    match todos.remove(&id) {
        Some(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "message": format!("待办事项 '{}' 已删除", id),
            })),
        )
            .into_response(),
        None => error_response(
            StatusCode::NOT_FOUND,
            format!("待办事项 '{}' 不存在", id),
        )
            .into_response(),
    }
}

/// GET /todos/stats
/// 获取待办事项的统计信息
async fn todo_stats(State(state): State<AppState>) -> impl IntoResponse {
    let todos = state.todos.read().unwrap();

    let total = todos.len();
    let completed = todos.values().filter(|t| t.completed).count();
    let pending = total - completed;
    let high_priority = todos
        .values()
        .filter(|t| t.priority == Priority::High && !t.completed)
        .count();

    Json(SuccessResponse {
        success: true,
        data: serde_json::json!({
            "total": total,
            "completed": completed,
            "pending": pending,
            "high_priority_pending": high_priority,
            "completion_rate": if total > 0 {
                format!("{:.1}%", completed as f64 / total as f64 * 100.0)
            } else {
                "0.0%".to_string()
            },
        }),
    })
}

/// DELETE /todos
/// 批量删除已完成的待办事项
async fn delete_completed(State(state): State<AppState>) -> impl IntoResponse {
    let mut todos = state.todos.write().unwrap();

    let before_count = todos.len();
    todos.retain(|_, todo| !todo.completed);
    let deleted_count = before_count - todos.len();

    Json(serde_json::json!({
        "success": true,
        "message": format!("已删除 {} 个已完成的待办事项", deleted_count),
        "deleted_count": deleted_count,
    }))
}

/// 健康检查
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "todo-api",
        "version": "1.0.0",
    }))
}

// ============================================================
// 6. 路由配置
// ============================================================

/// 创建 API 路由
fn api_routes() -> Router<AppState> {
    Router::new()
        // 待办事项 CRUD
        // GET /api/todos         -> 列表（支持过滤 & 分页）
        // POST /api/todos        -> 创建
        // DELETE /api/todos      -> 批量删除已完成
        .route("/todos", get(list_todos).post(create_todo).delete(delete_completed))
        // GET /api/todos/stats   -> 统计信息
        .route("/todos/stats", get(todo_stats))
        // GET /api/todos/:id     -> 获取单个
        // PUT /api/todos/:id     -> 更新
        // DELETE /api/todos/:id  -> 删除单个
        .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
}

/// 构建完整的应用
fn create_app() -> Router {
    let state = AppState::new();

    Router::new()
        .route("/health", get(health_check))
        // 所有 API 路由挂载在 /api 前缀下
        .nest("/api", api_routes())
        .with_state(state)
}

// ============================================================
// 7. 服务器启动
// ============================================================

#[tokio::main]
async fn main() {
    println!("=== Lesson 090: REST API 实战 ===\n");

    let app = create_app();

    println!("待办事项 REST API 服务器启动中...");
    println!("访问 http://127.0.0.1:3000 查看效果\n");
    println!("API 端点：");
    println!("  GET    /health                     -> 健康检查");
    println!("  GET    /api/todos                  -> 获取列表");
    println!("  GET    /api/todos?completed=false   -> 过滤未完成");
    println!("  GET    /api/todos?priority=high     -> 过滤高优先级");
    println!("  GET    /api/todos?page=1&per_page=5 -> 分页");
    println!("  GET    /api/todos/stats             -> 统计信息");
    println!("  GET    /api/todos/:id               -> 获取单个");
    println!("  POST   /api/todos                   -> 创建");
    println!("         Body: {{\"title\": \"...\", \"description\": \"...\", \"priority\": \"high\"}}");
    println!("  PUT    /api/todos/:id               -> 更新");
    println!("         Body: {{\"title\": \"...\", \"completed\": true}}");
    println!("  DELETE /api/todos/:id               -> 删除单个");
    println!("  DELETE /api/todos                   -> 删除所有已完成");
    println!("\n按 Ctrl+C 停止服务器");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

// ============================================================
// 知识点总结：
// ============================================================
//
// 1. RESTful API 设计原则：
//    - 资源用名词复数（/todos），不用动词
//    - HTTP 方法对应操作：GET=查 POST=增 PUT=改 DELETE=删
//    - 正确使用状态码：200=成功 201=创建 400=客户端错误 404=未找到
//    - 统一的响应格式（success + data/error）
//
// 2. 内存存储：
//    - Arc<RwLock<HashMap<K, V>>> 是常见的线程安全内存存储方案
//    - RwLock 允许多读单写，适合读多写少的场景
//    - 实际项目中应替换为数据库
//
// 3. 输入验证：
//    - 在 handler 中验证请求数据的合法性
//    - 返回 400 Bad Request 和描述性错误消息
//
// 4. 分页与过滤：
//    - 使用 Query 参数控制分页（page, per_page）
//    - 使用 Query 参数进行过滤（completed, priority）
//    - 返回分页元信息（total, total_pages）
//
// 5. UUID：
//    - 使用 uuid::Uuid::new_v4() 生成唯一标识
//    - 比自增 ID 更适合分布式系统
//    - 避免资源 ID 可预测性
