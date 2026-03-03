// ============================================================
// Lesson 092: WebSocket
// ============================================================
// WebSocket 是一种在单个 TCP 连接上提供全双工通信的协议。
// 与 HTTP 请求-响应模式不同，WebSocket 允许服务器主动向客户端推送消息。
//
// 本课将学习：
//   1. WebSocket 协议的基本概念
//   2. Axum 中的 WebSocket 支持
//   3. WebSocket 连接升级
//   4. 消息的收发处理
//   5. 构建简单的 Echo WebSocket 服务器

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

// ============================================================
// 1. WebSocket 概念讲解（通过代码注释）
// ============================================================
//
// WebSocket 协议要点：
//
// ┌─────────┐                    ┌─────────┐
// │  客户端  │ ── HTTP 升级请求 ──→ │  服务器  │
// │         │ ←── 101 切换协议 ── │         │
// │         │                    │         │
// │         │ ←──── 双向消息 ────→ │         │
// │         │ ←──── 双向消息 ────→ │         │
// │         │                    │         │
// │         │ ──── 关闭帧 ──────→ │         │
// │         │ ←─── 关闭帧 ────── │         │
// └─────────┘                    └─────────┘
//
// 1. 连接建立：客户端发送 HTTP GET 请求，带有 Upgrade: websocket 头
// 2. 服务器返回 101 Switching Protocols，连接升级为 WebSocket
// 3. 之后双方可以随时发送消息（全双工）
// 4. 消息类型：Text（文本）、Binary（二进制）、Ping/Pong（心跳）、Close（关闭）

// ============================================================
// 2. 应用状态
// ============================================================

/// 共享状态：追踪连接数
#[derive(Clone)]
struct AppState {
    /// 当前活跃的 WebSocket 连接数
    active_connections: Arc<AtomicU64>,
    /// 历史总连接数
    total_connections: Arc<AtomicU64>,
}

impl AppState {
    fn new() -> Self {
        Self {
            active_connections: Arc::new(AtomicU64::new(0)),
            total_connections: Arc::new(AtomicU64::new(0)),
        }
    }
}

// ============================================================
// 3. 辅助函数：发送文本消息
// ============================================================

/// 发送文本消息的辅助函数，封装类型转换
async fn send_text(socket: &mut WebSocket, text: &str) -> Result<(), axum::Error> {
    socket
        .send(Message::Text(text.to_string().into()))
        .await
}

// ============================================================
// 4. WebSocket Handler
// ============================================================

/// WebSocket 升级处理函数
///
/// `WebSocketUpgrade` 是 Axum 提供的 extractor，
/// 它会检查请求是否为合法的 WebSocket 升级请求，
/// 并提供 `.on_upgrade()` 方法来处理升级后的连接。
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    // 记录新连接
    let conn_id = state.total_connections.fetch_add(1, Ordering::SeqCst) + 1;
    println!("新的 WebSocket 连接请求 (ID: {})", conn_id);

    // on_upgrade 接收一个异步闭包，在连接升级成功后调用
    // 这里将实际的 WebSocket 处理逻辑委托给 handle_socket 函数
    ws.on_upgrade(move |socket| handle_socket(socket, state, conn_id))
}

/// 处理 WebSocket 连接
///
/// 这是 WebSocket 连接的核心处理逻辑。
/// `WebSocket` 类型实现了消息的发送和接收。
async fn handle_socket(mut socket: WebSocket, state: AppState, conn_id: u64) {
    // 增加活跃连接计数
    state.active_connections.fetch_add(1, Ordering::SeqCst);
    let active = state.active_connections.load(Ordering::SeqCst);
    println!(
        "WebSocket 连接已建立 (ID: {}, 当前活跃: {})",
        conn_id, active
    );

    // 发送欢迎消息
    let welcome_msg = format!(
        "欢迎连接到 WebSocket 服务器! 你的连接 ID 是 {}。\n\
         输入任意文本，服务器会回显。\n\
         输入 'ping' 测试心跳。\n\
         输入 'status' 查看服务器状态。\n\
         输入 'quit' 断开连接。",
        conn_id
    );

    if send_text(&mut socket, &welcome_msg).await.is_err() {
        println!("发送欢迎消息失败 (ID: {})", conn_id);
        cleanup_connection(&state, conn_id);
        return;
    }

    // ============================================================
    // 消息循环：持续接收和处理客户端消息
    // ============================================================
    // socket.recv() 返回 Option<Result<Message, Error>>
    //   - None        表示连接已关闭
    //   - Some(Ok(m)) 表示收到消息 m
    //   - Some(Err(e)) 表示发生错误
    while let Some(msg_result) = socket.recv().await {
        match msg_result {
            Ok(msg) => {
                // 处理不同类型的消息
                match msg {
                    // ---- 文本消息 ----
                    Message::Text(text) => {
                        let text_str: &str = &text;
                        println!("[ID:{}] 收到文本: {}", conn_id, text_str);

                        // 根据内容执行不同操作
                        let response = match text_str.trim().to_lowercase().as_str() {
                            "ping" => {
                                // 响应 ping 命令
                                "Pong!".to_string()
                            }
                            "status" => {
                                // 返回服务器状态
                                let active =
                                    state.active_connections.load(Ordering::SeqCst);
                                let total =
                                    state.total_connections.load(Ordering::SeqCst);
                                format!(
                                    "服务器状态:\n  活跃连接: {}\n  历史总连接: {}",
                                    active, total
                                )
                            }
                            "quit" => {
                                // 客户端请求断开
                                println!("[ID:{}] 客户端请求断开", conn_id);
                                let _ = send_text(&mut socket, "再见! 连接即将关闭。").await;
                                // 发送 Close 帧
                                let _ = socket.send(Message::Close(None)).await;
                                break;
                            }
                            "help" => {
                                "可用命令:\n  \
                                 ping   - 测试心跳\n  \
                                 status - 查看服务器状态\n  \
                                 help   - 显示帮助\n  \
                                 quit   - 断开连接\n  \
                                 其他   - 回显消息"
                                    .to_string()
                            }
                            _ => {
                                // Echo：回显收到的消息
                                format!("Echo: {}", text_str)
                            }
                        };

                        // 发送响应
                        if send_text(&mut socket, &response).await.is_err() {
                            println!("[ID:{}] 发送消息失败", conn_id);
                            break;
                        }
                    }

                    // ---- 二进制消息 ----
                    Message::Binary(data) => {
                        println!(
                            "[ID:{}] 收到二进制数据: {} 字节",
                            conn_id,
                            data.len()
                        );
                        // Echo 回二进制数据
                        if socket.send(Message::Binary(data.to_vec().into())).await.is_err() {
                            break;
                        }
                    }

                    // ---- Ping 消息 ----
                    // 客户端发送的 Ping，Axum 会自动回复 Pong
                    // 但我们也可以手动处理
                    Message::Ping(data) => {
                        println!("[ID:{}] 收到 Ping", conn_id);
                        // 手动回复 Pong（通常 Axum 会自动处理）
                        if socket.send(Message::Pong(data.to_vec().into())).await.is_err() {
                            break;
                        }
                    }

                    // ---- Pong 消息 ----
                    // 对方回复的 Pong，通常只需记录
                    Message::Pong(_) => {
                        println!("[ID:{}] 收到 Pong", conn_id);
                    }

                    // ---- Close 消息 ----
                    // 对方请求关闭连接
                    Message::Close(close_frame) => {
                        if let Some(ref cf) = close_frame {
                            println!(
                                "[ID:{}] 收到关闭请求: code={}, reason={}",
                                conn_id, cf.code, cf.reason
                            );
                        } else {
                            println!("[ID:{}] 收到关闭请求", conn_id);
                        }
                        // 回复 Close 帧（完成关闭握手）
                        let _ = socket.send(Message::Close(close_frame)).await;
                        break;
                    }
                }
            }
            Err(e) => {
                // 连接错误
                println!("[ID:{}] WebSocket 错误: {}", conn_id, e);
                break;
            }
        }
    }

    // 清理连接
    cleanup_connection(&state, conn_id);
}

/// 清理断开的连接
fn cleanup_connection(state: &AppState, conn_id: u64) {
    state.active_connections.fetch_sub(1, Ordering::SeqCst);
    let active = state.active_connections.load(Ordering::SeqCst);
    println!(
        "连接已断开 (ID: {}, 剩余活跃: {})",
        conn_id, active
    );
}

// ============================================================
// 5. 提供 HTML 测试页面
// ============================================================

/// 返回一个简单的 HTML 页面，用于测试 WebSocket 连接
/// 实际项目中，前端通常是独立的 SPA 应用
async fn index() -> impl IntoResponse {
    Html(
        r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <title>WebSocket Test</title>
    <style>
        body { font-family: sans-serif; max-width: 800px; margin: 50px auto; padding: 0 20px; }
        #log { background: #1e1e1e; color: #d4d4d4; padding: 15px; border-radius: 8px;
               height: 400px; overflow-y: auto; font-family: monospace; font-size: 14px; }
        .sent { color: #4ec9b0; }
        .received { color: #9cdcfe; }
        .system { color: #ce9178; }
        .error { color: #f44747; }
        input { padding: 10px; width: 70%; border: 1px solid #ccc; border-radius: 4px; }
        button { padding: 10px 20px; background: #0078d4; color: white; border: none;
                 border-radius: 4px; cursor: pointer; margin-left: 5px; }
        button:hover { background: #005a9e; }
        h1 { color: #333; }
    </style>
</head>
<body>
    <h1>WebSocket Test Page</h1>
    <p>ws://127.0.0.1:3000/ws</p>
    <div>
        <button onclick="connect()">Connect</button>
        <button onclick="disconnect()">Disconnect</button>
    </div>
    <br>
    <div>
        <input type="text" id="msg" placeholder="Type a message..." onkeypress="if(event.key==='Enter')send()">
        <button onclick="send()">Send</button>
    </div>
    <br>
    <div id="log"></div>

    <script>
        let ws = null;
        const log = document.getElementById('log');

        function addLog(msg, cls) {
            const div = document.createElement('div');
            div.className = cls;
            div.textContent = `[${new Date().toLocaleTimeString()}] ${msg}`;
            log.appendChild(div);
            log.scrollTop = log.scrollHeight;
        }

        function connect() {
            if (ws && ws.readyState === WebSocket.OPEN) {
                addLog('Already connected', 'system');
                return;
            }
            ws = new WebSocket('ws://127.0.0.1:3000/ws');
            ws.onopen = () => addLog('Connected', 'system');
            ws.onmessage = (e) => addLog('< ' + e.data, 'received');
            ws.onclose = (e) => addLog(`Disconnected (code: ${e.code})`, 'system');
            ws.onerror = () => addLog('Connection error', 'error');
        }

        function send() {
            const input = document.getElementById('msg');
            if (!ws || ws.readyState !== WebSocket.OPEN) {
                addLog('Not connected', 'error');
                return;
            }
            const msg = input.value.trim();
            if (!msg) return;
            ws.send(msg);
            addLog('> ' + msg, 'sent');
            input.value = '';
        }

        function disconnect() {
            if (ws) { ws.close(); ws = null; }
        }

        connect();
    </script>
</body>
</html>"#,
    )
}

/// 服务器状态接口（REST）
async fn server_status(State(state): State<AppState>) -> impl IntoResponse {
    let active = state.active_connections.load(Ordering::SeqCst);
    let total = state.total_connections.load(Ordering::SeqCst);

    axum::Json(serde_json::json!({
        "active_connections": active,
        "total_connections": total,
        "status": "running",
    }))
}

// ============================================================
// 6. 路由配置与服务器启动
// ============================================================

fn create_app() -> Router {
    let state = AppState::new();

    Router::new()
        // HTML 测试页面
        .route("/", get(index))
        // WebSocket 端点
        .route("/ws", get(ws_handler))
        // REST API：查看服务器状态
        .route("/status", get(server_status))
        // 注入状态
        .with_state(state)
}

#[tokio::main]
async fn main() {
    println!("=== Lesson 092: WebSocket ===\n");

    let app = create_app();

    println!("WebSocket 服务器启动中...");
    println!("打开浏览器访问 http://127.0.0.1:3000 查看测试页面\n");
    println!("端点：");
    println!("  GET /      -> HTML 测试页面");
    println!("  GET /ws    -> WebSocket 端点");
    println!("  GET /status -> 服务器状态（REST）\n");
    println!("WebSocket 命令：");
    println!("  ping   - 测试心跳");
    println!("  status - 查看服务器状态");
    println!("  help   - 显示帮助");
    println!("  quit   - 断开连接");
    println!("  其他   - 回显消息\n");
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
// 1. WebSocket 基本概念：
//    - 全双工通信协议，基于 TCP
//    - 通过 HTTP Upgrade 机制建立连接
//    - 适用于实时通信：聊天、游戏、推送通知等
//    - 消息类型：Text、Binary、Ping、Pong、Close
//
// 2. Axum WebSocket 支持：
//    - 需要在 Cargo.toml 中启用 axum 的 "ws" feature
//    - WebSocketUpgrade extractor 处理升级请求
//    - .on_upgrade(async fn(WebSocket)) 处理升级后的连接
//    - WebSocket 类型提供 send() 和 recv() 方法
//
// 3. 消息处理模式：
//    - while let Some(msg) = socket.recv().await { ... }
//    - 匹配 Message::Text / Binary / Ping / Pong / Close
//    - socket.send(Message::Text(...)) 发送消息
//    - axum 0.8 中 Message::Text 包装 Utf8Bytes 类型
//
// 4. 连接生命周期管理：
//    - 使用 AtomicU64 追踪活跃连接数
//    - 连接建立时递增，断开时递减
//    - Close 帧处理：收到后回复 Close 完成握手
//
// 5. 实际应用中的注意事项：
//    - 心跳检测：定期发送 Ping 检测连接是否存活
//    - 消息广播：使用 tokio::sync::broadcast 实现多客户端通信
//    - 认证：在 WebSocket 升级前进行身份验证
//    - 限流：限制每个连接的消息频率
//    - 断线重连：客户端应实现自动重连逻辑
