// ============================================================
// Lesson 070: Cargo Features (特性标志)
// ============================================================
// Cargo features 允许在编译时选择性地启用/禁用功能。
// 这对于：
//   - 减少编译时间和二进制体积
//   - 提供可选功能
//   - 条件编译不同实现
// 非常有用。
// ============================================================

// ============================================================
// 1. #[cfg(feature = "xxx")] 条件编译
// ============================================================
// 使用 #[cfg(feature = "xxx")] 属性，可以根据 feature 标志
// 决定是否编译某段代码。
//
// 在本项目的 Cargo.toml 中定义了以下 features：
// [features]
// default = ["greeting"]     # 默认启用 greeting
// greeting = []              # 问候功能
// advanced_math = []         # 高级数学功能
// logging = []               # 日志功能
// full = ["greeting", "advanced_math", "logging"]  # 全部功能

// ---- 条件编译函数 ----

/// 仅在启用 "greeting" feature 时编译此函数
#[cfg(feature = "greeting")]
fn greet(name: &str) -> String {
    format!("你好, {}! 欢迎学习 Rust Cargo Features!", name)
}

/// 当未启用 "greeting" feature 时的替代实现
#[cfg(not(feature = "greeting"))]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// 仅在启用 "advanced_math" feature 时编译
#[cfg(feature = "advanced_math")]
mod advanced_math {
    /// 计算阶乘
    pub fn factorial(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            n * factorial(n - 1)
        }
    }

    /// 计算斐波那契数列第 n 项
    pub fn fibonacci(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => {
                let mut a: u64 = 0;
                let mut b: u64 = 1;
                for _ in 2..=n {
                    let temp = b;
                    b = a + b;
                    a = temp;
                }
                b
            }
        }
    }

    /// 判断素数
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 || n == 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    /// 最大公约数
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

/// 仅在启用 "logging" feature 时编译
#[cfg(feature = "logging")]
mod logging {
    use std::time::SystemTime;

    pub enum Level {
        Info,
        Warn,
        Error,
        Debug,
    }

    impl Level {
        pub fn as_str(&self) -> &'static str {
            match self {
                Level::Info => "INFO",
                Level::Warn => "WARN",
                Level::Error => "ERROR",
                Level::Debug => "DEBUG",
            }
        }
    }

    pub fn log(level: Level, message: &str) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("[{} | {}] {}", timestamp, level.as_str(), message);
    }
}

// ============================================================
// 2. cfg! 宏（运行时判断）
// ============================================================
// cfg! 宏在编译时求值，返回 bool，可用于运行时分支
// 与 #[cfg(...)] 不同，cfg! 不会排除代码，只是返回 true/false
// 因此所有分支的代码都必须能编译通过

fn show_enabled_features() {
    println!("当前启用的 Features:");

    // cfg! 宏返回 bool
    if cfg!(feature = "greeting") {
        println!("  ✓ greeting (已启用)");
    } else {
        println!("  ✗ greeting (未启用)");
    }

    if cfg!(feature = "advanced_math") {
        println!("  ✓ advanced_math (已启用)");
    } else {
        println!("  ✗ advanced_math (未启用)");
    }

    if cfg!(feature = "logging") {
        println!("  ✓ logging (已启用)");
    } else {
        println!("  ✗ logging (未启用)");
    }

    if cfg!(feature = "full") {
        println!("  ✓ full (已启用)");
    } else {
        println!("  ✗ full (未启用)");
    }
}

// ============================================================
// 3. 条件编译结构体字段和 impl 块
// ============================================================

struct AppConfig {
    name: String,
    version: String,
    // 仅在启用 logging feature 时才有 log_level 字段
    #[cfg(feature = "logging")]
    log_level: String,
}

impl AppConfig {
    fn new(name: &str) -> Self {
        AppConfig {
            name: name.to_string(),
            version: String::from("0.1.0"),
            #[cfg(feature = "logging")]
            log_level: String::from("INFO"),
        }
    }

    fn display(&self) {
        println!("应用名: {}", self.name);
        println!("版本: {}", self.version);
        #[cfg(feature = "logging")]
        println!("日志级别: {}", self.log_level);
    }
}

// ============================================================
// 4. 其他常用 cfg 条件
// ============================================================
// 除了 feature，cfg 还支持很多其他条件：
//
// #[cfg(target_os = "linux")]       // 操作系统
// #[cfg(target_os = "windows")]
// #[cfg(target_os = "macos")]
// #[cfg(target_arch = "x86_64")]    // CPU 架构
// #[cfg(target_arch = "aarch64")]
// #[cfg(debug_assertions)]          // debug 模式
// #[cfg(test)]                      // 测试模式
// #[cfg(unix)]                      // Unix 系统
// #[cfg(windows)]                   // Windows 系统
//
// 组合条件：
// #[cfg(all(feature = "a", feature = "b"))]    // a AND b
// #[cfg(any(feature = "a", feature = "b"))]    // a OR b
// #[cfg(not(feature = "a"))]                   // NOT a

fn platform_info() -> String {
    let os = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "其他操作系统"
    };

    let mode = if cfg!(debug_assertions) {
        "Debug"
    } else {
        "Release"
    };

    format!("平台: {}, 编译模式: {}", os, mode)
}

// ============================================================
// 5. 可选依赖与 Feature 绑定（注释说明）
// ============================================================
//
// 在实际项目中，features 常与可选依赖结合使用：
//
// [dependencies]
// serde = { version = "1.0", optional = true }
// serde_json = { version = "1.0", optional = true }
// tokio = { version = "1", optional = true, features = ["full"] }
//
// [features]
// default = []
// json = ["serde", "serde_json"]      # 启用 json feature 会自动引入 serde 和 serde_json
// async = ["tokio"]                    # 启用 async feature 会自动引入 tokio
// full = ["json", "async"]            # 启用所有功能
//
// 在代码中：
// #[cfg(feature = "json")]
// use serde::{Serialize, Deserialize};
//
// #[cfg(feature = "json")]
// #[derive(Serialize, Deserialize)]
// struct Config { ... }
//
// 这样用户可以按需选择功能，不需要 json 的项目不会编译 serde。

fn main() {
    println!("=== Lesson 070: Cargo Features ===\n");

    // ---- 1. 查看启用的 features ----
    println!("--- 1. 查看启用的 Features ---");
    show_enabled_features();
    println!();

    // ---- 2. 条件编译函数 ----
    println!("--- 2. 条件编译函数 ---");
    println!("{}", greet("Rustacean"));
    // 根据是否启用 greeting feature，会调用不同的实现
    println!("(greeting feature {} 启用)",
        if cfg!(feature = "greeting") { "已" } else { "未" });
    println!();

    // ---- 3. 条件编译模块 ----
    println!("--- 3. 条件编译模块 ---");

    #[cfg(feature = "advanced_math")]
    {
        println!("advanced_math 模块已启用:");
        println!("  5! = {}", advanced_math::factorial(5));
        println!("  fibonacci(10) = {}", advanced_math::fibonacci(10));
        println!("  is_prime(17) = {}", advanced_math::is_prime(17));
        println!("  is_prime(18) = {}", advanced_math::is_prime(18));
        println!("  gcd(48, 18) = {}", advanced_math::gcd(48, 18));
    }

    #[cfg(not(feature = "advanced_math"))]
    println!("advanced_math 模块未启用 (使用 --features advanced_math 来启用)");
    println!();

    // ---- 4. 条件编译与日志 ----
    println!("--- 4. 条件编译日志模块 ---");

    #[cfg(feature = "logging")]
    {
        println!("logging 模块已启用:");
        logging::log(logging::Level::Info, "应用启动");
        logging::log(logging::Level::Debug, "正在加载配置...");
        logging::log(logging::Level::Warn, "配置文件未找到，使用默认值");
        logging::log(logging::Level::Error, "这是一个错误示例");
    }

    #[cfg(not(feature = "logging"))]
    println!("logging 模块未启用 (使用 --features logging 来启用)");
    println!();

    // ---- 5. cfg! 宏 vs #[cfg] ----
    println!("--- 5. cfg! 宏 vs #[cfg] 属性 ---");
    println!("#[cfg(...)]:  编译时排除代码（代码不存在于二进制文件中）");
    println!("cfg!(...):    编译时求值为 bool（所有分支代码都会编译）");
    println!();

    // cfg! 宏示例
    let feature_count = [
        cfg!(feature = "greeting"),
        cfg!(feature = "advanced_math"),
        cfg!(feature = "logging"),
    ]
    .iter()
    .filter(|&&x| x)
    .count();
    println!("当前启用了 {} 个 feature", feature_count);
    println!();

    // ---- 6. 条件编译配置 ----
    println!("--- 6. 条件编译配置 ---");
    let config = AppConfig::new("FeatureDemo");
    config.display();
    println!();

    // ---- 7. 平台信息 ----
    println!("--- 7. 平台条件编译 ---");
    println!("{}", platform_info());
    println!();

    // ---- 8. 使用方式说明 ----
    println!("--- 8. 使用方式说明 ---");
    println!("编译/运行命令:");
    println!("  cargo run                               # 使用默认 features (greeting)");
    println!("  cargo run --features \"advanced_math\"     # 额外启用 advanced_math");
    println!("  cargo run --features \"logging\"           # 额外启用 logging");
    println!("  cargo run --features \"full\"              # 启用所有 features");
    println!("  cargo run --no-default-features          # 禁用所有默认 features");
    println!("  cargo run --all-features                 # 启用所有 features");
    println!();

    // ---- 总结 ----
    println!("=== 总结 ===");
    println!("1. 在 Cargo.toml 的 [features] 中定义 feature 标志");
    println!("2. #[cfg(feature = \"xxx\")] 条件编译——代码不存在于二进制中");
    println!("3. cfg!(feature = \"xxx\") 宏——编译时返回 bool");
    println!("4. default features 默认启用，可用 --no-default-features 关闭");
    println!("5. 可选依赖 (optional = true) 可绑定到 feature");
    println!("6. cfg 还支持 target_os, debug_assertions, test 等条件");
}
