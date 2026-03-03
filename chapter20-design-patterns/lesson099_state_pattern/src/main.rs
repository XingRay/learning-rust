// ============================================================
// Lesson 099: 状态模式 / 类型状态模式 (Typestate Pattern)
// ============================================================
//
// 在传统 OOP 中，状态模式使用运行时状态对象来改变行为。
// 在 Rust 中，我们可以利用类型系统在 **编译期** 检查状态转换的合法性。
//
// 核心思想：
//   - 用不同的类型表示不同的状态
//   - 状态转换通过消耗旧状态、返回新状态来实现（所有权转移）
//   - 只有特定状态才有特定方法（编译期保证）
//   - 非法的状态转换会导致编译错误，而不是运行时错误

use std::fmt;

// ============================================================
// 1. 博客文章审批流程 —— 类型状态模式
// ============================================================
//
// 流程：草稿 -> 待审核 -> 已发布
//
//   Draft  --(request_review)--> PendingReview
//   PendingReview  --(approve)--> Published
//   PendingReview  --(reject)-->  Draft
//
// 关键：每个状态是一个独立的类型，非法操作在编译期就被阻止。

/// 草稿状态
struct Draft;

/// 待审核状态
struct PendingReview;

/// 已发布状态
struct Published;

/// 博客文章，泛型参数 `State` 表示当前状态
struct BlogPost<State> {
    title: String,
    content: String,
    // PhantomData 的替代：直接持有状态值
    // 这里用一个零大小类型（ZST）作为状态标记
    _state: State,
}

// --- Draft 状态的方法 ---
impl BlogPost<Draft> {
    /// 创建新文章（初始状态为 Draft）
    fn new(title: &str) -> Self {
        println!("[创建] 新文章草稿: \"{}\"", title);
        BlogPost {
            title: title.to_string(),
            content: String::new(),
            _state: Draft,
        }
    }

    /// 编辑内容 —— 只有草稿状态才能编辑
    fn edit_content(&mut self, content: &str) {
        println!("[编辑] 更新文章内容");
        self.content = content.to_string();
    }

    /// 追加内容 —— 只有草稿状态才能追加
    fn append_content(&mut self, extra: &str) {
        println!("[追加] 追加文章内容");
        self.content.push_str(extra);
    }

    /// 提交审核 —— 消耗 Draft，返回 PendingReview
    fn request_review(self) -> BlogPost<PendingReview> {
        println!("[提交] \"{}\" 已提交审核", self.title);
        BlogPost {
            title: self.title,
            content: self.content,
            _state: PendingReview,
        }
    }
}

// --- PendingReview 状态的方法 ---
impl BlogPost<PendingReview> {
    /// 审核通过 —— 消耗 PendingReview，返回 Published
    fn approve(self) -> BlogPost<Published> {
        println!("[通过] \"{}\" 审核通过，已发布！", self.title);
        BlogPost {
            title: self.title,
            content: self.content,
            _state: Published,
        }
    }

    /// 审核拒绝 —— 消耗 PendingReview，退回 Draft
    fn reject(self, reason: &str) -> BlogPost<Draft> {
        println!("[拒绝] \"{}\" 被拒绝，原因: {}", self.title, reason);
        BlogPost {
            title: self.title,
            content: self.content,
            _state: Draft,
        }
    }
}

// --- Published 状态的方法 ---
impl BlogPost<Published> {
    /// 获取文章内容 —— 只有已发布的文章才能被读取
    fn view(&self) -> &str {
        &self.content
    }

    /// 获取文章标题
    fn title(&self) -> &str {
        &self.title
    }
}

// --- 所有状态共享的方法 ---
impl<State> BlogPost<State> {
    /// 获取标题（任何状态都可以查看标题）
    fn get_title(&self) -> &str {
        &self.title
    }
}

// ============================================================
// 2. 更复杂的例子：网络连接的类型状态
// ============================================================

/// 连接状态类型
struct Disconnected;
struct Connecting;
struct Connected;
struct Authenticated;

/// TCP 连接（简化模型）
struct Connection<State> {
    address: String,
    _state: State,
}

impl Connection<Disconnected> {
    fn new(address: &str) -> Self {
        println!("[连接] 创建连接对象: {}", address);
        Connection {
            address: address.to_string(),
            _state: Disconnected,
        }
    }

    /// 开始连接 —— 从 Disconnected 转换到 Connecting
    fn connect(self) -> Connection<Connecting> {
        println!("[连接] 正在连接 {}...", self.address);
        Connection {
            address: self.address,
            _state: Connecting,
        }
    }
}

impl Connection<Connecting> {
    /// 连接成功 —— 从 Connecting 转换到 Connected
    fn on_connected(self) -> Connection<Connected> {
        println!("[连接] 已连接到 {}", self.address);
        Connection {
            address: self.address,
            _state: Connected,
        }
    }

    /// 连接失败 —— 退回 Disconnected
    fn on_failed(self) -> Connection<Disconnected> {
        println!("[连接] 连接 {} 失败", self.address);
        Connection {
            address: self.address,
            _state: Disconnected,
        }
    }
}

impl Connection<Connected> {
    /// 认证 —— 从 Connected 转换到 Authenticated
    fn authenticate(self, _user: &str, _pass: &str) -> Connection<Authenticated> {
        println!("[认证] 用户认证成功");
        Connection {
            address: self.address,
            _state: Authenticated,
        }
    }

    /// 断开连接
    fn disconnect(self) -> Connection<Disconnected> {
        println!("[断开] 已断开连接");
        Connection {
            address: self.address,
            _state: Disconnected,
        }
    }
}

impl Connection<Authenticated> {
    /// 发送数据 —— 只有认证后才能发送
    fn send_data(&self, data: &str) {
        println!("[发送] 发送数据到 {}: \"{}\"", self.address, data);
    }

    /// 接收数据 —— 只有认证后才能接收
    fn receive_data(&self) -> String {
        let data = format!("来自 {} 的响应数据", self.address);
        println!("[接收] {}", data);
        data
    }

    /// 断开连接
    fn disconnect(self) -> Connection<Disconnected> {
        println!("[断开] 已断开已认证的连接");
        Connection {
            address: self.address,
            _state: Disconnected,
        }
    }
}

// ============================================================
// 3. 运行时状态模式（传统 OOP 风格，作为对比）
// ============================================================

/// 使用 trait 对象的运行时状态模式
trait DocumentState: fmt::Display {
    fn name(&self) -> &str;
    fn can_edit(&self) -> bool;
    fn can_publish(&self) -> bool;
}

struct DraftState;
struct ReviewState;
struct PublishedState;

impl fmt::Display for DraftState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "草稿")
    }
}

impl DocumentState for DraftState {
    fn name(&self) -> &str { "草稿" }
    fn can_edit(&self) -> bool { true }
    fn can_publish(&self) -> bool { false }
}

impl fmt::Display for ReviewState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "审核中")
    }
}

impl DocumentState for ReviewState {
    fn name(&self) -> &str { "审核中" }
    fn can_edit(&self) -> bool { false }
    fn can_publish(&self) -> bool { false }
}

impl fmt::Display for PublishedState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "已发布")
    }
}

impl DocumentState for PublishedState {
    fn name(&self) -> &str { "已发布" }
    fn can_edit(&self) -> bool { false }
    fn can_publish(&self) -> bool { true }
}

struct Document {
    title: String,
    content: String,
    state: Box<dyn DocumentState>,
}

impl Document {
    fn new(title: &str) -> Self {
        Document {
            title: title.to_string(),
            content: String::new(),
            state: Box::new(DraftState),
        }
    }

    fn edit(&mut self, content: &str) {
        if self.state.can_edit() {
            self.content = content.to_string();
            println!("[运行时] 编辑成功");
        } else {
            println!("[运行时] 当前状态 ({}) 不允许编辑！", self.state);
        }
    }

    fn submit_review(&mut self) {
        println!("[运行时] \"{}\" 从 {} -> 审核中", self.title, self.state);
        self.state = Box::new(ReviewState);
    }

    fn publish(&mut self) {
        println!("[运行时] \"{}\" 从 {} -> 已发布", self.title, self.state);
        self.state = Box::new(PublishedState);
    }

    fn status(&self) {
        println!(
            "[运行时] 文档 \"{}\" 当前状态: {}",
            self.title, self.state
        );
    }
}

// ============================================================
// main 函数 —— 演示所有用法
// ============================================================

fn main() {
    println!("=== Lesson 099: 类型状态模式 (Typestate Pattern) ===\n");

    // ---------------------------------------------------------
    // 1. 博客文章审批流程
    // ---------------------------------------------------------
    println!("--- 1. 博客文章审批流程 ---");

    // 创建草稿
    let mut post = BlogPost::new("Rust 类型状态模式");

    // 编辑内容（只有 Draft 状态才有 edit_content 方法）
    post.edit_content("类型状态模式利用 Rust 的类型系统...");
    post.append_content("\n可以在编译期防止非法状态转换。");

    // 提交审核（消耗 Draft，返回 PendingReview）
    let post = post.request_review();

    // 以下代码会编译错误！因为 PendingReview 状态没有 edit_content 方法：
    // post.edit_content("试图修改");  // 编译错误！

    // 审核通过（消耗 PendingReview，返回 Published）
    let post = post.approve();

    // 查看已发布的文章
    println!("标题: {}", post.title());
    println!("内容: {}", post.view());

    println!();

    // 审核拒绝的流程
    println!("--- 审核拒绝的流程 ---");
    let mut post2 = BlogPost::new("待拒绝的文章");
    post2.edit_content("这篇文章有问题...");

    let post2 = post2.request_review();
    // 拒绝，退回草稿
    let mut post2 = post2.reject("内容不符合规范");

    // 修改后重新提交
    post2.edit_content("修改后的合规内容");
    let post2 = post2.request_review();
    let post2 = post2.approve();
    println!("最终发布: \"{}\" -> {}", post2.title(), post2.view());

    println!();

    // ---------------------------------------------------------
    // 2. 网络连接类型状态
    // ---------------------------------------------------------
    println!("--- 2. 网络连接类型状态 ---");

    let conn = Connection::new("192.168.1.100:5432");

    // 连接
    let conn = conn.connect();
    let conn = conn.on_connected();

    // 认证
    let conn = conn.authenticate("admin", "password");

    // 只有认证后才能发送/接收数据
    conn.send_data("SELECT * FROM users");
    let _response = conn.receive_data();

    // 断开连接
    let _conn = conn.disconnect();

    // 以下代码会编译错误！Disconnected 状态没有 send_data 方法：
    // _conn.send_data("不合法的操作");  // 编译错误！

    println!();

    // 连接失败的流程
    println!("--- 连接失败并重试 ---");
    let conn = Connection::new("10.0.0.1:3306");
    let conn = conn.connect();
    let conn = conn.on_failed();    // 模拟连接失败
    let conn = conn.connect();      // 重试连接
    let conn = conn.on_connected(); // 这次成功了
    let conn = conn.authenticate("root", "123456");
    conn.send_data("SHOW DATABASES");
    let _conn = conn.disconnect();

    println!();

    // ---------------------------------------------------------
    // 3. 运行时状态模式（对比）
    // ---------------------------------------------------------
    println!("--- 3. 运行时状态模式（传统 OOP 风格对比）---");

    let mut doc = Document::new("运行时状态文档");
    doc.status();

    doc.edit("文档内容...");
    doc.submit_review();

    // 审核中尝试编辑 —— 运行时检查，不会编译错误
    doc.edit("试图在审核中修改");

    doc.publish();
    doc.status();

    println!();

    // ---------------------------------------------------------
    // 4. 两种方式对比
    // ---------------------------------------------------------
    println!("--- 4. 类型状态 vs 运行时状态 对比 ---");
    println!("┌──────────────┬────────────────────────┬────────────────────────┐");
    println!("│ 特性         │ 类型状态 (Typestate)   │ 运行时状态 (Trait Obj) │");
    println!("├──────────────┼────────────────────────┼────────────────────────┤");
    println!("│ 错误检查时机 │ 编译期                 │ 运行时                 │");
    println!("│ 安全性       │ 更高（编译器保证）     │ 可能遗漏检查           │");
    println!("│ 灵活性       │ 状态必须编译期确定     │ 可动态切换             │");
    println!("│ 代码量       │ 每个状态一个 impl 块   │ 一个 trait + 多实现     │");
    println!("│ 适用场景     │ 状态转换规则固定       │ 状态转换规则动态       │");
    println!("└──────────────┴────────────────────────┴────────────────────────┘");

    println!("\n=== 类型状态模式学习完成！===");
}
