// ============================================================
// Lesson 079: TCP 服务器
// ============================================================
// 本课学习如何使用 Rust 标准库创建 TCP 服务器：
// - TcpListener::bind 绑定地址
// - accept() 接受客户端连接
// - 读写 TcpStream
// - 多线程处理并发连接
// - 实现一个简单的 echo server
//
// 启动方式（取消 main 中的注释即可运行）：
//   cargo run
// 然后用 telnet 或 netcat 测试：
//   telnet 127.0.0.1 7878
//   或 echo "hello" | nc 127.0.0.1 7878
// ============================================================

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

// ======================
// 1. 最简单的 TCP 服务器
// ======================
// TcpListener::bind 绑定到指定的地址和端口
// accept() 阻塞等待客户端连接，返回 (TcpStream, SocketAddr)
fn simple_server() {
    println!("=== 简单 TCP 服务器 ===");

    // bind 返回 Result<TcpListener>
    // 常见绑定地址：
    //   "127.0.0.1:7878" - 仅本地访问
    //   "0.0.0.0:7878"   - 允许外部访问
    let listener = TcpListener::bind("127.0.0.1:7878").expect("无法绑定到端口 7878");
    println!("服务器启动，监听 127.0.0.1:7878");

    // incoming() 返回一个迭代器，每次产生一个新的连接
    // 每个元素类型为 Result<TcpStream>
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("新连接: {}", stream.peer_addr().unwrap());
                handle_client_simple(stream);
            }
            Err(e) => {
                eprintln!("连接失败: {}", e);
            }
        }
    }
}

/// 处理单个客户端连接 - 读取数据并回显
fn handle_client_simple(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    // read() 从流中读取数据到缓冲区
    // 返回实际读取的字节数
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            if bytes_read == 0 {
                println!("客户端断开连接");
                return;
            }

            let received = String::from_utf8_lossy(&buffer[..bytes_read]);
            println!("收到: {}", received.trim());

            // write_all() 确保所有数据都被写入
            let response = format!("服务器回显: {}", received);
            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(e) => {
            eprintln!("读取数据失败: {}", e);
        }
    }
}

// ===========================
// 2. 多线程 TCP 服务器
// ===========================
// 使用 thread::spawn 为每个连接创建独立线程
// 实现并发处理多个客户端
fn multithreaded_server() {
    println!("=== 多线程 TCP 服务器 ===");

    let listener = TcpListener::bind("127.0.0.1:7878").expect("无法绑定到端口 7878");
    println!("多线程服务器启动，监听 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer_addr = stream.peer_addr().unwrap();
                println!("新连接: {}", peer_addr);

                // 为每个连接创建一个新线程
                // move 关键字将 stream 的所有权移入闭包
                thread::spawn(move || {
                    handle_client_threaded(stream);
                    println!("连接 {} 已关闭", peer_addr);
                });
            }
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
            }
        }
    }
}

/// 多线程版的客户端处理 - 持续读取直到断开
fn handle_client_threaded(mut stream: TcpStream) {
    let mut buffer = [0u8; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // 读取 0 字节表示客户端已断开
                break;
            }
            Ok(bytes_read) => {
                let received = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!(
                    "[{}] 收到: {}",
                    stream.peer_addr().unwrap(),
                    received.trim()
                );

                // 回显数据给客户端
                if stream.write_all(&buffer[..bytes_read]).is_err() {
                    eprintln!("写入失败，客户端可能已断开");
                    break;
                }
            }
            Err(e) => {
                eprintln!("读取错误: {}", e);
                break;
            }
        }
    }
}

// ==============================
// 3. 使用 BufReader 的行式服务器
// ==============================
// BufReader 提供缓冲读取，支持按行读取
// 适合处理文本协议（如 HTTP、SMTP 等）
fn line_based_server() {
    println!("=== 行式 TCP 服务器 ===");

    let listener = TcpListener::bind("127.0.0.1:7878").expect("无法绑定到端口 7878");
    println!("行式服务器启动，监听 127.0.0.1:7878");
    println!("客户端发送 'quit' 可断开连接");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client_line_based(stream);
                });
            }
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
            }
        }
    }
}

/// 按行处理客户端消息
fn handle_client_line_based(stream: TcpStream) {
    let peer = stream.peer_addr().unwrap();
    println!("[{}] 已连接", peer);

    // 克隆 stream 用于读写分离
    // TcpStream 实现了 Clone（底层共享同一个 socket）
    let mut writer = stream.try_clone().expect("无法克隆 TcpStream");

    // 使用 BufReader 包装，支持 read_line / lines
    let reader = BufReader::new(stream);

    // 发送欢迎消息
    let welcome = "欢迎连接到 Rust TCP 服务器！输入 'quit' 退出。\n";
    let _ = writer.write_all(welcome.as_bytes());

    // lines() 返回按行迭代的迭代器
    for line in reader.lines() {
        match line {
            Ok(line) => {
                println!("[{}] 收到: {}", peer, line);

                // 判断是否退出
                if line.trim().eq_ignore_ascii_case("quit") {
                    let _ = writer.write_all(b"Goodbye!\n");
                    break;
                }

                // 回显并换行
                let response = format!("回显: {}\n", line);
                if writer.write_all(response.as_bytes()).is_err() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("[{}] 读取错误: {}", peer, e);
                break;
            }
        }
    }

    println!("[{}] 已断开", peer);
}

// ==========================================
// 4. Echo Server 完整实现（带超时和优雅关闭）
// ==========================================
fn echo_server_with_timeout() {
    use std::time::Duration;

    println!("=== Echo Server（带超时）===");

    let listener = TcpListener::bind("127.0.0.1:7878").expect("无法绑定到端口 7878");
    println!("Echo 服务器启动，监听 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // 设置读取超时：30 秒无数据则断开
                stream
                    .set_read_timeout(Some(Duration::from_secs(30)))
                    .expect("设置读取超时失败");

                // 设置写入超时
                stream
                    .set_write_timeout(Some(Duration::from_secs(10)))
                    .expect("设置写入超时失败");

                thread::spawn(move || {
                    let peer = stream.peer_addr().unwrap();
                    println!("[{}] 已连接（30秒超时）", peer);

                    let mut buffer = [0u8; 4096];
                    loop {
                        match stream.read(&mut buffer) {
                            Ok(0) => {
                                println!("[{}] 客户端关闭连接", peer);
                                break;
                            }
                            Ok(n) => {
                                // 回显数据
                                if stream.write_all(&buffer[..n]).is_err() {
                                    println!("[{}] 写入失败", peer);
                                    break;
                                }
                                // flush 确保数据立即发送
                                let _ = stream.flush();
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                println!("[{}] 读取超时，断开连接", peer);
                                break;
                            }
                            Err(e) => {
                                eprintln!("[{}] 读取错误: {}", peer, e);
                                break;
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
            }
        }
    }
}

// ======================================
// 5. 非阻塞模式 - 设置 listener 为非阻塞
// ======================================
// 演示 TcpListener 的 set_nonblocking 方法
fn demo_nonblocking_listener() {
    println!("=== 非阻塞 TcpListener 演示 ===");

    let listener = TcpListener::bind("127.0.0.1:7879").expect("无法绑定端口");

    // 设置为非阻塞模式
    // accept() 不会阻塞，如果没有连接会立即返回 WouldBlock 错误
    listener.set_nonblocking(true).expect("设置非阻塞失败");

    println!("非阻塞模式：尝试接受连接...");

    // 在非阻塞模式下，accept 立即返回
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("收到连接: {}", addr);
            drop(stream);
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            println!("没有等待的连接（WouldBlock）- 这是非阻塞模式的正常行为");
        }
        Err(e) => {
            eprintln!("其他错误: {}", e);
        }
    }

    // 查看 listener 的本地地址
    println!("本地地址: {}", listener.local_addr().unwrap());
}

// ==============================
// 6. 获取连接信息
// ==============================
fn demo_connection_info() {
    println!("=== TCP 连接信息演示 ===");

    let listener = TcpListener::bind("127.0.0.1:0").expect("无法绑定");

    // local_addr() 获取绑定的本地地址
    // 使用端口 0 让系统自动分配可用端口
    let local_addr = listener.local_addr().unwrap();
    println!("服务器绑定到: {}", local_addr);
    println!("  IP: {}", local_addr.ip());
    println!("  端口: {}", local_addr.port());

    // 演示：创建一个客户端连接到自己
    let client = TcpStream::connect(local_addr).expect("连接失败");
    println!("\n客户端本地地址: {}", client.local_addr().unwrap());
    println!("客户端远端地址: {}", client.peer_addr().unwrap());

    // 接受连接
    let (server_stream, client_addr) = listener.accept().expect("接受连接失败");
    println!("\n服务端看到的客户端地址: {}", client_addr);
    println!(
        "服务端本地地址: {}",
        server_stream.local_addr().unwrap()
    );

    // TcpStream 在离开作用域时自动关闭（Drop trait）
    println!("\n连接将在离开作用域时自动关闭");
}

fn main() {
    println!("===================================================");
    println!("  Lesson 079: TCP 服务器");
    println!("===================================================\n");

    // ------- 可直接运行的演示 -------

    // 演示 1: 非阻塞 listener（不会阻塞）
    demo_nonblocking_listener();
    println!();

    // 演示 2: 连接信息（自连接演示，不会阻塞）
    demo_connection_info();

    // ------- 以下是阻塞式服务器，取消注释可运行 -------
    // 注意：同时只能运行一个，因为它们都绑定同一端口

    println!("\n===================================================");
    println!("以下服务器函数可通过取消注释来运行：");
    println!("  simple_server()          - 单线程简单服务器");
    println!("  multithreaded_server()   - 多线程服务器");
    println!("  line_based_server()      - 按行读取的服务器");
    println!("  echo_server_with_timeout() - 带超时的 Echo 服务器");
    println!("===================================================");
    println!("使用 telnet 127.0.0.1 7878 进行测试");

    // 取消下面某一行的注释来运行对应服务器：
    // simple_server();
    // multithreaded_server();
    // line_based_server();
    // echo_server_with_timeout();
}
