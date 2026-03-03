/// # Lesson 073 - 标准输入输出
///
/// 本课学习 Rust 中标准输入输出（stdin/stdout）的使用方法。
///
/// ## 学习目标
/// - 了解 std::io::stdin().read_line() 的用法
/// - 掌握 BufReader 和 BufWriter 的使用
/// - 学会使用 Write trait（write!/writeln!）
/// - 理解 stdout().lock() 如何提升性能
/// - 回顾格式化输出的各种技巧
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson073_stdin_stdout
/// ```

// =============================================================
// Lesson 073: 标准输入输出（stdin / stdout）
// =============================================================

use std::io::{self, BufWriter, Write};

fn main() {
    println!("===== Lesson 073: 标准输入输出 =====\n");

    // ---------------------------------------------------------
    // 1. 标准输入 stdin —— read_line 的用法
    // ---------------------------------------------------------
    // 注意：由于是教学示例，stdin 读取部分用注释说明，
    //       实际运行的代码用 stdout 和字符串处理演示。
    //
    // 标准输入用法示例（交互式场景下可取消注释运行）：
    //
    // ```rust
    // use std::io;
    //
    // let mut input = String::new();
    // println!("请输入你的名字：");
    //
    // // read_line 会将用户输入（包括换行符）追加到 input 中
    // // 返回 Result<usize>，usize 是读取的字节数
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("读取输入失败");
    //
    // // trim() 去掉末尾的换行符
    // let name = input.trim();
    // println!("你好，{}！", name);
    // ```
    //
    // 循环读取多行输入示例：
    //
    // ```rust
    // use std::io::{self, BufRead};
    //
    // let stdin = io::stdin();
    // println!("请逐行输入内容（Ctrl+D / Ctrl+Z 结束）：");
    //
    // // 使用 lock() 获取 StdinLock，避免每次读取都加锁
    // for line in stdin.lock().lines() {
    //     let line = line.expect("读取失败");
    //     println!("你输入了: {}", line);
    // }
    // ```

    println!("--- 1. stdin 用法说明 ---");
    println!("  io::stdin().read_line(&mut buf) 从标准输入读取一行");
    println!("  返回 Result<usize>，包含读取的字节数");
    println!("  读取的字符串末尾包含换行符，通常用 .trim() 去掉");

    // 用字符串模拟 stdin 读取来演示处理逻辑
    let simulated_input = "  Hello, Rust!  \n";
    let trimmed = simulated_input.trim();
    println!("  模拟输入: {:?}", simulated_input);
    println!("  trim 后:  {:?}", trimmed);
    println!();

    // ---------------------------------------------------------
    // 2. 标准输出 stdout —— 基本使用
    // ---------------------------------------------------------
    println!("--- 2. stdout 基本使用 ---");

    // print! 和 println! 是最常用的输出宏
    // 它们实际上是向 stdout 写入
    print!("  print! 不换行...");
    println!(" println! 会换行");

    // 也可以直接使用 stdout 的 write 方法
    let stdout = io::stdout();
    let mut handle = stdout.lock(); // lock() 获取锁，避免多线程竞争

    // write! 和 writeln! 宏需要 use std::io::Write
    // 它们返回 Result，需要处理错误
    write!(handle, "  使用 write! 直接写入 stdout...").unwrap();
    writeln!(handle, " 这里用 writeln! 换行").unwrap();
    writeln!(handle, "  write!/writeln! 返回 Result，需要 unwrap 或处理错误").unwrap();
    println!();

    // ---------------------------------------------------------
    // 3. BufWriter —— 带缓冲的写入
    // ---------------------------------------------------------
    println!("--- 3. BufWriter 带缓冲写入 ---");

    // BufWriter 包装一个 Writer，提供缓冲功能
    // 减少系统调用次数，对大量小写入操作性能提升显著
    {
        let stdout = io::stdout();
        let mut writer = BufWriter::new(stdout.lock());

        // 写入多行内容，BufWriter 会先缓存在内存中
        writeln!(writer, "  BufWriter 第一行：缓冲减少系统调用").unwrap();
        writeln!(writer, "  BufWriter 第二行：适合大量小写入操作").unwrap();
        writeln!(writer, "  BufWriter 第三行：离开作用域时自动 flush").unwrap();

        // 也可以手动 flush，确保数据立即写出
        writer.flush().unwrap();
        writeln!(writer, "  BufWriter 第四行：手动 flush 后继续写入").unwrap();

        // writer 在此处离开作用域，会自动 flush
    }
    println!();

    // ---------------------------------------------------------
    // 4. stdout().lock() 提升性能
    // ---------------------------------------------------------
    println!("--- 4. stdout().lock() 提升性能 ---");

    // 每次调用 println! 内部都会 lock stdout
    // 如果循环中大量输出，频繁加锁解锁会影响性能
    // 解决方案：提前 lock，整个循环只加锁一次
    {
        let stdout = io::stdout();
        let mut lock = stdout.lock();

        // 一次 lock，多次写入，性能更好
        for i in 1..=5 {
            writeln!(lock, "  [locked] 第 {} 次写入", i).unwrap();
        }

        // 对比：每次 println! 都会重新 lock（性能较差）
        // for i in 1..=5 {
        //     println!("  [unlocked] 第 {} 次写入", i); // 每次都加锁解锁
        // }
    }
    println!();

    // ---------------------------------------------------------
    // 5. eprint!/eprintln! —— 标准错误输出
    // ---------------------------------------------------------
    println!("--- 5. 标准错误输出 stderr ---");

    // eprintln! 输出到 stderr，不会被管道重定向
    // 适合输出错误信息和调试信息
    eprintln!("  这条消息输出到 stderr（标准错误输出）");
    println!("  这条消息输出到 stdout（标准输出）");
    println!("  提示：运行 `cargo run 2>/dev/null` 可以隐藏 stderr");
    println!();

    // ---------------------------------------------------------
    // 6. 格式化输出回顾
    // ---------------------------------------------------------
    println!("--- 6. 格式化输出回顾 ---");

    // 6.1 基本占位符 {{}}
    let name = "Rust";
    let version = 2021;
    println!("  {} 语言，edition = {}", name, version);

    // 6.2 位置参数
    println!("  {0} is great, {0} is fast, {0} is safe!", name);

    // 6.3 命名参数
    println!(
        "  {lang} edition {ed}",
        lang = "Rust",
        ed = 2021
    );

    // 6.4 Debug 格式 {:?} 和 美化 Debug {:#?}
    let numbers = vec![1, 2, 3, 4, 5];
    println!("  Debug:        {:?}", numbers);
    println!("  Pretty Debug: {:#?}", numbers);

    // 6.5 数字格式化
    let pi = 3.14159265;
    println!("  小数点后2位:  {:.2}", pi);
    println!("  小数点后4位:  {:.4}", pi);

    // 6.6 宽度和对齐
    println!("  右对齐(10宽): [{:>10}]", "hello");
    println!("  左对齐(10宽): [{:<10}]", "hello");
    println!("  居中(10宽):   [{:^10}]", "hello");
    println!("  填充字符:     [{:*>10}]", "hello");

    // 6.7 进制转换
    let num = 255;
    println!("  十进制:   {}", num);
    println!("  二进制:   {:b}", num);
    println!("  八进制:   {:o}", num);
    println!("  十六进制: {:x} (小写)", num);
    println!("  十六进制: {:X} (大写)", num);
    println!("  带前缀:   {:#b}, {:#o}, {:#x}", num, num, num);

    // 6.8 数字补零
    println!("  补零(5宽): {:05}", 42);
    println!("  补零(8宽): {:08b}", 42);

    // 6.9 用变量控制宽度和精度
    let width = 15;
    let precision = 3;
    println!("  动态宽度:  [{:>width$}]", "dynamic");
    println!("  动态精度:  {:.precision$}", pi);

    // ---------------------------------------------------------
    // 7. 将格式化结果存入字符串
    // ---------------------------------------------------------
    println!("\n--- 7. format! 宏 ---");

    // format! 和 println! 语法相同，但返回 String 而不是打印
    let formatted = format!(
        "{}语言, 精度={:.2}, 十六进制=0x{:X}",
        name, pi, num
    );
    println!("  format! 结果: {}", formatted);

    // 也可以用 write! 写入到实现了 Write trait 的类型
    let mut buf = String::new();
    use std::fmt::Write as FmtWrite; // 注意：这是 std::fmt::Write，不是 std::io::Write
    write!(buf, "写入到 String: {} + {}", 1, 2).unwrap();
    writeln!(buf, " = {}", 3).unwrap();
    println!("  {}", buf);

    println!("\n🎉 恭喜！你已完成 Lesson 073 —— 标准输入输出！");
}
