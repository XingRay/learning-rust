// ============================================================
// Lesson 061: 过程宏实战 (Procedural Macros Workshop) — 概念讲解
// ============================================================
// 本课全面讲解 Rust 过程宏 (procedural macros) 的三种类型、
// 核心概念、工具链，以及完整的伪代码讲解。
//
// 过程宏与声明式宏 (macro_rules!) 的区别：
// - macro_rules!: 基于模式匹配和替换，功能有限但简单
// - 过程宏: 是真正的 Rust 函数，可以操作 TokenStream，功能强大
//
// 由于过程宏必须定义在单独的 proc-macro crate 中，
// 本课以概念讲解为主，用注释详细说明原理，
// 代码侧重于展示使用现有过程宏的示例。
// ============================================================

use std::fmt;

// ============================================================
// 第一部分：过程宏三种类型概述
// ============================================================
//
// Rust 提供三种过程宏：
//
// ┌─────────────────────┬───────────────────────┬──────────────────────┐
// │    类型              │  语法                  │  作用                │
// ├─────────────────────┼───────────────────────┼──────────────────────┤
// │ 1. Derive 宏        │ #[derive(MyMacro)]    │ 为类型自动实现 trait  │
// │ 2. Attribute 宏     │ #[my_attr]            │ 修改/替换被标注的项   │
// │ 3. Function-like 宏 │ my_macro!(...)        │ 像函数调用但在编译期   │
// └─────────────────────┴───────────────────────┴──────────────────────┘
//
// 共同点：
// - 都必须定义在 proc-macro crate 中
// - 输入和输出都是 TokenStream
// - 在编译期执行，可以运行任意 Rust 代码
// - 可以使用 syn 和 quote 等辅助库

// ============================================================
// 第二部分：TokenStream 概念
// ============================================================
//
// TokenStream 是过程宏的核心概念。
// 它是一个 Token（标记）的序列，代表一段 Rust 代码。
//
// Token 的类型 (proc_macro::TokenTree):
// ┌────────────┬────────────────────────────────────────────┐
// │ 类型        │ 示例                                       │
// ├────────────┼────────────────────────────────────────────┤
// │ Ident      │ foo, String, my_var (标识符)               │
// │ Literal    │ 42, "hello", 3.14 (字面量)                 │
// │ Punct      │ +, -, ::, => (标点符号)                     │
// │ Group      │ (...), [...], {...} (被分隔符包围的 token 组) │
// └────────────┴────────────────────────────────────────────┘
//
// 过程宏的基本签名：
//
// ```rust
// use proc_macro::TokenStream;
//
// // Derive 宏
// #[proc_macro_derive(MyDerive)]
// pub fn my_derive(input: TokenStream) -> TokenStream {
//     // input: 被 derive 的类型定义
//     // 返回: 新生成的代码（通常是 impl 块）
//     todo!()
// }
//
// // Attribute 宏
// #[proc_macro_attribute]
// pub fn my_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
//     // attr: 属性的参数
//     // item: 被标注的代码项
//     // 返回: 替换后的代码
//     todo!()
// }
//
// // Function-like 宏
// #[proc_macro]
// pub fn my_function_like(input: TokenStream) -> TokenStream {
//     // input: 宏调用的参数
//     // 返回: 替换后的代码
//     todo!()
// }
// ```

// ============================================================
// 第三部分：syn 和 quote crate 简介
// ============================================================
//
// syn — 解析 TokenStream 为 Rust AST
// ─────────────────────────────────────
// syn 库将原始的 TokenStream 解析为结构化的语法树，
// 让我们可以方便地分析 Rust 代码的结构。
//
// 核心类型：
// - DeriveInput: 解析 derive 宏的输入
// - ItemFn: 函数定义
// - ItemStruct: 结构体定义
// - Fields: 字段集合（命名字段/元组字段/单元结构体）
// - Ident: 标识符
// - Type: 类型
//
// 使用示例（伪代码）：
// ```rust
// use syn::{parse_macro_input, DeriveInput};
//
// let input = parse_macro_input!(input as DeriveInput);
// let name = &input.ident;        // 类型名
// let generics = &input.generics; // 泛型参数
// let data = &input.data;         // 结构体/枚举的数据
//
// match data {
//     syn::Data::Struct(data) => {
//         // 处理结构体的字段
//         for field in &data.fields {
//             let field_name = &field.ident;
//             let field_type = &field.ty;
//         }
//     }
//     syn::Data::Enum(data) => {
//         // 处理枚举的变体
//         for variant in &data.variants {
//             let variant_name = &variant.ident;
//         }
//     }
//     _ => {}
// }
// ```
//
// quote — 将 Rust 代码转回 TokenStream
// ─────────────────────────────────────
// quote 库提供了 quote! 宏，让我们可以用"准引号"语法
// 方便地生成 Rust 代码的 TokenStream。
//
// quote! 宏中用 # 前缀来插入变量：
// ```rust
// use quote::quote;
//
// let name = format_ident!("MyStruct");
// let field_name = format_ident!("x");
// let field_type = quote!(i32);
//
// let generated = quote! {
//     impl #name {
//         pub fn get_field(&self) -> #field_type {
//             self.#field_name
//         }
//     }
// };
// ```
//
// 重复展开语法：
// ```rust
// let field_names = vec![
//     format_ident!("x"),
//     format_ident!("y"),
//     format_ident!("z"),
// ];
//
// let generated = quote! {
//     impl MyStruct {
//         fn field_names() -> Vec<&'static str> {
//             vec![ #( stringify!(#field_names) ),* ]
//         }
//     }
// };
// ```

// ============================================================
// 第四部分：完整的 Derive 宏伪代码讲解
// ============================================================
//
// 以下是一个完整的自定义 derive 宏的伪代码，
// 实现了一个 `Describe` trait，为结构体自动生成描述方法。
//
// ========== 第一步：定义 trait (在普通 crate 中) ==========
//
// ```rust
// // describe_trait/src/lib.rs
// pub trait Describe {
//     fn describe(&self) -> String;
//     fn field_names() -> Vec<&'static str>;
//     fn struct_name() -> &'static str;
// }
// ```
//
// ========== 第二步：实现 derive 宏 (在 proc-macro crate 中) ==========
//
// ```rust
// // describe_derive/Cargo.toml
// // [lib]
// // proc-macro = true
// //
// // [dependencies]
// // syn = { version = "2", features = ["full"] }
// // quote = "1"
// // proc-macro2 = "1"
//
// // describe_derive/src/lib.rs
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput, Data, Fields};
//
// #[proc_macro_derive(Describe)]
// pub fn describe_derive(input: TokenStream) -> TokenStream {
//     // 1. 解析输入
//     let input = parse_macro_input!(input as DeriveInput);
//     let name = &input.ident;  // 获取类型名，如 "User"
//
//     // 2. 提取字段信息
//     let field_names: Vec<_> = match &input.data {
//         Data::Struct(data) => {
//             match &data.fields {
//                 Fields::Named(fields) => {
//                     fields.named.iter()
//                         .map(|f| f.ident.as_ref().unwrap())
//                         .collect()
//                 }
//                 _ => vec![]
//             }
//         }
//         _ => panic!("Describe 只支持结构体"),
//     };
//
//     // 3. 生成 describe() 方法的格式化字符串
//     let field_formats: Vec<_> = field_names.iter()
//         .map(|name| {
//             quote! {
//                 format!("  {}: {:?}", stringify!(#name), self.#name)
//             }
//         })
//         .collect();
//
//     // 4. 使用 quote! 生成 impl 代码
//     let expanded = quote! {
//         impl Describe for #name {
//             fn describe(&self) -> String {
//                 let mut parts = vec![
//                     format!("{} {{", stringify!(#name))
//                 ];
//                 #( parts.push(#field_formats); )*
//                 parts.push("}".to_string());
//                 parts.join("\n")
//             }
//
//             fn field_names() -> Vec<&'static str> {
//                 vec![ #( stringify!(#field_names) ),* ]
//             }
//
//             fn struct_name() -> &'static str {
//                 stringify!(#name)
//             }
//         }
//     };
//
//     // 5. 返回生成的 TokenStream
//     TokenStream::from(expanded)
// }
// ```
//
// ========== 第三步：使用 (在应用 crate 中) ==========
//
// ```rust
// use describe_trait::Describe;
// use describe_derive::Describe;  // derive 宏
//
// #[derive(Describe)]
// struct User {
//     name: String,
//     age: u32,
//     email: String,
// }
//
// fn main() {
//     let user = User {
//         name: "张三".to_string(),
//         age: 25,
//         email: "zhangsan@example.com".to_string(),
//     };
//
//     println!("{}", user.describe());
//     // 输出:
//     // User {
//     //   name: "张三"
//     //   age: 25
//     //   email: "zhangsan@example.com"
//     // }
//
//     println!("字段: {:?}", User::field_names());
//     // 输出: ["name", "age", "email"]
// }
// ```

// ============================================================
// 第五部分：手动模拟 derive 宏的效果
// ============================================================
// 虽然我们不能在这个 binary crate 中创建真正的过程宏，
// 但我们可以手动实现一个 trait，展示 derive 宏生成的代码长什么样。

/// 自定义 trait：描述一个类型
trait Describe {
    /// 返回类型的详细描述
    fn describe(&self) -> String;
    /// 返回所有字段名
    fn field_names() -> Vec<&'static str>;
    /// 返回结构体名
    fn struct_name() -> &'static str;
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

// 以下 impl 就是 #[derive(Describe)] 会自动生成的代码
// 过程宏的价值在于：它能自动生成这些重复性代码
impl Describe for User {
    fn describe(&self) -> String {
        let mut parts = vec![format!("{} {{", stringify!(User))];
        parts.push(format!("  {}: {:?}", stringify!(name), self.name));
        parts.push(format!("  {}: {:?}", stringify!(age), self.age));
        parts.push(format!("  {}: {:?}", stringify!(email), self.email));
        parts.push("}".to_string());
        parts.join("\n")
    }

    fn field_names() -> Vec<&'static str> {
        vec![
            stringify!(name),
            stringify!(age),
            stringify!(email),
        ]
    }

    fn struct_name() -> &'static str {
        stringify!(User)
    }
}

// 再为另一个结构体手动实现——体会重复性
#[derive(Debug)]
struct Product {
    id: u64,
    name: String,
    price: f64,
    in_stock: bool,
}

// 如果有 #[derive(Describe)]，以下代码就不需要手动写了
impl Describe for Product {
    fn describe(&self) -> String {
        let mut parts = vec![format!("{} {{", stringify!(Product))];
        parts.push(format!("  {}: {:?}", stringify!(id), self.id));
        parts.push(format!("  {}: {:?}", stringify!(name), self.name));
        parts.push(format!("  {}: {:?}", stringify!(price), self.price));
        parts.push(format!("  {}: {:?}", stringify!(in_stock), self.in_stock));
        parts.push("}".to_string());
        parts.join("\n")
    }

    fn field_names() -> Vec<&'static str> {
        vec![
            stringify!(id),
            stringify!(name),
            stringify!(price),
            stringify!(in_stock),
        ]
    }

    fn struct_name() -> &'static str {
        stringify!(Product)
    }
}

// ============================================================
// 第六部分：Function-like 过程宏概念
// ============================================================
//
// Function-like 过程宏看起来像函数调用：my_macro!(...)
// 但与 macro_rules! 不同，它可以：
// - 接受任意 TokenStream 作为输入（不限于 macro_rules! 的模式）
// - 执行任意 Rust 代码来处理输入
// - 访问文件系统、网络等（编译期）
//
// 定义（在 proc-macro crate 中）：
// ```rust
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     // 可以解析 SQL 语句，在编译期验证语法
//     // 生成类型安全的数据库查询代码
//     todo!()
// }
// ```
//
// 使用：
// ```rust
// let query = sql!(SELECT * FROM users WHERE age > 18);
// ```
//
// 实际应用：
// - sqlx::query! — 编译期 SQL 验证
// - include_str! — 在编译期读取文件（标准库内置）
// - env! — 读取编译期环境变量（标准库内置）
// - lazy_static! — 惰性初始化的静态变量（虽然这个用 macro_rules! 实现）

// ============================================================
// 第七部分：Attribute 过程宏概念补充
// ============================================================
//
// Attribute 宏的实际应用举例：
//
// 1. Web 框架路由（如 actix-web / axum）：
// ```rust
// #[get("/api/users/{id}")]
// async fn get_user(id: web::Path<u32>) -> impl Responder {
//     // ...
// }
// ```
// 宏会将这个函数包装成路由处理器，自动：
// - 解析 URL 参数
// - 绑定到正确的 HTTP 方法
// - 生成路由注册代码
//
// 2. tokio 的异步测试：
// ```rust
// #[tokio::test]
// async fn test_async() {
//     let result = some_async_fn().await;
//     assert_eq!(result, 42);
// }
// ```
// 宏会将 async fn 包装在 tokio runtime 中运行
//
// 3. serde 的字段属性：
// ```rust
// #[derive(Serialize, Deserialize)]
// struct User {
//     #[serde(rename = "user_name")]
//     name: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     email: Option<String>,
//     #[serde(default)]
//     active: bool,
// }
// ```
// derive 宏通过 helper attributes（辅助属性）来自定义序列化行为

// ============================================================
// 第八部分：何时使用过程宏
// ============================================================
//
// 使用过程宏的场景：
// ✓ 需要根据类型结构自动生成代码（derive 宏）
// ✓ 需要在编译期验证 DSL 语法（如 SQL、HTML 模板）
// ✓ 需要修改/包装函数行为（attribute 宏）
// ✓ 需要生成大量重复性代码且 macro_rules! 无法满足
// ✓ 需要在编译期访问外部资源（文件、环境变量等）
//
// 不使用过程宏的场景：
// ✗ 简单的代码替换 → 使用 macro_rules!
// ✗ 简单的条件编译 → 使用 #[cfg]
// ✗ 可以用函数/trait 实现的 → 直接用函数/trait
// ✗ 对编译时间敏感 → 过程宏会增加编译时间
//
// 过程宏的代价：
// - 增加编译时间（syn 和 quote 本身也需要编译）
// - 增加项目复杂度（需要额外的 crate）
// - 调试困难（错误信息可能难以理解）
// - IDE 支持可能不完善

// ---- 模拟 Display 的自定义实现 ----
// 展示如果有一个 #[derive(AutoDisplay)] 宏，它会生成的代码

#[derive(Debug)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
    workers: u32,
}

// 如果有 #[derive(AutoDisplay)] 宏，会自动生成类似的代码：
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{ host: {}, port: {}, debug: {}, workers: {} }}",
            self.host, self.port, self.debug, self.workers
        )
    }
}

// ---- 模拟 Builder 模式的 derive ----
// 展示如果有一个 #[derive(Builder)] 宏，它会生成的代码

#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_secs: u64,
}

// 如果有 #[derive(Builder)]，以下代码会自动生成：
struct ServerBuilder {
    host: Option<String>,
    port: Option<u16>,
    max_connections: Option<u32>,
    timeout_secs: Option<u64>,
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder {
            host: None,
            port: None,
            max_connections: None,
            timeout_secs: None,
        }
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn max_connections(mut self, max_connections: u32) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    fn timeout_secs(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = Some(timeout_secs);
        self
    }

    fn build(self) -> Result<Server, String> {
        Ok(Server {
            host: self.host.ok_or("host is required")?,
            port: self.port.ok_or("port is required")?,
            max_connections: self.max_connections.unwrap_or(100),
            timeout_secs: self.timeout_secs.unwrap_or(30),
        })
    }
}

impl Server {
    fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }
}

fn main() {
    println!("=== Lesson 061: 过程宏实战 (Procedural Macros Workshop) ===\n");

    // ---- 第一部分：过程宏三种类型 ----
    println!("--- 第一部分：过程宏三种类型概述 ---");
    println!("┌─────────────────┬──────────────────────┬────────────────────────┐");
    println!("│ 类型            │ 语法                 │ 用途                   │");
    println!("├─────────────────┼──────────────────────┼────────────────────────┤");
    println!("│ Derive 宏       │ #[derive(MyMacro)]   │ 为类型自动实现 trait   │");
    println!("│ Attribute 宏    │ #[my_attr]           │ 修改/替换被标注的项    │");
    println!("│ Function-like 宏│ my_macro!(...)       │ 像函数调用，编译期执行 │");
    println!("└─────────────────┴──────────────────────┴────────────────────────┘");
    println!();

    // ---- 第二部分：TokenStream ----
    println!("--- 第二部分：TokenStream 概念 ---");
    println!("TokenStream 是 Token 的序列，代表一段 Rust 代码。");
    println!("Token 类型:");
    println!("  • Ident    — 标识符: foo, String");
    println!("  • Literal  — 字面量: 42, \"hello\"");
    println!("  • Punct    — 标点: +, -, ::");
    println!("  • Group    — 分组: (...), [...], {{...}}");
    println!();

    // ---- 第三部分：syn 和 quote ----
    println!("--- 第三部分：syn 和 quote ---");
    println!("syn crate:");
    println!("  • 将 TokenStream 解析为结构化的 AST");
    println!("  • 核心类型: DeriveInput, ItemFn, Fields, Ident, Type");
    println!("  • 可以轻松遍历结构体字段、枚举变体等");
    println!();
    println!("quote crate:");
    println!("  • 将 Rust 代码模板转回 TokenStream");
    println!("  • quote! 宏中用 # 插值: #name, #field_type");
    println!("  • 支持重复: #( #field_names ),*");
    println!();

    // ---- 第四/五部分：Describe trait 演示 ----
    println!("--- 第四/五部分：手动模拟 derive 宏效果 ---");

    let user = User {
        name: "张三".to_string(),
        age: 25,
        email: "zhangsan@example.com".to_string(),
    };

    println!("User::struct_name() = {}", User::struct_name());
    println!("User::field_names() = {:?}", User::field_names());
    println!("\nuser.describe() 输出:");
    println!("{}", user.describe());

    let product = Product {
        id: 1001,
        name: "Rust 编程语言".to_string(),
        price: 99.9,
        in_stock: true,
    };

    println!("\nProduct::struct_name() = {}", Product::struct_name());
    println!("Product::field_names() = {:?}", Product::field_names());
    println!("\nproduct.describe() 输出:");
    println!("{}", product.describe());

    println!("\n说明: 上面两个 Describe 实现结构几乎相同，");
    println!("这就是 derive 宏的价值——自动生成这些重复性代码！");
    println!();

    // ---- 模拟 AutoDisplay ----
    println!("--- 模拟 #[derive(AutoDisplay)] ---");
    let config = Config {
        host: "localhost".to_string(),
        port: 8080,
        debug: true,
        workers: 4,
    };
    println!("Display: {}", config);
    println!("Debug:   {:?}", config);
    println!();

    // ---- 模拟 Builder ----
    println!("--- 模拟 #[derive(Builder)] ---");
    let server = Server::builder()
        .host("0.0.0.0")
        .port(3000)
        .max_connections(500)
        .timeout_secs(60)
        .build()
        .unwrap();

    println!("Server: {:?}", server);

    // 使用默认值
    let server2 = Server::builder()
        .host("localhost")
        .port(8080)
        .build()
        .unwrap();

    println!(
        "Server (默认值): {:?} (max_connections 和 timeout_secs 使用默认值)",
        server2
    );

    // 缺少必要字段
    let result = Server::builder().host("localhost").build();
    println!("缺少 port: {:?}", result);
    println!();

    // ---- 何时使用过程宏 ----
    println!("--- 第八部分：何时使用过程宏 ---");
    println!("适合使用过程宏的场景:");
    println!("  ✓ 根据类型结构自动生成代码 (derive)");
    println!("  ✓ 编译期验证 DSL 语法 (sql!, html!)");
    println!("  ✓ 修改/包装函数行为 (路由、日志)");
    println!("  ✓ 大量重复性代码且 macro_rules! 不够用");
    println!();
    println!("不适合使用过程宏的场景:");
    println!("  ✗ 简单替换 → 用 macro_rules!");
    println!("  ✗ 条件编译 → 用 #[cfg]");
    println!("  ✗ 能用函数/trait → 直接用函数/trait");
    println!("  ✗ 编译时间敏感的项目");

    // ---- Proc-macro crate 结构说明 ----
    println!("\n--- Proc-macro crate 项目结构 ---");
    println!("典型的项目结构:");
    println!("  my_project/");
    println!("  ├── Cargo.toml         # workspace");
    println!("  ├── my_trait/");
    println!("  │   ├── Cargo.toml     # 定义 trait");
    println!("  │   └── src/lib.rs");
    println!("  ├── my_derive/");
    println!("  │   ├── Cargo.toml     # proc-macro = true");
    println!("  │   └── src/lib.rs     # derive 宏实现");
    println!("  └── my_app/");
    println!("      ├── Cargo.toml     # 依赖 my_trait + my_derive");
    println!("      └── src/main.rs    # 使用 #[derive(MyTrait)]");

    // ---- 常用过程宏 crate 推荐 ----
    println!("\n--- 常用过程宏 crate ---");
    println!("序列化:     serde, serde_derive");
    println!("错误处理:   thiserror, anyhow");
    println!("异步运行时: tokio (tokio::main, tokio::test)");
    println!("Web 框架:   actix-web, axum, rocket");
    println!("数据库:     sqlx, diesel, sea-orm");
    println!("CLI 解析:   clap (derive 模式)");
    println!("Builder:    derive_builder");
    println!("测试:       rstest, proptest");
    println!("调试:       derive_more (Display, From 等)");

    // ---- 总结 ----
    println!("\n--- 宏系统总结 (Chapter 12) ---");
    println!("┌────────────────────┬─────────────┬─────────────┬───────────┐");
    println!("│ 特性               │ macro_rules!│ derive 宏   │ 属性/函数宏│");
    println!("├────────────────────┼─────────────┼─────────────┼───────────┤");
    println!("│ 定义位置           │ 任意        │ proc-macro  │ proc-macro│");
    println!("│ 复杂度             │ 低          │ 中          │ 高        │");
    println!("│ 灵活性             │ 有限        │ 中等        │ 最强      │");
    println!("│ 编译时间影响       │ 小          │ 中          │ 中-大     │");
    println!("│ 需要额外依赖       │ 否          │ syn/quote   │ syn/quote │");
    println!("│ 调试难度           │ 低          │ 中          │ 高        │");
    println!("└────────────────────┴─────────────┴─────────────┴───────────┘");

    println!("\n=== Lesson 061 完成 ===");
}
