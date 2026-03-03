// ============================================================
// Lesson 068: 包与 Crate (Packages and Crates)
// ============================================================
// Rust 的代码组织层次（从大到小）：
//   Package（包）> Crate（编译单元）> Module（模块）> Item（项）
//
// 本课讲解 Package 和 Crate 的概念及其关系。
// ============================================================

// ============================================================
// 1. 基本概念
// ============================================================
//
// 【Crate（编译单元）】
// - Crate 是 Rust 编译器一次编译的最小代码单元
// - 分为两种类型：
//   (a) Binary Crate（二进制 crate）：有 main 函数，编译为可执行文件
//   (b) Library Crate（库 crate）：没有 main 函数，编译为库供其他代码使用
// - 每个 crate 有一个根文件（crate root），编译器从此开始编译
//
// 【Package（包）】
// - Package 是一个 Cargo.toml 文件 + 一组 crate 的集合
// - 一个 Package 可以包含：
//   * 最多 1 个 library crate
//   * 任意数量的 binary crate
//   * 但至少要有 1 个 crate（binary 或 library）
//

// ============================================================
// 2. src/main.rs vs src/lib.rs
// ============================================================
//
// Cargo 遵循约定（convention over configuration）：
//
// src/main.rs
//   → 表示这是一个 binary crate 的根文件
//   → crate 的名字与 package 名字相同
//   → 编译后会生成可执行文件
//
// src/lib.rs
//   → 表示这是一个 library crate 的根文件
//   → crate 的名字与 package 名字相同
//   → 编译后会生成 .rlib 文件
//
// 同时存在 src/main.rs 和 src/lib.rs：
//   → 一个 package 同时拥有一个 binary crate 和一个 library crate
//   → 两者同名（与 package 名相同）
//   → binary crate 可以通过 `use package_name::xxx` 来使用 library crate 中的内容
//
// 示例目录结构：
//   my_project/
//   ├── Cargo.toml
//   └── src/
//       ├── main.rs    // binary crate root
//       └── lib.rs     // library crate root

// ============================================================
// 3. 多 Binary Crate（src/bin/ 目录）
// ============================================================
//
// 一个 package 可以有多个 binary crate：
// - src/main.rs 是默认的 binary crate
// - src/bin/ 目录下的每个 .rs 文件都是一个独立的 binary crate
//
// 示例目录结构：
//   my_project/
//   ├── Cargo.toml
//   └── src/
//       ├── main.rs        // 默认 binary crate（名字: my_project）
//       ├── lib.rs          // library crate（名字: my_project）
//       └── bin/
//           ├── tool_a.rs   // 额外 binary crate（名字: tool_a）
//           └── tool_b.rs   // 额外 binary crate（名字: tool_b）
//
// 运行方式：
//   cargo run               → 运行 src/main.rs
//   cargo run --bin tool_a   → 运行 src/bin/tool_a.rs
//   cargo run --bin tool_b   → 运行 src/bin/tool_b.rs
//
// 也可以使用目录形式：
//   src/bin/
//   └── complex_tool/
//       ├── main.rs         // 这个 binary crate 的入口
//       └── helpers.rs      // 辅助模块

// ============================================================
// 4. Cargo.toml 配置说明
// ============================================================
//
// Cargo.toml 是 package 的配置文件，使用 TOML 格式。
//
// 基本结构：
// ```toml
// [package]
// name = "my_project"       # package 名称（也是默认 crate 名称）
// version = "0.1.0"         # 版本号（遵循 SemVer）
// edition = "2021"          # Rust 版本（2015/2018/2021）
// authors = ["Your Name"]   # 作者信息
// description = "简短描述"   # 包描述
// license = "MIT"           # 许可证
// repository = "https://github.com/xxx"  # 仓库地址
//
// [dependencies]            # 生产依赖
// serde = "1.0"             # 简写版本
// tokio = { version = "1", features = ["full"] }  # 完整写法
//
// [dev-dependencies]        # 开发/测试依赖
// criterion = "0.5"
//
// [build-dependencies]      # 构建脚本依赖
// cc = "1.0"
//
// [[bin]]                   # 自定义 binary crate 配置
// name = "my_tool"
// path = "src/bin/my_tool.rs"
//
// [lib]                     # library crate 配置
// name = "my_lib"
// path = "src/lib.rs"
// ```
//
// 版本指定方式：
//   "1.0"     → ^1.0（兼容版本，>=1.0.0 且 <2.0.0）
//   "=1.0.5"  → 精确版本
//   ">=1.0"   → 大于等于 1.0
//   "~1.0"    → 约等于（>=1.0.0 且 <1.1.0）
//   "*"       → 任意版本（不推荐）

// ============================================================
// 5. 代码演示：模拟 package 和 crate 的概念
// ============================================================

/// 模拟一个 "库" 模块，展示 library crate 的概念
mod my_library {
    /// 计算器模块 —— 类似于 library crate 中的公有 API
    pub mod calculator {
        pub fn add(a: f64, b: f64) -> f64 {
            a + b
        }

        pub fn multiply(a: f64, b: f64) -> f64 {
            a * b
        }

        pub fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err(String::from("除数不能为零"))
            } else {
                Ok(a / b)
            }
        }
    }

    /// 格式化器模块
    pub mod formatter {
        pub fn format_result(operation: &str, result: f64) -> String {
            format!("[{}] 结果 = {:.2}", operation, result)
        }

        pub fn format_error(operation: &str, error: &str) -> String {
            format!("[{}] 错误: {}", operation, error)
        }
    }

    /// 库的版本信息 —— 类似于 library crate 的公开常量
    pub const VERSION: &str = "1.0.0";
}

/// 模拟另一个 "工具" 模块，展示多 binary crate 的概念
/// 在实际项目中，这可能是 src/bin/statistics.rs
mod statistics_tool {
    pub fn mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let sum: f64 = values.iter().sum();
        sum / values.len() as f64
    }

    pub fn median(values: &mut Vec<f64>) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = values.len() / 2;
        if values.len() % 2 == 0 {
            (values[mid - 1] + values[mid]) / 2.0
        } else {
            values[mid]
        }
    }

    pub fn variance(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let m = super::statistics_tool::mean(values);
        let sum_sq: f64 = values.iter().map(|x| (x - m).powi(2)).sum();
        sum_sq / values.len() as f64
    }
}

/// 展示 crate 属性和配置
/// 在实际项目中，这些通常在 crate 根文件顶部：
/// #![allow(unused)]        // crate 级属性
/// #![deny(missing_docs)]   // 要求所有公有项都有文档

fn main() {
    println!("=== Lesson 068: 包与 Crate ===\n");

    // ---- 1. Package vs Crate 概念回顾 ----
    println!("--- 1. Package vs Crate 概念 ---");
    println!("Package（包）= Cargo.toml + 一组 Crate");
    println!("Crate（编译单元）= 编译器一次编译的最小单位");
    println!("  - Binary Crate: 有 main()，编译为可执行文件");
    println!("  - Library Crate: 无 main()，编译为库");
    println!();

    // ---- 2. 使用 "库" 模块 ----
    println!("--- 2. 模拟 Library Crate 的使用 ---");
    println!("库版本: {}", my_library::VERSION);

    let sum = my_library::calculator::add(10.0, 20.0);
    println!("{}", my_library::formatter::format_result("加法 10+20", sum));

    let product = my_library::calculator::multiply(3.0, 7.0);
    println!(
        "{}",
        my_library::formatter::format_result("乘法 3*7", product)
    );

    match my_library::calculator::divide(10.0, 3.0) {
        Ok(result) => println!(
            "{}",
            my_library::formatter::format_result("除法 10/3", result)
        ),
        Err(e) => println!(
            "{}",
            my_library::formatter::format_error("除法 10/3", &e)
        ),
    }

    match my_library::calculator::divide(10.0, 0.0) {
        Ok(result) => println!(
            "{}",
            my_library::formatter::format_result("除法 10/0", result)
        ),
        Err(e) => println!(
            "{}",
            my_library::formatter::format_error("除法 10/0", &e)
        ),
    }
    println!();

    // ---- 3. 使用 "工具" 模块 ----
    println!("--- 3. 模拟多 Binary Crate (statistics 工具) ---");
    let mut data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0];
    println!("数据: {:?}", data);
    println!("均值: {:.2}", statistics_tool::mean(&data));
    println!("中位数: {:.2}", statistics_tool::median(&mut data));
    println!("方差: {:.2}", statistics_tool::variance(&data));
    println!();

    // ---- 4. 目录结构回顾 ----
    println!("--- 4. 常见 Package 目录结构 ---");
    println!("最简单的 binary package:");
    println!("  my_app/");
    println!("  ├── Cargo.toml");
    println!("  └── src/");
    println!("      └── main.rs");
    println!();

    println!("library + binary package:");
    println!("  my_app/");
    println!("  ├── Cargo.toml");
    println!("  └── src/");
    println!("      ├── main.rs    (binary crate)");
    println!("      └── lib.rs     (library crate)");
    println!();

    println!("多 binary package:");
    println!("  my_app/");
    println!("  ├── Cargo.toml");
    println!("  └── src/");
    println!("      ├── main.rs");
    println!("      ├── lib.rs");
    println!("      └── bin/");
    println!("          ├── tool_a.rs");
    println!("          └── tool_b.rs");
    println!();

    // ---- 5. Cargo.toml 配置要点 ----
    println!("--- 5. Cargo.toml 配置要点 ---");
    println!("[package]       → 包的基本信息（name, version, edition）");
    println!("[dependencies]  → 生产依赖");
    println!("[dev-dependencies] → 开发/测试依赖");
    println!("[build-dependencies] → 构建脚本依赖");
    println!("[[bin]]         → 自定义 binary crate");
    println!("[lib]           → 自定义 library crate");
    println!("[features]      → 可选功能标志");
    println!();

    // ---- 总结 ----
    println!("=== 总结 ===");
    println!("1. Package 包含 Cargo.toml 和至少一个 Crate");
    println!("2. Binary Crate (main.rs) 编译为可执行文件");
    println!("3. Library Crate (lib.rs) 编译为库");
    println!("4. src/bin/ 目录下可放置多个 binary crate");
    println!("5. Cargo.toml 管理依赖、版本、features 等配置");
}
