// ============================================================
// Lesson 093: SQLx 基础
// ============================================================
// 本课介绍 Rust 生态中最流行的异步数据库库之一 —— SQLx。
//
// SQLx 的核心特点：
// 1. **编译时检查 SQL**：通过 sqlx::query! 宏可以在编译时验证 SQL 语句
// 2. **纯 Rust 异步**：原生支持 async/await，无需 ORM 层
// 3. **多数据库支持**：支持 PostgreSQL、MySQL、SQLite、MSSQL
// 4. **零成本抽象**：直接执行 SQL，不生成中间查询层
//
// 注意：SQLx 编译需要 C 编译工具链（编译 SQLite C 源码等），
// 因此本课以概念讲解和代码展示为主，所有 SQLx 代码在注释中展示。
// 实际使用时，在 Cargo.toml 中添加：
//
// ```toml
// [dependencies]
// sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
// tokio = { version = "1", features = ["full"] }
// ```
// ============================================================

use std::collections::HashMap;

fn main() {
    println!("=== Lesson 093: SQLx 基础 ===\n");

    // ============================================================
    // 1. SQLx 项目配置
    // ============================================================
    // Cargo.toml 配置示例：
    //
    // [dependencies]
    // # SQLite 支持
    // sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite"] }
    //
    // # PostgreSQL 支持
    // sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
    //
    // # MySQL 支持
    // sqlx = { version = "0.8", features = ["runtime-tokio", "mysql"] }
    //
    // # 通用依赖
    // tokio = { version = "1", features = ["full"] }

    println!("--- 1. SQLx 项目配置 ---");
    println!("SQLx 通过 feature flags 选择数据库后端：");
    println!("  sqlite   → 嵌入式数据库，无需安装服务");
    println!("  postgres → PostgreSQL，适合生产环境");
    println!("  mysql    → MySQL/MariaDB");
    println!();

    // ============================================================
    // 2. 连接数据库（连接池）
    // ============================================================
    // SQLx 使用连接池管理数据库连接，支持并发访问。
    //
    // ```rust
    // use sqlx::sqlite::SqlitePool;
    //
    // #[tokio::main]
    // async fn main() -> Result<(), sqlx::Error> {
    //     // SQLite 内存数据库（无需文件）
    //     let pool = SqlitePool::connect("sqlite::memory:").await?;
    //
    //     // SQLite 文件数据库
    //     let pool = SqlitePool::connect("sqlite://my_database.db?mode=rwc").await?;
    //
    //     // PostgreSQL
    //     let pool = PgPool::connect("postgres://user:pass@localhost/dbname").await?;
    //
    //     Ok(())
    // }
    // ```

    println!("--- 2. 连接数据库 ---");
    println!("  SqlitePool::connect(\"sqlite::memory:\")  → 内存数据库");
    println!("  SqlitePool::connect(\"sqlite://app.db\")   → 文件数据库");
    println!("  PgPool::connect(\"postgres://...\")         → PostgreSQL");
    println!();

    // ============================================================
    // 3. 定义数据模型（FromRow）
    // ============================================================
    // 使用 #[derive(sqlx::FromRow)] 自动将查询结果映射到结构体。
    //
    // ```rust
    // use sqlx::FromRow;
    //
    // #[derive(Debug, FromRow)]
    // struct User {
    //     id: i64,
    //     name: String,
    //     email: String,
    //     age: i64,
    // }
    //
    // // 可以使用 #[sqlx(rename = "...")] 重命名字段
    // #[derive(Debug, FromRow)]
    // struct Post {
    //     id: i64,
    //     #[sqlx(rename = "user_id")]
    //     author_id: i64,
    //     title: String,
    //     content: String,
    // }
    // ```

    // 我们用普通结构体模拟数据模型
    #[derive(Debug, Clone)]
    struct User {
        id: i64,
        name: String,
        email: String,
        age: i64,
    }

    println!("--- 3. 数据模型 ---");
    println!("  #[derive(Debug, FromRow)]");
    println!("  struct User {{ id: i64, name: String, email: String, age: i64 }}");
    println!("  → FromRow 自动将数据库行映射为结构体");
    println!();

    // ============================================================
    // 4. 创建表（DDL）
    // ============================================================
    // ```rust
    // sqlx::query(
    //     "CREATE TABLE users (
    //         id INTEGER PRIMARY KEY AUTOINCREMENT,
    //         name TEXT NOT NULL,
    //         email TEXT NOT NULL UNIQUE,
    //         age INTEGER NOT NULL
    //     )"
    // )
    // .execute(&pool)
    // .await?;
    // ```

    println!("--- 4. 创建表 ---");
    println!("  sqlx::query(\"CREATE TABLE ...\").execute(&pool).await?;");
    println!("  → 使用原始 SQL 创建表结构");
    println!();

    // ============================================================
    // 5. 插入数据（参数绑定）
    // ============================================================
    // 使用 .bind() 进行参数绑定，防止 SQL 注入：
    //
    // ```rust
    // let result = sqlx::query(
    //     "INSERT INTO users (name, email, age) VALUES (?, ?, ?)"
    // )
    // .bind("张三")
    // .bind("zhangsan@example.com")
    // .bind(28)
    // .execute(&pool)
    // .await?;
    //
    // let id = result.last_insert_rowid();
    // println!("新用户 ID: {}", id);
    // ```
    //
    // 注意：
    // - SQLite 使用 ? 作为占位符
    // - PostgreSQL 使用 $1, $2, $3...
    // - MySQL 使用 ?

    println!("--- 5. 插入数据 ---");
    println!("  sqlx::query(\"INSERT INTO users (name, email, age) VALUES (?, ?, ?)\")");
    println!("      .bind(\"张三\")");
    println!("      .bind(\"zhangsan@example.com\")");
    println!("      .bind(28)");
    println!("      .execute(&pool).await?;");
    println!("  → .bind() 防止 SQL 注入，按顺序绑定参数");
    println!();

    // 用 HashMap 模拟数据库
    let mut db: HashMap<i64, User> = HashMap::new();
    let users_data = vec![
        User { id: 1, name: "张三".into(), email: "zhangsan@example.com".into(), age: 28 },
        User { id: 2, name: "李四".into(), email: "lisi@example.com".into(), age: 32 },
        User { id: 3, name: "王五".into(), email: "wangwu@example.com".into(), age: 25 },
    ];
    for user in &users_data {
        db.insert(user.id, user.clone());
    }
    println!("  [模拟] 已插入 {} 条用户数据", db.len());
    println!();

    // ============================================================
    // 6. 查询数据 —— query_as 与 fetch 系列
    // ============================================================
    // ```rust
    // // 查询所有用户
    // let users = sqlx::query_as::<_, User>(
    //     "SELECT id, name, email, age FROM users"
    // )
    // .fetch_all(&pool)
    // .await?;
    //
    // // 条件查询
    // let adults = sqlx::query_as::<_, User>(
    //     "SELECT id, name, email, age FROM users WHERE age > ?"
    // )
    // .bind(26)
    // .fetch_all(&pool)
    // .await?;
    //
    // // 查询单条记录
    // let user = sqlx::query_as::<_, User>(
    //     "SELECT id, name, email, age FROM users WHERE id = ?"
    // )
    // .bind(1)
    // .fetch_one(&pool)        // 必须恰好1条，否则报错
    // .await?;
    //
    // // 查询可能不存在的记录
    // let maybe_user = sqlx::query_as::<_, User>(
    //     "SELECT id, name, email, age FROM users WHERE email = ?"
    // )
    // .bind("nobody@example.com")
    // .fetch_optional(&pool)   // 返回 Option<User>
    // .await?;
    // ```

    println!("--- 6. 查询数据 ---");
    println!("  fetch_all(&pool)      → 返回 Vec<T>，获取所有匹配行");
    println!("  fetch_one(&pool)      → 返回 T，必须恰好 1 行");
    println!("  fetch_optional(&pool) → 返回 Option<T>，0 或 1 行");
    println!();

    // 模拟查询
    println!("  [模拟] 所有用户:");
    for user in db.values() {
        println!("    ID={}, 姓名={}, 邮箱={}, 年龄={}",
            user.id, user.name, user.email, user.age);
    }
    println!();

    // 模拟条件查询
    println!("  [模拟] 年龄 > 26 的用户:");
    for user in db.values().filter(|u| u.age > 26) {
        println!("    {}: {} 岁", user.name, user.age);
    }
    println!();

    // ============================================================
    // 7. 使用 Row trait 手动提取字段
    // ============================================================
    // 如果不想定义结构体，可以用 Row::get() 按列名提取：
    //
    // ```rust
    // use sqlx::Row;
    //
    // let rows = sqlx::query(
    //     "SELECT u.name, COUNT(p.id) as post_count
    //      FROM users u
    //      LEFT JOIN posts p ON u.id = p.user_id
    //      GROUP BY u.id"
    // )
    // .fetch_all(&pool)
    // .await?;
    //
    // for row in &rows {
    //     let name: String = row.get("name");         // 按列名
    //     let count: i32 = row.get(1);                // 按索引
    //     println!("{} 发布了 {} 篇文章", name, count);
    // }
    // ```

    println!("--- 7. Row::get 手动提取 ---");
    println!("  row.get(\"name\")  → 按列名提取");
    println!("  row.get(0)       → 按列索引提取");
    println!("  → 适合 JOIN 查询或不想定义结构体的场景");
    println!();

    // ============================================================
    // 8. 更新与删除
    // ============================================================
    // ```rust
    // // 更新
    // let result = sqlx::query("UPDATE users SET age = ? WHERE name = ?")
    //     .bind(29)
    //     .bind("张三")
    //     .execute(&pool)
    //     .await?;
    // println!("更新了 {} 行", result.rows_affected());
    //
    // // 删除
    // let result = sqlx::query("DELETE FROM users WHERE id = ?")
    //     .bind(3)
    //     .execute(&pool)
    //     .await?;
    // println!("删除了 {} 行", result.rows_affected());
    // ```

    println!("--- 8. 更新与删除 ---");
    println!("  sqlx::query(\"UPDATE ...\").bind(...).execute(&pool).await?;");
    println!("  sqlx::query(\"DELETE ...\").bind(...).execute(&pool).await?;");
    println!("  → result.rows_affected() 返回受影响的行数");
    println!();

    // 模拟更新
    if let Some(user) = db.get_mut(&1) {
        user.age = 29;
        println!("  [模拟] 更新张三年龄为 {}", user.age);
    }
    // 模拟删除
    if let Some(user) = db.remove(&3) {
        println!("  [模拟] 删除用户: {}", user.name);
    }
    println!();

    // ============================================================
    // 9. 事务处理（Transaction）
    // ============================================================
    // 事务确保一组操作要么全部成功，要么全部回滚：
    //
    // ```rust
    // // 开始事务
    // let mut tx = pool.begin().await?;
    //
    // // 在事务中执行操作（注意使用 &mut *tx 而非 &pool）
    // sqlx::query("INSERT INTO users (name, email, age) VALUES (?, ?, ?)")
    //     .bind("赵六")
    //     .bind("zhaoliu@example.com")
    //     .bind(30)
    //     .execute(&mut *tx)
    //     .await?;
    //
    // sqlx::query("INSERT INTO posts (user_id, title, content) VALUES (last_insert_rowid(), ?, ?)")
    //     .bind("赵六的文章")
    //     .bind("内容...")
    //     .execute(&mut *tx)
    //     .await?;
    //
    // // 提交事务（两个 INSERT 一起生效）
    // tx.commit().await?;
    //
    // // 如果不调用 commit()，tx 被 drop 时自动回滚
    // // 也可以显式调用 tx.rollback().await?;
    // ```

    println!("--- 9. 事务处理 ---");
    println!("  let mut tx = pool.begin().await?;      // 开始事务");
    println!("  sqlx::query(...).execute(&mut *tx).await?; // 事务中执行");
    println!("  tx.commit().await?;                     // 提交");
    println!("  // drop(tx) 或 tx.rollback().await?     // 回滚");
    println!("  → 确保一组操作的原子性");
    println!();

    // ============================================================
    // 10. 编译时 SQL 检查（sqlx::query! 宏）
    // ============================================================
    // SQLx 最强大的功能之一是编译时 SQL 检查：
    //
    // ```rust
    // // 需要设置环境变量 DATABASE_URL 指向真实数据库
    // // export DATABASE_URL="sqlite://my_database.db"
    //
    // // sqlx::query! 在编译时连接数据库验证 SQL
    // let users = sqlx::query!(
    //     "SELECT id, name, email FROM users WHERE age > ?",
    //     26
    // )
    // .fetch_all(&pool)
    // .await?;
    //
    // for user in users {
    //     // 编译器知道 user.id 是 i64, user.name 是 String
    //     println!("{}: {}", user.id, user.name);
    // }
    //
    // // 如果 SQL 有语法错误或列名错误，编译时就会报错！
    //
    // // 离线模式：
    // // 1. cargo sqlx prepare   → 生成 .sqlx/ 目录
    // // 2. 提交 .sqlx/ 到 git   → CI 不需要真实数据库
    // ```

    println!("--- 10. 编译时 SQL 检查 ---");
    println!("  sqlx::query!(\"SELECT id, name FROM users WHERE age > ?\", 26)");
    println!("  → 编译时验证 SQL 语法和列名！");
    println!("  → 需要 DATABASE_URL 环境变量");
    println!("  → cargo sqlx prepare 可生成离线检查数据");
    println!();

    // ============================================================
    // 11. 常用 API 速查表
    // ============================================================
    println!("--- 11. SQLx 常用 API 速查表 ---");
    println!("┌──────────────────────────────────┬────────────────────────────────┐");
    println!("│ 操作                             │ API                            │");
    println!("├──────────────────────────────────┼────────────────────────────────┤");
    println!("│ 创建连接池                       │ SqlitePool::connect(url)       │");
    println!("│ 执行 SQL（无返回）               │ query(sql).execute(&pool)      │");
    println!("│ 查询映射到结构体                 │ query_as::<_, T>(sql)          │");
    println!("│ 获取所有结果                     │ .fetch_all(&pool)              │");
    println!("│ 获取一条结果                     │ .fetch_one(&pool)              │");
    println!("│ 获取可选结果                     │ .fetch_optional(&pool)         │");
    println!("│ 参数绑定                         │ .bind(value)                   │");
    println!("│ 影响行数                         │ result.rows_affected()         │");
    println!("│ 最后插入 ID                      │ result.last_insert_rowid()     │");
    println!("│ 开始事务                         │ pool.begin().await?            │");
    println!("│ 提交事务                         │ tx.commit().await?             │");
    println!("│ 编译时检查                       │ query!(sql, args...)           │");
    println!("│ 手动提取列值                     │ row.get(\"column\")              │");
    println!("└──────────────────────────────────┴────────────────────────────────┘");
    println!();

    // ============================================================
    // 12. 最终数据验证
    // ============================================================
    println!("--- 12. [模拟] 最终数据库状态 ---");
    let mut users: Vec<&User> = db.values().collect();
    users.sort_by_key(|u| u.id);
    for user in users {
        println!("  ID={}, 姓名={}, 邮箱={}, 年龄={}",
            user.id, user.name, user.email, user.age);
    }

    // ============================================================
    // 总结
    // ============================================================
    println!("\n=== SQLx 基础要点总结 ===");
    println!("1. SqlitePool::connect() 创建异步连接池");
    println!("2. sqlx::query() 执行原始 SQL");
    println!("3. sqlx::query_as::<_, T>() 将结果映射到结构体");
    println!("4. #[derive(FromRow)] 自动实现行到结构体的转换");
    println!("5. .bind() 进行参数绑定，防止 SQL 注入");
    println!("6. fetch_all / fetch_one / fetch_optional 获取不同数量的结果");
    println!("7. pool.begin() 开始事务，tx.commit() 提交");
    println!("8. sqlx::query! 宏支持编译时 SQL 检查");

    println!("\n🎉 恭喜！你已经完成了 SQLx 基础课程！");
    println!("💡 实际使用时，请在 Cargo.toml 中添加 sqlx 和 tokio 依赖。");
}
