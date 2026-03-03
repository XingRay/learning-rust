/// # Lesson 074 - 文件读写
///
/// 本课学习 Rust 中文件的读取和写入操作。
///
/// ## 学习目标
/// - 掌握 File::open 和 File::create 的用法
/// - 了解 fs::read_to_string 的便捷读取方式
/// - 学会使用 BufReader 逐行读取文件
/// - 掌握 write!/writeln! 写入文件
/// - 了解 fs::write 便捷写入
/// - 学会使用 OpenOptions 进行追加模式写入
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson074_file_read_write
/// ```

// =============================================================
// Lesson 074: 文件读写（File Read & Write）
// =============================================================

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

/// 获取临时目录下的文件路径
fn temp_file(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}

fn main() -> io::Result<()> {
    println!("===== Lesson 074: 文件读写 =====\n");

    // ---------------------------------------------------------
    // 1. File::create 和 write! —— 创建并写入文件
    // ---------------------------------------------------------
    println!("--- 1. File::create + write!/writeln! ---");

    let file_path = temp_file("lesson074_example.txt");
    println!("  文件路径: {}", file_path.display());

    {
        // File::create 创建文件（如果已存在则清空内容）
        // 返回 Result<File>
        let mut file = File::create(&file_path)?;

        // 使用 write! 和 writeln! 向文件写入内容
        // File 实现了 std::io::Write trait
        writeln!(file, "第一行：Hello, Rust!")?;
        writeln!(file, "第二行：文件读写学习")?;
        writeln!(file, "第三行：这是第三行内容")?;
        write!(file, "第四行：最后一行（无换行）")?;

        println!("  ✅ 文件写入完成");
        // file 在这里离开作用域，自动关闭（Drop trait）
    }
    println!();

    // ---------------------------------------------------------
    // 2. File::open + read_to_string —— 手动读取
    // ---------------------------------------------------------
    println!("--- 2. File::open + 手动读取 ---");

    {
        use std::io::Read;

        let mut file = File::open(&file_path)?;
        let mut contents = String::new();

        // read_to_string 将整个文件内容读入字符串
        file.read_to_string(&mut contents)?;

        println!("  文件内容（手动读取）:");
        for line in contents.lines() {
            println!("    | {}", line);
        }
    }
    println!();

    // ---------------------------------------------------------
    // 3. fs::read_to_string —— 便捷读取
    // ---------------------------------------------------------
    println!("--- 3. fs::read_to_string 便捷读取 ---");

    // fs::read_to_string 是最简洁的读取方式
    // 一行代码就完成：打开文件 -> 读取全部内容 -> 关闭文件
    let contents = fs::read_to_string(&file_path)?;
    println!("  文件内容（便捷读取）:");
    for (i, line) in contents.lines().enumerate() {
        println!("    行{}: {}", i + 1, line);
    }
    println!();

    // ---------------------------------------------------------
    // 4. BufReader —— 逐行读取（适合大文件）
    // ---------------------------------------------------------
    println!("--- 4. BufReader 逐行读取 ---");

    // 对于大文件，一次性读入内存可能不现实
    // BufReader 带缓冲的读取器，配合 lines() 迭代器逐行读取
    {
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);

        println!("  逐行读取:");
        // lines() 返回 Lines 迭代器，每次 yield 一行（自动去掉换行符）
        for (i, line) in reader.lines().enumerate() {
            let line = line?; // lines() 返回的每一行都是 Result<String>
            println!("    [{:2}] {}", i + 1, line);
        }
    }
    println!();

    // ---------------------------------------------------------
    // 5. BufReader 的其他读取方式
    // ---------------------------------------------------------
    println!("--- 5. BufReader 的 read_line 方法 ---");

    {
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);

        // read_line 读取一行到已有 String 中（保留换行符）
        let mut first_line = String::new();
        let bytes_read = reader.read_line(&mut first_line)?;
        println!("  第一行: {:?}（{} 字节）", first_line.trim(), bytes_read);

        let mut second_line = String::new();
        reader.read_line(&mut second_line)?;
        println!("  第二行: {:?}", second_line.trim());
    }
    println!();

    // ---------------------------------------------------------
    // 6. fs::write —— 便捷写入
    // ---------------------------------------------------------
    println!("--- 6. fs::write 便捷写入 ---");

    let file_path2 = temp_file("lesson074_quick.txt");

    // fs::write 一行代码完成写入（如果文件存在则覆盖）
    fs::write(&file_path2, "快速写入的内容\n第二行\n第三行\n")?;
    println!("  ✅ fs::write 写入完成: {}", file_path2.display());

    // 验证写入
    let verify = fs::read_to_string(&file_path2)?;
    println!("  验证内容: {:?}", verify.trim());
    println!();

    // ---------------------------------------------------------
    // 7. fs::write 写入字节数据
    // ---------------------------------------------------------
    println!("--- 7. 写入字节数据 ---");

    let bytes_path = temp_file("lesson074_bytes.bin");

    // fs::write 也可以写入 &[u8] 字节数据
    let data: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello" 的 ASCII
    fs::write(&bytes_path, &data)?;

    // fs::read 读取为 Vec<u8>
    let read_bytes = fs::read(&bytes_path)?;
    println!("  写入字节: {:?}", data);
    println!("  读取字节: {:?}", read_bytes);
    println!("  转为字符串: {}", String::from_utf8_lossy(&read_bytes));
    println!();

    // ---------------------------------------------------------
    // 8. OpenOptions —— 追加模式
    // ---------------------------------------------------------
    println!("--- 8. OpenOptions 追加模式 ---");

    let append_path = temp_file("lesson074_append.txt");

    // 先创建并写入初始内容
    fs::write(&append_path, "初始内容\n")?;

    {
        // OpenOptions 提供细粒度的文件打开控制
        let mut file = OpenOptions::new()
            .write(true)   // 允许写入
            .append(true)  // 追加模式（不清空原有内容）
            .open(&append_path)?;

        writeln!(file, "追加的第一行")?;
        writeln!(file, "追加的第二行")?;
    }

    let result = fs::read_to_string(&append_path)?;
    println!("  追加写入后的完整内容:");
    for line in result.lines() {
        println!("    | {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 9. OpenOptions —— 更多选项
    // ---------------------------------------------------------
    println!("--- 9. OpenOptions 更多选项 ---");

    let opts_path = temp_file("lesson074_options.txt");

    {
        // create(true): 文件不存在则创建
        // truncate(true): 如果文件存在则清空
        // write(true): 允许写入
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&opts_path)?;

        writeln!(file, "OpenOptions 创建的文件")?;
        writeln!(file, "write + create + truncate")?;
    }

    // create_new: 文件必须不存在，否则返回错误
    let new_path = temp_file("lesson074_must_new.txt");
    // 先确保文件不存在
    let _ = fs::remove_file(&new_path);
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true) // 文件必须不存在
            .open(&new_path)?;

        writeln!(file, "这个文件是 create_new 创建的")?;
    }

    // 再次尝试 create_new 会失败
    let result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&new_path);

    match result {
        Ok(_) => println!("  不应该到这里"),
        Err(e) => println!("  ✅ create_new 再次打开已存在文件报错: {}", e),
    }
    println!();

    // ---------------------------------------------------------
    // 10. 读写模式同时打开
    // ---------------------------------------------------------
    println!("--- 10. 读写模式 ---");

    let rw_path = temp_file("lesson074_readwrite.txt");
    fs::write(&rw_path, "原始内容ABC\n")?;

    {
        use std::io::{Read, Seek, SeekFrom};

        // 同时以读和写模式打开
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&rw_path)?;

        // 先读取
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        println!("  读取: {:?}", contents.trim());

        // Seek 到文件末尾追加
        file.seek(SeekFrom::End(0))?;
        writeln!(file, "追加的新行")?;
    }

    let final_content = fs::read_to_string(&rw_path)?;
    println!("  最终内容:");
    for line in final_content.lines() {
        println!("    | {}", line);
    }
    println!();

    // ---------------------------------------------------------
    // 清理临时文件
    // ---------------------------------------------------------
    println!("--- 清理临时文件 ---");
    for path in &[
        &file_path,
        &file_path2,
        &bytes_path,
        &append_path,
        &opts_path,
        &new_path,
        &rw_path,
    ] {
        if path.exists() {
            fs::remove_file(path)?;
            println!("  已删除: {}", path.display());
        }
    }

    println!("\n🎉 恭喜！你已完成 Lesson 074 —— 文件读写！");
    Ok(())
}
