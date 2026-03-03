/// # Lesson 075 - 路径操作
///
/// 本课学习 Rust 中 Path 和 PathBuf 的使用方法。
///
/// ## 学习目标
/// - 理解 Path 与 PathBuf 的区别（类似 str 与 String）
/// - 掌握路径拼接 push/join
/// - 学会获取 parent/file_name/extension
/// - 使用 exists/is_file/is_dir 检查路径状态
/// - 了解 canonicalize 获取绝对路径
/// - 理解跨平台路径处理（MAIN_SEPARATOR）
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson075_path_operations
/// ```

// =============================================================
// Lesson 075: 路径操作（Path & PathBuf）
// =============================================================

use std::path::{Path, PathBuf, MAIN_SEPARATOR};

fn main() {
    println!("===== Lesson 075: 路径操作 =====\n");

    // ---------------------------------------------------------
    // 1. Path 与 PathBuf 的基本概念
    // ---------------------------------------------------------
    println!("--- 1. Path 与 PathBuf ---");

    // Path 是不可变引用类型（类似 &str）
    // PathBuf 是拥有所有权的类型（类似 String）
    // Path : PathBuf = str : String

    // 从字符串创建 Path（借用）
    let path = Path::new("/usr/local/bin/rustc");
    println!("  Path: {}", path.display());

    // 创建 PathBuf（拥有所有权）
    let mut path_buf = PathBuf::new();
    path_buf.push("usr");
    path_buf.push("local");
    path_buf.push("bin");
    println!("  PathBuf (push): {}", path_buf.display());

    // Path 和 PathBuf 之间的转换
    let path_ref: &Path = path_buf.as_path(); // PathBuf -> &Path
    let path_owned: PathBuf = path_ref.to_path_buf(); // &Path -> PathBuf
    println!("  PathBuf -> &Path: {}", path_ref.display());
    println!("  &Path -> PathBuf: {}", path_owned.display());

    // 从 String / &str 创建
    let from_str = PathBuf::from("/home/user/documents");
    let from_string = PathBuf::from(String::from("/home/user/pictures"));
    println!("  from &str:   {}", from_str.display());
    println!("  from String: {}", from_string.display());
    println!();

    // ---------------------------------------------------------
    // 2. 路径拼接 —— push 和 join
    // ---------------------------------------------------------
    println!("--- 2. 路径拼接 ---");

    // push: 在原有 PathBuf 上追加（可变操作）
    let mut project = PathBuf::from("/home/user");
    println!("  初始路径: {}", project.display());

    project.push("projects");
    project.push("my_app");
    project.push("src");
    println!("  push 后:  {}", project.display());

    // pop: 移除最后一个组件（返回 bool 表示是否成功）
    project.pop();
    println!("  pop 后:   {}", project.display());

    // join: 创建新路径（不修改原路径）
    let base = Path::new("/home/user");
    let full = base.join("projects").join("my_app").join("src/main.rs");
    println!("  join 结果: {}", full.display());
    println!("  原路径不变: {}", base.display());

    // 注意：如果 join 的参数是绝对路径，会替换整个路径
    let replaced = base.join("/etc/config");
    println!("  join 绝对路径: {} (替换了原路径！)", replaced.display());
    println!();

    // ---------------------------------------------------------
    // 3. 路径组件 —— parent / file_name / extension
    // ---------------------------------------------------------
    println!("--- 3. 路径组件 ---");

    let file_path = Path::new("/home/user/documents/report.pdf");

    // parent: 获取父目录（返回 Option<&Path>）
    println!("  完整路径:   {}", file_path.display());
    println!(
        "  parent:     {}",
        file_path.parent().map_or("无".to_string(), |p| p.display().to_string())
    );

    // file_name: 获取文件名（返回 Option<&OsStr>）
    println!(
        "  file_name:  {}",
        file_path.file_name().map_or("无", |n| n.to_str().unwrap_or("无"))
    );

    // file_stem: 获取不带扩展名的文件名
    println!(
        "  file_stem:  {}",
        file_path.file_stem().map_or("无", |n| n.to_str().unwrap_or("无"))
    );

    // extension: 获取扩展名
    println!(
        "  extension:  {}",
        file_path.extension().map_or("无", |n| n.to_str().unwrap_or("无"))
    );
    println!();

    // 多级扩展名的情况
    let tar_gz = Path::new("archive.tar.gz");
    println!("  文件: {}", tar_gz.display());
    println!(
        "  extension: {} (只取最后一个扩展名)",
        tar_gz.extension().unwrap().to_str().unwrap()
    );
    println!(
        "  file_stem: {}",
        tar_gz.file_stem().unwrap().to_str().unwrap()
    );
    println!();

    // ---------------------------------------------------------
    // 4. 修改路径组件
    // ---------------------------------------------------------
    println!("--- 4. 修改路径组件 ---");

    let mut mutable_path = PathBuf::from("/home/user/document.txt");
    println!("  原始路径: {}", mutable_path.display());

    // set_file_name: 修改文件名
    mutable_path.set_file_name("report.doc");
    println!("  set_file_name: {}", mutable_path.display());

    // set_extension: 修改扩展名
    mutable_path.set_extension("pdf");
    println!("  set_extension: {}", mutable_path.display());

    // 移除扩展名
    mutable_path.set_extension("");
    println!("  移除扩展名: {}", mutable_path.display());
    println!();

    // ---------------------------------------------------------
    // 5. 路径遍历 —— components / ancestors
    // ---------------------------------------------------------
    println!("--- 5. 路径遍历 ---");

    let deep_path = Path::new("/home/user/projects/rust/src/main.rs");

    // components: 遍历路径的每个组件
    println!("  路径组件 (components):");
    for component in deep_path.components() {
        println!("    {:?}", component);
    }
    println!();

    // ancestors: 遍历所有祖先路径
    println!("  祖先路径 (ancestors):");
    for ancestor in deep_path.ancestors() {
        println!("    {}", ancestor.display());
    }
    println!();

    // ---------------------------------------------------------
    // 6. 路径检查 —— exists / is_file / is_dir
    // ---------------------------------------------------------
    println!("--- 6. 路径存在性检查 ---");

    // 使用当前目录和临时目录进行测试
    let temp_dir = std::env::temp_dir();
    let current_dir = std::env::current_dir().unwrap_or_default();

    println!("  临时目录: {}", temp_dir.display());
    println!("    exists:  {}", temp_dir.exists());
    println!("    is_dir:  {}", temp_dir.is_dir());
    println!("    is_file: {}", temp_dir.is_file());
    println!();

    println!("  当前目录: {}", current_dir.display());
    println!("    exists:  {}", current_dir.exists());
    println!("    is_dir:  {}", current_dir.is_dir());
    println!();

    let nonexistent = Path::new("/this/path/does/not/exist");
    println!("  不存在的路径: {}", nonexistent.display());
    println!("    exists:  {}", nonexistent.exists());
    println!("    is_file: {}", nonexistent.is_file());
    println!("    is_dir:  {}", nonexistent.is_dir());
    println!();

    // ---------------------------------------------------------
    // 7. canonicalize —— 获取绝对路径
    // ---------------------------------------------------------
    println!("--- 7. canonicalize 绝对路径 ---");

    // canonicalize 将路径解析为绝对路径（解析 . 和 .. 以及符号链接）
    // 注意：路径必须存在，否则返回 Err
    let relative = Path::new(".");
    match relative.canonicalize() {
        Ok(absolute) => {
            println!("  相对路径 '.' 的绝对路径:");
            println!("    {}", absolute.display());
        }
        Err(e) => println!("  canonicalize 失败: {}", e),
    }

    // 使用 .. 的例子
    let parent_relative = Path::new("..");
    match parent_relative.canonicalize() {
        Ok(absolute) => {
            println!("  相对路径 '..' 的绝对路径:");
            println!("    {}", absolute.display());
        }
        Err(e) => println!("  canonicalize 失败: {}", e),
    }
    println!();

    // ---------------------------------------------------------
    // 8. is_absolute / is_relative
    // ---------------------------------------------------------
    println!("--- 8. 绝对路径与相对路径判断 ---");

    let abs_path = Path::new("/usr/local/bin");
    let rel_path = Path::new("src/main.rs");

    println!("  路径: {}", abs_path.display());
    println!("    is_absolute: {}", abs_path.is_absolute());
    println!("    is_relative: {}", abs_path.is_relative());

    println!("  路径: {}", rel_path.display());
    println!("    is_absolute: {}", rel_path.is_absolute());
    println!("    is_relative: {}", rel_path.is_relative());
    println!();

    // ---------------------------------------------------------
    // 9. 跨平台路径 —— MAIN_SEPARATOR
    // ---------------------------------------------------------
    println!("--- 9. 跨平台路径 ---");

    // MAIN_SEPARATOR 是当前操作系统的路径分隔符
    // Windows: '\\', Unix/macOS: '/'
    println!("  当前系统路径分隔符: '{}'", MAIN_SEPARATOR);

    // std::path::MAIN_SEPARATOR_STR 是字符串版本（Rust 1.68+）
    println!("  MAIN_SEPARATOR_STR: \"{}\"", std::path::MAIN_SEPARATOR_STR);

    // Path 会自动处理跨平台路径，推荐使用 Path/PathBuf
    // 而不是手动拼接字符串
    let cross_platform = Path::new("src").join("module").join("file.rs");
    println!("  跨平台路径: {}", cross_platform.display());

    // has_root: 检查路径是否有根
    println!("  '/' has_root:     {}", Path::new("/").has_root());
    println!("  'src' has_root:   {}", Path::new("src").has_root());
    println!();

    // ---------------------------------------------------------
    // 10. 路径与字符串的转换
    // ---------------------------------------------------------
    println!("--- 10. 路径与字符串转换 ---");

    let path = Path::new("/home/user/文档/笔记.md");

    // Path -> &str（可能失败，因为路径可能不是有效 UTF-8）
    match path.to_str() {
        Some(s) => println!("  to_str:     {:?}", s),
        None => println!("  to_str:     路径不是有效 UTF-8"),
    }

    // Path -> String（有损转换，非 UTF-8 字符替换为 U+FFFD）
    let string = path.to_string_lossy();
    println!("  to_string_lossy: {:?}", string);

    // Path -> OsStr
    let os_str = path.as_os_str();
    println!("  as_os_str:  {:?}", os_str);

    // display() 返回可以直接 format 的对象
    println!("  display:    {}", path.display());

    // String / &str -> PathBuf / &Path
    let from_str: &Path = Path::new("/some/path");
    let from_string: PathBuf = PathBuf::from("/some/path".to_string());
    println!("  &str -> &Path:     {}", from_str.display());
    println!("  String -> PathBuf: {}", from_string.display());

    // ---------------------------------------------------------
    // 11. starts_with / ends_with / strip_prefix
    // ---------------------------------------------------------
    println!("\n--- 11. 路径比较与前缀操作 ---");

    let full = Path::new("/home/user/projects/rust/src/main.rs");

    println!("  路径: {}", full.display());
    println!(
        "  starts_with '/home/user': {}",
        full.starts_with("/home/user")
    );
    println!(
        "  ends_with 'src/main.rs':  {}",
        full.ends_with("src/main.rs")
    );

    // strip_prefix: 移除路径前缀
    match full.strip_prefix("/home/user") {
        Ok(relative) => println!("  strip_prefix:  {}", relative.display()),
        Err(e) => println!("  strip_prefix 失败: {}", e),
    }

    // with_file_name / with_extension
    let changed = full.with_file_name("lib.rs");
    println!("  with_file_name('lib.rs'): {}", changed.display());

    let changed_ext = full.with_extension("txt");
    println!("  with_extension('txt'):    {}", changed_ext.display());

    println!("\n🎉 恭喜！你已完成 Lesson 075 —— 路径操作！");
}
