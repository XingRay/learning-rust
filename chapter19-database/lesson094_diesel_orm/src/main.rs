// ============================================================
// Lesson 094: Diesel ORM（概念讲解）
// ============================================================
// Diesel 是 Rust 生态中最成熟的 ORM（对象关系映射）框架。
// 由于 Diesel 编译需要 C 库依赖（如 libpq、libmysqlclient 或 libsqlite3），
// 本课以概念讲解和注释代码示例为主，不引入实际依赖。
//
// Diesel 的核心理念：
// 1. **编译时安全**：查询在编译时检查类型和 SQL 正确性
// 2. **零成本抽象**：生成的 SQL 与手写 SQL 性能相同
// 3. **可组合查询**：查询可以像积木一样组合拼装
// 4. **类型安全的 Schema**：数据库模式在 Rust 类型系统中表达
// ============================================================

fn main() {
    println!("=== Lesson 094: Diesel ORM（概念讲解）===\n");

    // ============================================================
    // 1. Diesel 架构概览
    // ============================================================
    diesel_architecture();

    // ============================================================
    // 2. Schema 定义
    // ============================================================
    schema_definition();

    // ============================================================
    // 3. 模型定义
    // ============================================================
    model_definition();

    // ============================================================
    // 4. CRUD 操作
    // ============================================================
    crud_operations();

    // ============================================================
    // 5. 查询 DSL
    // ============================================================
    query_dsl();

    // ============================================================
    // 6. 数据库迁移
    // ============================================================
    database_migrations();

    // ============================================================
    // 7. 关联查询与高级特性
    // ============================================================
    advanced_features();

    // ============================================================
    // 8. Diesel vs SQLx 对比
    // ============================================================
    comparison_with_sqlx();

    println!("\n=== Diesel ORM 概念讲解完毕 ===");
}

/// 1. Diesel 架构概览
fn diesel_architecture() {
    println!("--- 1. Diesel 架构概览 ---\n");

    println!("Diesel 的分层架构：");
    println!("  ┌─────────────────────────────────────┐");
    println!("  │         应用层 (Your Code)            │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │       查询 DSL (Query DSL)           │");
    println!("  │   类型安全的查询构建器                  │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │       Schema 层 (table! 宏)          │");
    println!("  │   数据库表结构的 Rust 类型表达          │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │       连接层 (Connection)             │");
    println!("  │   PostgreSQL / MySQL / SQLite        │");
    println!("  └─────────────────────────────────────┘");

    println!("\n核心组件：");
    println!("  • diesel CLI：命令行工具，管理迁移和 schema");
    println!("  • table! 宏：从 schema.rs 生成类型安全的表定义");
    println!("  • derive 宏：Queryable, Insertable, Identifiable 等");
    println!("  • QueryDsl：提供 filter, order, limit 等查询方法");

    // Cargo.toml 依赖配置示例：
    //
    // [dependencies]
    // diesel = { version = "2.2", features = ["sqlite", "r2d2"] }
    // dotenvy = "0.15"
    //
    // # 安装 diesel CLI:
    // # cargo install diesel_cli --no-default-features --features sqlite

    println!();
}

/// 2. Schema 定义
fn schema_definition() {
    println!("--- 2. Schema 定义 ---\n");

    println!("Diesel 使用 table! 宏定义数据库 schema：");
    println!();

    // -------------------------------------------------------
    // Diesel 的 schema.rs 文件由 diesel CLI 自动生成：
    //
    // ```rust
    // // src/schema.rs（由 diesel print-schema 自动生成）
    //
    // diesel::table! {
    //     users (id) {
    //         id -> Integer,
    //         name -> Text,
    //         email -> Text,
    //         age -> Integer,
    //         created_at -> Timestamp,
    //     }
    // }
    //
    // diesel::table! {
    //     posts (id) {
    //         id -> Integer,
    //         user_id -> Integer,
    //         title -> Text,
    //         body -> Text,
    //         published -> Bool,
    //     }
    // }
    //
    // // 定义表之间的关联
    // diesel::joinable!(posts -> users (user_id));
    // diesel::allow_tables_to_appear_in_same_query!(users, posts);
    // ```
    // -------------------------------------------------------

    println!("  table! 宏做了什么？");
    println!("  1. 创建一个模块（如 users::），包含所有列的类型信息");
    println!("  2. 每个列变成一个类型安全的标识符（如 users::name）");
    println!("  3. 列类型映射：Text → String, Integer → i32, Bool → bool");
    println!("  4. 主键信息编码在类型系统中");
    println!();

    println!("Diesel 类型映射（SQLite）：");
    println!("  ┌──────────────────┬───────────────────┐");
    println!("  │ Diesel 类型       │ Rust 类型          │");
    println!("  ├──────────────────┼───────────────────┤");
    println!("  │ Integer          │ i32               │");
    println!("  │ BigInt           │ i64               │");
    println!("  │ Text             │ String            │");
    println!("  │ Bool             │ bool              │");
    println!("  │ Float            │ f32               │");
    println!("  │ Double           │ f64               │");
    println!("  │ Timestamp        │ NaiveDateTime     │");
    println!("  │ Nullable<T>      │ Option<T>         │");
    println!("  └──────────────────┴───────────────────┘");
    println!();
}

/// 3. 模型定义
fn model_definition() {
    println!("--- 3. 模型定义 ---\n");

    // -------------------------------------------------------
    // 在 Diesel 中，模型通常分为两个结构体：
    //
    // ```rust
    // use diesel::prelude::*;
    // use crate::schema::users;
    //
    // // 查询模型 —— 从数据库读取数据时使用
    // #[derive(Queryable, Selectable, Debug)]
    // #[diesel(table_name = users)]
    // #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    // pub struct User {
    //     pub id: i32,
    //     pub name: String,
    //     pub email: String,
    //     pub age: i32,
    //     pub created_at: String,
    // }
    //
    // // 插入模型 —— 向数据库写入数据时使用（不含自增 ID）
    // #[derive(Insertable)]
    // #[diesel(table_name = users)]
    // pub struct NewUser<'a> {
    //     pub name: &'a str,
    //     pub email: &'a str,
    //     pub age: i32,
    // }
    //
    // // 更新模型 —— 使用 AsChangeset 支持部分更新
    // #[derive(AsChangeset)]
    // #[diesel(table_name = users)]
    // pub struct UpdateUser<'a> {
    //     pub name: Option<&'a str>,
    //     pub email: Option<&'a str>,
    //     pub age: Option<i32>,
    // }
    // ```
    // -------------------------------------------------------

    // 用普通结构体模拟 Diesel 模型
    #[derive(Debug)]
    #[allow(dead_code)]
    struct User {
        id: i32,
        name: String,
        email: String,
        age: i32,
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    struct NewUser {
        name: String,
        email: String,
        age: i32,
    }

    println!("Diesel 模型的关键 derive 宏：");
    println!("  • Queryable    - 从数据库行反序列化（查询结果 → 结构体）");
    println!("  • Selectable   - 自动选择需要的列，避免 SELECT *");
    println!("  • Insertable   - 序列化为可插入数据库的格式");
    println!("  • Identifiable - 标记拥有主键的模型（用于更新/删除）");
    println!("  • AsChangeset  - 支持部分更新（Option 字段为 None 时不更新）");
    println!("  • Associations - 定义模型间的关联关系");
    println!();

    // 模拟创建模型实例
    let new_user = NewUser {
        name: "张三".to_string(),
        email: "zhangsan@example.com".to_string(),
        age: 28,
    };
    println!("  创建新用户模型: {:?}", new_user);

    let user = User {
        id: 1,
        name: new_user.name,
        email: new_user.email,
        age: new_user.age,
    };
    println!("  查询到的用户模型: {:?}", user);
    println!();
}

/// 4. CRUD 操作
fn crud_operations() {
    println!("--- 4. CRUD 操作（Diesel 代码示例）---\n");

    // -------------------------------------------------------
    // Diesel 的 CRUD 操作示例：
    //
    // ```rust
    // use diesel::prelude::*;
    // use crate::schema::users::dsl::*;
    //
    // // === 建立连接 ===
    // let database_url = "database.db";
    // let mut conn = SqliteConnection::establish(&database_url)
    //     .expect("连接数据库失败");
    //
    // // === CREATE（插入）===
    // let new_user = NewUser {
    //     name: "张三",
    //     email: "zhangsan@example.com",
    //     age: 28,
    // };
    //
    // diesel::insert_into(users)
    //     .values(&new_user)
    //     .execute(&mut conn)?;
    //
    // // 批量插入
    // let new_users = vec![
    //     NewUser { name: "李四", email: "lisi@example.com", age: 30 },
    //     NewUser { name: "王五", email: "wangwu@example.com", age: 25 },
    // ];
    // diesel::insert_into(users)
    //     .values(&new_users)
    //     .execute(&mut conn)?;
    //
    // // === READ（查询）===
    // // 查询所有用户
    // let all_users = users
    //     .load::<User>(&mut conn)?;
    //
    // // 条件查询
    // let adult_users = users
    //     .filter(age.ge(18))
    //     .order(name.asc())
    //     .limit(10)
    //     .load::<User>(&mut conn)?;
    //
    // // 查询单条
    // let user = users
    //     .find(1)  // 按主键查找
    //     .first::<User>(&mut conn)?;
    //
    // // === UPDATE（更新）===
    // diesel::update(users.find(1))
    //     .set(age.eq(29))
    //     .execute(&mut conn)?;
    //
    // // 使用 AsChangeset 部分更新
    // let changes = UpdateUser {
    //     name: Some("张三丰"),
    //     email: None,  // 不更新
    //     age: Some(29),
    // };
    // diesel::update(users.find(1))
    //     .set(&changes)
    //     .execute(&mut conn)?;
    //
    // // === DELETE（删除）===
    // diesel::delete(users.find(1))
    //     .execute(&mut conn)?;
    //
    // // 条件删除
    // diesel::delete(users.filter(age.lt(18)))
    //     .execute(&mut conn)?;
    // ```
    // -------------------------------------------------------

    println!("CREATE（插入）：");
    println!("  diesel::insert_into(users).values(&new_user).execute(&mut conn)?;");
    println!();

    println!("READ（查询）：");
    println!("  users.load::<User>(&mut conn)?;                    // 查询所有");
    println!("  users.filter(age.ge(18)).load::<User>(&mut conn)?; // 条件查询");
    println!("  users.find(1).first::<User>(&mut conn)?;           // 按主键查找");
    println!();

    println!("UPDATE（更新）：");
    println!("  diesel::update(users.find(1)).set(age.eq(29)).execute(&mut conn)?;");
    println!();

    println!("DELETE（删除）：");
    println!("  diesel::delete(users.find(1)).execute(&mut conn)?;");
    println!();
}

/// 5. 查询 DSL 详解
fn query_dsl() {
    println!("--- 5. 查询 DSL 详解 ---\n");

    // -------------------------------------------------------
    // Diesel 查询 DSL 的强大之处在于可组合性：
    //
    // ```rust
    // // 基础查询
    // let query = users::table
    //     .select(users::all_columns)
    //     .filter(users::age.gt(18))
    //     .filter(users::name.like("%张%"))
    //     .order(users::created_at.desc())
    //     .limit(20)
    //     .offset(0);
    //
    // // 查询可以被拆分和组合
    // fn active_users() -> _ {
    //     users::table.filter(users::active.eq(true))
    // }
    //
    // fn young_users() -> _ {
    //     users::table.filter(users::age.lt(30))
    // }
    //
    // // 聚合查询
    // let user_count: i64 = users::table
    //     .count()
    //     .get_result(&mut conn)?;
    //
    // let avg_age: f64 = users::table
    //     .select(diesel::dsl::avg(users::age))
    //     .first(&mut conn)?;
    //
    // // JOIN 查询
    // let user_posts = users::table
    //     .inner_join(posts::table)
    //     .select((users::name, posts::title))
    //     .load::<(String, String)>(&mut conn)?;
    //
    // // GROUP BY
    // let post_counts = users::table
    //     .inner_join(posts::table)
    //     .group_by(users::id)
    //     .select((users::name, diesel::dsl::count(posts::id)))
    //     .load::<(String, i64)>(&mut conn)?;
    // ```
    // -------------------------------------------------------

    println!("Diesel 查询 DSL 方法链：");
    println!();
    println!("  过滤方法：");
    println!("    .filter(column.eq(value))     等于");
    println!("    .filter(column.ne(value))     不等于");
    println!("    .filter(column.gt(value))     大于");
    println!("    .filter(column.ge(value))     大于等于");
    println!("    .filter(column.lt(value))     小于");
    println!("    .filter(column.le(value))     小于等于");
    println!("    .filter(column.like(pattern)) LIKE 模式");
    println!("    .filter(column.is_null())     IS NULL");
    println!("    .filter(column.is_not_null()) IS NOT NULL");
    println!();
    println!("  排序与分页：");
    println!("    .order(column.asc())     升序");
    println!("    .order(column.desc())    降序");
    println!("    .limit(n)               限制数量");
    println!("    .offset(n)              偏移量");
    println!();
    println!("  聚合函数：");
    println!("    .count()                  COUNT");
    println!("    diesel::dsl::avg(column)  AVG");
    println!("    diesel::dsl::sum(column)  SUM");
    println!("    diesel::dsl::max(column)  MAX");
    println!("    diesel::dsl::min(column)  MIN");
    println!();
    println!("  关联查询：");
    println!("    .inner_join(other_table)   INNER JOIN");
    println!("    .left_join(other_table)    LEFT JOIN");
    println!();
}

/// 6. 数据库迁移
fn database_migrations() {
    println!("--- 6. 数据库迁移 ---\n");

    println!("Diesel 使用 diesel CLI 管理数据库迁移：");
    println!();
    println!("  # 初始化（创建 migrations 目录和 diesel.toml）");
    println!("  $ diesel setup");
    println!();
    println!("  # 创建新迁移");
    println!("  $ diesel migration generate create_users");
    println!("  → 生成 migrations/YYYY-MM-DD-HHMMSS_create_users/");
    println!("    ├── up.sql     # 正向迁移（创建/修改）");
    println!("    └── down.sql   # 回滚迁移（撤销变更）");
    println!();

    // -------------------------------------------------------
    // 迁移 SQL 示例：
    //
    // -- up.sql
    // CREATE TABLE users (
    //     id INTEGER PRIMARY KEY AUTOINCREMENT,
    //     name TEXT NOT NULL,
    //     email TEXT NOT NULL UNIQUE,
    //     age INTEGER NOT NULL DEFAULT 0,
    //     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    // );
    //
    // CREATE INDEX idx_users_email ON users(email);
    //
    // -- down.sql
    // DROP TABLE users;
    // -------------------------------------------------------

    println!("  # 执行迁移");
    println!("  $ diesel migration run");
    println!();
    println!("  # 回滚最近一次迁移");
    println!("  $ diesel migration revert");
    println!();
    println!("  # 重做迁移（回滚 + 重新执行）");
    println!("  $ diesel migration redo");
    println!();
    println!("  # 查看迁移状态");
    println!("  $ diesel migration list");
    println!();
    println!("  # 从数据库生成 schema.rs");
    println!("  $ diesel print-schema > src/schema.rs");
    println!();

    println!("迁移文件结构：");
    println!("  migrations/");
    println!("  ├── 2024-01-01-000000_create_users/");
    println!("  │   ├── up.sql");
    println!("  │   └── down.sql");
    println!("  ├── 2024-01-02-000000_create_posts/");
    println!("  │   ├── up.sql");
    println!("  │   └── down.sql");
    println!("  └── 2024-01-03-000000_add_user_avatar/");
    println!("      ├── up.sql");
    println!("      └── down.sql");
    println!();
}

/// 7. 关联查询与高级特性
fn advanced_features() {
    println!("--- 7. 关联查询与高级特性 ---\n");

    // -------------------------------------------------------
    // Diesel 关联关系示例：
    //
    // ```rust
    // use diesel::prelude::*;
    //
    // // 定义 belongs_to 关系
    // #[derive(Queryable, Identifiable, Associations, Debug)]
    // #[diesel(belongs_to(User))]
    // #[diesel(table_name = posts)]
    // pub struct Post {
    //     pub id: i32,
    //     pub user_id: i32,
    //     pub title: String,
    //     pub body: String,
    // }
    //
    // // 加载用户及其关联的文章
    // let user = users::table.find(1).first::<User>(&mut conn)?;
    // let user_posts = Post::belonging_to(&user)
    //     .load::<Post>(&mut conn)?;
    //
    // // 批量加载关联（避免 N+1 查询问题）
    // let all_users = users::table.load::<User>(&mut conn)?;
    // let all_posts = Post::belonging_to(&all_users)
    //     .load::<Post>(&mut conn)?
    //     .grouped_by(&all_users);
    //
    // let users_with_posts: Vec<(User, Vec<Post>)> =
    //     all_users.into_iter().zip(all_posts).collect();
    // ```
    // -------------------------------------------------------

    println!("关联关系：");
    println!("  • belongs_to   —— 多对一关系（Post 属于 User）");
    println!("  • has_many     —— 一对多关系（通过 belonging_to 反查）");
    println!("  • grouped_by   —— 将关联数据按父模型分组");
    println!();

    // -------------------------------------------------------
    // 连接池（使用 r2d2）：
    //
    // ```rust
    // use diesel::r2d2::{self, ConnectionManager};
    //
    // type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
    //
    // let manager = ConnectionManager::<SqliteConnection>::new("database.db");
    // let pool = r2d2::Pool::builder()
    //     .max_size(10)            // 最大连接数
    //     .min_idle(Some(2))       // 最小空闲连接数
    //     .build(manager)
    //     .expect("创建连接池失败");
    //
    // // 从池中获取连接
    // let mut conn = pool.get().expect("获取连接失败");
    // ```
    // -------------------------------------------------------

    println!("连接池（r2d2）：");
    println!("  Diesel 内置支持 r2d2 连接池");
    println!("  Pool::builder().max_size(10).build(manager) 创建连接池");
    println!("  pool.get() 从池中获取连接");
    println!();

    // -------------------------------------------------------
    // 自定义 SQL：
    //
    // ```rust
    // // 使用 sql_query 执行原始 SQL
    // let users = diesel::sql_query("SELECT * FROM users WHERE age > ?")
    //     .bind::<Integer, _>(18)
    //     .load::<User>(&mut conn)?;
    //
    // // 使用 sql_literal 在查询中嵌入 SQL 片段
    // let users = users::table
    //     .filter(diesel::dsl::sql::<Bool>("age > 18 AND name LIKE '%张%'"))
    //     .load::<User>(&mut conn)?;
    // ```
    // -------------------------------------------------------

    println!("自定义 SQL：");
    println!("  diesel::sql_query(\"...\").bind::<Type, _>(value) 执行原始 SQL");
    println!("  diesel::dsl::sql::<Bool>(\"...\") 在查询中嵌入 SQL 片段");
    println!();
}

/// 8. Diesel vs SQLx 对比
fn comparison_with_sqlx() {
    println!("--- 8. Diesel vs SQLx 对比 ---\n");

    println!("  ┌──────────────────┬──────────────────┬──────────────────┐");
    println!("  │ 特性              │ Diesel           │ SQLx             │");
    println!("  ├──────────────────┼──────────────────┼──────────────────┤");
    println!("  │ 类型              │ ORM              │ SQL 工具库        │");
    println!("  │ 异步支持           │ 同步为主(有异步) │ 原生异步          │");
    println!("  │ 查询方式           │ 类型安全 DSL     │ 原始 SQL          │");
    println!("  │ 编译时检查         │ 通过类型系统     │ 通过宏 + 数据库   │");
    println!("  │ 学习曲线           │ 较陡             │ 较平缓            │");
    println!("  │ Schema 管理       │ 迁移系统         │ 手动/migrate      │");
    println!("  │ 成熟度            │ 非常成熟          │ 成熟             │");
    println!("  │ C 依赖            │ 需要 (libpq等)   │ 纯 Rust          │");
    println!("  │ 适用场景           │ 复杂业务逻辑     │ 简单/性能敏感     │");
    println!("  └──────────────────┴──────────────────┴──────────────────┘");
    println!();

    println!("选择建议：");
    println!("  • 需要 ORM 功能（模型映射、关联查询、迁移管理）→ Diesel");
    println!("  • 需要异步 + 灵活 SQL → SQLx");
    println!("  • 需要异步 ORM → SeaORM（下节课介绍）");
    println!();
}
