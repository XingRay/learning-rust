// ============================================================
// Lesson 096: Redis 客户端（概念讲解）
// ============================================================
// Redis 是一个高性能的内存键值数据库，广泛用于缓存、消息队列、
// 会话管理等场景。本课以概念讲解为主，使用 HashMap 等标准库
// 类型模拟 Redis 的核心操作，帮助理解 Redis 数据模型和使用模式。
//
// Rust 中常用的 Redis 客户端库：
// 1. redis crate  —— 最流行的 Redis 客户端（同步 + 异步）
// 2. fred         —— 高性能异步 Redis 客户端
// 3. deadpool-redis —— 基于 redis crate 的连接池
//
// redis crate 的 Cargo.toml 配置：
// [dependencies]
// redis = { version = "0.27", features = ["tokio-comp", "connection-manager"] }
// tokio = { version = "1", features = ["full"] }
// ============================================================

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

fn main() {
    println!("=== Lesson 096: Redis 客户端（概念讲解）===\n");

    // ============================================================
    // 1. Redis 数据类型概览
    // ============================================================
    redis_data_types();

    // ============================================================
    // 2. 模拟 Redis 操作 —— String 类型
    // ============================================================
    simulate_string_operations();

    // ============================================================
    // 3. 模拟 Redis 操作 —— List 类型
    // ============================================================
    simulate_list_operations();

    // ============================================================
    // 4. 模拟 Redis 操作 —— Set 类型
    // ============================================================
    simulate_set_operations();

    // ============================================================
    // 5. 模拟 Redis 操作 —— Hash 类型
    // ============================================================
    simulate_hash_operations();

    // ============================================================
    // 6. 模拟 Redis 操作 —— Sorted Set 类型
    // ============================================================
    simulate_sorted_set_operations();

    // ============================================================
    // 7. Redis 客户端使用模式（注释代码示例）
    // ============================================================
    redis_client_patterns();

    // ============================================================
    // 8. 连接池与最佳实践
    // ============================================================
    connection_pool_and_best_practices();

    println!("\n=== Redis 客户端概念讲解完毕 ===");
}

/// 1. Redis 数据类型概览
fn redis_data_types() {
    println!("--- 1. Redis 数据类型概览 ---\n");

    println!("Redis 支持以下核心数据类型：");
    println!();
    println!("  ┌──────────────┬──────────────────────┬──────────────────────┐");
    println!("  │ 类型          │ 描述                  │ 典型用途              │");
    println!("  ├──────────────┼──────────────────────┼──────────────────────┤");
    println!("  │ String       │ 字符串/数字/二进制      │ 缓存、计数器、分布式锁 │");
    println!("  │ List         │ 双端链表               │ 消息队列、时间线       │");
    println!("  │ Set          │ 无序不重复集合          │ 标签、共同好友         │");
    println!("  │ Hash         │ 字段-值映射表          │ 对象存储、用户信息     │");
    println!("  │ Sorted Set   │ 带分数的有序集合        │ 排行榜、延迟队列       │");
    println!("  │ Stream       │ 日志型数据结构          │ 事件流、消息队列       │");
    println!("  │ Bitmap       │ 位操作                 │ 用户签到、布隆过滤器   │");
    println!("  │ HyperLogLog  │ 基数估算               │ UV 统计              │");
    println!("  └──────────────┴──────────────────────┴──────────────────────┘");
    println!();
}

/// 2. 模拟 String 操作
fn simulate_string_operations() {
    println!("--- 2. String 类型操作（模拟）---\n");

    // 使用 HashMap 模拟 Redis 的 String 类型
    let mut redis = HashMap::<String, String>::new();

    // SET key value
    redis.insert("user:1:name".into(), "张三".into());
    redis.insert("user:1:email".into(), "zhangsan@example.com".into());
    redis.insert("counter:visits".into(), "0".into());

    println!("  SET user:1:name \"张三\"");
    println!("  SET user:1:email \"zhangsan@example.com\"");
    println!("  SET counter:visits \"0\"");

    // GET key
    if let Some(name) = redis.get("user:1:name") {
        println!("  GET user:1:name → \"{}\"", name);
    }

    // INCR（模拟计数器递增）
    if let Some(counter) = redis.get_mut("counter:visits") {
        let val: i64 = counter.parse().unwrap_or(0);
        *counter = (val + 1).to_string();
    }
    println!(
        "  INCR counter:visits → {}",
        redis.get("counter:visits").unwrap()
    );

    // MSET（批量设置）
    let pairs = vec![
        ("config:timeout", "30"),
        ("config:retries", "3"),
        ("config:debug", "false"),
    ];
    for (k, v) in &pairs {
        redis.insert(k.to_string(), v.to_string());
    }
    println!("  MSET config:timeout 30 config:retries 3 config:debug false");

    // DEL
    redis.remove("config:debug");
    println!("  DEL config:debug");

    // EXISTS
    let exists = redis.contains_key("config:debug");
    println!("  EXISTS config:debug → {}", exists);

    println!();

    // -------------------------------------------------------
    // 对应的 redis crate 代码：
    //
    // ```rust
    // use redis::Commands;
    //
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    //
    // // SET / GET
    // con.set("user:1:name", "张三")?;
    // let name: String = con.get("user:1:name")?;
    //
    // // SET with expiration (EX = seconds)
    // con.set_ex("session:abc123", "user_data", 3600)?;
    //
    // // INCR
    // let new_val: i64 = con.incr("counter:visits", 1)?;
    //
    // // MSET / MGET
    // con.set_multiple(&[("k1", "v1"), ("k2", "v2")])?;
    // let values: Vec<String> = con.get(&["k1", "k2"])?;
    //
    // // SETNX (不存在时才设置，用于分布式锁)
    // let set: bool = con.set_nx("lock:resource", "holder_id")?;
    //
    // // DEL / EXISTS
    // con.del("key")?;
    // let exists: bool = con.exists("key")?;
    //
    // // TTL / EXPIRE
    // con.expire("key", 60)?;       // 设置过期时间
    // let ttl: i64 = con.ttl("key")?;  // 查看剩余时间
    // ```
    // -------------------------------------------------------
}

/// 3. 模拟 List 操作
fn simulate_list_operations() {
    println!("--- 3. List 类型操作（模拟）---\n");

    // 使用 VecDeque 模拟 Redis List（双端队列）
    let mut task_queue: VecDeque<String> = VecDeque::new();

    // LPUSH（左端插入）
    task_queue.push_front("任务C".into());
    task_queue.push_front("任务B".into());
    task_queue.push_front("任务A".into());
    println!("  LPUSH task_queue 任务A 任务B 任务C");
    println!("  队列状态: {:?}", task_queue);

    // RPUSH（右端插入）
    task_queue.push_back("任务D".into());
    println!("  RPUSH task_queue 任务D");
    println!("  队列状态: {:?}", task_queue);

    // LPOP（左端弹出）
    if let Some(task) = task_queue.pop_front() {
        println!("  LPOP task_queue → \"{}\"", task);
    }

    // RPOP（右端弹出）
    if let Some(task) = task_queue.pop_back() {
        println!("  RPOP task_queue → \"{}\"", task);
    }

    // LLEN（长度）
    println!("  LLEN task_queue → {}", task_queue.len());

    // LRANGE（范围查看）
    let range: Vec<&String> = task_queue.iter().collect();
    println!("  LRANGE task_queue 0 -1 → {:?}", range);

    println!();
    println!("  List 典型使用场景：");
    println!("    • 消息队列：LPUSH 生产消息，RPOP/BRPOP 消费消息");
    println!("    • 最新动态：LPUSH 添加新动态，LTRIM 保留最新 N 条");
    println!("    • 任务队列：RPUSH 添加任务，BLPOP 阻塞等待任务");
    println!();

    // -------------------------------------------------------
    // redis crate 代码：
    //
    // ```rust
    // // LPUSH / RPUSH
    // con.lpush("queue", "task1")?;
    // con.rpush("queue", "task2")?;
    //
    // // LPOP / RPOP
    // let task: String = con.lpop("queue", None)?;
    //
    // // BLPOP (阻塞弹出，超时 0 = 永久等待)
    // let (key, val): (String, String) = con.blpop("queue", 0)?;
    //
    // // LRANGE
    // let items: Vec<String> = con.lrange("queue", 0, -1)?;
    //
    // // LLEN
    // let len: i64 = con.llen("queue")?;
    //
    // // LTRIM (保留指定范围)
    // con.ltrim("timeline", 0, 99)?;  // 只保留最新 100 条
    // ```
    // -------------------------------------------------------
}

/// 4. 模拟 Set 操作
fn simulate_set_operations() {
    println!("--- 4. Set 类型操作（模拟）---\n");

    // 使用 BTreeSet 模拟 Redis Set
    let mut user1_tags: BTreeSet<String> = BTreeSet::new();
    let mut user2_tags: BTreeSet<String> = BTreeSet::new();

    // SADD（添加成员）
    user1_tags.insert("Rust".into());
    user1_tags.insert("Python".into());
    user1_tags.insert("Go".into());
    println!("  SADD user:1:tags Rust Python Go");

    user2_tags.insert("Rust".into());
    user2_tags.insert("Java".into());
    user2_tags.insert("Go".into());
    println!("  SADD user:2:tags Rust Java Go");

    // SMEMBERS（查看所有成员）
    println!("  SMEMBERS user:1:tags → {:?}", user1_tags);
    println!("  SMEMBERS user:2:tags → {:?}", user2_tags);

    // SISMEMBER（是否为成员）
    println!(
        "  SISMEMBER user:1:tags Rust → {}",
        user1_tags.contains("Rust")
    );
    println!(
        "  SISMEMBER user:1:tags Java → {}",
        user1_tags.contains("Java")
    );

    // SINTER（交集 —— 共同标签）
    let common: BTreeSet<_> = user1_tags.intersection(&user2_tags).collect();
    println!("  SINTER user:1:tags user:2:tags → {:?}", common);

    // SUNION（并集）
    let all: BTreeSet<_> = user1_tags.union(&user2_tags).collect();
    println!("  SUNION user:1:tags user:2:tags → {:?}", all);

    // SDIFF（差集）
    let diff: BTreeSet<_> = user1_tags.difference(&user2_tags).collect();
    println!(
        "  SDIFF user:1:tags user:2:tags → {:?}（user1 独有）",
        diff
    );

    // SCARD（成员数量）
    println!("  SCARD user:1:tags → {}", user1_tags.len());

    // SREM（移除成员）
    user1_tags.remove("Python");
    println!("  SREM user:1:tags Python");
    println!("  SMEMBERS user:1:tags → {:?}", user1_tags);

    println!();

    // -------------------------------------------------------
    // redis crate 代码：
    //
    // ```rust
    // con.sadd("user:1:tags", "Rust")?;
    // con.sadd("user:1:tags", &["Python", "Go"])?;
    //
    // let members: Vec<String> = con.smembers("user:1:tags")?;
    // let is_member: bool = con.sismember("user:1:tags", "Rust")?;
    // let common: Vec<String> = con.sinter(&["user:1:tags", "user:2:tags"])?;
    // let all: Vec<String> = con.sunion(&["user:1:tags", "user:2:tags"])?;
    // let count: i64 = con.scard("user:1:tags")?;
    // con.srem("user:1:tags", "Python")?;
    // ```
    // -------------------------------------------------------
}

/// 5. 模拟 Hash 操作
fn simulate_hash_operations() {
    println!("--- 5. Hash 类型操作（模拟）---\n");

    // 使用 HashMap<String, HashMap<String, String>> 模拟 Redis Hash
    let mut redis_hashes: HashMap<String, HashMap<String, String>> = HashMap::new();

    // HSET（设置字段）
    let user_hash = redis_hashes
        .entry("user:1".into())
        .or_insert_with(HashMap::new);
    user_hash.insert("name".into(), "张三".into());
    user_hash.insert("email".into(), "zhangsan@example.com".into());
    user_hash.insert("age".into(), "28".into());
    user_hash.insert("city".into(), "北京".into());

    println!("  HSET user:1 name 张三 email zhangsan@example.com age 28 city 北京");

    // HGET（获取单个字段）
    if let Some(user) = redis_hashes.get("user:1") {
        if let Some(name) = user.get("name") {
            println!("  HGET user:1 name → \"{}\"", name);
        }
    }

    // HGETALL（获取所有字段）
    if let Some(user) = redis_hashes.get("user:1") {
        println!("  HGETALL user:1 →");
        for (field, value) in user {
            println!("    {} → {}", field, value);
        }
    }

    // HMGET（批量获取字段）
    if let Some(user) = redis_hashes.get("user:1") {
        let fields = ["name", "age"];
        let values: Vec<Option<&String>> =
            fields.iter().map(|f| user.get(*f)).collect();
        println!("  HMGET user:1 name age → {:?}", values);
    }

    // HDEL（删除字段）
    if let Some(user) = redis_hashes.get_mut("user:1") {
        user.remove("city");
        println!("  HDEL user:1 city");
    }

    // HLEN（字段数量）
    if let Some(user) = redis_hashes.get("user:1") {
        println!("  HLEN user:1 → {}", user.len());
    }

    // HEXISTS（字段是否存在）
    if let Some(user) = redis_hashes.get("user:1") {
        println!(
            "  HEXISTS user:1 city → {}",
            user.contains_key("city")
        );
        println!(
            "  HEXISTS user:1 name → {}",
            user.contains_key("name")
        );
    }

    println!();
    println!("  Hash 典型使用场景：");
    println!("    • 存储对象：用户信息、商品信息、配置项");
    println!("    • 比多个 String key 更节省内存");
    println!("    • 支持部分更新（只修改某个字段）");
    println!();

    // -------------------------------------------------------
    // redis crate 代码：
    //
    // ```rust
    // // HSET (单个/多个字段)
    // con.hset("user:1", "name", "张三")?;
    // con.hset_multiple("user:1", &[("email", "zhangsan@example.com"), ("age", "28")])?;
    //
    // // HGET / HGETALL
    // let name: String = con.hget("user:1", "name")?;
    // let all: HashMap<String, String> = con.hgetall("user:1")?;
    //
    // // HMGET
    // let values: Vec<String> = con.hget("user:1", &["name", "age"])?;
    //
    // // HINCRBY (字段数值递增)
    // con.hincr("user:1", "login_count", 1)?;
    //
    // // HDEL / HEXISTS / HLEN
    // con.hdel("user:1", "city")?;
    // let exists: bool = con.hexists("user:1", "name")?;
    // let len: i64 = con.hlen("user:1")?;
    // ```
    // -------------------------------------------------------
}

/// 6. 模拟 Sorted Set 操作
fn simulate_sorted_set_operations() {
    println!("--- 6. Sorted Set 类型操作（模拟）---\n");

    // 使用 BTreeMap<String, f64> 模拟 Redis Sorted Set
    // 实际 Redis ZSet 按 score 排序，这里简化模拟
    let mut leaderboard: BTreeMap<String, f64> = BTreeMap::new();

    // ZADD（添加成员和分数）
    leaderboard.insert("玩家A".into(), 1500.0);
    leaderboard.insert("玩家B".into(), 2300.0);
    leaderboard.insert("玩家C".into(), 1800.0);
    leaderboard.insert("玩家D".into(), 2100.0);
    leaderboard.insert("玩家E".into(), 1200.0);

    println!("  ZADD leaderboard 1500 玩家A 2300 玩家B 1800 玩家C 2100 玩家D 1200 玩家E");

    // ZCARD（成员数量）
    println!("  ZCARD leaderboard → {}", leaderboard.len());

    // ZSCORE（查看分数）
    if let Some(score) = leaderboard.get("玩家B") {
        println!("  ZSCORE leaderboard 玩家B → {}", score);
    }

    // ZINCRBY（增加分数）
    if let Some(score) = leaderboard.get_mut("玩家A") {
        *score += 200.0;
        println!("  ZINCRBY leaderboard 200 玩家A → {}", score);
    }

    // ZRANGEBYSCORE（按分数范围查询）
    // 模拟 ZREVRANGE（按分数从高到低排序）
    let mut sorted: Vec<(&String, &f64)> = leaderboard.iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    println!("  ZREVRANGE leaderboard 0 -1 WITHSCORES →");
    for (rank, (name, score)) in sorted.iter().enumerate() {
        println!("    第{}名: {} ({} 分)", rank + 1, name, score);
    }

    // ZRANGEBYSCORE（分数范围）
    let range_result: Vec<(&String, &f64)> = leaderboard
        .iter()
        .filter(|(_, &score)| score >= 1500.0 && score <= 2000.0)
        .collect();
    println!("  ZRANGEBYSCORE leaderboard 1500 2000 →");
    for (name, score) in &range_result {
        println!("    {} ({} 分)", name, score);
    }

    // ZREM（移除成员）
    leaderboard.remove("玩家E");
    println!("  ZREM leaderboard 玩家E");
    println!("  ZCARD leaderboard → {}", leaderboard.len());

    println!();
    println!("  Sorted Set 典型使用场景：");
    println!("    • 排行榜（游戏积分、销售排名）");
    println!("    • 延迟队列（score = 执行时间戳）");
    println!("    • 范围查询（价格区间、时间窗口）");
    println!("    • 优先级队列");
    println!();

    // -------------------------------------------------------
    // redis crate 代码：
    //
    // ```rust
    // // ZADD
    // con.zadd("leaderboard", "玩家A", 1500)?;
    // con.zadd_multiple("leaderboard", &[("玩家B", 2300), ("玩家C", 1800)])?;
    //
    // // ZSCORE / ZRANK
    // let score: f64 = con.zscore("leaderboard", "玩家A")?;
    // let rank: i64 = con.zrevrank("leaderboard", "玩家A")?; // 从高到低的排名
    //
    // // ZINCRBY
    // let new_score: f64 = con.zincr("leaderboard", "玩家A", 200)?;
    //
    // // ZREVRANGE（按分数从高到低）
    // let top3: Vec<(String, f64)> = con.zrevrange_withscores("leaderboard", 0, 2)?;
    //
    // // ZRANGEBYSCORE（分数范围）
    // let in_range: Vec<String> = con.zrangebyscore("leaderboard", 1500, 2000)?;
    //
    // // ZREM / ZCARD
    // con.zrem("leaderboard", "玩家E")?;
    // let count: i64 = con.zcard("leaderboard")?;
    // ```
    // -------------------------------------------------------
}

/// 7. Redis 客户端使用模式
fn redis_client_patterns() {
    println!("--- 7. Redis 客户端使用模式 ---\n");

    // -------------------------------------------------------
    // 基本连接模式：
    //
    // ```rust
    // use redis::{Client, Commands, AsyncCommands};
    //
    // // === 同步连接 ===
    // let client = Client::open("redis://127.0.0.1:6379/")?;
    // let mut con = client.get_connection()?;
    //
    // // 带密码连接
    // let client = Client::open("redis://:password@127.0.0.1:6379/0")?;
    // // URL 格式: redis://[username][:password]@host:port/db_number
    //
    // // === 异步连接（tokio）===
    // let client = Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_multiplexed_async_connection().await?;
    //
    // // 异步操作
    // con.set("key", "value").await?;
    // let val: String = con.get("key").await?;
    // ```
    // -------------------------------------------------------

    println!("连接模式：");
    println!("  同步: client.get_connection()");
    println!("  异步: client.get_multiplexed_async_connection().await");
    println!("  URL:  redis://[user][:pass]@host:port/db_number");
    println!();

    // -------------------------------------------------------
    // Pipeline（管道）—— 批量发送命令减少网络往返：
    //
    // ```rust
    // use redis::pipe;
    //
    // let (val1, val2, val3): (String, String, i64) = pipe()
    //     .cmd("SET").arg("key1").arg("hello").ignore()
    //     .cmd("SET").arg("key2").arg("world").ignore()
    //     .cmd("GET").arg("key1")
    //     .cmd("GET").arg("key2")
    //     .cmd("INCR").arg("counter")
    //     .query(&mut con)?;
    //
    // // 原子管道（类似事务）
    // let results: Vec<i64> = pipe()
    //     .atomic()  // 包裹在 MULTI/EXEC 中
    //     .incr("counter1", 1)
    //     .incr("counter2", 2)
    //     .query(&mut con)?;
    // ```
    // -------------------------------------------------------

    println!("Pipeline（管道）：");
    println!("  将多个命令打包发送，减少网络往返次数");
    println!("  pipe().cmd(\"SET\").arg(k).arg(v).ignore()");
    println!("  .atomic() 将管道包裹在 MULTI/EXEC 事务中");
    println!();

    // -------------------------------------------------------
    // Pub/Sub（发布订阅）：
    //
    // ```rust
    // // 订阅者
    // let mut pubsub = con.as_pubsub();
    // pubsub.subscribe("news")?;
    //
    // loop {
    //     let msg = pubsub.get_message()?;
    //     let payload: String = msg.get_payload()?;
    //     println!("收到频道 {} 的消息: {}", msg.get_channel_name(), payload);
    // }
    //
    // // 发布者
    // con.publish("news", "突发新闻：Rust 发布新版本！")?;
    // ```
    // -------------------------------------------------------

    println!("Pub/Sub（发布订阅）：");
    println!("  con.as_pubsub() → pubsub.subscribe(\"channel\")");
    println!("  pubsub.get_message() 接收消息");
    println!("  con.publish(\"channel\", message) 发布消息");
    println!();

    // 模拟 Redis 常用命令映射表
    println!("Redis 命令 → redis crate 方法映射：");
    println!("  ┌─────────────────┬───────────────────────────────┐");
    println!("  │ Redis 命令       │ Rust 方法                     │");
    println!("  ├─────────────────┼───────────────────────────────┤");
    println!("  │ SET k v         │ con.set(\"k\", \"v\")?            │");
    println!("  │ GET k           │ con.get(\"k\")?                 │");
    println!("  │ DEL k           │ con.del(\"k\")?                 │");
    println!("  │ EXISTS k        │ con.exists(\"k\")?              │");
    println!("  │ EXPIRE k secs   │ con.expire(\"k\", secs)?       │");
    println!("  │ TTL k           │ con.ttl(\"k\")?                 │");
    println!("  │ INCR k          │ con.incr(\"k\", 1)?             │");
    println!("  │ LPUSH k v       │ con.lpush(\"k\", \"v\")?          │");
    println!("  │ RPOP k          │ con.rpop(\"k\", None)?          │");
    println!("  │ SADD k v        │ con.sadd(\"k\", \"v\")?           │");
    println!("  │ HSET k f v      │ con.hset(\"k\", \"f\", \"v\")?      │");
    println!("  │ ZADD k score m  │ con.zadd(\"k\", \"m\", score)?    │");
    println!("  └─────────────────┴───────────────────────────────┘");
    println!();
}

/// 8. 连接池与最佳实践
fn connection_pool_and_best_practices() {
    println!("--- 8. 连接池与最佳实践 ---\n");

    // -------------------------------------------------------
    // 连接池配置（使用 deadpool-redis 或 bb8-redis）：
    //
    // ```rust
    // // 方式一：redis crate 内置 ConnectionManager
    // use redis::aio::ConnectionManager;
    //
    // let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut manager = ConnectionManager::new(client).await?;
    // // ConnectionManager 自动重连，可以在多个任务间共享（Clone）
    //
    // // 方式二：使用 deadpool-redis 连接池
    // // [dependencies]
    // // deadpool-redis = "0.18"
    //
    // use deadpool_redis::{Config, Runtime};
    //
    // let cfg = Config::from_url("redis://127.0.0.1/");
    // let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    //
    // let mut conn = pool.get().await?;
    // conn.set::<_, _, ()>("key", "value").await?;
    //
    // // 方式三：使用 bb8-redis 连接池
    // // [dependencies]
    // // bb8-redis = "0.17"
    //
    // use bb8_redis::{bb8, RedisConnectionManager};
    //
    // let manager = RedisConnectionManager::new("redis://127.0.0.1/")?;
    // let pool = bb8::Pool::builder()
    //     .max_size(20)
    //     .build(manager)
    //     .await?;
    // ```
    // -------------------------------------------------------

    println!("连接池方案：");
    println!("  1. ConnectionManager（redis crate 内置）");
    println!("     自动重连，适合简单场景");
    println!();
    println!("  2. deadpool-redis");
    println!("     功能完整的连接池，自动管理连接生命周期");
    println!();
    println!("  3. bb8-redis");
    println!("     基于 bb8 的连接池，配置灵活");
    println!();

    println!("最佳实践：");
    println!("  1. 使用连接池而非单连接（并发场景必须）");
    println!("  2. 设置合理的 key 过期时间（避免内存泄漏）");
    println!("  3. 使用 Pipeline 批量操作（减少网络往返）");
    println!("  4. key 命名规范：模块:实体:id:字段（如 user:123:name）");
    println!("  5. 敏感数据设置 TTL 自动过期");
    println!("  6. 大 value 考虑压缩或拆分");
    println!("  7. 避免使用 KEYS * 命令（生产环境用 SCAN 代替）");
    println!("  8. 合理选择数据类型（Hash 存对象比多个 String 更节省内存）");
    println!();
}
