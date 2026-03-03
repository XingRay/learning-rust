// ============================================================
// Lesson 102: 实战项目 - CLI 工具 (类似 wc 命令)
// ============================================================
//
// 使用 std::env::args 手动解析命令行参数，实现一个类似 Unix `wc` 命令的工具。
//
// 功能：
//   - 统计文件的行数、单词数、字符数、字节数
//   - 支持子命令：count（统计）、info（文件信息）、help（帮助）
//   - 支持选项标志：-l（行数）、-w（单词数）、-c（字符数）、-b（字节数）
//   - 支持多文件统计
//   - 完善的错误处理
//
// 用法示例：
//   lesson102 count file.txt           # 统计所有指标
//   lesson102 count -l -w file.txt     # 只统计行数和单词数
//   lesson102 count file1.txt file2.txt # 多文件统计
//   lesson102 info file.txt            # 显示文件信息
//   lesson102 help                     # 显示帮助

use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process;

// ============================================================
// 1. 命令行参数解析
// ============================================================

/// 子命令类型
#[derive(Debug)]
enum SubCommand {
    /// 统计文件内容
    Count {
        files: Vec<String>,
        show_lines: bool,
        show_words: bool,
        show_chars: bool,
        show_bytes: bool,
    },
    /// 显示文件信息
    Info {
        files: Vec<String>,
    },
    /// 显示帮助
    Help,
    /// 显示版本
    Version,
}

/// 解析错误
#[derive(Debug)]
enum ParseError {
    NoSubCommand,
    UnknownSubCommand(String),
    NoFilesSpecified(String),
    UnknownFlag(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoSubCommand => write!(f, "请指定子命令。使用 'help' 查看帮助。"),
            ParseError::UnknownSubCommand(cmd) => write!(f, "未知子命令: '{}'。使用 'help' 查看帮助。", cmd),
            ParseError::NoFilesSpecified(cmd) => write!(f, "子命令 '{}' 需要至少一个文件参数。", cmd),
            ParseError::UnknownFlag(flag) => write!(f, "未知选项: '{}'", flag),
        }
    }
}

/// 解析命令行参数
fn parse_args(args: Vec<String>) -> Result<SubCommand, ParseError> {
    // args[0] 是程序名，从 args[1] 开始是实际参数
    if args.len() < 2 {
        return Err(ParseError::NoSubCommand);
    }

    let subcmd = &args[1];

    match subcmd.as_str() {
        "help" | "--help" | "-h" => Ok(SubCommand::Help),
        "version" | "--version" | "-V" => Ok(SubCommand::Version),
        "count" => parse_count_args(&args[2..]),
        "info" => parse_info_args(&args[2..]),
        other => Err(ParseError::UnknownSubCommand(other.to_string())),
    }
}

/// 解析 count 子命令的参数
fn parse_count_args(args: &[String]) -> Result<SubCommand, ParseError> {
    let mut show_lines = false;
    let mut show_words = false;
    let mut show_chars = false;
    let mut show_bytes = false;
    let mut files = Vec::new();
    let mut has_flags = false;

    for arg in args {
        if arg.starts_with('-') {
            // 解析标志
            for ch in arg[1..].chars() {
                match ch {
                    'l' => { show_lines = true; has_flags = true; }
                    'w' => { show_words = true; has_flags = true; }
                    'c' => { show_chars = true; has_flags = true; }
                    'b' => { show_bytes = true; has_flags = true; }
                    _ => return Err(ParseError::UnknownFlag(format!("-{}", ch))),
                }
            }
        } else {
            files.push(arg.clone());
        }
    }

    if files.is_empty() {
        return Err(ParseError::NoFilesSpecified("count".to_string()));
    }

    // 如果没有指定任何标志，默认显示所有
    if !has_flags {
        show_lines = true;
        show_words = true;
        show_chars = true;
        show_bytes = true;
    }

    Ok(SubCommand::Count {
        files,
        show_lines,
        show_words,
        show_chars,
        show_bytes,
    })
}

/// 解析 info 子命令的参数
fn parse_info_args(args: &[String]) -> Result<SubCommand, ParseError> {
    let files: Vec<String> = args.iter()
        .filter(|a| !a.starts_with('-'))
        .cloned()
        .collect();

    if files.is_empty() {
        return Err(ParseError::NoFilesSpecified("info".to_string()));
    }

    Ok(SubCommand::Info { files })
}

// ============================================================
// 2. 文件统计功能
// ============================================================

/// 文件统计结果
#[derive(Debug, Default)]
struct FileStats {
    file_name: String,
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

/// 统计文件内容
fn count_file(file_path: &str) -> Result<FileStats, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("无法读取文件 '{}': {}", file_path, e))?;

    let lines = content.lines().count();
    let words = content.split_whitespace().count();
    let chars = content.chars().count();
    let bytes = content.len();

    Ok(FileStats {
        file_name: file_path.to_string(),
        lines,
        words,
        chars,
        bytes,
    })
}

/// 显示统计结果
fn display_stats(
    stats: &FileStats,
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
    show_bytes: bool,
) {
    let mut parts = Vec::new();

    if show_lines {
        parts.push(format!("{:>8} 行", stats.lines));
    }
    if show_words {
        parts.push(format!("{:>8} 词", stats.words));
    }
    if show_chars {
        parts.push(format!("{:>8} 字符", stats.chars));
    }
    if show_bytes {
        parts.push(format!("{:>8} 字节", stats.bytes));
    }

    println!("{}  {}", parts.join(""), stats.file_name);
}

/// 显示多个文件的统计结果和汇总
fn display_multi_stats(
    all_stats: &[FileStats],
    show_lines: bool,
    show_words: bool,
    show_chars: bool,
    show_bytes: bool,
) {
    for stats in all_stats {
        display_stats(stats, show_lines, show_words, show_chars, show_bytes);
    }

    // 如果有多个文件，显示合计
    if all_stats.len() > 1 {
        let total = FileStats {
            file_name: "合计".to_string(),
            lines: all_stats.iter().map(|s| s.lines).sum(),
            words: all_stats.iter().map(|s| s.words).sum(),
            chars: all_stats.iter().map(|s| s.chars).sum(),
            bytes: all_stats.iter().map(|s| s.bytes).sum(),
        };
        println!("{}", "-".repeat(60));
        display_stats(&total, show_lines, show_words, show_chars, show_bytes);
    }
}

// ============================================================
// 3. 文件信息功能
// ============================================================

/// 显示文件信息
fn show_file_info(file_path: &str) -> Result<(), String> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(format!("文件不存在: '{}'", file_path));
    }

    let metadata = fs::metadata(path)
        .map_err(|e| format!("无法获取文件元数据 '{}': {}", file_path, e))?;

    println!("文件: {}", file_path);
    println!("  大小: {} 字节", metadata.len());
    println!("  类型: {}", if metadata.is_file() { "普通文件" }
        else if metadata.is_dir() { "目录" }
        else { "其他" });
    println!("  只读: {}", metadata.permissions().readonly());

    // 如果是文本文件，显示额外信息
    if metadata.is_file() {
        match fs::read_to_string(path) {
            Ok(content) => {
                let line_count = content.lines().count();
                let is_empty = content.is_empty();

                println!("  编码: UTF-8（可读取）");
                println!("  行数: {}", line_count);
                println!("  空文件: {}", is_empty);

                // 显示前几行预览
                if !is_empty {
                    println!("  预览（前 3 行）:");
                    for (i, line) in content.lines().take(3).enumerate() {
                        let display_line = if line.len() > 60 {
                            format!("{}...", &line[..60])
                        } else {
                            line.to_string()
                        };
                        println!("    {}: {}", i + 1, display_line);
                    }
                }
            }
            Err(_) => {
                println!("  编码: 非 UTF-8（二进制文件）");
            }
        }
    }

    Ok(())
}

// ============================================================
// 4. 帮助信息
// ============================================================

fn print_help(program_name: &str) {
    println!("rwc - Rust Word Counter (类似 wc 命令)");
    println!();
    println!("用法:");
    println!("  {} <子命令> [选项] [文件...]", program_name);
    println!();
    println!("子命令:");
    println!("  count    统计文件的行数、单词数、字符数、字节数");
    println!("  info     显示文件的详细信息");
    println!("  help     显示本帮助信息");
    println!("  version  显示版本号");
    println!();
    println!("count 选项:");
    println!("  -l    显示行数");
    println!("  -w    显示单词数");
    println!("  -c    显示字符数");
    println!("  -b    显示字节数");
    println!("  （不指定选项时默认显示所有）");
    println!();
    println!("示例:");
    println!("  {} count file.txt              # 统计所有指标", program_name);
    println!("  {} count -l -w file.txt        # 只统计行数和单词数", program_name);
    println!("  {} count -lw file.txt          # 同上（合并标志）", program_name);
    println!("  {} count f1.txt f2.txt         # 多文件统计", program_name);
    println!("  {} info file.txt               # 显示文件信息", program_name);
}

fn print_version() {
    println!("rwc 0.1.0");
    println!("Rust Word Counter - 使用纯标准库实现的 wc 工具");
}

// ============================================================
// 5. 主函数 —— 执行子命令
// ============================================================

/// 运行 CLI 应用（提取出来方便测试）
fn run(args: Vec<String>) -> Result<(), String> {
    let program_name = args.first()
        .cloned()
        .unwrap_or_else(|| "rwc".to_string());

    let subcmd = parse_args(args).map_err(|e| format!("{}", e))?;

    match subcmd {
        SubCommand::Help => {
            print_help(&program_name);
        }
        SubCommand::Version => {
            print_version();
        }
        SubCommand::Count { files, show_lines, show_words, show_chars, show_bytes } => {
            let mut all_stats = Vec::new();
            let mut errors = Vec::new();

            for file in &files {
                match count_file(file) {
                    Ok(stats) => all_stats.push(stats),
                    Err(e) => errors.push(e),
                }
            }

            // 先显示错误
            for err in &errors {
                eprintln!("错误: {}", err);
            }

            // 再显示统计结果
            if !all_stats.is_empty() {
                display_multi_stats(&all_stats, show_lines, show_words, show_chars, show_bytes);
            }

            if !errors.is_empty() && all_stats.is_empty() {
                return Err("所有文件都无法读取".to_string());
            }
        }
        SubCommand::Info { files } => {
            for (i, file) in files.iter().enumerate() {
                if i > 0 {
                    println!(); // 文件之间空一行
                }
                if let Err(e) = show_file_info(file) {
                    eprintln!("错误: {}", e);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    println!("=== Lesson 102: 实战项目 - CLI 工具 (Rust Word Counter) ===\n");

    // 获取真实的命令行参数
    let real_args: Vec<String> = env::args().collect();
    println!("程序接收到的参数: {:?}\n", real_args);

    // 如果有真实的命令行参数（不只是程序名），执行真实逻辑
    if real_args.len() > 1 {
        println!("--- 执行真实命令行参数 ---\n");
        if let Err(e) = run(real_args) {
            eprintln!("错误: {}", e);
            process::exit(1);
        }
        return;
    }

    // 没有参数时，运行演示模式
    println!("未提供参数，进入演示模式...\n");

    // 创建一个临时测试文件
    let test_content = "Hello, World!\n\
                        这是一个测试文件。\n\
                        Rust 编程语言非常强大。\n\
                        第四行内容。\n\
                        The quick brown fox jumps over the lazy dog.\n";

    let test_file = "test_rwc_demo.txt";
    let test_file2 = "test_rwc_demo2.txt";

    // 写入测试文件
    fs::write(test_file, test_content).expect("写入测试文件失败");
    fs::write(test_file2, "Another file.\nWith two lines.\n").expect("写入测试文件 2 失败");

    // 演示 1: 帮助信息
    println!("========== 演示 1: 帮助信息 ==========\n");
    let _ = run(vec!["rwc".into(), "help".into()]);

    // 演示 2: 版本信息
    println!("\n========== 演示 2: 版本信息 ==========\n");
    let _ = run(vec!["rwc".into(), "version".into()]);

    // 演示 3: 统计单个文件（所有指标）
    println!("\n========== 演示 3: 统计单个文件 ==========\n");
    let _ = run(vec!["rwc".into(), "count".into(), test_file.into()]);

    // 演示 4: 只统计行数和单词数
    println!("\n========== 演示 4: 只统计行数和单词数 ==========\n");
    let _ = run(vec!["rwc".into(), "count".into(), "-lw".into(), test_file.into()]);

    // 演示 5: 多文件统计
    println!("\n========== 演示 5: 多文件统计 ==========\n");
    let _ = run(vec!["rwc".into(), "count".into(), test_file.into(), test_file2.into()]);

    // 演示 6: 文件信息
    println!("\n========== 演示 6: 文件信息 ==========\n");
    let _ = run(vec!["rwc".into(), "info".into(), test_file.into()]);

    // 演示 7: 错误处理 —— 文件不存在
    println!("\n========== 演示 7: 错误处理 ==========\n");
    let result = run(vec!["rwc".into(), "count".into(), "nonexistent.txt".into()]);
    if let Err(e) = result {
        println!("预期的错误: {}", e);
    }

    // 演示 8: 错误处理 —— 无子命令
    println!("\n========== 演示 8: 无子命令 ==========\n");
    let result = run(vec!["rwc".into()]);
    if let Err(e) = result {
        println!("预期的错误: {}", e);
    }

    // 演示 9: 错误处理 —— 未知子命令
    println!("\n========== 演示 9: 未知子命令 ==========\n");
    let result = run(vec!["rwc".into(), "unknown".into()]);
    if let Err(e) = result {
        println!("预期的错误: {}", e);
    }

    // 清理测试文件
    let _ = fs::remove_file(test_file);
    let _ = fs::remove_file(test_file2);

    println!("\n=== CLI 工具实战完成！===");
}
