/// # Lesson 076 - 目录遍历
///
/// 本课学习 Rust 中目录的遍历和文件系统操作。
///
/// ## 学习目标
/// - 掌握 fs::read_dir 遍历目录
/// - 了解 DirEntry 的各种方法
/// - 实现递归遍历目录（手动实现）
/// - 使用 fs::create_dir_all 和 fs::remove_dir_all
/// - 获取文件 metadata（大小、修改时间等）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson076_directory_traversal
/// ```

// =============================================================
// Lesson 076: 目录遍历（Directory Traversal）
// =============================================================

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    println!("===== Lesson 076: 目录遍历 =====\n");

    // 创建演示用的临时目录结构
    let base_dir = std::env::temp_dir().join("lesson076_demo");
    setup_demo_directory(&base_dir)?;
    println!("  ✅ 演示目录已创建: {}\n", base_dir.display());

    // ---------------------------------------------------------
    // 1. fs::read_dir —— 遍历目录
    // ---------------------------------------------------------
    println!("--- 1. fs::read_dir 遍历目录 ---");

    // read_dir 返回 Result<ReadDir>
    // ReadDir 是一个迭代器，产出 Result<DirEntry>
    let entries = fs::read_dir(&base_dir)?;

    println!("  {} 的直接子项:", base_dir.display());
    for entry in entries {
        let entry = entry?; // 每个 entry 也是 Result
        println!("    {}", entry.file_name().to_string_lossy());
    }
    println!();

    // ---------------------------------------------------------
    // 2. DirEntry 的方法
    // ---------------------------------------------------------
    println!("--- 2. DirEntry 的方法 ---");

    for entry in fs::read_dir(&base_dir)? {
        let entry = entry?;

        // path(): 获取完整路径
        let path = entry.path();

        // file_name(): 获取文件名（OsString）
        let name = entry.file_name();

        // file_type(): 获取文件类型
        let file_type = entry.file_type()?;

        // metadata(): 获取文件元数据
        let metadata = entry.metadata()?;

        let type_str = if file_type.is_dir() {
            "📁 目录"
        } else if file_type.is_file() {
            "📄 文件"
        } else if file_type.is_symlink() {
            "🔗 符号链接"
        } else {
            "❓ 其他"
        };

        println!(
            "  {} {:20} 大小: {:>6} 字节  路径: {}",
            type_str,
            name.to_string_lossy(),
            metadata.len(),
            path.display()
        );
    }
    println!();

    // ---------------------------------------------------------
    // 3. 过滤和排序目录内容
    // ---------------------------------------------------------
    println!("--- 3. 过滤和排序 ---");

    // 只列出 .txt 文件
    let mut txt_files: Vec<_> = fs::read_dir(&base_dir)?
        .filter_map(|entry| entry.ok()) // 过滤掉错误
        .filter(|entry| {
            entry.path().extension().map_or(false, |ext| ext == "txt")
        })
        .collect();

    // 按文件名排序
    txt_files.sort_by_key(|entry| entry.file_name());

    println!("  .txt 文件（排序后）:");
    for entry in &txt_files {
        println!("    {}", entry.file_name().to_string_lossy());
    }
    println!();

    // 分离文件和目录
    let (dirs, files): (Vec<_>, Vec<_>) = fs::read_dir(&base_dir)?
        .filter_map(|e| e.ok())
        .partition(|e| e.file_type().map_or(false, |ft| ft.is_dir()));

    println!("  目录数量: {}", dirs.len());
    println!("  文件数量: {}", files.len());
    println!();

    // ---------------------------------------------------------
    // 4. 递归遍历目录（手动实现）
    // ---------------------------------------------------------
    println!("--- 4. 递归遍历目录 ---");

    println!("  完整目录树:");
    visit_dir(&base_dir, 0)?;
    println!();

    // ---------------------------------------------------------
    // 5. 递归遍历 —— 收集所有文件
    // ---------------------------------------------------------
    println!("--- 5. 收集所有文件 ---");

    let mut all_files = Vec::new();
    collect_files(&base_dir, &mut all_files)?;

    println!("  递归发现的所有文件:");
    for file in &all_files {
        // 显示相对路径
        if let Ok(relative) = file.strip_prefix(&base_dir) {
            println!("    {}", relative.display());
        }
    }
    println!("  共 {} 个文件", all_files.len());
    println!();

    // ---------------------------------------------------------
    // 6. fs::create_dir_all —— 递归创建目录
    // ---------------------------------------------------------
    println!("--- 6. fs::create_dir_all ---");

    let nested_dir = base_dir.join("deep").join("nested").join("path");

    // create_dir_all 递归创建所有不存在的中间目录
    // 类似 mkdir -p
    fs::create_dir_all(&nested_dir)?;
    println!("  ✅ 创建嵌套目录: {}", nested_dir.display());
    println!("  存在: {}", nested_dir.exists());

    // fs::create_dir 只创建一级目录，父目录必须存在
    // 如果目录已存在，create_dir 会报错，create_dir_all 不会
    match fs::create_dir(&nested_dir) {
        Ok(_) => println!("  不应该到这里"),
        Err(e) => println!("  ⚠ create_dir 对已存在目录报错: {}", e),
    }

    // create_dir_all 对已存在目录不报错
    fs::create_dir_all(&nested_dir)?;
    println!("  ✅ create_dir_all 对已存在目录不报错");
    println!();

    // ---------------------------------------------------------
    // 7. metadata —— 文件元数据
    // ---------------------------------------------------------
    println!("--- 7. metadata 文件元数据 ---");

    let sample_file = base_dir.join("hello.txt");

    // fs::metadata 获取文件元信息（不跟随符号链接用 symlink_metadata）
    let meta = fs::metadata(&sample_file)?;

    println!("  文件: {}", sample_file.display());
    println!("  大小: {} 字节", meta.len());
    println!("  是文件: {}", meta.is_file());
    println!("  是目录: {}", meta.is_dir());
    println!("  只读: {}", meta.permissions().readonly());

    // 修改时间
    if let Ok(modified) = meta.modified() {
        let duration = SystemTime::now()
            .duration_since(modified)
            .unwrap_or_default();
        println!("  修改时间: {:.1} 秒前", duration.as_secs_f64());
    }

    // 访问时间
    if let Ok(accessed) = meta.accessed() {
        let duration = SystemTime::now()
            .duration_since(accessed)
            .unwrap_or_default();
        println!("  访问时间: {:.1} 秒前", duration.as_secs_f64());
    }

    // 创建时间（部分系统可能不支持）
    match meta.created() {
        Ok(created) => {
            let duration = SystemTime::now()
                .duration_since(created)
                .unwrap_or_default();
            println!("  创建时间: {:.1} 秒前", duration.as_secs_f64());
        }
        Err(_) => println!("  创建时间: 不支持"),
    }
    println!();

    // ---------------------------------------------------------
    // 8. 计算目录总大小
    // ---------------------------------------------------------
    println!("--- 8. 计算目录总大小 ---");

    let total_size = dir_size(&base_dir)?;
    println!("  目录 {} 的总大小: {} 字节", base_dir.display(), total_size);
    println!();

    // ---------------------------------------------------------
    // 9. fs::rename —— 重命名/移动文件
    // ---------------------------------------------------------
    println!("--- 9. fs::rename 重命名 ---");

    let old_name = base_dir.join("hello.txt");
    let new_name = base_dir.join("hello_renamed.txt");

    fs::rename(&old_name, &new_name)?;
    println!("  ✅ 重命名: {} -> {}", old_name.display(), new_name.display());
    println!("  旧文件存在: {}", old_name.exists());
    println!("  新文件存在: {}", new_name.exists());

    // 改回来，以便后续清理正常
    fs::rename(&new_name, &old_name)?;
    println!();

    // ---------------------------------------------------------
    // 10. fs::copy —— 复制文件
    // ---------------------------------------------------------
    println!("--- 10. fs::copy 复制文件 ---");

    let src = base_dir.join("hello.txt");
    let dst = base_dir.join("hello_copy.txt");

    // fs::copy 返回复制的字节数
    let bytes_copied = fs::copy(&src, &dst)?;
    println!("  ✅ 复制 {} 字节: {} -> {}", bytes_copied, src.display(), dst.display());
    println!();

    // ---------------------------------------------------------
    // 11. fs::remove_dir_all —— 递归删除目录
    // ---------------------------------------------------------
    println!("--- 11. 清理 - fs::remove_dir_all ---");

    // remove_dir_all 递归删除目录及其所有内容（类似 rm -rf）
    // ⚠ 危险操作！请确保路径正确
    fs::remove_dir_all(&base_dir)?;
    println!("  ✅ 已递归删除: {}", base_dir.display());
    println!("  目录存在: {}", base_dir.exists());

    // remove_dir 只能删除空目录
    // remove_file 只能删除文件

    println!("\n🎉 恭喜！你已完成 Lesson 076 —— 目录遍历！");
    Ok(())
}

// =============================================================
// 辅助函数
// =============================================================

/// 创建演示用的目录结构
fn setup_demo_directory(base: &Path) -> io::Result<()> {
    // 如果已存在先清理
    if base.exists() {
        fs::remove_dir_all(base)?;
    }

    // 创建目录结构
    fs::create_dir_all(base.join("src"))?;
    fs::create_dir_all(base.join("docs"))?;
    fs::create_dir_all(base.join("tests").join("unit"))?;

    // 创建一些文件
    File::create(base.join("hello.txt"))?.write_all(b"Hello, World!\n")?;
    File::create(base.join("notes.txt"))?.write_all(b"Some notes here.\n")?;
    File::create(base.join("data.csv"))?.write_all(b"name,age\nAlice,30\nBob,25\n")?;
    File::create(base.join("src").join("main.rs"))?.write_all(b"fn main() {}\n")?;
    File::create(base.join("src").join("lib.rs"))?.write_all(b"pub fn hello() {}\n")?;
    File::create(base.join("docs").join("README.md"))?.write_all(b"# My Project\n")?;
    File::create(base.join("tests").join("unit").join("test1.rs"))?
        .write_all(b"#[test]\nfn test_one() {}\n")?;

    Ok(())
}

/// 递归遍历目录并打印目录树
fn visit_dir(dir: &Path, depth: usize) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let indent = "  ".repeat(depth + 1);

    // 收集并排序条目（目录优先，然后按名称排序）
    let mut entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(|e| e.ok())
        .collect();

    entries.sort_by(|a, b| {
        let a_is_dir = a.file_type().map_or(false, |ft| ft.is_dir());
        let b_is_dir = b.file_type().map_or(false, |ft| ft.is_dir());
        // 目录优先排在前面
        b_is_dir.cmp(&a_is_dir).then(a.file_name().cmp(&b.file_name()))
    });

    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if entry.file_type()?.is_dir() {
            println!("{}📁 {}/", indent, name_str);
            visit_dir(&entry.path(), depth + 1)?; // 递归
        } else {
            let size = entry.metadata()?.len();
            println!("{}📄 {} ({} 字节)", indent, name_str, size);
        }
    }

    Ok(())
}

/// 递归收集目录中的所有文件路径
fn collect_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files(&path, files)?; // 递归进入子目录
        } else {
            files.push(path);
        }
    }

    Ok(())
}

/// 递归计算目录的总大小
fn dir_size(path: &Path) -> io::Result<u64> {
    let mut total = 0;

    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                total += dir_size(&entry_path)?;
            } else {
                total += entry.metadata()?.len();
            }
        }
    } else {
        total = fs::metadata(path)?.len();
    }

    Ok(total)
}
