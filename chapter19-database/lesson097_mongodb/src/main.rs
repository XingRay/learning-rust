// ============================================================
// Lesson 097: MongoDB（概念讲解）
// ============================================================
// MongoDB 是一个文档型 NoSQL 数据库，使用 BSON（Binary JSON）
// 格式存储数据。本课以概念讲解为主，使用 serde_json 模拟
// MongoDB 的文档操作，帮助理解文档数据库的核心概念。
//
// Rust 中的 MongoDB 客户端：
// - mongodb crate（官方驱动）：原生异步，功能完整
//
// MongoDB Cargo.toml 配置：
// [dependencies]
// mongodb = "3.1"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1", features = ["derive"] }
// bson = "2"
// ============================================================

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// ============================================================
// 文档模型定义
// ============================================================
// 在 MongoDB 中，数据以文档（Document）形式存储。
// 在 Rust 中，通常使用 serde 的 Serialize/Deserialize
// 将结构体与 BSON 文档相互转换。

/// 用户文档模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<String>, // MongoDB 的 _id 字段（通常为 ObjectId）
    name: String,
    email: String,
    age: u32,
    tags: Vec<String>,
    address: Address,
}

/// 嵌套文档：地址
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct Address {
    city: String,
    street: String,
    zip: String,
}

/// 文章文档模型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]
    _id: Option<String>,
    author_id: String,
    title: String,
    content: String,
    tags: Vec<String>,
    comments: Vec<Comment>,
    likes: u32,
}

/// 嵌入式文档：评论
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
struct Comment {
    user: String,
    text: String,
    rating: u32,
}

fn main() {
    println!("=== Lesson 097: MongoDB（概念讲解）===\n");

    // ============================================================
    // 1. MongoDB 核心概念
    // ============================================================
    core_concepts();

    // ============================================================
    // 2. BSON 与 JSON
    // ============================================================
    bson_and_json();

    // ============================================================
    // 3. 模拟 CRUD 操作
    // ============================================================
    simulate_crud();

    // ============================================================
    // 4. 查询操作符
    // ============================================================
    query_operators();

    // ============================================================
    // 5. 聚合管道
    // ============================================================
    aggregation_pipeline();

    // ============================================================
    // 6. 索引
    // ============================================================
    indexes();

    // ============================================================
    // 7. MongoDB Rust 驱动使用模式
    // ============================================================
    mongodb_rust_driver();

    // ============================================================
    // 8. 数据建模最佳实践
    // ============================================================
    data_modeling();

    println!("\n=== MongoDB 概念讲解完毕 ===");
}

/// 1. MongoDB 核心概念
fn core_concepts() {
    println!("--- 1. MongoDB 核心概念 ---\n");

    println!("MongoDB 与关系型数据库的概念对应：");
    println!("  ┌──────────────────┬──────────────────┐");
    println!("  │ 关系型数据库      │ MongoDB          │");
    println!("  ├──────────────────┼──────────────────┤");
    println!("  │ Database（数据库）│ Database（数据库）│");
    println!("  │ Table（表）       │ Collection（集合）│");
    println!("  │ Row（行）         │ Document（文档）  │");
    println!("  │ Column（列）      │ Field（字段）     │");
    println!("  │ Primary Key      │ _id              │");
    println!("  │ JOIN             │ $lookup / 嵌入    │");
    println!("  │ INDEX            │ Index            │");
    println!("  └──────────────────┴──────────────────┘");
    println!();

    println!("MongoDB 的特点：");
    println!("  • 无固定 Schema：同一集合中的文档可以有不同的字段");
    println!("  • 嵌入式文档：支持文档嵌套（如地址嵌入用户中）");
    println!("  • 数组字段：字段值可以是数组");
    println!("  • 水平扩展：支持分片（Sharding）分布式部署");
    println!("  • 丰富的查询语法：支持正则、地理位置、全文搜索");
    println!("  • 聚合管道：强大的数据处理和分析能力");
    println!();
}

/// 2. BSON 与 JSON
fn bson_and_json() {
    println!("--- 2. BSON 与 JSON ---\n");

    // 用 serde_json 展示 MongoDB 文档的结构
    let user_doc = json!({
        "_id": "ObjectId(\"65a1b2c3d4e5f6a7b8c9d0e1\")",
        "name": "张三",
        "email": "zhangsan@example.com",
        "age": 28,
        "tags": ["Rust", "数据库", "后端"],
        "address": {
            "city": "北京",
            "street": "中关村大街1号",
            "zip": "100080"
        },
        "created_at": "2024-01-15T10:30:00Z"
    });

    println!("MongoDB 文档示例（JSON 表示）：");
    println!(
        "{}",
        serde_json::to_string_pretty(&user_doc).unwrap()
    );
    println!();

    println!("BSON 特有的数据类型：");
    println!("  • ObjectId    —— 12 字节的唯一标识符（默认 _id 类型）");
    println!("  • DateTime    —— 毫秒精度的 UTC 时间戳");
    println!("  • Decimal128  —— 128 位十进制浮点数（精确货币计算）");
    println!("  • Binary      —— 二进制数据");
    println!("  • Regex       —— 正则表达式");
    println!("  • Int32/Int64 —— 明确区分 32/64 位整数");
    println!("  • Timestamp   —— 内部时间戳（用于 oplog）");
    println!();

    // -------------------------------------------------------
    // bson crate 使用示例：
    //
    // ```rust
    // use bson::{doc, oid::ObjectId, DateTime};
    //
    // // 使用 doc! 宏创建 BSON 文档
    // let doc = doc! {
    //     "_id": ObjectId::new(),
    //     "name": "张三",
    //     "age": 28,
    //     "created_at": DateTime::now(),
    //     "tags": ["Rust", "MongoDB"],
    //     "address": {
    //         "city": "北京",
    //         "zip": "100080"
    //     }
    // };
    //
    // // BSON 文档是有序的键值对
    // println!("name = {}", doc.get_str("name").unwrap());
    // println!("age = {}", doc.get_i32("age").unwrap());
    // ```
    // -------------------------------------------------------
}

/// 3. 模拟 CRUD 操作
fn simulate_crud() {
    println!("--- 3. 模拟 CRUD 操作 ---\n");

    // 使用 HashMap<String, Vec<Value>> 模拟 MongoDB 集合
    let mut db: HashMap<String, Vec<Value>> = HashMap::new();
    db.insert("users".into(), Vec::new());
    db.insert("posts".into(), Vec::new());

    // === INSERT（插入）===
    println!("[INSERT] 插入文档：");

    let users_collection = db.get_mut("users").unwrap();

    let user1 = json!({
        "_id": "user_001",
        "name": "张三",
        "email": "zhangsan@example.com",
        "age": 28,
        "tags": ["Rust", "后端"],
        "address": { "city": "北京", "street": "中关村", "zip": "100080" }
    });

    let user2 = json!({
        "_id": "user_002",
        "name": "李四",
        "email": "lisi@example.com",
        "age": 32,
        "tags": ["Python", "数据分析"],
        "address": { "city": "上海", "street": "浦东新区", "zip": "200120" }
    });

    let user3 = json!({
        "_id": "user_003",
        "name": "王五",
        "email": "wangwu@example.com",
        "age": 25,
        "tags": ["Rust", "前端"],
        "address": { "city": "北京", "street": "望京", "zip": "100102" }
    });

    users_collection.push(user1);
    users_collection.push(user2);
    users_collection.push(user3);

    println!("  insert_one: 张三");
    println!("  insert_many: [李四, 王五]");
    println!("  集合文档数: {}", users_collection.len());
    println!();

    // === FIND（查询）===
    println!("[FIND] 查询文档：");

    // 模拟 find_one: { "_id": "user_001" }
    let found = users_collection
        .iter()
        .find(|doc| doc["_id"] == "user_001");

    if let Some(user) = found {
        println!("  find_one({{\"_id\": \"user_001\"}}) →");
        println!("    name: {}, age: {}", user["name"], user["age"]);
    }

    // 模拟 find: { "address.city": "北京" }
    let beijing_users: Vec<&Value> = users_collection
        .iter()
        .filter(|doc| doc["address"]["city"] == "北京")
        .collect();

    println!(
        "  find({{\"address.city\": \"北京\"}}) → {} 个结果",
        beijing_users.len()
    );
    for u in &beijing_users {
        println!("    - {}", u["name"]);
    }

    // 模拟 find: { "age": { "$gt": 26 } }
    let older_users: Vec<&Value> = users_collection
        .iter()
        .filter(|doc| doc["age"].as_u64().unwrap_or(0) > 26)
        .collect();

    println!(
        "  find({{\"age\": {{\"$gt\": 26}}}}) → {} 个结果",
        older_users.len()
    );
    for u in &older_users {
        println!("    - {} ({}岁)", u["name"], u["age"]);
    }

    // 模拟 find: { "tags": "Rust" }
    let rust_users: Vec<&Value> = users_collection
        .iter()
        .filter(|doc| {
            doc["tags"]
                .as_array()
                .map_or(false, |tags| tags.contains(&json!("Rust")))
        })
        .collect();

    println!(
        "  find({{\"tags\": \"Rust\"}}) → {} 个结果",
        rust_users.len()
    );
    for u in &rust_users {
        println!("    - {}", u["name"]);
    }
    println!();

    // === UPDATE（更新）===
    println!("[UPDATE] 更新文档：");

    // 模拟 update_one: { "_id": "user_001" }, { "$set": { "age": 29 } }
    let users_collection = db.get_mut("users").unwrap();
    for doc in users_collection.iter_mut() {
        if doc["_id"] == "user_001" {
            doc["age"] = json!(29);
            println!("  update_one({{\"_id\": \"user_001\"}}, {{\"$set\": {{\"age\": 29}}}})");
            println!("    更新后: name={}, age={}", doc["name"], doc["age"]);
            break;
        }
    }

    // 模拟 update_one: $push 添加数组元素
    for doc in users_collection.iter_mut() {
        if doc["_id"] == "user_001" {
            if let Some(tags) = doc["tags"].as_array_mut() {
                tags.push(json!("数据库"));
            }
            println!(
                "  update_one({{\"_id\": \"user_001\"}}, {{\"$push\": {{\"tags\": \"数据库\"}}}})"
            );
            println!("    tags: {}", doc["tags"]);
            break;
        }
    }
    println!();

    // === DELETE（删除）===
    println!("[DELETE] 删除文档：");

    let users_collection = db.get_mut("users").unwrap();
    let before_count = users_collection.len();
    users_collection.retain(|doc| doc["_id"] != "user_003");
    let deleted_count = before_count - users_collection.len();

    println!(
        "  delete_one({{\"_id\": \"user_003\"}}) → 删除了 {} 个文档",
        deleted_count
    );
    println!("  集合剩余文档数: {}", users_collection.len());
    println!();

    // === 最终状态 ===
    println!("[最终状态] 所有用户：");
    for doc in db.get("users").unwrap() {
        println!(
            "  {} - {} ({}岁) @ {}",
            doc["_id"], doc["name"], doc["age"], doc["address"]["city"]
        );
    }
    println!();

    // -------------------------------------------------------
    // mongodb crate CRUD 代码示例：
    //
    // ```rust
    // use mongodb::{Client, options::ClientOptions, bson::doc};
    //
    // // 连接
    // let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    // let db = client.database("mydb");
    // let collection = db.collection::<User>("users");
    //
    // // INSERT
    // let user = User { name: "张三".into(), age: 28, ... };
    // collection.insert_one(user).await?;
    //
    // let users = vec![user1, user2, user3];
    // collection.insert_many(users).await?;
    //
    // // FIND
    // let filter = doc! { "age": { "$gt": 25 } };
    // let mut cursor = collection.find(filter).await?;
    // while let Some(user) = cursor.try_next().await? {
    //     println!("{:?}", user);
    // }
    //
    // // FIND ONE
    // let user = collection.find_one(doc! { "_id": oid }).await?;
    //
    // // UPDATE
    // collection.update_one(
    //     doc! { "_id": oid },
    //     doc! { "$set": { "age": 29 } },
    // ).await?;
    //
    // // DELETE
    // collection.delete_one(doc! { "_id": oid }).await?;
    // collection.delete_many(doc! { "age": { "$lt": 18 } }).await?;
    // ```
    // -------------------------------------------------------
}

/// 4. 查询操作符
fn query_operators() {
    println!("--- 4. MongoDB 查询操作符 ---\n");

    println!("比较操作符：");
    println!("  $eq    等于           {{\"age\": {{\"$eq\": 28}}}}");
    println!("  $ne    不等于          {{\"status\": {{\"$ne\": \"deleted\"}}}}");
    println!("  $gt    大于           {{\"age\": {{\"$gt\": 18}}}}");
    println!("  $gte   大于等于        {{\"age\": {{\"$gte\": 18}}}}");
    println!("  $lt    小于           {{\"age\": {{\"$lt\": 65}}}}");
    println!("  $lte   小于等于        {{\"age\": {{\"$lte\": 65}}}}");
    println!("  $in    在列表中        {{\"status\": {{\"$in\": [\"active\", \"pending\"]}}}}");
    println!("  $nin   不在列表中      {{\"status\": {{\"$nin\": [\"deleted\"]}}}}");
    println!();

    println!("逻辑操作符：");
    println!("  $and   逻辑与    {{\"$and\": [{{\"age\": {{\"$gt\": 18}}}}, {{\"age\": {{\"$lt\": 65}}}}]}}");
    println!("  $or    逻辑或    {{\"$or\": [{{\"city\": \"北京\"}}, {{\"city\": \"上海\"}}]}}");
    println!("  $not   逻辑非    {{\"age\": {{\"$not\": {{\"$gt\": 65}}}}}}");
    println!("  $nor   都不是    {{\"$nor\": [{{\"status\": \"deleted\"}}, {{\"banned\": true}}]}}");
    println!();

    println!("数组操作符：");
    println!("  $all       包含所有     {{\"tags\": {{\"$all\": [\"Rust\", \"后端\"]}}}}");
    println!("  $elemMatch 元素匹配     {{\"scores\": {{\"$elemMatch\": {{\"$gt\": 80}}}}}}");
    println!("  $size      数组长度     {{\"tags\": {{\"$size\": 3}}}}");
    println!();

    println!("更新操作符：");
    println!("  $set       设置字段     {{\"$set\": {{\"name\": \"新名字\"}}}}");
    println!("  $unset     删除字段     {{\"$unset\": {{\"temp_field\": 1}}}}");
    println!("  $inc       递增         {{\"$inc\": {{\"likes\": 1}}}}");
    println!("  $push      数组追加     {{\"$push\": {{\"tags\": \"新标签\"}}}}");
    println!("  $pull      数组移除     {{\"$pull\": {{\"tags\": \"旧标签\"}}}}");
    println!("  $addToSet  数组去重添加  {{\"$addToSet\": {{\"tags\": \"标签\"}}}}");
    println!();

    // 用 serde_json 模拟查询构造
    let complex_query = json!({
        "$and": [
            { "age": { "$gte": 18, "$lte": 65 } },
            { "$or": [
                { "address.city": "北京" },
                { "address.city": "上海" }
            ]},
            { "tags": { "$in": ["Rust", "Go"] } }
        ]
    });

    println!("复合查询示例：");
    println!(
        "{}",
        serde_json::to_string_pretty(&complex_query).unwrap()
    );
    println!();
}

/// 5. 聚合管道
fn aggregation_pipeline() {
    println!("--- 5. 聚合管道（Aggregation Pipeline）---\n");

    // 模拟文章数据
    let posts = vec![
        json!({"author": "张三", "category": "技术", "likes": 42, "tags": ["Rust", "后端"]}),
        json!({"author": "张三", "category": "技术", "likes": 18, "tags": ["Rust", "数据库"]}),
        json!({"author": "李四", "category": "生活", "likes": 25, "tags": ["旅行"]}),
        json!({"author": "李四", "category": "技术", "likes": 30, "tags": ["Python"]}),
        json!({"author": "王五", "category": "技术", "likes": 55, "tags": ["Rust", "系统编程"]}),
    ];

    println!("示例数据（文章集合）：");
    for post in &posts {
        println!(
            "  作者: {}, 分类: {}, 点赞: {}",
            post["author"], post["category"], post["likes"]
        );
    }
    println!();

    // 模拟聚合：按作者分组统计
    println!("聚合示例 1：按作者统计文章数和总点赞");
    println!("  管道: $match → $group");

    let mut author_stats: HashMap<&str, (u32, u64)> = HashMap::new();
    for post in &posts {
        let author = post["author"].as_str().unwrap();
        let likes = post["likes"].as_u64().unwrap();
        let entry = author_stats.entry(author).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += likes;
    }

    for (author, (count, total_likes)) in &author_stats {
        println!(
            "  {} → 文章数: {}, 总点赞: {}",
            author, count, total_likes
        );
    }
    println!();

    // 模拟聚合：筛选 + 统计
    println!("聚合示例 2：技术文章的平均点赞数");
    println!("  管道: $match(category=技术) → $group(_id=null, avg)");

    let tech_posts: Vec<&Value> = posts
        .iter()
        .filter(|p| p["category"] == "技术")
        .collect();
    let avg_likes: f64 = tech_posts
        .iter()
        .map(|p| p["likes"].as_f64().unwrap())
        .sum::<f64>()
        / tech_posts.len() as f64;

    println!("  技术文章数: {}, 平均点赞: {:.1}", tech_posts.len(), avg_likes);
    println!();

    // -------------------------------------------------------
    // mongodb crate 聚合管道示例：
    //
    // ```rust
    // use mongodb::bson::doc;
    //
    // let pipeline = vec![
    //     // Stage 1: 过滤
    //     doc! { "$match": { "category": "技术" } },
    //
    //     // Stage 2: 分组统计
    //     doc! { "$group": {
    //         "_id": "$author",
    //         "post_count": { "$sum": 1 },
    //         "avg_likes": { "$avg": "$likes" },
    //         "total_likes": { "$sum": "$likes" }
    //     }},
    //
    //     // Stage 3: 排序
    //     doc! { "$sort": { "total_likes": -1 } },
    //
    //     // Stage 4: 限制结果数
    //     doc! { "$limit": 10 },
    //
    //     // Stage 5: 重塑输出
    //     doc! { "$project": {
    //         "author": "$_id",
    //         "post_count": 1,
    //         "avg_likes": { "$round": ["$avg_likes", 1] },
    //         "_id": 0
    //     }}
    // ];
    //
    // let mut cursor = collection.aggregate(pipeline).await?;
    // while let Some(doc) = cursor.try_next().await? {
    //     println!("{:?}", doc);
    // }
    // ```
    // -------------------------------------------------------

    println!("常用聚合阶段：");
    println!("  $match    过滤文档（类似 WHERE）");
    println!("  $group    分组聚合（类似 GROUP BY）");
    println!("  $sort     排序");
    println!("  $limit    限制结果数");
    println!("  $skip     跳过文档数");
    println!("  $project  选择/重塑字段");
    println!("  $unwind   展开数组字段");
    println!("  $lookup   关联查询（类似 LEFT JOIN）");
    println!("  $count    计数");
    println!("  $addFields 添加计算字段");
    println!();
}

/// 6. 索引
fn indexes() {
    println!("--- 6. 索引 ---\n");

    println!("MongoDB 索引类型：");
    println!("  ┌──────────────────┬────────────────────────────────┐");
    println!("  │ 索引类型          │ 说明                           │");
    println!("  ├──────────────────┼────────────────────────────────┤");
    println!("  │ 单字段索引        │ db.users.createIndex({{age: 1}})│");
    println!("  │ 复合索引          │ createIndex({{city: 1, age: -1}})│");
    println!("  │ 唯一索引          │ createIndex({{email: 1}}, {{unique: true}})│");
    println!("  │ 文本索引          │ createIndex({{content: \"text\"}})│");
    println!("  │ 地理空间索引      │ createIndex({{location: \"2dsphere\"}})│");
    println!("  │ 哈希索引          │ createIndex({{field: \"hashed\"}})│");
    println!("  │ TTL 索引          │ 自动过期删除文档                  │");
    println!("  └──────────────────┴────────────────────────────────┘");
    println!();

    // -------------------------------------------------------
    // mongodb crate 索引创建：
    //
    // ```rust
    // use mongodb::IndexModel;
    // use mongodb::options::IndexOptions;
    //
    // // 创建唯一索引
    // let index = IndexModel::builder()
    //     .keys(doc! { "email": 1 })
    //     .options(IndexOptions::builder().unique(true).build())
    //     .build();
    // collection.create_index(index).await?;
    //
    // // 创建复合索引
    // let index = IndexModel::builder()
    //     .keys(doc! { "category": 1, "created_at": -1 })
    //     .build();
    // collection.create_index(index).await?;
    //
    // // 创建 TTL 索引（文档自动过期）
    // let index = IndexModel::builder()
    //     .keys(doc! { "expire_at": 1 })
    //     .options(IndexOptions::builder()
    //         .expire_after(std::time::Duration::from_secs(0))
    //         .build())
    //     .build();
    // collection.create_index(index).await?;
    // ```
    // -------------------------------------------------------

    println!("索引最佳实践：");
    println!("  1. 为常用查询条件创建索引");
    println!("  2. 复合索引遵循 ESR 规则：等值 → 排序 → 范围");
    println!("  3. 避免过多索引（每个索引都会增加写入开销）");
    println!("  4. 使用 explain() 分析查询计划");
    println!("  5. 对需要自动清理的数据使用 TTL 索引");
    println!();
}

/// 7. MongoDB Rust 驱动使用模式
fn mongodb_rust_driver() {
    println!("--- 7. MongoDB Rust 驱动使用模式 ---\n");

    // -------------------------------------------------------
    // 完整使用示例：
    //
    // ```rust
    // use mongodb::{Client, options::ClientOptions, Collection};
    // use mongodb::bson::{doc, oid::ObjectId};
    // use serde::{Serialize, Deserialize};
    // use futures::TryStreamExt;
    //
    // #[derive(Debug, Serialize, Deserialize)]
    // struct User {
    //     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    //     id: Option<ObjectId>,
    //     name: String,
    //     email: String,
    //     age: i32,
    // }
    //
    // #[tokio::main]
    // async fn main() -> mongodb::error::Result<()> {
    //     // 1. 连接
    //     let client_options = ClientOptions::parse(
    //         "mongodb://localhost:27017"
    //     ).await?;
    //     let client = Client::with_options(client_options)?;
    //
    //     // 2. 获取数据库和集合（强类型）
    //     let db = client.database("myapp");
    //     let users: Collection<User> = db.collection("users");
    //
    //     // 3. 插入
    //     let new_user = User {
    //         id: None,
    //         name: "张三".into(),
    //         email: "zhangsan@example.com".into(),
    //         age: 28,
    //     };
    //     let result = users.insert_one(new_user).await?;
    //     println!("插入 ID: {}", result.inserted_id);
    //
    //     // 4. 查询
    //     let filter = doc! {
    //         "age": { "$gte": 18 },
    //         "name": { "$regex": "张", "$options": "i" }
    //     };
    //     let options = mongodb::options::FindOptions::builder()
    //         .sort(doc! { "age": -1 })
    //         .limit(10)
    //         .build();
    //     let mut cursor = users.find(filter)
    //         .with_options(options)
    //         .await?;
    //
    //     while let Some(user) = cursor.try_next().await? {
    //         println!("{:?}", user);
    //     }
    //
    //     // 5. 更新
    //     users.update_one(
    //         doc! { "email": "zhangsan@example.com" },
    //         doc! { "$set": { "age": 29 }, "$push": { "tags": "MongoDB" } },
    //     ).await?;
    //
    //     // 6. 事务
    //     let mut session = client.start_session().await?;
    //     session.start_transaction().await?;
    //
    //     users.insert_one(new_user).session(&mut session).await?;
    //     // ... 更多操作 ...
    //
    //     session.commit_transaction().await?;
    //     // 或 session.abort_transaction().await? 回滚
    //
    //     // 7. 删除
    //     users.delete_one(doc! { "_id": oid }).await?;
    //
    //     Ok(())
    // }
    // ```
    // -------------------------------------------------------

    println!("MongoDB Rust 驱动 API 概览：");
    println!();
    println!("  连接：");
    println!("    Client::with_uri_str(\"mongodb://...\").await?");
    println!("    client.database(\"mydb\").collection::<T>(\"name\")");
    println!();
    println!("  插入：");
    println!("    collection.insert_one(doc).await?");
    println!("    collection.insert_many(docs).await?");
    println!();
    println!("  查询：");
    println!("    collection.find_one(filter).await?");
    println!("    collection.find(filter).await?  → Cursor");
    println!("    cursor.try_next().await?         → Option<T>");
    println!();
    println!("  更新：");
    println!("    collection.update_one(filter, update).await?");
    println!("    collection.update_many(filter, update).await?");
    println!("    collection.replace_one(filter, replacement).await?");
    println!();
    println!("  删除：");
    println!("    collection.delete_one(filter).await?");
    println!("    collection.delete_many(filter).await?");
    println!();
    println!("  事务：");
    println!("    let mut session = client.start_session().await?;");
    println!("    session.start_transaction().await?;");
    println!("    // ... operations with .session(&mut session) ...");
    println!("    session.commit_transaction().await?;");
    println!();
}

/// 8. 数据建模最佳实践
fn data_modeling() {
    println!("--- 8. 数据建模最佳实践 ---\n");

    println!("嵌入 vs 引用的选择：");
    println!();

    // 嵌入式文档示例
    let embedded_example = json!({
        "_id": "order_001",
        "customer": "张三",
        "items": [
            { "product": "Rust 编程", "price": 59.9, "qty": 1 },
            { "product": "数据库原理", "price": 45.0, "qty": 2 }
        ],
        "shipping_address": {
            "city": "北京",
            "street": "中关村大街1号"
        }
    });

    println!("  嵌入式设计（适合一对少、经常一起查询的数据）：");
    println!(
        "  {}",
        serde_json::to_string_pretty(&embedded_example).unwrap()
    );
    println!();

    // 引用式文档示例
    let reference_example = json!({
        "_id": "comment_001",
        "post_id": "post_001",
        "user_id": "user_001",
        "text": "好文章！"
    });

    println!("  引用式设计（适合一对多、数据独立增长的场景）：");
    println!(
        "  {}",
        serde_json::to_string_pretty(&reference_example).unwrap()
    );
    println!();

    println!("选择策略：");
    println!("  ┌──────────────────┬──────────────┬──────────────┐");
    println!("  │ 考虑因素          │ 选择嵌入      │ 选择引用      │");
    println!("  ├──────────────────┼──────────────┼──────────────┤");
    println!("  │ 数据量            │ 少量(≤100)   │ 大量/无限增长 │");
    println!("  │ 查询模式          │ 一起查询      │ 独立查询      │");
    println!("  │ 更新频率          │ 很少更新      │ 频繁更新      │");
    println!("  │ 数据关系          │ 一对一/一对少  │ 一对多/多对多 │");
    println!("  │ 一致性要求        │ 强一致        │ 最终一致可接受│");
    println!("  └──────────────────┴──────────────┴──────────────┘");
    println!();

    println!("MongoDB vs 关系型数据库选型：");
    println!("  适合 MongoDB 的场景：");
    println!("    • 灵活的文档结构（Schema 经常变化）");
    println!("    • 层次化数据（嵌套文档和数组）");
    println!("    • 大数据量、高吞吐（水平扩展）");
    println!("    • 地理空间查询、全文搜索");
    println!("    • 日志和事件存储");
    println!();
    println!("  适合关系型数据库的场景：");
    println!("    • 强一致性和事务要求");
    println!("    • 复杂的多表 JOIN 查询");
    println!("    • 固定 Schema 的结构化数据");
    println!("    • 严格的数据完整性约束");
    println!();
}
