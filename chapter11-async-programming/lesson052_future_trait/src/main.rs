// ============================================================
// Lesson 052: Future Trait
// ============================================================
//
// 本课深入学习 Rust 异步编程的核心：Future trait。
//
// 知识点：
// 1. Future trait 的定义（poll / Pin / Context）
// 2. 手动实现一个简单的 Future
// 3. Ready future
// 4. 异步执行流程讲解（图解）
//
// ============================================================
// Future Trait 定义
// ============================================================
//
// ```rust
// pub trait Future {
//     type Output;
//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
// }
// ```
//
// - Output: Future 完成时返回的值类型
// - poll(): 运行时调用此方法来"推进"Future 的执行
// - Pin<&mut Self>: 确保 Future 在内存中不会被移动（自引用安全）
// - Context: 包含 Waker，用于通知运行时 Future 可以继续执行
// - Poll: 枚举，表示 Future 的状态
//
// ============================================================
// Poll 枚举
// ============================================================
//
// ```rust
// pub enum Poll<T> {
//     Ready(T),    // Future 已完成，返回结果
//     Pending,     // Future 尚未完成，稍后再试
// }
// ```
//
// ============================================================
// 异步执行流程图解
// ============================================================
//
//   运行时（Runtime / Executor）
//     │
//     ├─→ poll(future, cx) ──→ Poll::Pending  （还没准备好）
//     │                           │
//     │    ┌──────────────────────┘
//     │    │  Future 内部注册 Waker
//     │    │  当条件满足时调用 waker.wake()
//     │    │
//     │    └──→ 运行时收到通知
//     │
//     ├─→ poll(future, cx) ──→ Poll::Pending  （再次尝试，还没好）
//     │         ...
//     │
//     └─→ poll(future, cx) ──→ Poll::Ready(value)  ✅ 完成！
//
//   关键点：
//   - 运行时不会"忙等"（busy-wait），而是靠 Waker 通知
//   - 每次 poll 返回 Pending 时，必须确保 Waker 已被注册
//   - Future 返回 Ready 后，不应再被 poll
//
// ============================================================

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::time::sleep;

// ============================================================
// 1. 手动实现一个最简单的 Future —— 立即就绪
// ============================================================

/// ReadyFuture：一个立即返回值的 Future
/// 类似于标准库中的 std::future::ready()
struct ReadyFuture<T> {
    value: Option<T>,
}

// ReadyFuture 没有自引用结构，可以安全地实现 Unpin
impl<T> Unpin for ReadyFuture<T> {}

impl<T> ReadyFuture<T> {
    fn new(value: T) -> Self {
        ReadyFuture {
            value: Some(value),
        }
    }
}

impl<T> Future for ReadyFuture<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 立即返回值，不需要等待
        // 使用 Option::take() 来转移所有权
        match self.value.take() {
            Some(v) => {
                println!("  [ReadyFuture] poll -> Ready!");
                Poll::Ready(v)
            }
            None => {
                // 正常情况下不应该再次 poll 一个已完成的 Future
                panic!("ReadyFuture 已经被 poll 过了！");
            }
        }
    }
}

// ============================================================
// 2. 手动实现一个延时 Future
// ============================================================

/// DelayFuture：等待指定时间后返回一个值
///
/// 工作原理：
///   1. 第一次 poll 时记录开始时间
///   2. 后续 poll 检查是否已过期
///   3. 未过期则注册 waker 并返回 Pending
///   4. 过期则返回 Ready
struct DelayFuture {
    /// 延迟时长
    duration: Duration,
    /// 开始时间（第一次 poll 时设置）
    started_at: Option<Instant>,
    /// poll 的次数（用于演示）
    poll_count: u32,
}

// DelayFuture 没有自引用结构，可以安全地实现 Unpin
impl Unpin for DelayFuture {}

impl DelayFuture {
    fn new(duration: Duration) -> Self {
        DelayFuture {
            duration,
            started_at: None,
            poll_count: 0,
        }
    }
}

impl Future for DelayFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_count += 1;
        let count = self.poll_count;

        // 第一次 poll：记录开始时间
        let started_at = self.started_at.get_or_insert_with(Instant::now);
        let elapsed = started_at.elapsed();

        if elapsed >= self.duration {
            // 时间到了，返回 Ready
            println!("  [DelayFuture] 第 {} 次 poll -> Ready! (已过 {:?})", count, elapsed);
            Poll::Ready(format!("延迟 {:?} 完成", self.duration))
        } else {
            // 时间未到，返回 Pending
            println!("  [DelayFuture] 第 {} 次 poll -> Pending (已过 {:?})", count, elapsed);

            // 重要：必须注册 Waker！
            // 这里我们立即唤醒，让运行时尽快再次 poll
            // （在实际场景中，应该在条件满足时才唤醒）
            let waker = cx.waker().clone();
            let remaining = self.duration - elapsed;

            // 创建一个后台任务，在剩余时间后唤醒
            tokio::spawn(async move {
                sleep(remaining).await;
                waker.wake(); // 通知运行时：可以再次 poll 了
            });

            Poll::Pending
        }
    }
}

// ============================================================
// 3. 组合 Future —— 链式执行
// ============================================================

/// ThenFuture：先执行第一个 Future，用其结果创建并执行第二个 Future
/// 这就是 .await 链式调用的底层原理
enum ThenFuture<F1, F2, Func>
where
    F1: Future,
    F2: Future,
    Func: FnOnce(F1::Output) -> F2,
{
    /// 第一阶段：正在执行第一个 Future
    First(F1, Option<Func>),
    /// 第二阶段：正在执行第二个 Future
    Second(F2),
    /// 已完成（占位）
    Done,
}

impl<F1, F2, Func> Future for ThenFuture<F1, F2, Func>
where
    F1: Future + Unpin,
    F2: Future + Unpin,
    Func: FnOnce(F1::Output) -> F2 + Unpin,
{
    type Output = F2::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match &mut *self {
                ThenFuture::First(f1, func) => {
                    // poll 第一个 Future
                    match Pin::new(f1).poll(cx) {
                        Poll::Ready(result) => {
                            // 第一个完成，用结果创建第二个
                            let func = func.take().unwrap();
                            let f2 = func(result);
                            *self = ThenFuture::Second(f2);
                            // 继续循环，立即 poll 第二个
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                ThenFuture::Second(f2) => {
                    // poll 第二个 Future
                    let result = Pin::new(f2).poll(cx);
                    if result.is_ready() {
                        *self = ThenFuture::Done;
                    }
                    return result;
                }
                ThenFuture::Done => {
                    panic!("ThenFuture 已经完成，不应再次 poll");
                }
            }
        }
    }
}

// ============================================================
// 4. 计数器 Future —— 演示多次 poll
// ============================================================

/// CountdownFuture：每次 poll 减 1，到 0 时就绪
struct CountdownFuture {
    count: u32,
}

impl Unpin for CountdownFuture {}

impl Future for CountdownFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("  [CountdownFuture] count=0 -> Ready!");
            Poll::Ready("倒计时完成！".to_string())
        } else {
            println!("  [CountdownFuture] count={} -> Pending", self.count);
            self.count -= 1;

            // 立即唤醒自己，让运行时尽快再次 poll
            cx.waker().wake_by_ref();

            Poll::Pending
        }
    }
}

// ============================================================
// 5. 使用 std::future::ready() 和 std::future::pending()
// ============================================================

async fn demo_std_futures() {
    println!("\n--- 5. 标准库提供的 Future 工具 ---");

    // std::future::ready() —— 立即返回一个值
    let value = std::future::ready(42).await;
    println!("  std::future::ready(42) -> {}", value);

    // std::future::poll_fn() —— 用闭包创建 Future
    let mut count = 0;
    let result = std::future::poll_fn(move |cx| {
        count += 1;
        if count >= 3 {
            println!("  [poll_fn] 第 {} 次 poll -> Ready!", count);
            Poll::Ready(format!("经过 {} 次 poll 完成", count))
        } else {
            println!("  [poll_fn] 第 {} 次 poll -> Pending", count);
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }).await;
    println!("  poll_fn 结果: {}", result);

    // 注意：std::future::pending() 永远不会就绪
    // let _ = std::future::pending::<()>().await; // 这会永远挂起！
    println!("  std::future::pending() 永远不会就绪（不要单独 await！）");
}

// ============================================================
// main 函数
// ============================================================

#[tokio::main]
async fn main() {
    println!("=== Lesson 052: Future Trait ===\n");

    // ----- 演示 1: ReadyFuture -----
    println!("--- 1. ReadyFuture（立即就绪的 Future）---");
    let future = ReadyFuture::new(42);
    let result = future.await;
    println!("  ReadyFuture 结果: {}\n", result);

    // ----- 演示 2: DelayFuture -----
    println!("--- 2. DelayFuture（延时 Future）---");
    let future = DelayFuture::new(Duration::from_millis(200));
    let result = future.await;
    println!("  DelayFuture 结果: {}\n", result);

    // ----- 演示 3: CountdownFuture -----
    println!("--- 3. CountdownFuture（倒计时 Future）---");
    let future = CountdownFuture { count: 3 };
    let result = future.await;
    println!("  CountdownFuture 结果: {}\n", result);

    // ----- 演示 4: ThenFuture（组合 Future）-----
    println!("--- 4. ThenFuture（组合 Future 链）---");
    let combined = ThenFuture::First(
        ReadyFuture::new(10_u32),
        Some(|value: u32| {
            println!("  [ThenFuture] 第一个 Future 完成，值为 {}", value);
            ReadyFuture::new(value * 2)
        }),
    );
    let result = combined.await;
    println!("  ThenFuture 结果: {}", result);

    // ----- 演示 5: 标准库 Future 工具 -----
    demo_std_futures().await;

    // ============================================================
    // 总结
    // ============================================================
    println!("\n=== 总结 ===");
    println!("  1. Future trait 核心方法是 poll()");
    println!("  2. poll() 返回 Poll::Ready(T) 或 Poll::Pending");
    println!("  3. Pending 时必须注册 Waker，以便稍后通知运行时");
    println!("  4. Pin 保证 Future 在内存中不被移动");
    println!("  5. async fn 会被编译器自动转换为实现 Future 的状态机");
    println!("  6. .await 本质上就是反复 poll 直到 Ready");
    println!();

    // ============================================================
    // 补充：async fn 的状态机展开（概念演示）
    // ============================================================
    println!("=== 补充：async fn 的状态机本质 ===");
    println!("  以下 async fn：");
    println!("    async fn example() -> u32 {{");
    println!("        let a = step1().await;");
    println!("        let b = step2(a).await;");
    println!("        a + b");
    println!("    }}");
    println!();
    println!("  编译器大致生成：");
    println!("    enum ExampleFuture {{");
    println!("        State0 {{ step1_future }},       // 等待 step1");
    println!("        State1 {{ a, step2_future }},     // 等待 step2");
    println!("        Done,");
    println!("    }}");
    println!("  每次 poll 根据当前状态推进执行。");
}
