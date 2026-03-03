#![allow(dead_code)]
// ============================================================
// Lesson 072: 发布 Crate（概念讲解）
// ============================================================
// 本课讲解如何将 crate 发布到 crates.io，包括：
// - crates.io 简介
// - Cargo.toml 元数据配置
// - cargo publish 发布流程
// - SemVer 版本管理
// - cargo doc 生成文档
//
// 注意：本课为概念讲解，不会实际执行发布操作。
// ============================================================

// ============================================================
// 1. crates.io 简介
// ============================================================
//
// crates.io 是 Rust 官方的 crate 仓库（registry）：
// - 网址: https://crates.io
// - 任何人都可以发布自己的 crate
// - 使用 GitHub 账号登录
// - 发布后不可删除（只能 yank 标记为不推荐）
// - 所有发布的 crate 必须是开源的
//
// 常用命令：
//   cargo search <关键词>    → 搜索 crate
//   cargo install <crate>   → 安装 binary crate（全局命令行工具）
//   cargo add <crate>       → 添加依赖到当前项目（Cargo 1.62+）

// ============================================================
// 2. Cargo.toml 元数据配置
// ============================================================
//
// 发布到 crates.io 时，Cargo.toml 需要包含必要的元数据：
//
// ```toml
// [package]
// name = "my-awesome-crate"       # crate 名称（全局唯一）
// version = "0.1.0"               # 版本号（遵循 SemVer）
// edition = "2021"                # Rust 版本
// authors = ["Your Name <you@example.com>"]  # 作者
// description = "A short description of what this crate does"  # 必填！
// license = "MIT"                 # 必填！许可证（SPDX 标识符）
// # 或者使用 license-file：
// # license-file = "LICENSE"
// repository = "https://github.com/you/my-awesome-crate"  # 源码仓库
// homepage = "https://my-crate-docs.com"      # 主页（可选）
// documentation = "https://docs.rs/my-awesome-crate"  # 文档地址（可选）
// readme = "README.md"            # README 文件（可选，默认 README.md）
// keywords = ["rust", "utility"]  # 关键词（最多 5 个，方便搜索）
// categories = ["command-line-utilities"]  # 分类（从固定列表中选择）
// exclude = ["tests/*", "benches/*"]  # 发布时排除的文件
// include = ["src/**/*", "Cargo.toml", "README.md"]  # 发布时包含的文件
// ```
//
// 常用许可证 SPDX 标识符：
//   MIT          → MIT 许可证
//   Apache-2.0   → Apache 2.0 许可证
//   MIT OR Apache-2.0  → 双许可证（Rust 生态惯用）
//   GPL-3.0      → GPL v3
//   BSD-3-Clause → BSD 3-Clause

// ============================================================
// 3. cargo publish 发布流程（注释说明）
// ============================================================
//
// 步骤 1: 注册并登录
//   (1) 访问 https://crates.io 并用 GitHub 登录
//   (2) 在 Account Settings 中生成 API Token
//   (3) 在终端执行: cargo login <your-api-token>
//       token 会保存在 ~/.cargo/credentials.toml
//
// 步骤 2: 准备发布
//   (1) 确保 Cargo.toml 包含必要的元数据（description, license）
//   (2) 确保代码能编译通过: cargo build
//   (3) 确保测试通过: cargo test
//   (4) 检查打包内容: cargo package --list
//   (5) 试打包（不发布）: cargo package
//
// 步骤 3: 发布
//   cargo publish
//   → 会自动打包、上传到 crates.io
//   → 发布后几分钟内就能被其他人使用
//
// 步骤 4: 后续版本更新
//   (1) 修改 Cargo.toml 中的 version
//   (2) 再次执行 cargo publish
//
// 注意事项：
//   - 发布后的版本不能删除（永久的！）
//   - 可以使用 cargo yank --vers 1.0.0 标记某版本不推荐使用
//   - cargo yank --vers 1.0.0 --undo 撤销 yank
//   - yanked 的版本仍可下载，但不会被新项目选择

// ============================================================
// 4. SemVer 版本管理
// ============================================================

/// 演示 SemVer（语义化版本）的概念
mod semver_demo {
    /// 版本号由三部分组成：MAJOR.MINOR.PATCH
    #[derive(Debug, Clone)]
    pub struct Version {
        pub major: u32, // 主版本号：不兼容的 API 变更
        pub minor: u32, // 次版本号：向后兼容的功能新增
        pub patch: u32, // 修订号：向后兼容的问题修正
    }

    impl Version {
        pub fn new(major: u32, minor: u32, patch: u32) -> Self {
            Version {
                major,
                minor,
                patch,
            }
        }

        pub fn display(&self) -> String {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        }

        /// 修订版本升级（bug 修复）
        pub fn bump_patch(&self) -> Version {
            Version::new(self.major, self.minor, self.patch + 1)
        }

        /// 次版本升级（新增功能，保持向后兼容）
        pub fn bump_minor(&self) -> Version {
            Version::new(self.major, self.minor + 1, 0)
        }

        /// 主版本升级（破坏性变更）
        pub fn bump_major(&self) -> Version {
            Version::new(self.major + 1, 0, 0)
        }

        /// 判断是否与另一个版本兼容（Cargo 的 ^ 规则）
        pub fn is_compatible_with(&self, other: &Version) -> bool {
            if self.major == 0 && other.major == 0 {
                // 0.x.y 版本：只有 minor 相同才兼容
                self.minor == other.minor
            } else {
                // 1.0+ 版本：major 相同就兼容
                self.major == other.major
            }
        }
    }

    impl std::fmt::Display for Version {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }

    /// 演示版本范围匹配
    pub fn explain_version_range(spec: &str) -> String {
        match spec {
            "^1.2.3" => ">=1.2.3 且 <2.0.0（默认的兼容性规则）".to_string(),
            "~1.2.3" => ">=1.2.3 且 <1.3.0（只允许 patch 升级）".to_string(),
            "1.2.*" => ">=1.2.0 且 <1.3.0（通配符）".to_string(),
            ">=1.0,<2.0" => ">=1.0.0 且 <2.0.0（显式范围）".to_string(),
            "=1.2.3" => "精确版本 1.2.3".to_string(),
            "^0.1.2" => ">=0.1.2 且 <0.2.0（0.x 版本更严格）".to_string(),
            "^0.0.3" => ">=0.0.3 且 <0.0.4（0.0.x 只匹配精确版本）".to_string(),
            _ => format!("未知版本规格: {}", spec),
        }
    }
}

// ============================================================
// 5. cargo doc 文档生成
// ============================================================

/// 这是一个示范文档注释的模块。
///
/// # 文档注释语法
///
/// Rust 使用 `///` 为接下来的项写文档（外部文档注释），
/// 使用 `//!` 为包含此注释的项写文档（内部文档注释）。
///
/// 文档注释支持 Markdown 语法。
mod doc_demo {
    /// 一个简单的计算器结构体。
    ///
    /// `Calculator` 可以执行基本的数学运算，并记录运算历史。
    ///
    /// # Examples
    ///
    /// ```
    /// // 注意：这里的代码块在 cargo doc --test 时会被测试（doc test）
    /// let mut calc = Calculator::new();
    /// calc.add(10.0);
    /// calc.add(20.0);
    /// assert_eq!(calc.result(), 30.0);
    /// ```
    pub struct Calculator {
        /// 当前结果
        current: f64,
        /// 运算历史记录
        history: Vec<String>,
    }

    impl Calculator {
        /// 创建一个新的计算器实例。
        ///
        /// 初始值为 0。
        ///
        /// # Returns
        ///
        /// 返回一个新的 `Calculator` 实例。
        pub fn new() -> Self {
            Calculator {
                current: 0.0,
                history: Vec::new(),
            }
        }

        /// 加法运算。
        ///
        /// # Arguments
        ///
        /// * `value` - 要加的数值
        ///
        /// # Examples
        ///
        /// ```
        /// let mut calc = Calculator::new();
        /// calc.add(5.0);
        /// assert_eq!(calc.result(), 5.0);
        /// ```
        pub fn add(&mut self, value: f64) {
            self.current += value;
            self.history.push(format!("+ {}", value));
        }

        /// 减法运算。
        ///
        /// # Arguments
        ///
        /// * `value` - 要减的数值
        pub fn subtract(&mut self, value: f64) {
            self.current -= value;
            self.history.push(format!("- {}", value));
        }

        /// 乘法运算。
        ///
        /// # Arguments
        ///
        /// * `value` - 要乘的数值
        pub fn multiply(&mut self, value: f64) {
            self.current *= value;
            self.history.push(format!("× {}", value));
        }

        /// 除法运算。
        ///
        /// # Arguments
        ///
        /// * `value` - 要除的数值（不能为零）
        ///
        /// # Errors
        ///
        /// 如果 `value` 为 0，返回错误信息。
        ///
        /// # Examples
        ///
        /// ```
        /// let mut calc = Calculator::new();
        /// calc.add(10.0);
        /// assert!(calc.divide(2.0).is_ok());
        /// assert!(calc.divide(0.0).is_err());
        /// ```
        pub fn divide(&mut self, value: f64) -> Result<(), String> {
            if value == 0.0 {
                return Err("除数不能为零".to_string());
            }
            self.current /= value;
            self.history.push(format!("÷ {}", value));
            Ok(())
        }

        /// 获取当前结果。
        pub fn result(&self) -> f64 {
            self.current
        }

        /// 获取运算历史。
        pub fn history(&self) -> &[String] {
            &self.history
        }

        /// 重置计算器。
        pub fn reset(&mut self) {
            self.current = 0.0;
            self.history.clear();
        }
    }
}

// ============================================================
// 6. 发布检查清单
// ============================================================

fn print_publish_checklist() {
    println!("发布 Crate 检查清单:");
    println!("┌────┬───────────────────────────────────────────────┐");
    println!("│ ## │ 检查项                                        │");
    println!("├────┼───────────────────────────────────────────────┤");
    println!("│  1 │ Cargo.toml 包含 name, version, edition       │");
    println!("│  2 │ Cargo.toml 包含 description (必填)            │");
    println!("│  3 │ Cargo.toml 包含 license 或 license-file (必填)│");
    println!("│  4 │ 有 README.md 文件                             │");
    println!("│  5 │ 有 LICENSE 文件                               │");
    println!("│  6 │ cargo build 编译通过                          │");
    println!("│  7 │ cargo test 测试通过                           │");
    println!("│  8 │ cargo clippy 无警告                           │");
    println!("│  9 │ cargo doc 文档生成正常                        │");
    println!("│ 10 │ cargo package 打包成功                        │");
    println!("│ 11 │ 版本号遵循 SemVer                             │");
    println!("│ 12 │ CHANGELOG.md 记录变更（推荐）                  │");
    println!("└────┴───────────────────────────────────────────────┘");
}

fn main() {
    println!("=== Lesson 072: 发布 Crate ===\n");

    // ---- 1. crates.io 简介 ----
    println!("--- 1. crates.io 简介 ---");
    println!("crates.io 是 Rust 官方的 crate 仓库");
    println!("  • 网址: https://crates.io");
    println!("  • 使用 GitHub 账号登录");
    println!("  • 搜索: cargo search <关键词>");
    println!("  • 安装: cargo install <crate>");
    println!("  • 添加: cargo add <crate>");
    println!();

    // ---- 2. Cargo.toml 元数据 ----
    println!("--- 2. Cargo.toml 发布元数据 ---");
    println!("必填字段:");
    println!("  name        = \"my-crate\"           # 全局唯一的名称");
    println!("  version     = \"0.1.0\"              # SemVer 版本");
    println!("  description = \"简短描述\"            # 包描述");
    println!("  license     = \"MIT OR Apache-2.0\"  # 许可证");
    println!();
    println!("推荐字段:");
    println!("  repository  = \"https://github.com/...\"");
    println!("  readme      = \"README.md\"");
    println!("  keywords    = [\"rust\", \"utility\"]");
    println!("  categories  = [\"command-line-utilities\"]");
    println!();

    // ---- 3. cargo publish 流程 ----
    println!("--- 3. cargo publish 发布流程 ---");
    println!("步骤 1: cargo login <token>     # 登录（一次性）");
    println!("步骤 2: cargo package --list    # 查看打包内容");
    println!("步骤 3: cargo package           # 试打包");
    println!("步骤 4: cargo publish           # 正式发布");
    println!();
    println!("版本更新:");
    println!("  修改 Cargo.toml 中的 version → 再次 cargo publish");
    println!();
    println!("撤回版本（不推荐使用，但不删除）:");
    println!("  cargo yank --vers 1.0.0       # 标记为不推荐");
    println!("  cargo yank --vers 1.0.0 --undo # 撤销 yank");
    println!();

    // ---- 4. SemVer 版本管理 ----
    println!("--- 4. SemVer 语义化版本 ---");
    let v = semver_demo::Version::new(1, 2, 3);
    println!("当前版本: {}", v);
    println!("  MAJOR({}) = 不兼容的 API 变更", v.major);
    println!("  MINOR({}) = 向后兼容的功能新增", v.minor);
    println!("  PATCH({}) = 向后兼容的问题修正", v.patch);
    println!();

    println!("版本升级示例:");
    let v_patch = v.bump_patch();
    println!("  bug 修复:   {} → {}", v, v_patch);
    let v_minor = v.bump_minor();
    println!("  新增功能:   {} → {}", v, v_minor);
    let v_major = v.bump_major();
    println!("  破坏性变更: {} → {}", v, v_major);
    println!();

    // 版本兼容性
    println!("版本兼容性 (Cargo ^ 规则):");
    let v1 = semver_demo::Version::new(1, 2, 3);
    let v2 = semver_demo::Version::new(1, 5, 0);
    let v3 = semver_demo::Version::new(2, 0, 0);
    println!(
        "  {} 与 {} 兼容? {}",
        v1,
        v2,
        v1.is_compatible_with(&v2)
    );
    println!(
        "  {} 与 {} 兼容? {}",
        v1,
        v3,
        v1.is_compatible_with(&v3)
    );

    let v4 = semver_demo::Version::new(0, 1, 0);
    let v5 = semver_demo::Version::new(0, 1, 5);
    let v6 = semver_demo::Version::new(0, 2, 0);
    println!(
        "  {} 与 {} 兼容? {} (0.x 版本更严格)",
        v4,
        v5,
        v4.is_compatible_with(&v5)
    );
    println!(
        "  {} 与 {} 兼容? {} (0.x minor 不同就不兼容)",
        v4,
        v6,
        v4.is_compatible_with(&v6)
    );
    println!();

    // 版本范围说明
    println!("Cargo.toml 版本指定规则:");
    let specs = vec![
        "^1.2.3",
        "~1.2.3",
        "1.2.*",
        ">=1.0,<2.0",
        "=1.2.3",
        "^0.1.2",
        "^0.0.3",
    ];
    for spec in specs {
        println!("  {:<12} → {}", spec, semver_demo::explain_version_range(spec));
    }
    println!();

    // ---- 5. cargo doc 文档 ----
    println!("--- 5. cargo doc 生成文档 ---");
    println!("命令:");
    println!("  cargo doc              # 生成文档（含依赖）");
    println!("  cargo doc --open       # 生成并在浏览器中打开");
    println!("  cargo doc --no-deps    # 只生成本 crate 的文档");
    println!("  cargo doc --document-private-items  # 包含私有项");
    println!();

    println!("文档注释语法:");
    println!("  /// 外部文档注释（用于函数、结构体、模块等）");
    println!("  //! 内部文档注释（用于 crate 或模块的开头）");
    println!();

    println!("常用文档段落标题:");
    println!("  # Examples    → 使用示例（会被 doc test 测试）");
    println!("  # Panics      → 何时 panic");
    println!("  # Errors      → 何时返回错误");
    println!("  # Safety      → unsafe 函数的安全条件");
    println!("  # Arguments   → 参数说明");
    println!("  # Returns     → 返回值说明");
    println!();

    // 使用文档注释中的 Calculator
    println!("--- Calculator 文档演示 ---");
    let mut calc = doc_demo::Calculator::new();
    calc.add(100.0);
    calc.subtract(30.0);
    calc.multiply(2.0);
    match calc.divide(7.0) {
        Ok(()) => {}
        Err(e) => println!("错误: {}", e),
    }
    println!("计算: 0 + 100 - 30 × 2 ÷ 7 = {:.2}", calc.result());
    println!("历史: {:?}", calc.history());

    // 测试除零
    match calc.divide(0.0) {
        Ok(()) => println!("成功"),
        Err(e) => println!("除零错误: {}", e),
    }
    println!();

    // ---- 6. docs.rs ----
    println!("--- 6. docs.rs 自动文档托管 ---");
    println!("发布到 crates.io 后，文档自动托管在 docs.rs:");
    println!("  https://docs.rs/<crate-name>/<version>");
    println!("例如:");
    println!("  https://docs.rs/serde/1.0.0");
    println!("  https://docs.rs/tokio/latest");
    println!();

    // ---- 7. 发布检查清单 ----
    println!("--- 7. 发布检查清单 ---");
    print_publish_checklist();
    println!();

    // ---- 总结 ----
    println!("=== 总结 ===");
    println!("1. crates.io 是 Rust 官方包仓库，使用 cargo publish 发布");
    println!("2. Cargo.toml 必须包含 description 和 license");
    println!("3. SemVer: MAJOR.MINOR.PATCH 语义化版本管理");
    println!("4. cargo doc 生成 HTML 文档，支持 Markdown 和 doc test");
    println!("5. 发布前务必完成检查清单中的所有项");
    println!("6. 发布是永久的——谨慎操作！");
}
