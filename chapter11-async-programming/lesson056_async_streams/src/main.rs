// ============================================================
// Lesson 056: 异步流（Async Streams）
// ============================================================
//
// 本课学习异步流（Stream），它是异步版本的迭代器。
//
// 知识点：
// 1. Stream 概念（异步迭代器）
// 2. async 生成器模式（用 async + channel 模拟 yield）
// 3. StreamExt 方法：next / map / filter / take 等
// 4. 实际应用场景
//
// ============================================================
// Stream vs Iterator 对比
// ============================================================
//
//   Iterator（同步迭代器）：
//     trait Iterator {
//         type Item;
//         fn next(&mut self) -> Option<Self::Item>;
//     }
//
//   Stream（异步迭代器）：
//     trait Stream {
//         type Item;
//         fn poll_next(self: Pin<&mut Self>, cx: &mut Context)
//             -> Poll<Option<Self::Item>>;
//     }
//
//   对比：
//     Iterator::next()      → 同步返回 Option<Item>
//     Stream::poll_next()   → 异步返回 Poll<Option<Item>>
//
//   使用 StreamExt::next() 可以 .await 获取下一个元素：
//     while let Some(item) = stream.next().await {
//         // 处理 item
//     }
//
// ============================================================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration, Instant};
use tokio_stream::{self, Stream, StreamExt};

// ============================================================
// 1. 基本的 Stream 使用
// ============================================================

async fn demo_basic_stream() {
    println!("--- 1. 基本的 Stream 使用 ---");

    // 从迭代器创建 Stream
    let stream = tokio_stream::iter(vec![1, 2, 3, 4, 5]);

    // 使用 StreamExt::next() 逐个获取元素
    println!("  从 vec 创建 Stream：");
    tokio::pin!(stream); // Stream 需要被 pin 住才能调用 next()
    while let Some(value) = stream.next().await {
        print!("  {}", value);
    }
    println!();

    // 也可以使用 collect 收集结果
    let stream = tokio_stream::iter(vec![10, 20, 30]);
    let collected: Vec<i32> = stream.collect().await;
    println!("  collect 结果: {:?}", collected);
    println!();
}

// ============================================================
// 2. 手动实现 Stream
// ============================================================

/// 一个简单的计数器 Stream
/// 每次 poll 产生下一个数字，直到达到最大值
struct CounterStream {
    current: u32,
    max: u32,
}

impl CounterStream {
    fn new(max: u32) -> Self {
        CounterStream { current: 0, max }
    }
}

impl Stream for CounterStream {
    type Item = u32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            self.current += 1;
            println!("    [CounterStream] 产生值: {}", self.current);
            Poll::Ready(Some(self.current))
        } else {
            println!("    [CounterStream] 流结束");
            Poll::Ready(None) // Stream 结束
        }
    }
}

async fn demo_custom_stream() {
    println!("--- 2. 手动实现 Stream ---");

    let stream = CounterStream::new(5);
    tokio::pin!(stream);

    let mut values = Vec::new();
    while let Some(v) = stream.next().await {
        values.push(v);
    }
    println!("  CounterStream 结果: {:?}\n", values);
}

// ============================================================
// 3. 带延迟的异步 Stream
// ============================================================

/// 一个定时产生值的 Stream
/// 每隔指定时间产生一个新值
struct IntervalStream {
    interval_ms: u64,
    current: u32,
    max: u32,
    /// 用于追踪是否需要等待
    sleep_future: Option<Pin<Box<tokio::time::Sleep>>>,
}

impl IntervalStream {
    fn new(interval_ms: u64, max: u32) -> Self {
        IntervalStream {
            interval_ms,
            current: 0,
            max,
            sleep_future: None,
        }
    }
}

impl Stream for IntervalStream {
    type Item = (u32, std::time::Duration);

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // 如果有等待中的 sleep，先检查它
        if let Some(sleep_fut) = self.sleep_future.as_mut() {
            match sleep_fut.as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending, // 还在等待
                Poll::Ready(()) => {
                    self.sleep_future = None; // sleep 完成
                }
            }
        }

        if self.current < self.max {
            self.current += 1;
            let value = self.current;
            // 为下一次调用设置延迟
            if self.current < self.max {
                self.sleep_future = Some(Box::pin(sleep(Duration::from_millis(self.interval_ms))));
            }
            Poll::Ready(Some((value, std::time::Instant::now().elapsed())))
        } else {
            Poll::Ready(None)
        }
    }
}

async fn demo_interval_stream() {
    println!("--- 3. 带延迟的异步 Stream ---");

    let start = std::time::Instant::now();
    let stream = IntervalStream::new(80, 5);
    tokio::pin!(stream);

    while let Some((value, _)) = stream.next().await {
        println!("    值: {}, 时间: {:?}", value, start.elapsed());
    }
    println!();
}

// ============================================================
// 4. StreamExt 方法（链式操作）
// ============================================================

async fn demo_stream_ext() {
    println!("--- 4. StreamExt 方法 ---");

    // map —— 转换每个元素
    println!("  map: 将每个元素乘以 10");
    let stream = tokio_stream::iter(1..=5).map(|x| x * 10);
    let result: Vec<_> = stream.collect().await;
    println!("    结果: {:?}", result);

    // filter —— 过滤元素
    println!("  filter: 只保留偶数");
    let stream = tokio_stream::iter(1..=10).filter(|x| x % 2 == 0);
    let result: Vec<_> = stream.collect().await;
    println!("    结果: {:?}", result);

    // take —— 只取前 N 个元素
    println!("  take: 只取前 3 个");
    let stream = tokio_stream::iter(1..=100).take(3);
    let result: Vec<_> = stream.collect().await;
    println!("    结果: {:?}", result);

    // skip —— 跳过前 N 个元素
    println!("  skip: 跳过前 3 个");
    let stream = tokio_stream::iter(1..=6).skip(3);
    let result: Vec<_> = stream.collect().await;
    println!("    结果: {:?}", result);

    // filter_map —— 过滤 + 转换
    println!("  filter_map: 解析整数，忽略无效值");
    let items = vec!["1", "hello", "3", "world", "5"];
    let stream = tokio_stream::iter(items)
        .filter_map(|s| s.parse::<i32>().ok());
    let result: Vec<_> = stream.collect().await;
    println!("    结果: {:?}", result);

    // fold —— 归约（类似 Iterator::fold）
    println!("  fold: 求和");
    let sum = tokio_stream::iter(1..=10)
        .fold(0_i32, |acc, x| acc + x)
        .await;
    println!("    1+2+...+10 = {}", sum);

    // any —— 是否存在满足条件的元素
    println!("  any: 是否包含大于 5 的元素");
    let has_large = tokio_stream::iter(vec![1, 3, 7, 2])
        .any(|x| x > 5)
        .await;
    println!("    结果: {}", has_large);

    // 链式组合
    println!("  链式组合: iter → filter → map → take → collect");
    let result: Vec<_> = tokio_stream::iter(1..=100)
        .filter(|x| x % 3 == 0) // 能被 3 整除
        .map(|x| x * x) // 平方
        .take(5) // 取前 5 个
        .collect()
        .await;
    println!("    结果: {:?}", result);
    println!();
}

// ============================================================
// 5. 用 async + channel 模拟 yield（生成器模式）
// ============================================================

/// 使用 mpsc channel 模拟异步生成器
/// 这是在 Rust 没有原生 async yield 语法时的常用模式
fn fibonacci_stream(count: usize) -> impl Stream<Item = u64> {
    // 创建一个通道，将发送端传给异步任务
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 0..count {
            // 类似于 yield a
            if tx.send(a).await.is_err() {
                break; // 接收端已关闭
            }
            let next = a + b;
            a = b;
            b = next;
        }
        // tx 被 drop，流自然结束
    });

    // 将 ReceiverStream 包装为 Stream
    tokio_stream::wrappers::ReceiverStream::new(rx)
}

/// 另一个生成器示例：异步产生倒计时
fn countdown_stream(from: u32) -> impl Stream<Item = String> {
    let (tx, rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        for i in (0..=from).rev() {
            let msg = if i == 0 {
                "🚀 发射！".to_string()
            } else {
                format!("倒计时: {}", i)
            };
            if tx.send(msg).await.is_err() {
                break;
            }
            sleep(Duration::from_millis(50)).await;
        }
    });

    tokio_stream::wrappers::ReceiverStream::new(rx)
}

async fn demo_generator_pattern() {
    println!("--- 5. 生成器模式（async + channel 模拟 yield）---");

    // 斐波那契数列流
    println!("  斐波那契数列（前 10 个）：");
    let stream = fibonacci_stream(10);
    tokio::pin!(stream);
    let mut fibs = Vec::new();
    while let Some(n) = stream.next().await {
        fibs.push(n);
    }
    println!("    {:?}", fibs);

    // 倒计时流
    println!("  倒计时：");
    let stream = countdown_stream(5);
    tokio::pin!(stream);
    while let Some(msg) = stream.next().await {
        println!("    {}", msg);
    }
    println!();
}

// ============================================================
// 6. 实际应用场景
// ============================================================

async fn demo_practical_usage() {
    println!("--- 6. 实际应用场景 ---");

    // 场景 1：分页查询模拟
    println!("  场景 1: 分页查询");

    /// 模拟分页 API 调用
    fn paginated_query(total_items: usize, page_size: usize) -> impl Stream<Item = Vec<String>> {
        let (tx, rx) = tokio::sync::mpsc::channel(1);

        tokio::spawn(async move {
            let total_pages = (total_items + page_size - 1) / page_size;
            for page in 0..total_pages {
                // 模拟网络延迟
                sleep(Duration::from_millis(30)).await;

                let start = page * page_size;
                let end = (start + page_size).min(total_items);
                let items: Vec<String> = (start..end)
                    .map(|i| format!("item_{}", i))
                    .collect();

                println!("    [查询] 获取第 {} 页: {} 条记录", page + 1, items.len());
                if tx.send(items).await.is_err() {
                    break;
                }
            }
        });

        tokio_stream::wrappers::ReceiverStream::new(rx)
    }

    let stream = paginated_query(7, 3);
    tokio::pin!(stream);
    let mut all_items = Vec::new();
    while let Some(page) = stream.next().await {
        all_items.extend(page);
    }
    println!("    总共获取: {:?}\n", all_items);

    // 场景 2：流的超时控制
    println!("  场景 2: 流的超时控制");

    let slow_stream = tokio_stream::iter(1..=10).then(|x| async move {
        sleep(Duration::from_millis(40)).await;
        x
    });

    // 使用 timeout 包装每个元素
    let start = Instant::now();
    let timed_stream = slow_stream.timeout(Duration::from_millis(150));
    tokio::pin!(timed_stream);

    let mut results = Vec::new();
    while let Some(result) = timed_stream.next().await {
        match result {
            Ok(val) => results.push(val),
            Err(_) => {
                println!("    某个元素超时！");
                break;
            }
        }
    }
    println!("    在 {:?} 内获取到: {:?}\n", start.elapsed(), results);

    // 场景 3：合并多个流
    println!("  场景 3: 使用 StreamMap 合并流");

    // StreamMap 要求所有流具有相同的类型
    // 所以我们直接用相同类型的 Stream
    let mut stream_map = tokio_stream::StreamMap::new();
    stream_map.insert(
        "数字",
        tokio_stream::iter(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
    );
    stream_map.insert(
        "字母",
        tokio_stream::iter(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
    );

    tokio::pin!(stream_map);
    while let Some((key, value)) = stream_map.next().await {
        println!("    来源: {}, 值: {}", key, value);
    }
    println!();
}

// ============================================================
// main 函数
// ============================================================

#[tokio::main]
async fn main() {
    println!("=== Lesson 056: 异步流（Async Streams）===\n");

    // 1. 基本 Stream 使用
    demo_basic_stream().await;

    // 2. 手动实现 Stream
    demo_custom_stream().await;

    // 3. 带延迟的异步 Stream
    demo_interval_stream().await;

    // 4. StreamExt 方法
    demo_stream_ext().await;

    // 5. 生成器模式
    demo_generator_pattern().await;

    // 6. 实际应用
    demo_practical_usage().await;

    // ============================================================
    // 总结
    // ============================================================
    println!("=== 总结 ===");
    println!("  1. Stream 是异步版本的迭代器，逐个异步产生值");
    println!("  2. StreamExt 提供了丰富的适配器：map, filter, take, fold 等");
    println!("  3. 使用 async + mpsc channel 可以模拟生成器模式");
    println!("  4. tokio_stream::wrappers 可以将 Tokio 类型转换为 Stream");
    println!("  5. StreamMap 可以合并多个 Stream");
    println!("  6. 实际应用：分页查询、事件流、定时器流等");
}
