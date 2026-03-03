// ============================================================
// Lesson 080: TCP 客户端
// ============================================================
// 本课学习如何使用 Rust 标准库创建 TCP 客户端：
// - TcpStream::connect 连接服务器
// - write/read 发送和接收数据
// - BufReader 按行读取响应
// - 超时设置 set_read_timeout / set_write_timeout
// - 连接远程服务的模式代码
//
// 注意：部分示例需要配合 lesson079 的服务器运行
// ============================================================

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

// ============================
// 1. 基本的 TCP 连接与读写
// ============================
fn basic_tcp_client() {
    println!("=== 基本 TCP 客户端 ===\n");

    // TcpStream::connect 连接到指定地址
    // 支持的地址格式：
    //   "127.0.0.1:7878"        - IPv4 + 端口
    //   "[::1]:7878"            - IPv6 + 端口
    //   "example.com:80"        - 域名 + 端口（会自动 DNS 解析）
    //   ("127.0.0.1", 7878)     - 元组形式
    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            println!("成功连接到服务器!");
            println!("本地地址: {}", stream.local_addr().unwrap());
            println!("远端地址: {}", stream.peer_addr().unwrap());

            // 发送数据
            let message = "Hello from Rust TCP client!";
            stream.write_all(message.as_bytes()).unwrap();
            println!("已发送: {}", message);

            // 读取响应
            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                    println!("收到响应: {}", response);
                }
                Err(e) => eprintln!("读取失败: {}", e),
            }
        }
        Err(e) => {
            eprintln!("连接失败: {} (请确保服务器已启动)", e);
        }
    }
}

// ============================
// 2. 带超时的 TCP 客户端
// ============================
fn client_with_timeout() {
    println!("=== 带超时的 TCP 客户端 ===\n");

    // connect_timeout 方法可以设置连接超时
    // 注意：需要使用 SocketAddr 而不是字符串
    let addr = "127.0.0.1:7878".parse().unwrap();
    match TcpStream::connect_timeout(&addr, Duration::from_secs(5)) {
        Ok(mut stream) => {
            println!("连接成功（5 秒超时限制内）");

            // 设置读取超时
            // 如果在指定时间内没有收到数据，read 操作将返回错误
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .expect("设置读取超时失败");

            // 设置写入超时
            stream
                .set_write_timeout(Some(Duration::from_secs(3)))
                .expect("设置写入超时失败");

            // 查询当前超时设置
            println!(
                "读取超时: {:?}",
                stream.read_timeout().unwrap()
            );
            println!(
                "写入超时: {:?}",
                stream.write_timeout().unwrap()
            );

            // 发送数据
            stream.write_all(b"timeout test\n").unwrap();

            // 尝试读取（可能超时）
            let mut buffer = [0u8; 1024];
            match stream.read(&mut buffer) {
                Ok(n) => {
                    println!("收到 {} 字节: {}", n, String::from_utf8_lossy(&buffer[..n]));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock
                    || e.kind() == io::ErrorKind::TimedOut =>
                {
                    println!("读取超时！（这是预期行为）");
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("连接超时或失败: {}", e);
        }
    }
}

// ==========================================
// 3. 使用 BufReader 按行读取
// ==========================================
fn client_with_bufreader() {
    println!("=== 使用 BufReader 的客户端 ===\n");

    match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            // 克隆 stream 以分别用于读和写
            let mut writer = stream.try_clone().expect("克隆 TcpStream 失败");

            // BufReader 提供缓冲读取能力
            let reader = BufReader::new(stream);

            // 发送带换行符的消息（行协议）
            writer.write_all(b"Hello line 1\n").unwrap();
            writer.write_all(b"Hello line 2\n").unwrap();
            writer.write_all(b"quit\n").unwrap();

            // 按行读取响应
            // lines() 返回迭代器，每次返回一行（不含换行符）
            for (i, line) in reader.lines().enumerate() {
                match line {
                    Ok(line) => {
                        println!("第 {} 行: {}", i + 1, line);
                    }
                    Err(e) => {
                        eprintln!("读取行失败: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("连接失败: {}", e);
        }
    }
}

// ==========================================
// 4. 逐块读取大量数据
// ==========================================
fn client_read_all_data() {
    println!("=== 读取全部数据 ===\n");

    match TcpStream::connect("127.0.0.1:7878") {
        Ok(mut stream) => {
            // 发送请求
            stream.write_all(b"send me data").unwrap();

            // 关闭写端，通知服务器我们不再发送
            // shutdown 可以关闭读端、写端或两端
            stream
                .shutdown(std::net::Shutdown::Write)
                .expect("关闭写端失败");

            // 使用 read_to_end 读取所有数据到 Vec
            let mut response = Vec::new();
            match stream.read_to_end(&mut response) {
                Ok(total) => {
                    println!("总共读取 {} 字节", total);
                    println!("内容: {}", String::from_utf8_lossy(&response));
                }
                Err(e) => {
                    eprintln!("读取失败: {}", e);
                }
            }

            // 也可以用 read_to_string 直接读取为字符串
            // let mut response = String::new();
            // stream.read_to_string(&mut response)?;
        }
        Err(e) => {
            eprintln!("连接失败: {}", e);
        }
    }
}

// ==============================================
// 5. 连接公共服务的模式代码（HTTP GET 示例）
// ==============================================
// 用原始 TCP 发送 HTTP 请求（仅作教学演示）
fn raw_http_get() {
    println!("=== 原始 TCP 发送 HTTP 请求 ===\n");

    // 连接到一个 HTTP 服务器（示例用 httpbin.org）
    // 注意：这是明文 HTTP，不是 HTTPS
    match TcpStream::connect("httpbin.org:80") {
        Ok(mut stream) => {
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();

            // 手工构建 HTTP 请求
            // HTTP/1.1 要求 Host 头部
            let request = "GET /get HTTP/1.1\r\n\
                           Host: httpbin.org\r\n\
                           Connection: close\r\n\
                           User-Agent: rust-tcp-client/1.0\r\n\
                           \r\n";

            stream.write_all(request.as_bytes()).unwrap();
            println!("已发送 HTTP GET 请求");

            // 读取响应
            let mut response = String::new();
            match stream.read_to_string(&mut response) {
                Ok(_) => {
                    // 只打印前 500 个字符
                    let preview: String = response.chars().take(500).collect();
                    println!("响应预览:\n{}", preview);
                    if response.len() > 500 {
                        println!("... (共 {} 字节)", response.len());
                    }
                }
                Err(e) => {
                    eprintln!("读取响应失败: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("连接失败: {} (可能需要网络连接)", e);
        }
    }
}

// ==========================================
// 6. 演示 TcpStream 的各种方法（自连接）
// ==========================================
fn demo_stream_methods() {
    println!("=== TcpStream 方法演示 ===\n");

    // 创建服务端 listener（端口 0 表示自动分配）
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    println!("监听地址: {}", addr);

    // 客户端连接
    let mut client = TcpStream::connect(addr).unwrap();
    let (mut server, _) = listener.accept().unwrap();

    // --- 基本信息 ---
    println!("\n--- 连接信息 ---");
    println!("客户端本地地址: {}", client.local_addr().unwrap());
    println!("客户端远端地址: {}", client.peer_addr().unwrap());

    // --- 设置 nodelay（禁用 Nagle 算法）---
    // Nagle 算法会合并小数据包，禁用它可以降低延迟
    client.set_nodelay(true).unwrap();
    println!("Nodelay: {}", client.nodelay().unwrap());

    // --- 设置 TTL (Time To Live) ---
    client.set_ttl(128).unwrap();
    println!("TTL: {}", client.ttl().unwrap());

    // --- 读写演示 ---
    println!("\n--- 读写演示 ---");

    // 客户端写，服务端读
    client.write_all(b"Hello Server!").unwrap();
    let mut buf = [0u8; 64];
    let n = server.read(&mut buf).unwrap();
    println!("服务端收到: {}", String::from_utf8_lossy(&buf[..n]));

    // 服务端写，客户端读
    server.write_all(b"Hello Client!").unwrap();
    let n = client.read(&mut buf).unwrap();
    println!("客户端收到: {}", String::from_utf8_lossy(&buf[..n]));

    // --- peek（窥视数据，不从缓冲区移除）---
    println!("\n--- peek 演示 ---");
    server.write_all(b"peek test").unwrap();

    // peek 读取数据但不消耗
    let n = client.peek(&mut buf).unwrap();
    println!("peek 到: {}", String::from_utf8_lossy(&buf[..n]));

    // 再次 read 仍然能读到同样的数据
    let n = client.read(&mut buf).unwrap();
    println!("read 到: {}", String::from_utf8_lossy(&buf[..n]));

    // --- take（限制可读字节数）---
    println!("\n--- take 限制读取 ---");
    server.write_all(b"1234567890").unwrap();

    // 使用 take 限制只读 5 个字节
    let mut limited = (&client).take(5);
    let mut small_buf = [0u8; 10];
    let n = limited.read(&mut small_buf).unwrap();
    println!("限制读取 5 字节: {}", String::from_utf8_lossy(&small_buf[..n]));

    // 读取剩余部分
    let n = client.read(&mut small_buf).unwrap();
    println!("剩余部分: {}", String::from_utf8_lossy(&small_buf[..n]));

    println!("\n所有演示完成！连接将自动关闭。");
}

fn main() {
    println!("===================================================");
    println!("  Lesson 080: TCP 客户端");
    println!("===================================================\n");

    // ------- 可直接运行的演示（自连接，无需外部服务器）-------

    // 演示 TcpStream 的各种方法
    demo_stream_methods();

    // ------- 以下需要服务器配合，取消注释后运行 -------

    println!("\n===================================================");
    println!("以下函数需要配合 lesson079 的服务器运行：");
    println!("  1. 先运行: cd ../lesson079_tcp_server && cargo run");
    println!("  2. 再取消注释下面的函数调用");
    println!("===================================================");
    println!("可用的客户端函数：");
    println!("  basic_tcp_client()      - 基本连接与读写");
    println!("  client_with_timeout()   - 带超时设置的客户端");
    println!("  client_with_bufreader() - 使用 BufReader 按行读取");
    println!("  client_read_all_data()  - 读取全部数据");
    println!("  raw_http_get()          - 原始 TCP 发送 HTTP 请求");

    // 取消注释运行（需要先启动 lesson079 服务器）：
    // basic_tcp_client();
    // client_with_timeout();
    // client_with_bufreader();
    // client_read_all_data();

    // 取消注释运行（需要网络连接）：
    // raw_http_get();
}
