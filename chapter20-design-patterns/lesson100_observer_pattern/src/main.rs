// ============================================================
// Lesson 100: 观察者模式 (Observer Pattern)
// ============================================================
//
// 观察者模式定义了对象之间的一对多依赖关系：
// 当一个对象（发布者/主题）状态改变时，所有依赖它的对象（观察者）都会收到通知。
//
// 在 Rust 中实现观察者模式有两种主要方式：
//   1. 使用 trait 对象 (Box<dyn Observer>) —— 传统面向对象方式
//   2. 使用闭包 (Box<dyn Fn(...)>) —— 更 Rust 风格的简化方式

use std::fmt;

// ============================================================
// 1. 基于 Trait 的观察者模式
// ============================================================

/// 事件类型
#[derive(Debug, Clone)]
enum StockEvent {
    PriceChanged { symbol: String, old_price: f64, new_price: f64 },
    VolumeAlert { symbol: String, volume: u64 },
    MarketOpen,
    MarketClose,
}

impl fmt::Display for StockEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StockEvent::PriceChanged { symbol, old_price, new_price } => {
                let change = new_price - old_price;
                let arrow = if change >= 0.0 { "↑" } else { "↓" };
                write!(f, "[价格变动] {} {} {:.2} -> {:.2} ({}{:.2})",
                    symbol, arrow, old_price, new_price,
                    if change >= 0.0 { "+" } else { "" }, change)
            }
            StockEvent::VolumeAlert { symbol, volume } => {
                write!(f, "[成交量警报] {} 成交量: {}", symbol, volume)
            }
            StockEvent::MarketOpen => write!(f, "[市场] 开盘"),
            StockEvent::MarketClose => write!(f, "[市场] 收盘"),
        }
    }
}

/// 观察者 trait
trait Observer {
    /// 观察者的唯一标识，用于移除观察者
    fn id(&self) -> &str;
    /// 当事件发生时被调用
    fn on_event(&self, event: &StockEvent);
}

/// 事件发布者 —— 持有观察者列表
struct StockExchange {
    name: String,
    observers: Vec<Box<dyn Observer>>,
    prices: Vec<(String, f64)>, // (股票代码, 当前价格)
}

impl StockExchange {
    fn new(name: &str) -> Self {
        println!("创建交易所: {}", name);
        StockExchange {
            name: name.to_string(),
            observers: Vec::new(),
            prices: Vec::new(),
        }
    }

    /// 注册观察者
    fn register(&mut self, observer: Box<dyn Observer>) {
        println!("  [注册] 观察者 \"{}\" 已注册到 {}", observer.id(), self.name);
        self.observers.push(observer);
    }

    /// 移除观察者（通过 id）
    fn unregister(&mut self, observer_id: &str) {
        let before = self.observers.len();
        self.observers.retain(|obs| obs.id() != observer_id);
        let after = self.observers.len();
        if before != after {
            println!("  [移除] 观察者 \"{}\" 已从 {} 移除", observer_id, self.name);
        } else {
            println!("  [移除] 未找到观察者 \"{}\"", observer_id);
        }
    }

    /// 通知所有观察者
    fn notify_all(&self, event: &StockEvent) {
        for observer in &self.observers {
            observer.on_event(event);
        }
    }

    /// 更新股票价格，触发事件
    fn update_price(&mut self, symbol: &str, new_price: f64) {
        // 查找旧价格
        let old_price = self.prices.iter()
            .find(|(s, _)| s == symbol)
            .map(|(_, p)| *p)
            .unwrap_or(0.0);

        // 更新价格
        if let Some(entry) = self.prices.iter_mut().find(|(s, _)| s == symbol) {
            entry.1 = new_price;
        } else {
            self.prices.push((symbol.to_string(), new_price));
        }

        // 通知观察者
        let event = StockEvent::PriceChanged {
            symbol: symbol.to_string(),
            old_price,
            new_price,
        };
        self.notify_all(&event);
    }

    /// 发送成交量警报
    fn volume_alert(&self, symbol: &str, volume: u64) {
        let event = StockEvent::VolumeAlert {
            symbol: symbol.to_string(),
            volume,
        };
        self.notify_all(&event);
    }

    /// 开盘
    fn open_market(&self) {
        self.notify_all(&StockEvent::MarketOpen);
    }

    /// 收盘
    fn close_market(&self) {
        self.notify_all(&StockEvent::MarketClose);
    }

    /// 获取当前观察者数量
    fn observer_count(&self) -> usize {
        self.observers.len()
    }
}

// --- 具体的观察者实现 ---

/// 价格监控器 —— 只关注价格变动
struct PriceMonitor {
    id: String,
    threshold: f64, // 价格变动阈值
}

impl PriceMonitor {
    fn new(id: &str, threshold: f64) -> Self {
        PriceMonitor {
            id: id.to_string(),
            threshold,
        }
    }
}

impl Observer for PriceMonitor {
    fn id(&self) -> &str {
        &self.id
    }

    fn on_event(&self, event: &StockEvent) {
        if let StockEvent::PriceChanged { symbol, old_price, new_price } = event {
            let change_pct = ((new_price - old_price) / old_price * 100.0).abs();
            if change_pct >= self.threshold {
                println!(
                    "    📊 [{}] 警告！{} 价格变动 {:.1}% 超过阈值 {:.1}%",
                    self.id, symbol, change_pct, self.threshold
                );
            }
        }
    }
}

/// 日志记录器 —— 记录所有事件
struct EventLogger {
    id: String,
}

impl EventLogger {
    fn new(id: &str) -> Self {
        EventLogger { id: id.to_string() }
    }
}

impl Observer for EventLogger {
    fn id(&self) -> &str {
        &self.id
    }

    fn on_event(&self, event: &StockEvent) {
        println!("    📝 [{}] 日志: {}", self.id, event);
    }
}

/// 交易策略 —— 根据价格变动做出反应
struct TradingStrategy {
    id: String,
    buy_threshold: f64,  // 下跌超过此比例时买入
}

impl TradingStrategy {
    fn new(id: &str, buy_threshold: f64) -> Self {
        TradingStrategy {
            id: id.to_string(),
            buy_threshold,
        }
    }
}

impl Observer for TradingStrategy {
    fn id(&self) -> &str {
        &self.id
    }

    fn on_event(&self, event: &StockEvent) {
        match event {
            StockEvent::PriceChanged { symbol, old_price, new_price } => {
                let change_pct = (new_price - old_price) / old_price * 100.0;
                if change_pct <= -self.buy_threshold {
                    println!(
                        "    🤖 [{}] 策略触发: {} 下跌 {:.1}%，建议买入！",
                        self.id, symbol, change_pct.abs()
                    );
                } else if change_pct >= self.buy_threshold {
                    println!(
                        "    🤖 [{}] 策略触发: {} 上涨 {:.1}%，建议卖出！",
                        self.id, symbol, change_pct
                    );
                }
            }
            StockEvent::MarketOpen => {
                println!("    🤖 [{}] 市场开盘，启动交易策略", self.id);
            }
            StockEvent::MarketClose => {
                println!("    🤖 [{}] 市场收盘，停止交易策略", self.id);
            }
            _ => {}
        }
    }
}

// ============================================================
// 2. 基于闭包的简化观察者
// ============================================================

/// 使用闭包实现的简化版事件系统
struct EventBus {
    /// 每个监听器是一个 (id, 闭包) 的元组
    listeners: Vec<(String, Box<dyn Fn(&str, &str)>)>,
}

impl EventBus {
    fn new() -> Self {
        EventBus {
            listeners: Vec::new(),
        }
    }

    /// 用闭包注册事件监听器
    fn on(&mut self, id: &str, handler: impl Fn(&str, &str) + 'static) {
        self.listeners.push((id.to_string(), Box::new(handler)));
    }

    /// 发射事件，通知所有监听器
    fn emit(&self, event_name: &str, data: &str) {
        println!("  [EventBus] 发射事件: {} -> \"{}\"", event_name, data);
        for (id, handler) in &self.listeners {
            print!("    [{}] ", id);
            handler(event_name, data);
        }
    }

    /// 移除监听器
    fn off(&mut self, id: &str) {
        self.listeners.retain(|(listener_id, _)| listener_id != id);
        println!("  [EventBus] 移除监听器: {}", id);
    }

    /// 获取监听器数量
    fn listener_count(&self) -> usize {
        self.listeners.len()
    }
}

// ============================================================
// 3. 带事件过滤的观察者
// ============================================================

/// 支持事件名过滤的高级事件系统
struct TypedEventBus {
    /// (监听器id, 关注的事件名, 处理闭包)
    handlers: Vec<(String, String, Box<dyn Fn(&str)>)>,
}

impl TypedEventBus {
    fn new() -> Self {
        TypedEventBus {
            handlers: Vec::new(),
        }
    }

    /// 注册针对特定事件的监听器
    fn on(&mut self, event_name: &str, id: &str, handler: impl Fn(&str) + 'static) {
        self.handlers.push((
            id.to_string(),
            event_name.to_string(),
            Box::new(handler),
        ));
    }

    /// 发射事件，只通知关注该事件的监听器
    fn emit(&self, event_name: &str, data: &str) {
        for (id, subscribed_event, handler) in &self.handlers {
            if subscribed_event == event_name {
                print!("    [{}] ", id);
                handler(data);
            }
        }
    }

    /// 移除某个监听器的所有订阅
    fn off(&mut self, id: &str) {
        self.handlers.retain(|(lid, _, _)| lid != id);
    }
}

// ============================================================
// main 函数
// ============================================================

fn main() {
    println!("=== Lesson 100: 观察者模式 (Observer Pattern) ===\n");

    // ---------------------------------------------------------
    // 1. 基于 Trait 的观察者模式
    // ---------------------------------------------------------
    println!("--- 1. 股票交易所 —— Trait 观察者模式 ---");

    let mut exchange = StockExchange::new("上海证券交易所");

    // 注册观察者
    exchange.register(Box::new(PriceMonitor::new("风控监控", 3.0)));
    exchange.register(Box::new(EventLogger::new("系统日志")));
    exchange.register(Box::new(TradingStrategy::new("量化策略A", 2.0)));

    println!("\n当前观察者数量: {}\n", exchange.observer_count());

    // 开盘
    println!(">> 开盘");
    exchange.open_market();

    // 价格变动
    println!("\n>> 股票价格更新");
    exchange.update_price("AAPL", 150.0);
    println!();
    exchange.update_price("AAPL", 145.0); // 下跌 3.3%
    println!();
    exchange.update_price("GOOGL", 2800.0);
    println!();
    exchange.update_price("GOOGL", 2900.0); // 上涨 3.6%

    // 成交量警报
    println!("\n>> 成交量警报");
    exchange.volume_alert("AAPL", 1_000_000);

    // 移除一个观察者
    println!("\n>> 移除观察者");
    exchange.unregister("系统日志");
    println!("移除后观察者数量: {}\n", exchange.observer_count());

    // 继续发送事件 —— 已移除的不再收到通知
    println!(">> 移除日志后的价格更新");
    exchange.update_price("AAPL", 148.0);

    // 收盘
    println!("\n>> 收盘");
    exchange.close_market();

    println!();

    // ---------------------------------------------------------
    // 2. 闭包版观察者
    // ---------------------------------------------------------
    println!("--- 2. EventBus —— 闭包简化版 ---\n");

    let mut bus = EventBus::new();

    // 用闭包注册不同的事件处理器
    bus.on("logger", |event, data| {
        println!("收到事件 [{}]: {}", event, data);
    });

    bus.on("counter", {
        // 闭包可以捕获外部变量（但这里用的是不可变闭包，所以不能修改）
        let prefix = "计数器";
        move |event, _data| {
            println!("{} 记录了事件: {}", prefix, event);
        }
    });

    bus.on("filter", |event, data| {
        if event == "error" {
            println!("⚠️ 错误事件: {}", data);
        }
    });

    println!("监听器数量: {}\n", bus.listener_count());

    bus.emit("info", "系统启动完成");
    println!();
    bus.emit("error", "数据库连接失败");
    println!();

    // 移除 counter 监听器
    bus.off("counter");
    println!("移除后监听器数量: {}\n", bus.listener_count());

    bus.emit("info", "用户登录");

    println!();

    // ---------------------------------------------------------
    // 3. 带事件过滤的 EventBus
    // ---------------------------------------------------------
    println!("--- 3. TypedEventBus —— 事件名过滤 ---\n");

    let mut typed_bus = TypedEventBus::new();

    // 不同监听器只关注特定事件
    typed_bus.on("user:login", "auth_handler", |data| {
        println!("处理用户登录: {}", data);
    });

    typed_bus.on("user:logout", "auth_handler", |data| {
        println!("处理用户登出: {}", data);
    });

    typed_bus.on("order:created", "order_handler", |data| {
        println!("新订单创建: {}", data);
    });

    typed_bus.on("order:created", "notification", |data| {
        println!("发送通知: 新订单 {}", data);
    });

    println!(">> 用户登录事件");
    typed_bus.emit("user:login", "用户 Alice");
    println!();

    println!(">> 创建订单事件");
    typed_bus.emit("order:created", "订单 #1001");
    println!();

    println!(">> 用户登出事件");
    typed_bus.emit("user:logout", "用户 Alice");
    println!();

    // 移除 notification 的所有订阅
    typed_bus.off("notification");
    println!(">> 移除 notification 后再次创建订单");
    typed_bus.emit("order:created", "订单 #1002");

    println!();

    // ---------------------------------------------------------
    // 4. 总结
    // ---------------------------------------------------------
    println!("--- 4. 观察者模式总结 ---");
    println!("┌─────────────────┬─────────────────────────────────────┐");
    println!("│ 实现方式        │ 特点                                │");
    println!("├─────────────────┼─────────────────────────────────────┤");
    println!("│ Trait 对象      │ 类型安全、可携带状态、适合复杂逻辑  │");
    println!("│ 闭包            │ 简洁、灵活、适合简单回调            │");
    println!("│ 带过滤的 EventBus│ 精确订阅、减少无关通知             │");
    println!("└─────────────────┴─────────────────────────────────────┘");

    println!("\n=== 观察者模式学习完成！===");
}
