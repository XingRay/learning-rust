// ============================================================
// Lesson 081: UDP 通信
// ============================================================
// 本课学习 Rust 标准库中的 UDP 通信：
// - UdpSocket::bind 绑定地址
// - send_to / recv_from 发送和接收数据
// - connect 后使用 send / recv
// - UDP 与 TCP 的区别
//
// UDP 与 TCP 的关键区别：
// ┌──────────────────┬─────────────────────┬──────────────────────┐
// │     特性         │       TCP           │        UDP           │
// ├──────────────────┼─────────────────────┼──────────────────────┤
// │ 连接方式         │ 面向连接（三次握手）│ 无连接               │
// │ 可靠性           │ 可靠传输（确认重传）│ 不可靠（可能丢包）   │
// │ 顺序保证         │ 保证顺序           │ 不保证顺序           │
// │ 数据边界         │ 字节流（无边界）    │ 数据报（有边界）     │
// │ 传输效率         │ 较低（开销大）      │ 较高（开销小）       │
// │ 适用场景         │ 文件传输、HTTP、SSH │ 视频流、DNS、游戏    │
// │ 头部大小         │ 20 字节以上         │ 8 字节               │
// │ 流量控制         │ 有                  │ 无                   │
// │ 拥塞控制         │ 有                  │ 无                   │
// └──────────────────┴─────────────────────┴──────────────────────┘
// ============================================================

use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

// ===========================
// 1. 基本的 UDP 发送与接收
// ===========================
fn basic_udp_demo() {
    println!("=== 基本 UDP 发送与接收 ===\n");

    // 创建两个 UDP socket，模拟通信双方
    // UdpSocket::bind 绑定到指定地址
    // 端口 0 表示让系统自动分配可用端口
    let socket_a = UdpSocket::bind("127.0.0.1:0").expect("Socket A 绑定失败");
    let socket_b = UdpSocket::bind("127.0.0.1:0").expect("Socket B 绑定失败");

    let addr_a = socket_a.local_addr().unwrap();
    let addr_b = socket_b.local_addr().unwrap();
    println!("Socket A 地址: {}", addr_a);
    println!("Socket B 地址: {}", addr_b);

    // send_to: 向指定地址发送数据
    // 返回成功发送的字节数
    let message = "你好，UDP！";
    let bytes_sent = socket_a
        .send_to(message.as_bytes(), addr_b)
        .expect("发送失败");
    println!("\nA -> B 发送了 {} 字节: {}", bytes_sent, message);

    // recv_from: 接收数据，返回 (字节数, 发送方地址)
    let mut buf = [0u8; 1024];
    let (bytes_received, src_addr) = socket_b.recv_from(&mut buf).expect("接收失败");
    let received = String::from_utf8_lossy(&buf[..bytes_received]);
    println!("B 从 {} 收到 {} 字节: {}", src_addr, bytes_received, received);

    // B 回复 A
    let reply = "收到你的消息了！";
    socket_b.send_to(reply.as_bytes(), addr_a).expect("回复失败");

    let (n, from) = socket_a.recv_from(&mut buf).expect("接收回复失败");
    println!(
        "A 从 {} 收到回复: {}",
        from,
        String::from_utf8_lossy(&buf[..n])
    );
}

// ==========================================
// 2. 使用 connect 建立"关联"的 UDP socket
// ==========================================
// UDP 的 connect 不像 TCP 那样建立真正的连接
// 它只是记住目标地址，之后可以用 send/recv 代替 send_to/recv_from
fn connected_udp_demo() {
    println!("=== 使用 connect 的 UDP ===\n");

    let socket_a = UdpSocket::bind("127.0.0.1:0").unwrap();
    let socket_b = UdpSocket::bind("127.0.0.1:0").unwrap();

    let addr_a = socket_a.local_addr().unwrap();
    let addr_b = socket_b.local_addr().unwrap();

    // connect 将 socket 与远端地址关联
    // 之后的 send/recv 都针对这个地址
    socket_a.connect(addr_b).expect("connect 失败");
    socket_b.connect(addr_a).expect("connect 失败");

    println!("Socket A ({}) 已关联到 {}", addr_a, addr_b);
    println!("Socket B ({}) 已关联到 {}", addr_b, addr_a);

    // 使用 send（而不是 send_to）
    socket_a.send(b"connected UDP message").expect("send 失败");

    // 使用 recv（而不是 recv_from）
    let mut buf = [0u8; 1024];
    let n = socket_b.recv(&mut buf).expect("recv 失败");
    println!("B 收到: {}", String::from_utf8_lossy(&buf[..n]));

    // 回复
    socket_b.send(b"reply via connected socket").unwrap();
    let n = socket_a.recv(&mut buf).unwrap();
    println!("A 收到: {}", String::from_utf8_lossy(&buf[..n]));

    // 注意：connect 后只能接收来自关联地址的数据
    // 来自其他地址的数据会被丢弃
    println!("\n提示：connect 后，只接收来自关联地址的数据");
}

// ==================================
// 3. 多线程 UDP 收发
// ==================================
fn threaded_udp_demo() {
    println!("=== 多线程 UDP 收发 ===\n");

    let server = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server.local_addr().unwrap();

    // 设置超时，避免无限等待
    server
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    println!("UDP 服务端地址: {}", server_addr);

    // 启动"客户端"线程，发送多条消息
    let handle = thread::spawn(move || {
        let client = UdpSocket::bind("127.0.0.1:0").unwrap();
        println!("UDP 客户端地址: {}", client.local_addr().unwrap());

        let messages = vec!["消息一", "消息二", "消息三", "结束"];

        for msg in &messages {
            client
                .send_to(msg.as_bytes(), server_addr)
                .expect("发送失败");
            println!("客户端发送: {}", msg);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 服务端接收消息
    let mut buf = [0u8; 1024];
    let mut count = 0;

    loop {
        match server.recv_from(&mut buf) {
            Ok((n, src)) => {
                let msg = String::from_utf8_lossy(&buf[..n]);
                println!("服务端从 {} 收到: {}", src, msg);
                count += 1;

                if msg == "结束" {
                    println!("收到结束信号，退出循环");
                    break;
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                println!("接收超时，退出");
                break;
            }
            Err(e) => {
                eprintln!("接收错误: {}", e);
                break;
            }
        }
    }

    handle.join().unwrap();
    println!("共收到 {} 条消息", count);
}

// ==========================================
// 4. UDP 广播
// ==========================================
fn broadcast_demo() {
    println!("=== UDP 广播 ===\n");

    let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
    let receiver = UdpSocket::bind("127.0.0.1:0").unwrap();

    let receiver_addr = receiver.local_addr().unwrap();
    receiver
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    // 设置广播权限
    // set_broadcast(true) 允许 socket 发送广播数据包
    sender.set_broadcast(true).expect("设置广播失败");
    println!("广播已启用: {}", sender.broadcast().unwrap());

    // 注意：真正的广播需要发送到广播地址如 255.255.255.255
    // 这里为了演示，我们直接发送到已知接收端
    sender
        .send_to(b"broadcast message", receiver_addr)
        .unwrap();
    println!("发送广播消息");

    let mut buf = [0u8; 1024];
    match receiver.recv_from(&mut buf) {
        Ok((n, from)) => {
            println!(
                "接收到来自 {} 的消息: {}",
                from,
                String::from_utf8_lossy(&buf[..n])
            );
        }
        Err(e) => eprintln!("接收失败: {}", e),
    }
}

// ==========================================
// 5. UDP Socket 选项与属性
// ==========================================
fn socket_options_demo() {
    println!("=== UDP Socket 选项 ===\n");

    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    // 本地地址
    println!("本地地址: {}", socket.local_addr().unwrap());

    // TTL (Time To Live) - 数据报的最大跳数
    socket.set_ttl(64).unwrap();
    println!("TTL: {}", socket.ttl().unwrap());

    // 非阻塞模式
    socket.set_nonblocking(true).unwrap();
    println!("已设置为非阻塞模式");

    // 在非阻塞模式下，recv_from 立即返回
    let mut buf = [0u8; 1024];
    match socket.recv_from(&mut buf) {
        Ok((n, addr)) => println!("收到: {} 字节，来自 {}", n, addr),
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            println!("非阻塞模式：没有数据可读（WouldBlock）- 正常行为");
        }
        Err(e) => eprintln!("错误: {}", e),
    }

    // 恢复阻塞模式
    socket.set_nonblocking(false).unwrap();

    // 读写超时
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    socket
        .set_write_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    println!("读取超时: {:?}", socket.read_timeout().unwrap());
    println!("写入超时: {:?}", socket.write_timeout().unwrap());

    // 广播设置
    socket.set_broadcast(false).unwrap();
    println!("广播: {}", socket.broadcast().unwrap());

    // 多播 TTL（组播数据报的 TTL）
    socket.set_multicast_ttl_v4(32).unwrap();
    println!("组播 TTL: {}", socket.multicast_ttl_v4().unwrap());

    // 多播环回（是否接收自己发出的组播数据）
    socket.set_multicast_loop_v4(true).unwrap();
    println!("组播环回: {}", socket.multicast_loop_v4().unwrap());
}

// ============================================
// 6. 简单的 UDP Echo Server（自包含演示）
// ============================================
fn udp_echo_demo() {
    println!("=== UDP Echo 演示 ===\n");

    // 创建 echo 服务器
    let server = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server.local_addr().unwrap();
    server
        .set_read_timeout(Some(Duration::from_secs(2)))
        .unwrap();

    println!("Echo 服务器地址: {}", server_addr);

    // 在线程中运行 echo 服务器
    let server_handle = thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut count = 0;

        loop {
            match server.recv_from(&mut buf) {
                Ok((n, src)) => {
                    // 原样回送数据（Echo）
                    server.send_to(&buf[..n], src).expect("回送失败");
                    count += 1;

                    let msg = String::from_utf8_lossy(&buf[..n]);
                    if msg == "STOP" {
                        break;
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(e) => {
                    eprintln!("服务器错误: {}", e);
                    break;
                }
            }
        }

        println!("Echo 服务器处理了 {} 条消息", count);
    });

    // 客户端发送并验证 echo
    let client = UdpSocket::bind("127.0.0.1:0").unwrap();
    client
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();

    let test_messages = vec!["Hello", "World", "Rust UDP", "STOP"];
    let mut buf = [0u8; 1024];

    for msg in &test_messages {
        client
            .send_to(msg.as_bytes(), server_addr)
            .expect("发送失败");

        match client.recv_from(&mut buf) {
            Ok((n, _)) => {
                let echo = String::from_utf8_lossy(&buf[..n]);
                let ok = if echo == *msg { "✓" } else { "✗" };
                println!("{} 发送: {:>10} | 回显: {:>10}", ok, msg, echo);
            }
            Err(e) => {
                eprintln!("接收 echo 失败: {}", e);
            }
        }
    }

    server_handle.join().unwrap();
    println!("\nEcho 演示完成！");
}

fn main() {
    println!("===================================================");
    println!("  Lesson 081: UDP 通信");
    println!("===================================================\n");

    // 1. 基本收发
    basic_udp_demo();
    println!("\n---------------------------------------------------\n");

    // 2. connect 关联
    connected_udp_demo();
    println!("\n---------------------------------------------------\n");

    // 3. 多线程收发
    threaded_udp_demo();
    println!("\n---------------------------------------------------\n");

    // 4. 广播
    broadcast_demo();
    println!("\n---------------------------------------------------\n");

    // 5. Socket 选项
    socket_options_demo();
    println!("\n---------------------------------------------------\n");

    // 6. Echo 演示
    udp_echo_demo();
}
