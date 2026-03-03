# 🦀 Learning Rust — 从入门到实战的 Rust 系统学习项目

<p align="center">
  <strong>A comprehensive, hands-on Rust learning project — from beginner to production-ready.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/lessons-103-blue" alt="103 Lessons">
  <img src="https://img.shields.io/badge/chapters-20-green" alt="20 Chapters">
  <img src="https://img.shields.io/badge/code_lines-42000%2B-orange" alt="42000+ Lines">
  <img src="https://img.shields.io/badge/language-Rust-red" alt="Rust">
  <img src="https://img.shields.io/badge/comments-中文-yellow" alt="Chinese Comments">
</p>

---

## 📖 项目简介 | About

本项目是一套**完整的 Rust 编程语言学习课程**，采用 Cargo Workspace 组织，包含 **20 个章节、103 节课**，覆盖从基础语法到 Web 开发的完整知识体系。每节课都是一个独立可运行的 Rust 程序，配有丰富的中文注释和循序渐进的代码示例。

This is a **complete Rust programming language course** organized as a Cargo Workspace, containing **20 chapters and 103 lessons** — covering everything from basic syntax to web development. Each lesson is a standalone, runnable Rust program with detailed Chinese comments and progressive code examples.

## ✨ 特点 | Features

- 🎯 **103 节课**，从 Hello World 到完整 Web API 项目 | 103 lessons, from Hello World to full Web API projects
- 📂 **清晰的两级目录结构**：章节 → 课程 | Clear two-level structure: Chapter → Lesson
- 🏃 **每课独立可运行**，无需按顺序学习 | Each lesson runs independently
- 📝 **全中文注释**，适合中文 Rust 学习者 | Full Chinese comments for Chinese learners
- 🔧 **Cargo Workspace** 统一管理，一键编译 | Unified management with Cargo Workspace
- 📈 **渐进式学习路线**，4 大阶段由浅入深 | Progressive 4-stage learning path

## 🚀 快速开始 | Quick Start

### 环境要求 | Prerequisites

- [Rust](https://rustup.rs/) 1.70+ (推荐最新稳定版 | latest stable recommended)
- 推荐 IDE：[RustRover](https://www.jetbrains.com/rust/) 或 [VS Code](https://code.visualstudio.com/) + rust-analyzer

### 克隆与运行 | Clone & Run

```bash
# 克隆项目 | Clone the project
git clone https://github.com/your-username/learning-rust.git
cd learning-rust

# 运行第一课 | Run the first lesson
cargo run -p lesson001_helloworld

# 运行任意一课 | Run any lesson
cargo run -p lesson045_threads
cargo run -p lesson098_builder_pattern

# 编译所有课程 | Build all lessons
cargo check --workspace

# 运行某课的测试 | Run tests for a lesson
cargo test -p lesson062_unit_tests
```

## 📚 课程目录 | Course Outline

### 🟢 第一阶段：基础入门 | Stage 1: Fundamentals (23 lessons)

<details>
<summary><b>Chapter 01 — Rust 基础入门 | Rust Basics</b> (7 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 001 | `lesson001_helloworld` | Hello, World! — 程序结构、println! 宏 |
| 002 | `lesson002_variables` | 变量与可变性 — let、mut、const、shadowing |
| 003 | `lesson003_data_types` | 数据类型 — 整数、浮点、布尔、字符、元组、数组 |
| 004 | `lesson004_functions` | 函数 — 参数、返回值、表达式与语句 |
| 005 | `lesson005_control_flow` | 控制流 — if/else、loop、while、for |
| 006 | `lesson006_comments` | 注释与文档 — ///、//!、rustdoc |
| 007 | `lesson007_strings` | 字符串 — String vs &str、UTF-8 |

</details>

<details>
<summary><b>Chapter 02 — 所有权系统 | Ownership System</b> (4 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 008 | `lesson008_ownership` | 所有权基础 — 移动语义、Clone、Copy |
| 009 | `lesson009_references` | 引用与借用 — &T、&mut T、借用规则 |
| 010 | `lesson010_slices` | 切片 — &str、&[T]、切片语法 |
| 011 | `lesson011_lifetime_basics` | 生命周期 — 'a 标注、省略规则、'static |

</details>

<details>
<summary><b>Chapter 03 — 结构体与枚举 | Structs & Enums</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 012 | `lesson012_struct_basics` | 结构体基础 — 定义、元组结构体、单元结构体 |
| 013 | `lesson013_struct_methods` | 结构体方法 — impl、关联函数、方法链 |
| 014 | `lesson014_enum_basics` | 枚举基础 — 变体、枚举方法 |
| 015 | `lesson015_option_enum` | Option 枚举 — Some/None、常用方法 |
| 016 | `lesson016_match_expression` | match 表达式 — 模式匹配、守卫、绑定 |

</details>

<details>
<summary><b>Chapter 04 — 泛型与模式匹配 | Generics & Patterns</b> (3 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 017 | `lesson017_generics` | 泛型 — 函数/结构体/枚举泛型、单态化 |
| 018 | `lesson018_pattern_matching` | 高级模式匹配 — 解构、嵌套模式、@ 绑定 |
| 019 | `lesson019_if_let_while_let` | if let / while let — let-else、matches! |

</details>

<details>
<summary><b>Chapter 05 — 错误处理 | Error Handling</b> (4 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 020 | `lesson020_panic` | panic! — 不可恢复错误、catch_unwind |
| 021 | `lesson021_result_type` | Result 类型 — Ok/Err、map/and_then |
| 022 | `lesson022_error_propagation` | 错误传播 — ? 操作符、From trait |
| 023 | `lesson023_custom_errors` | 自定义错误 — Display/Error trait |

</details>

---

### 🟡 第二阶段：核心进阶 | Stage 2: Core Intermediate (22 lessons)

<details>
<summary><b>Chapter 06 — 集合与迭代器 | Collections & Iterators</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 024 | `lesson024_vector` | Vec — 创建、增删改查、内存布局 |
| 025 | `lesson025_hashmap` | HashMap — entry API、词频统计 |
| 026 | `lesson026_hashset` | HashSet — 集合运算、去重 |
| 027 | `lesson027_iterator_basics` | 迭代器基础 — Iterator trait、iter/into_iter |
| 028 | `lesson028_iterator_adapters` | 迭代器适配器 — map/filter/fold/collect |
| 029 | `lesson029_custom_iterator` | 自定义迭代器 — Fibonacci、DoubleEndedIterator |

</details>

<details>
<summary><b>Chapter 07 — Trait 与高级类型 | Traits & Advanced Types</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 030 | `lesson030_trait_basics` | Trait 基础 — 定义、实现、默认方法 |
| 031 | `lesson031_trait_bounds` | Trait 约束 — where 子句、blanket impl |
| 032 | `lesson032_common_traits` | 常用 Trait — Display/Debug/Clone/Copy/From |
| 033 | `lesson033_trait_objects` | Trait 对象 — dyn、动态分发、对象安全 |
| 034 | `lesson034_associated_types` | 关联类型 — 运算符重载 |
| 035 | `lesson035_newtype_pattern` | Newtype 模式 — 类型安全、孤儿规则 |

</details>

<details>
<summary><b>Chapter 08 — 闭包与函数式编程 | Closures & Functional</b> (4 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 036 | `lesson036_closure_basics` | 闭包基础 — 语法、捕获环境、move |
| 037 | `lesson037_closure_as_param` | 闭包参数 — impl Fn、函数指针 |
| 038 | `lesson038_fn_traits` | Fn 系列 Trait — Fn/FnMut/FnOnce |
| 039 | `lesson039_functional_combinators` | 函数式组合子 — 链式调用 |

</details>

<details>
<summary><b>Chapter 09 — 智能指针 | Smart Pointers</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 040 | `lesson040_box` | Box — 堆分配、递归类型 |
| 041 | `lesson041_rc_arc` | Rc / Arc — 引用计数、线程安全 |
| 042 | `lesson042_refcell` | RefCell — 内部可变性、Rc\<RefCell\<T\>\> |
| 043 | `lesson043_deref_drop` | Deref / Drop — 自动解引用、析构 |
| 044 | `lesson044_weak_reference` | Weak — 避免循环引用 |

</details>

---

### 🔴 第三阶段：高级特性 | Stage 3: Advanced (28 lessons)

<details>
<summary><b>Chapter 10 — 并发编程 | Concurrency</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 045 | `lesson045_threads` | 线程 — spawn、JoinHandle、move |
| 046 | `lesson046_message_passing` | 消息传递 — mpsc 通道 |
| 047 | `lesson047_shared_state` | 共享状态 — Arc\<Mutex\<T\>\> |
| 048 | `lesson048_mutex_rwlock` | Mutex / RwLock — 死锁避免 |
| 049 | `lesson049_atomic` | 原子操作 — AtomicUsize、Ordering |
| 050 | `lesson050_send_sync` | Send / Sync — 线程安全 trait |

</details>

<details>
<summary><b>Chapter 11 — 异步编程 | Async Programming</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 051 | `lesson051_async_await_basics` | async/await 基础 |
| 052 | `lesson052_future_trait` | Future Trait — Poll/Pin/Waker |
| 053 | `lesson053_tokio_runtime` | Tokio 运行时 — spawn、任务调度 |
| 054 | `lesson054_async_channels` | 异步通道 — mpsc/oneshot/broadcast |
| 055 | `lesson055_select_join` | select! / join! — 并发等待、超时 |
| 056 | `lesson056_async_streams` | 异步流 — Stream trait |

</details>

<details>
<summary><b>Chapter 12 — 宏编程 | Macros</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 057 | `lesson057_declarative_macros` | 声明式宏 — macro_rules! |
| 058 | `lesson058_macro_rules_advanced` | 宏进阶 — 递归、TT Muncher |
| 059 | `lesson059_derive_macros` | 派生宏 — #[derive] 原理与使用 |
| 060 | `lesson060_attribute_macros` | 属性宏 — #[cfg]、条件编译 |
| 061 | `lesson061_proc_macro_workshop` | 过程宏 — TokenStream、syn、quote |

</details>

<details>
<summary><b>Chapter 13 — 测试 | Testing</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 062 | `lesson062_unit_tests` | 单元测试 — #[test]、assert! 系列 |
| 063 | `lesson063_integration_tests` | 集成测试 — tests/ 目录 |
| 064 | `lesson064_doc_tests` | 文档测试 — 代码示例测试 |
| 065 | `lesson065_benchmark` | 性能基准测试 — Instant、criterion |
| 066 | `lesson066_mocking` | Mock 测试 — trait 注入、测试替身 |

</details>

<details>
<summary><b>Chapter 14 — Cargo 与模块系统 | Cargo & Modules</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 067 | `lesson067_module_system` | 模块系统 — mod、路径、可见性 |
| 068 | `lesson068_packages_crates` | 包与 Crate — lib.rs vs main.rs |
| 069 | `lesson069_use_and_pub` | use 与可见性 — pub(crate)、re-export |
| 070 | `lesson070_cargo_features` | Cargo Features — 条件编译 |
| 071 | `lesson071_workspace` | Workspace — 工作空间管理 |
| 072 | `lesson072_publish_crate` | 发布 Crate — crates.io、semver |

</details>

---

### 🟣 第四阶段：应用实战 | Stage 4: Real-World Applications (30 lessons)

<details>
<summary><b>Chapter 15 — I/O 与文件系统 | I/O & Filesystem</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 073 | `lesson073_stdin_stdout` | 标准输入输出 — BufReader/BufWriter |
| 074 | `lesson074_file_read_write` | 文件读写 — File、OpenOptions |
| 075 | `lesson075_path_operations` | 路径操作 — Path/PathBuf |
| 076 | `lesson076_directory_traversal` | 目录遍历 — read_dir、metadata |
| 077 | `lesson077_serde_json` | JSON — serde、序列化/反序列化 |
| 078 | `lesson078_serde_toml_yaml` | TOML / YAML — 配置文件处理 |

</details>

<details>
<summary><b>Chapter 16 — 网络编程 | Networking</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 079 | `lesson079_tcp_server` | TCP 服务器 — TcpListener、多线程 |
| 080 | `lesson080_tcp_client` | TCP 客户端 — TcpStream |
| 081 | `lesson081_udp` | UDP 通信 — UdpSocket |
| 082 | `lesson082_http_client_reqwest` | HTTP 客户端 — reqwest |
| 083 | `lesson083_http_server_basics` | HTTP 服务基础 — 手写 HTTP 解析 |

</details>

<details>
<summary><b>Chapter 17 — Unsafe 与 FFI | Unsafe & FFI</b> (4 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 084 | `lesson084_unsafe_basics` | unsafe 基础 — 五种超能力 |
| 085 | `lesson085_raw_pointers` | 裸指针 — *const T、*mut T |
| 086 | `lesson086_ffi_c_interop` | FFI — extern "C"、#[repr(C)] |
| 087 | `lesson087_extern_functions` | 外部函数 — ABI、全局分配器 |

</details>

<details>
<summary><b>Chapter 18 — Web 开发 | Web Development</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 088 | `lesson088_actix_web_basics` | Actix-web — 路由、Handler |
| 089 | `lesson089_axum_basics` | Axum — Router、Extractor |
| 090 | `lesson090_rest_api` | REST API — 完整 CRUD |
| 091 | `lesson091_middleware` | 中间件 — Tower、日志、CORS |
| 092 | `lesson092_websocket` | WebSocket — 实时通信 |

</details>

<details>
<summary><b>Chapter 19 — 数据库 | Database</b> (5 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 093 | `lesson093_sqlx_basics` | SQLx — 异步 SQL、连接池 |
| 094 | `lesson094_diesel_orm` | Diesel — ORM、查询 DSL |
| 095 | `lesson095_sea_orm` | SeaORM — 异步 ORM |
| 096 | `lesson096_redis_client` | Redis — 数据类型、命令 |
| 097 | `lesson097_mongodb` | MongoDB — 文档数据库 |

</details>

<details>
<summary><b>Chapter 20 — 设计模式与项目实战 | Design Patterns & Projects</b> (6 lessons)</summary>

| # | Lesson | Topic |
|---|--------|-------|
| 098 | `lesson098_builder_pattern` | 建造者模式 — 链式调用 |
| 099 | `lesson099_state_pattern` | 状态模式 — 类型状态、状态机 |
| 100 | `lesson100_observer_pattern` | 观察者模式 — 事件驱动 |
| 101 | `lesson101_command_pattern` | 命令模式 — 撤销/重做 |
| 102 | `lesson102_mini_project_cli_tool` | 🛠️ 实战: CLI 工具 (类 wc) |
| 103 | `lesson103_mini_project_web_api` | 🌐 实战: Web API 服务器 |

</details>

## 📂 项目结构 | Project Structure

```
learning-rust/
├── Cargo.toml                              # Workspace 根配置
├── README.md                               # 本文件
├── OUTLINE.md                              # 详细课程大纲
│
├── chapter01-basic/                        # 第01章: Rust 基础入门
│   ├── lesson001_helloworld/               #   第001课
│   │   ├── Cargo.toml                      #     课程依赖配置
│   │   └── src/main.rs                     #     课程代码（含中文注释）
│   ├── lesson002_variables/
│   └── ...
│
├── chapter02-ownership/                    # 第02章: 所有权系统
├── chapter03-struct-and-enum/              # 第03章: 结构体与枚举
├── ...
└── chapter20-design-patterns/              # 第20章: 设计模式与实战
    ├── lesson102_mini_project_cli_tool/
    └── lesson103_mini_project_web_api/
```

## 🗺️ 学习路线图 | Learning Roadmap

```
第1阶段 基础入门                            第2阶段 核心进阶
┌────────────────────┐                   ┌────────────────────────┐
│ CH01 基础语法       │                   │ CH06 集合与迭代器       │
│ CH02 所有权         │─────────────────▶│ CH07 Trait 与类型       │
│ CH03 结构体/枚举    │                   │ CH08 闭包与函数式       │
│ CH04 泛型/匹配      │                   │ CH09 智能指针           │
│ CH05 错误处理       │                   └───────────┬────────────┘
└────────────────────┘                               │
                                                     ▼
第4阶段 应用实战                            第3阶段 高级特性
┌────────────────────┐                   ┌────────────────────────┐
│ CH15 I/O 文件系统   │                   │ CH10 并发编程           │
│ CH16 网络编程       │◀─────────────────│ CH11 异步编程           │
│ CH17 Unsafe/FFI     │                   │ CH12 宏编程             │
│ CH18 Web 开发       │                   │ CH13 测试               │
│ CH19 数据库         │                   │ CH14 Cargo 与模块       │
│ CH20 设计模式/实战  │                   └────────────────────────┘
└────────────────────┘
```

## 🔧 常用命令 | Common Commands

```bash
# 运行指定课程 | Run a specific lesson
cargo run -p lesson001_helloworld

# 编译检查（不生成二进制） | Check without building
cargo check -p lesson042_refcell

# 编译所有课程 | Build all lessons
cargo build --workspace

# 运行测试 | Run tests
cargo test -p lesson062_unit_tests

# 查看文档注释 | View documentation
cargo doc -p lesson006_comments --open

# 以 release 模式运行 | Run in release mode
cargo run -p lesson065_benchmark --release
```

## 🤝 如何使用 | How to Use

### 初学者推荐路线 | Recommended Path for Beginners

1. **按章节顺序学习** — 从 Chapter 01 开始，每章按课程编号递增
2. **阅读代码注释** — 每个 `main.rs` 都有详细的中文注释说明概念
3. **运行并修改** — 先 `cargo run` 看效果，再修改代码实验
4. **完成练习** — 部分课程底部有被注释掉的练习，取消注释试试看
5. **运行测试** — Chapter 13 的课程用 `cargo test` 验证

### 有经验的开发者 | For Experienced Developers

- 直接跳到感兴趣的章节，每课独立运行
- 重点关注 Chapter 10-12（并发/异步/宏）和 Chapter 18-20（Web/数据库/实战）
- 参考 `OUTLINE.md` 快速定位知识点

## 📋 第三方依赖 | Dependencies

大部分课程仅使用 Rust 标准库，以下章节使用了第三方 crate：

| Chapter | Crate | Purpose |
|---------|-------|---------|
| CH11 异步编程 | `tokio`, `tokio-stream` | 异步运行时 |
| CH15 I/O | `serde`, `serde_json`, `toml` | 序列化/配置文件 |
| CH16 网络 | `reqwest` | HTTP 客户端 |
| CH18 Web | `actix-web`, `axum`, `tower-http` | Web 框架 |

## 📄 License

This project is for educational purposes. Feel free to use it for learning Rust.

---

<p align="center">
  <i>Happy Rustacean! 🦀 愿你在 Rust 的学习旅途中收获满满！</i>
</p>
