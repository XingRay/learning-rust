# 🦀 Rust 学习课程大纲

> 项目结构: Cargo Workspace
> 运行方式: `cargo run -p lessonXXX_名称`
> 总课时: 103 课 | 20 章节

---

## 📖 课程总览

| 阶段 | 章节 | 难度 | 课时数 |
|------|------|------|--------|
| 🟢 基础入门 | Chapter 01-05 | 初级 | 23 课 |
| 🟡 核心进阶 | Chapter 06-09 | 中级 | 22 课 |
| 🔴 高级特性 | Chapter 10-14 | 高级 | 28 课 |
| 🟣 应用实战 | Chapter 15-20 | 实战 | 30 课 |

---

## 🟢 第一阶段：基础入门

### Chapter 01 - Rust 基础入门 (`chapter01-basic`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 001 | `lesson001_helloworld` | Hello, World! | 程序结构、main 函数、println! 宏、格式化输出 |
| 002 | `lesson002_variables` | 变量与可变性 | let 绑定、mut、常量 const、遮蔽 shadowing |
| 003 | `lesson003_data_types` | 数据类型 | 整数、浮点、布尔、字符、元组、数组 |
| 004 | `lesson004_functions` | 函数 | 函数定义、参数、返回值、表达式与语句 |
| 005 | `lesson005_control_flow` | 控制流 | if/else、loop、while、for、break、continue |
| 006 | `lesson006_comments` | 注释与文档 | 行注释、块注释、文档注释、rustdoc |
| 007 | `lesson007_strings` | 字符串 | String 与 &str、字符串操作、UTF-8 编码 |

### Chapter 02 - 所有权系统 (`chapter02-ownership`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 008 | `lesson008_ownership` | 所有权基础 | 所有权规则、移动语义、Clone 与 Copy |
| 009 | `lesson009_references` | 引用与借用 | 不可变引用、可变引用、借用规则 |
| 010 | `lesson010_slices` | 切片 | 字符串切片、数组切片、切片作为参数 |
| 011 | `lesson011_lifetime_basics` | 生命周期基础 | 生命周期标注、函数中的生命周期、'static |

### Chapter 03 - 结构体与枚举 (`chapter03-struct-and-enum`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 012 | `lesson012_struct_basics` | 结构体基础 | 定义、实例化、字段简写、元组结构体 |
| 013 | `lesson013_struct_methods` | 结构体方法 | impl 块、self 参数、关联函数 |
| 014 | `lesson014_enum_basics` | 枚举基础 | 枚举定义、变体、枚举中携带数据 |
| 015 | `lesson015_option_enum` | Option 枚举 | Some、None、Option 的常用方法 |
| 016 | `lesson016_match_expression` | match 表达式 | 模式匹配、穷尽匹配、_ 通配符 |

### Chapter 04 - 泛型与模式匹配 (`chapter04-pattern-and-generics`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 017 | `lesson017_generics` | 泛型 | 函数泛型、结构体泛型、枚举泛型、单态化 |
| 018 | `lesson018_pattern_matching` | 高级模式匹配 | 解构、守卫、绑定、@、嵌套模式 |
| 019 | `lesson019_if_let_while_let` | if let 与 while let | 简化匹配、let-else 语法 |

### Chapter 05 - 错误处理 (`chapter05-error-handling`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 020 | `lesson020_panic` | panic! 与不可恢复错误 | panic!、栈展开 vs 终止、RUST_BACKTRACE |
| 021 | `lesson021_result_type` | Result 类型 | Ok、Err、unwrap、expect |
| 022 | `lesson022_error_propagation` | 错误传播 | ? 操作符、From trait、错误链 |
| 023 | `lesson023_custom_errors` | 自定义错误 | 自定义错误类型、thiserror、anyhow |

---

## 🟡 第二阶段：核心进阶

### Chapter 06 - 集合与迭代器 (`chapter06-collections-and-iterators`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 024 | `lesson024_vector` | Vec 动态数组 | 创建、增删改查、遍历、内存布局 |
| 025 | `lesson025_hashmap` | HashMap | 创建、插入、查询、更新策略、entry API |
| 026 | `lesson026_hashset` | HashSet | 集合操作、交集、并集、差集 |
| 027 | `lesson027_iterator_basics` | 迭代器基础 | Iterator trait、next、into_iter/iter/iter_mut |
| 028 | `lesson028_iterator_adapters` | 迭代器适配器 | map、filter、fold、collect、chain、zip |
| 029 | `lesson029_custom_iterator` | 自定义迭代器 | 实现 Iterator trait、IntoIterator |

### Chapter 07 - Trait 与高级类型 (`chapter07-traits-and-advanced-types`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 030 | `lesson030_trait_basics` | Trait 基础 | trait 定义、实现、默认方法 |
| 031 | `lesson031_trait_bounds` | Trait 约束 | 泛型约束、where 子句、多重约束 |
| 032 | `lesson032_common_traits` | 常用 Trait | Display、Debug、Clone、Copy、PartialEq、Ord |
| 033 | `lesson033_trait_objects` | Trait 对象 | dyn、动态分发 vs 静态分发、对象安全 |
| 034 | `lesson034_associated_types` | 关联类型 | type 关联类型、泛型 vs 关联类型 |
| 035 | `lesson035_newtype_pattern` | Newtype 模式 | 类型包装、绕过孤儿规则 |

### Chapter 08 - 闭包与函数式编程 (`chapter08-closures-and-functional`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 036 | `lesson036_closure_basics` | 闭包基础 | 闭包语法、类型推断、捕获环境 |
| 037 | `lesson037_closure_as_param` | 闭包作为参数 | 闭包参数、impl Fn/FnMut/FnOnce |
| 038 | `lesson038_fn_traits` | Fn 系列 Trait | Fn、FnMut、FnOnce 区别与选择 |
| 039 | `lesson039_functional_combinators` | 函数式组合子 | map、and_then、unwrap_or_else 链式调用 |

### Chapter 09 - 智能指针 (`chapter09-smart-pointers`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 040 | `lesson040_box` | Box 堆分配 | Box<T>、递归类型、堆 vs 栈 |
| 041 | `lesson041_rc_arc` | Rc 与 Arc | 引用计数、共享所有权、线程安全引用计数 |
| 042 | `lesson042_refcell` | RefCell 内部可变性 | 运行时借用检查、Rc<RefCell<T>> 模式 |
| 043 | `lesson043_deref_drop` | Deref 与 Drop | 自动解引用、自定义析构 |
| 044 | `lesson044_weak_reference` | Weak 引用 | 避免循环引用、Weak<T>、upgrade/downgrade |

---

## 🔴 第三阶段：高级特性

### Chapter 10 - 并发编程 (`chapter10-concurrency`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 045 | `lesson045_threads` | 线程基础 | thread::spawn、JoinHandle、move 闭包 |
| 046 | `lesson046_message_passing` | 消息传递 | mpsc 通道、Sender/Receiver、多生产者 |
| 047 | `lesson047_shared_state` | 共享状态 | Arc<Mutex<T>>、线程安全共享 |
| 048 | `lesson048_mutex_rwlock` | Mutex 与 RwLock | 互斥锁、读写锁、死锁避免 |
| 049 | `lesson049_atomic` | 原子操作 | AtomicBool、AtomicUsize、Ordering |
| 050 | `lesson050_send_sync` | Send 与 Sync | 线程安全 trait、自动实现、手动实现 |

### Chapter 11 - 异步编程 (`chapter11-async-programming`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 051 | `lesson051_async_await_basics` | async/await 基础 | async fn、.await、异步 vs 同步 |
| 052 | `lesson052_future_trait` | Future Trait | Future 工作原理、Pin、Waker |
| 053 | `lesson053_tokio_runtime` | Tokio 运行时 | #[tokio::main]、spawn、任务调度 |
| 054 | `lesson054_async_channels` | 异步通道 | tokio::sync::mpsc、oneshot、broadcast |
| 055 | `lesson055_select_join` | select! 与 join! | 并发等待、超时、竞争 |
| 056 | `lesson056_async_streams` | 异步流 | Stream trait、async 迭代 |

### Chapter 12 - 宏编程 (`chapter12-macros`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 057 | `lesson057_declarative_macros` | 声明式宏 | macro_rules!、模式匹配、重复 |
| 058 | `lesson058_macro_rules_advanced` | 宏进阶 | 递归宏、宏卫生、宏调试 |
| 059 | `lesson059_derive_macros` | 派生宏 | #[derive]、自定义 derive 宏 |
| 060 | `lesson060_attribute_macros` | 属性宏 | #[属性]、proc_macro_attribute |
| 061 | `lesson061_proc_macro_workshop` | 过程宏实战 | TokenStream、syn、quote |

### Chapter 13 - 测试 (`chapter13-testing`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 062 | `lesson062_unit_tests` | 单元测试 | #[test]、assert! 系列、#[should_panic] |
| 063 | `lesson063_integration_tests` | 集成测试 | tests 目录、公共模块、测试组织 |
| 064 | `lesson064_doc_tests` | 文档测试 | 文档中的代码示例、# 隐藏行 |
| 065 | `lesson065_benchmark` | 性能基准测试 | criterion、基准测试编写 |
| 066 | `lesson066_mocking` | Mock 测试 | mockall、测试替身 |

### Chapter 14 - Cargo 与模块系统 (`chapter14-cargo-and-modules`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 067 | `lesson067_module_system` | 模块系统 | mod、mod.rs、文件模块 |
| 068 | `lesson068_packages_crates` | 包与 Crate | 包结构、lib.rs vs main.rs、多 binary |
| 069 | `lesson069_use_and_pub` | use 与可见性 | use 导入、pub(crate)、re-export |
| 070 | `lesson070_cargo_features` | Cargo Features | feature 标志、条件编译、可选依赖 |
| 071 | `lesson071_workspace` | Workspace | 工作空间管理、成员间依赖 |
| 072 | `lesson072_publish_crate` | 发布 Crate | crates.io、cargo publish、版本管理 |

---

## 🟣 第四阶段：应用实战

### Chapter 15 - I/O 与文件系统 (`chapter15-io-and-filesystem`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 073 | `lesson073_stdin_stdout` | 标准输入输出 | stdin().read_line、BufReader、Write trait |
| 074 | `lesson074_file_read_write` | 文件读写 | File::open、fs::read_to_string、写入 |
| 075 | `lesson075_path_operations` | 路径操作 | Path、PathBuf、路径拼接、跨平台 |
| 076 | `lesson076_directory_traversal` | 目录遍历 | fs::read_dir、walkdir crate、递归遍历 |
| 077 | `lesson077_serde_json` | JSON 序列化 | serde、serde_json、#[derive(Serialize)] |
| 078 | `lesson078_serde_toml_yaml` | TOML 与 YAML | toml crate、serde_yaml、配置文件 |

### Chapter 16 - 网络编程 (`chapter16-networking`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 079 | `lesson079_tcp_server` | TCP 服务器 | TcpListener、accept、多线程处理 |
| 080 | `lesson080_tcp_client` | TCP 客户端 | TcpStream::connect、读写流 |
| 081 | `lesson081_udp` | UDP 通信 | UdpSocket、send_to、recv_from |
| 082 | `lesson082_http_client_reqwest` | HTTP 客户端 | reqwest、GET/POST、异步请求 |
| 083 | `lesson083_http_server_basics` | HTTP 服务基础 | 手写 HTTP 解析、状态码、响应 |

### Chapter 17 - Unsafe 与 FFI (`chapter17-unsafe-and-ffi`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 084 | `lesson084_unsafe_basics` | unsafe 基础 | unsafe 块、何时使用、安全抽象 |
| 085 | `lesson085_raw_pointers` | 裸指针 | *const T、*mut T、指针运算 |
| 086 | `lesson086_ffi_c_interop` | FFI 与 C 互操作 | extern "C"、#[no_mangle]、bindgen |
| 087 | `lesson087_extern_functions` | 外部函数 | 调用 C 库、libc crate、ABI |

### Chapter 18 - Web 开发 (`chapter18-web-development`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 088 | `lesson088_actix_web_basics` | Actix-web 基础 | 路由、Handler、App 配置 |
| 089 | `lesson089_axum_basics` | Axum 基础 | Router、Handler、Extractor |
| 090 | `lesson090_rest_api` | REST API | CRUD 接口、JSON 请求/响应、状态管理 |
| 091 | `lesson091_middleware` | 中间件 | 日志、认证、CORS、Tower 中间件 |
| 092 | `lesson092_websocket` | WebSocket | 实时通信、ws 升级、消息推送 |

### Chapter 19 - 数据库 (`chapter19-database`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 093 | `lesson093_sqlx_basics` | SQLx 基础 | 连接池、查询、编译时检查 |
| 094 | `lesson094_diesel_orm` | Diesel ORM | schema、模型、迁移、查询构建 |
| 095 | `lesson095_sea_orm` | SeaORM | 实体定义、异步 ORM、关系查询 |
| 096 | `lesson096_redis_client` | Redis 客户端 | redis crate、命令、连接池 |
| 097 | `lesson097_mongodb` | MongoDB | mongodb crate、文档操作、异步驱动 |

### Chapter 20 - 设计模式与项目实战 (`chapter20-design-patterns`)

| 课时 | 目录名 | 主题 | 知识点 |
|------|--------|------|--------|
| 098 | `lesson098_builder_pattern` | 建造者模式 | Builder Pattern、链式调用、derive_builder |
| 099 | `lesson099_state_pattern` | 状态模式 | 类型状态模式、状态机、编译期状态检查 |
| 100 | `lesson100_observer_pattern` | 观察者模式 | 事件驱动、回调、发布-订阅 |
| 101 | `lesson101_command_pattern` | 命令模式 | 命令封装、撤销/重做、trait 对象 |
| 102 | `lesson102_mini_project_cli_tool` | 实战: CLI 工具 | clap、命令行解析、文件处理工具 |
| 103 | `lesson103_mini_project_web_api` | 实战: Web API | 完整 Web 服务、数据库、认证、部署 |

---

## 🗺️ 学习路线图

```
第 1 阶段 (基础入门)                    第 2 阶段 (核心进阶)
┌─────────────────┐                  ┌──────────────────────┐
│ CH01 基础语法     │──────────────────▶│ CH06 集合与迭代器     │
│ CH02 所有权       │                  │ CH07 Trait 与类型     │
│ CH03 结构体/枚举  │                  │ CH08 闭包与函数式     │
│ CH04 泛型/匹配    │                  │ CH09 智能指针         │
│ CH05 错误处理     │                  └──────────┬───────────┘
└─────────────────┘                              │
                                                 ▼
第 4 阶段 (应用实战)                    第 3 阶段 (高级特性)
┌─────────────────┐                  ┌──────────────────────┐
│ CH15 I/O 文件    │◀─────────────────│ CH10 并发编程         │
│ CH16 网络编程    │                  │ CH11 异步编程         │
│ CH17 Unsafe/FFI  │                  │ CH12 宏编程           │
│ CH18 Web 开发    │                  │ CH13 测试             │
│ CH19 数据库      │                  │ CH14 Cargo 与模块     │
│ CH20 设计模式    │                  └──────────────────────┘
└─────────────────┘
```

---

## 📂 项目目录结构

```
learning-rust/
├── Cargo.toml                          # Workspace 根配置
├── OUTLINE.md                          # 本课程大纲
│
├── chapter01-basic/                    # 第01章: Rust 基础入门
│   ├── lesson001_helloworld/           # 第001课: Hello, World!
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── lesson002_variables/            # 第002课: 变量与可变性
│   ├── ...
│
├── chapter02-ownership/                # 第02章: 所有权系统
│   ├── lesson008_ownership/
│   ├── ...
│
├── ... (共 20 个章节)
│
└── chapter20-design-patterns/          # 第20章: 设计模式与项目实战
    ├── lesson098_builder_pattern/
    ├── ...
    └── lesson103_mini_project_web_api/
```

## 🚀 使用方式

```bash
# 运行某一课
cargo run -p lesson001_helloworld

# 编译某一课
cargo build -p lesson001_helloworld

# 运行某一课的测试
cargo test -p lesson001_helloworld

# 编译所有已启用的课程
cargo build --workspace
```
