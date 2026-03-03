// ============================================================
// Lesson 095: SeaORM（概念讲解）
// ============================================================
// SeaORM 是 Rust 生态中的异步 ORM 框架，构建在 SQLx 之上。
// 它填补了 Diesel（同步 ORM）和 SQLx（异步 SQL 工具）之间的空白，
// 提供了异步环境下的完整 ORM 体验。
//
// 本课以概念讲解和注释代码示例为主，不引入重依赖。
//
// SeaORM 的核心特点：
// 1. **原生异步**：基于 async/await，底层使用 SQLx
// 2. **动态查询构建**：SeaQuery 提供类型安全的查询构建器
// 3. **Entity 驱动**：通过 Entity trait 定义模型和关系
// 4. **多数据库支持**：PostgreSQL、MySQL、SQLite
// 5. **完整 ORM 功能**：关联查询、事务、迁移、活动记录模式
// ============================================================

fn main() {
    println!("=== Lesson 095: SeaORM（概念讲解）===\n");

    // ============================================================
    // 1. SeaORM 架构概览
    // ============================================================
    architecture_overview();

    // ============================================================
    // 2. Entity 定义
    // ============================================================
    entity_definition();

    // ============================================================
    // 3. 数据库连接
    // ============================================================
    database_connection();

    // ============================================================
    // 4. CRUD 操作
    // ============================================================
    crud_operations();

    // ============================================================
    // 5. 查询构建器
    // ============================================================
    query_builder();

    // ============================================================
    // 6. 关系与关联查询
    // ============================================================
    relations_and_joins();

    // ============================================================
    // 7. 数据库迁移
    // ============================================================
    migration_system();

    // ============================================================
    // 8. 高级特性
    // ============================================================
    advanced_features();

    // ============================================================
    // 9. ORM 选型对比总结
    // ============================================================
    orm_comparison();

    println!("\n=== SeaORM 概念讲解完毕 ===");
}

/// 1. SeaORM 架构概览
fn architecture_overview() {
    println!("--- 1. SeaORM 架构概览 ---\n");

    println!("SeaORM 的技术栈：");
    println!("  ┌─────────────────────────────────────┐");
    println!("  │         应用层 (Your Code)            │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │   SeaORM（Entity / ActiveModel）     │");
    println!("  │   关联查询、事务、CRUD 封装            │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │   SeaQuery（查询构建器）               │");
    println!("  │   类型安全的 SQL 生成                  │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │   SQLx（异步数据库驱动）               │");
    println!("  │   连接池、原始 SQL 执行                │");
    println!("  ├─────────────────────────────────────┤");
    println!("  │   PostgreSQL / MySQL / SQLite        │");
    println!("  └─────────────────────────────────────┘");
    println!();

    // Cargo.toml 依赖配置：
    //
    // [dependencies]
    // sea-orm = { version = "1.0", features = [
    //     "sqlx-sqlite",          # 数据库驱动
    //     "runtime-tokio-rustls", # 异步运行时
    //     "macros",               # derive 宏
    // ] }
    // tokio = { version = "1", features = ["full"] }
    //
    // # 安装 sea-orm CLI:
    // # cargo install sea-orm-cli

    println!("核心概念：");
    println!("  • Entity：数据库表的 Rust 表示，定义列和关系");
    println!("  • Model：Entity 的具体数据结构（查询结果）");
    println!("  • ActiveModel：用于插入/更新的可变数据结构");
    println!("  • SeaQuery：底层查询构建器，可独立使用");
    println!("  • DatabaseConnection：异步数据库连接（池）");
    println!();
}

/// 2. Entity 定义
fn entity_definition() {
    println!("--- 2. Entity 定义 ---\n");

    // -------------------------------------------------------
    // SeaORM Entity 定义示例：
    //
    // ```rust
    // // src/entity/user.rs
    //
    // use sea_orm::entity::prelude::*;
    //
    // #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    // #[sea_orm(table_name = "users")]
    // pub struct Model {
    //     #[sea_orm(primary_key)]
    //     pub id: i32,
    //     pub name: String,
    //     #[sea_orm(unique)]
    //     pub email: String,
    //     pub age: i32,
    //     pub created_at: DateTimeUtc,
    // }
    //
    // // 定义关系
    // #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    // pub enum Relation {
    //     #[sea_orm(has_many = "super::post::Entity")]
    //     Posts,
    // }
    //
    // // 实现 Related trait
    // impl Related<super::post::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         Relation::Posts.def()
    //     }
    // }
    //
    // impl ActiveModelBehavior for ActiveModel {}
    // ```
    // -------------------------------------------------------

    println!("Entity 定义要素：");
    println!();
    println!("  1. Model 结构体（#[derive(DeriveEntityModel)]）：");
    println!("     - #[sea_orm(table_name = \"users\")] 指定表名");
    println!("     - #[sea_orm(primary_key)]            标记主键");
    println!("     - #[sea_orm(unique)]                  唯一约束");
    println!("     - #[sea_orm(column_type = \"Text\")]   自定义列类型");
    println!("     - #[sea_orm(nullable)]                可空列");
    println!();
    println!("  2. Relation 枚举（#[derive(DeriveRelation)]）：");
    println!("     - has_many  一对多");
    println!("     - has_one   一对一");
    println!("     - belongs_to 多对一");
    println!();
    println!("  3. ActiveModelBehavior trait：");
    println!("     - before_save() / after_save()      保存前后钩子");
    println!("     - before_delete() / after_delete()  删除前后钩子");
    println!();

    // -------------------------------------------------------
    // Post Entity 示例：
    //
    // ```rust
    // // src/entity/post.rs
    //
    // use sea_orm::entity::prelude::*;
    //
    // #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    // #[sea_orm(table_name = "posts")]
    // pub struct Model {
    //     #[sea_orm(primary_key)]
    //     pub id: i32,
    //     pub user_id: i32,
    //     pub title: String,
    //     #[sea_orm(column_type = "Text")]
    //     pub body: String,
    //     pub published: bool,
    // }
    //
    // #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    // pub enum Relation {
    //     #[sea_orm(
    //         belongs_to = "super::user::Entity",
    //         from = "Column::UserId",
    //         to = "super::user::Column::Id"
    //     )]
    //     User,
    // }
    //
    // impl Related<super::user::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         Relation::User.def()
    //     }
    // }
    //
    // impl ActiveModelBehavior for ActiveModel {}
    // ```
    // -------------------------------------------------------

    println!("  SeaORM 还支持从数据库自动生成 Entity：");
    println!("  $ sea-orm-cli generate entity -o src/entity");
    println!();
}

/// 3. 数据库连接
fn database_connection() {
    println!("--- 3. 数据库连接 ---\n");

    // -------------------------------------------------------
    // ```rust
    // use sea_orm::{Database, DatabaseConnection, ConnectOptions};
    //
    // // 简单连接
    // let db: DatabaseConnection = Database::connect("sqlite::memory:").await?;
    //
    // // 带选项的连接
    // let mut opt = ConnectOptions::new("postgres://user:pass@host/db");
    // opt.max_connections(100)        // 最大连接数
    //    .min_connections(5)          // 最小连接数
    //    .connect_timeout(Duration::from_secs(8))
    //    .acquire_timeout(Duration::from_secs(8))
    //    .idle_timeout(Duration::from_secs(8))
    //    .max_lifetime(Duration::from_secs(8))
    //    .sqlx_logging(true)          // 开启 SQL 日志
    //    .sqlx_logging_level(log::LevelFilter::Info);
    //
    // let db = Database::connect(opt).await?;
    // ```
    // -------------------------------------------------------

    println!("连接方式：");
    println!("  // SQLite 内存数据库");
    println!("  Database::connect(\"sqlite::memory:\").await?;");
    println!();
    println!("  // SQLite 文件数据库");
    println!("  Database::connect(\"sqlite://data.db?mode=rwc\").await?;");
    println!();
    println!("  // PostgreSQL");
    println!("  Database::connect(\"postgres://user:pass@localhost/mydb\").await?;");
    println!();
    println!("  // MySQL");
    println!("  Database::connect(\"mysql://user:pass@localhost/mydb\").await?;");
    println!();

    println!("连接池配置：");
    println!("  • max_connections  —— 最大连接数（默认 10）");
    println!("  • min_connections  —— 最小空闲连接数");
    println!("  • connect_timeout —— 连接超时");
    println!("  • idle_timeout    —— 空闲连接超时");
    println!("  • sqlx_logging    —— 开启 SQL 日志");
    println!();
}

/// 4. CRUD 操作
fn crud_operations() {
    println!("--- 4. CRUD 操作 ---\n");

    // -------------------------------------------------------
    // SeaORM CRUD 操作示例：
    //
    // ```rust
    // use sea_orm::*;
    // use entity::user;
    //
    // // === CREATE（插入）===
    //
    // // 方式一：使用 ActiveModel
    // let new_user = user::ActiveModel {
    //     name: Set("张三".to_string()),
    //     email: Set("zhangsan@example.com".to_string()),
    //     age: Set(28),
    //     ..Default::default()  // id 自动生成
    // };
    // let inserted = new_user.insert(&db).await?;
    //
    // // 方式二：批量插入
    // let users = vec![
    //     user::ActiveModel {
    //         name: Set("李四".to_string()),
    //         email: Set("lisi@example.com".to_string()),
    //         age: Set(30),
    //         ..Default::default()
    //     },
    //     user::ActiveModel {
    //         name: Set("王五".to_string()),
    //         email: Set("wangwu@example.com".to_string()),
    //         age: Set(25),
    //         ..Default::default()
    //     },
    // ];
    // user::Entity::insert_many(users).exec(&db).await?;
    //
    // // === READ（查询）===
    //
    // // 查询所有
    // let all_users: Vec<user::Model> = user::Entity::find()
    //     .all(&db).await?;
    //
    // // 按主键查询
    // let user: Option<user::Model> = user::Entity::find_by_id(1)
    //     .one(&db).await?;
    //
    // // 条件查询
    // let users = user::Entity::find()
    //     .filter(user::Column::Age.gt(25))
    //     .order_by_asc(user::Column::Name)
    //     .limit(10)
    //     .all(&db).await?;
    //
    // // === UPDATE（更新）===
    //
    // let mut user: user::ActiveModel = user::Entity::find_by_id(1)
    //     .one(&db).await?
    //     .unwrap()
    //     .into();
    //
    // user.name = Set("张三丰".to_string());
    // let updated = user.update(&db).await?;
    //
    // // === DELETE（删除）===
    //
    // let user = user::Entity::find_by_id(1).one(&db).await?.unwrap();
    // user.delete(&db).await?;
    //
    // // 条件删除
    // user::Entity::delete_many()
    //     .filter(user::Column::Age.lt(18))
    //     .exec(&db).await?;
    // ```
    // -------------------------------------------------------

    println!("CREATE（插入）：");
    println!("  let model = user::ActiveModel {{ name: Set(\"张三\".into()), ..Default::default() }};");
    println!("  model.insert(&db).await?;");
    println!();

    println!("READ（查询）：");
    println!("  Entity::find().all(&db).await?;                          // 全部");
    println!("  Entity::find_by_id(1).one(&db).await?;                   // 按 ID");
    println!("  Entity::find().filter(Column::Age.gt(25)).all(&db).await?; // 条件");
    println!();

    println!("UPDATE（更新）：");
    println!("  let mut active: ActiveModel = model.into();");
    println!("  active.name = Set(\"新名字\".into());");
    println!("  active.update(&db).await?;");
    println!();

    println!("DELETE（删除）：");
    println!("  model.delete(&db).await?;                                // 删除单个");
    println!("  Entity::delete_many().filter(...).exec(&db).await?;      // 批量删除");
    println!();
}

/// 5. 查询构建器
fn query_builder() {
    println!("--- 5. 查询构建器 ---\n");

    // -------------------------------------------------------
    // SeaORM 查询构建器（基于 SeaQuery）示例：
    //
    // ```rust
    // use sea_orm::*;
    //
    // // 复杂条件查询
    // let users = user::Entity::find()
    //     .filter(
    //         Condition::all()
    //             .add(user::Column::Age.between(20, 30))
    //             .add(user::Column::Name.contains("张"))
    //             .add(
    //                 Condition::any()
    //                     .add(user::Column::Email.ends_with("@gmail.com"))
    //                     .add(user::Column::Email.ends_with("@qq.com"))
    //             )
    //     )
    //     .order_by_desc(user::Column::CreatedAt)
    //     .limit(20)
    //     .offset(0)
    //     .all(&db).await?;
    //
    // // 选择特定列
    // let names: Vec<(String, i32)> = user::Entity::find()
    //     .select_only()
    //     .column(user::Column::Name)
    //     .column(user::Column::Age)
    //     .into_tuple()
    //     .all(&db).await?;
    //
    // // 聚合查询
    // let count = user::Entity::find()
    //     .filter(user::Column::Age.gt(18))
    //     .count(&db).await?;
    //
    // // 分页查询
    // let paginator = user::Entity::find()
    //     .order_by_asc(user::Column::Id)
    //     .paginate(&db, 10);  // 每页 10 条
    //
    // let total_pages = paginator.num_pages().await?;
    // let page_1 = paginator.fetch_page(0).await?;
    // let page_2 = paginator.fetch_page(1).await?;
    //
    // // 使用 SeaQuery 的原始查询
    // let result = user::Entity::find()
    //     .from_raw_sql(Statement::from_sql_and_values(
    //         DbBackend::Sqlite,
    //         "SELECT * FROM users WHERE age > $1",
    //         vec![18.into()],
    //     ))
    //     .all(&db).await?;
    // ```
    // -------------------------------------------------------

    println!("条件过滤方法：");
    println!("  Column::eq(value)          等于");
    println!("  Column::ne(value)          不等于");
    println!("  Column::gt(value)          大于");
    println!("  Column::gte(value)         大于等于");
    println!("  Column::lt(value)          小于");
    println!("  Column::lte(value)         小于等于");
    println!("  Column::between(a, b)      范围");
    println!("  Column::like(pattern)      LIKE");
    println!("  Column::contains(s)        包含");
    println!("  Column::starts_with(s)     以...开头");
    println!("  Column::ends_with(s)       以...结尾");
    println!("  Column::is_null()          IS NULL");
    println!("  Column::is_in(vec)         IN (...)");
    println!();

    println!("组合条件：");
    println!("  Condition::all() —— AND 组合（所有条件都要满足）");
    println!("  Condition::any() —— OR  组合（任一条件满足即可）");
    println!();

    println!("分页查询：");
    println!("  let paginator = Entity::find().paginate(&db, page_size);");
    println!("  paginator.num_pages().await?;        // 总页数");
    println!("  paginator.fetch_page(n).await?;      // 获取第 n 页");
    println!();
}

/// 6. 关系与关联查询
fn relations_and_joins() {
    println!("--- 6. 关系与关联查询 ---\n");

    // -------------------------------------------------------
    // 关联查询示例：
    //
    // ```rust
    // // 预加载关联（避免 N+1 问题）
    // let users_with_posts: Vec<(user::Model, Vec<post::Model>)> =
    //     user::Entity::find()
    //         .find_with_related(post::Entity)
    //         .all(&db).await?;
    //
    // for (user, posts) in &users_with_posts {
    //     println!("用户 {} 有 {} 篇文章", user.name, posts.len());
    // }
    //
    // // 反向查询：查找文章的作者
    // let post_with_user: Vec<(post::Model, Option<user::Model>)> =
    //     post::Entity::find()
    //         .find_also_related(user::Entity)
    //         .all(&db).await?;
    //
    // // JOIN 查询
    // let results = user::Entity::find()
    //     .join(JoinType::InnerJoin, user::Relation::Posts.def())
    //     .filter(post::Column::Published.eq(true))
    //     .all(&db).await?;
    //
    // // 多对多关系（通过中间表）
    // // 假设 users <-> tags 通过 user_tags 中间表关联
    // #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    // pub enum Relation {}
    //
    // impl Related<tag::Entity> for Entity {
    //     fn to() -> RelationDef {
    //         user_tag::Relation::Tag.def()
    //     }
    //     fn via() -> Option<RelationDef> {
    //         Some(user_tag::Relation::User.def().rev())
    //     }
    // }
    // ```
    // -------------------------------------------------------

    println!("关联查询方式：");
    println!();
    println!("  1. find_with_related()  —— 预加载一对多关联");
    println!("     user::Entity::find()");
    println!("         .find_with_related(post::Entity)");
    println!("         .all(&db).await?;");
    println!();
    println!("  2. find_also_related() —— 查询并附带关联记录");
    println!("     post::Entity::find()");
    println!("         .find_also_related(user::Entity)");
    println!("         .all(&db).await?;");
    println!();
    println!("  3. join()              —— 手动 JOIN 查询");
    println!("     Entity::find()");
    println!("         .join(JoinType::InnerJoin, relation.def())");
    println!("         .all(&db).await?;");
    println!();
    println!("  4. 多对多关系         —— 通过 via() 中间表");
    println!();

    println!("关系类型总结：");
    println!("  ┌──────────────┬────────────────────┬─────────────┐");
    println!("  │ 关系          │ 属性               │ 示例         │");
    println!("  ├──────────────┼────────────────────┼─────────────┤");
    println!("  │ has_many     │ 一对多              │ User→Posts  │");
    println!("  │ has_one      │ 一对一              │ User→Profile│");
    println!("  │ belongs_to   │ 多对一              │ Post→User   │");
    println!("  │ via()        │ 多对多(中间表)       │ User↔Tags   │");
    println!("  └──────────────┴────────────────────┴─────────────┘");
    println!();
}

/// 7. 数据库迁移
fn migration_system() {
    println!("--- 7. 数据库迁移 ---\n");

    // -------------------------------------------------------
    // SeaORM 迁移示例：
    //
    // ```rust
    // // migration/src/m20240101_000001_create_users_table.rs
    //
    // use sea_orm_migration::prelude::*;
    //
    // #[derive(DeriveMigrationName)]
    // pub struct Migration;
    //
    // #[async_trait::async_trait]
    // impl MigrationTrait for Migration {
    //     async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    //         manager
    //             .create_table(
    //                 Table::create()
    //                     .table(Users::Table)
    //                     .if_not_exists()
    //                     .col(ColumnDef::new(Users::Id)
    //                         .integer()
    //                         .not_null()
    //                         .auto_increment()
    //                         .primary_key())
    //                     .col(ColumnDef::new(Users::Name)
    //                         .string()
    //                         .not_null())
    //                     .col(ColumnDef::new(Users::Email)
    //                         .string()
    //                         .not_null()
    //                         .unique_key())
    //                     .col(ColumnDef::new(Users::Age)
    //                         .integer()
    //                         .not_null())
    //                     .to_owned(),
    //             )
    //             .await
    //     }
    //
    //     async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    //         manager
    //             .drop_table(Table::drop().table(Users::Table).to_owned())
    //             .await
    //     }
    // }
    //
    // #[derive(Iden)]
    // enum Users {
    //     Table,
    //     Id,
    //     Name,
    //     Email,
    //     Age,
    // }
    // ```
    // -------------------------------------------------------

    println!("SeaORM 迁移系统（sea-orm-migration）：");
    println!();
    println!("  # 初始化迁移项目");
    println!("  $ sea-orm-cli migrate init");
    println!();
    println!("  # 生成新迁移文件");
    println!("  $ sea-orm-cli migrate generate create_users_table");
    println!();
    println!("  # 执行迁移");
    println!("  $ sea-orm-cli migrate up");
    println!();
    println!("  # 回滚迁移");
    println!("  $ sea-orm-cli migrate down");
    println!();
    println!("  # 查看状态");
    println!("  $ sea-orm-cli migrate status");
    println!();

    println!("与 Diesel 迁移的区别：");
    println!("  • Diesel：迁移用 SQL 文件（up.sql / down.sql）");
    println!("  • SeaORM：迁移用 Rust 代码（类型安全的 Schema API）");
    println!("  • SeaORM 迁移可以跨数据库执行（SQL 自动适配）");
    println!();
}

/// 8. 高级特性
fn advanced_features() {
    println!("--- 8. 高级特性 ---\n");

    // -------------------------------------------------------
    // 事务处理：
    //
    // ```rust
    // // 方式一：闭包事务
    // db.transaction::<_, _, DbErr>(|txn| {
    //     Box::pin(async move {
    //         let user = user::ActiveModel {
    //             name: Set("张三".into()),
    //             ..Default::default()
    //         };
    //         let user = user.insert(txn).await?;
    //
    //         let post = post::ActiveModel {
    //             user_id: Set(user.id),
    //             title: Set("Hello".into()),
    //             ..Default::default()
    //         };
    //         post.insert(txn).await?;
    //
    //         Ok(()) // 返回 Ok 自动提交，Err 自动回滚
    //     })
    // }).await?;
    //
    // // 方式二：手动事务
    // let txn = db.begin().await?;
    // // ... 执行操作 ...
    // txn.commit().await?;
    // ```
    // -------------------------------------------------------

    println!("事务处理：");
    println!("  db.transaction(|txn| {{ ... }}).await?;  // 闭包事务（自动提交/回滚）");
    println!("  let txn = db.begin().await?;             // 手动事务");
    println!("  txn.commit().await?;                     // 手动提交");
    println!();

    // -------------------------------------------------------
    // 生命周期钩子：
    //
    // ```rust
    // impl ActiveModelBehavior for ActiveModel {
    //     fn new() -> Self {
    //         Self {
    //             created_at: Set(Utc::now()),
    //             ..ActiveModelTrait::default()
    //         }
    //     }
    //
    //     async fn before_save<C>(self, _db: &C, insert: bool) -> Result<Self, DbErr>
    //     where C: ConnectionTrait,
    //     {
    //         if insert {
    //             println!("即将插入新记录");
    //         } else {
    //             println!("即将更新记录");
    //         }
    //         Ok(self)
    //     }
    //
    //     async fn after_delete<C>(self, _db: &C) -> Result<Self, DbErr>
    //     where C: ConnectionTrait,
    //     {
    //         println!("记录已删除");
    //         Ok(self)
    //     }
    // }
    // ```
    // -------------------------------------------------------

    println!("ActiveModel 生命周期钩子：");
    println!("  • new()           —— 创建新 ActiveModel 时");
    println!("  • before_save()   —— 插入/更新前");
    println!("  • after_save()    —— 插入/更新后");
    println!("  • before_delete() —— 删除前");
    println!("  • after_delete()  —— 删除后");
    println!();

    println!("其他高级特性：");
    println!("  • 自定义选择结果类型（into_model / into_json）");
    println!("  • 流式查询（stream() 逐行处理大数据集）");
    println!("  • 原始 SQL 查询（from_raw_sql）");
    println!("  • 数据库模拟（MockDatabase，用于测试）");
    println!("  • 软删除（通过 ActiveModelBehavior 实现）");
    println!();
}

/// 9. ORM 选型对比总结
fn orm_comparison() {
    println!("--- 9. Rust ORM/数据库库选型总结 ---\n");

    println!("  ┌──────────┬──────────┬────────────┬──────────────┐");
    println!("  │          │ SQLx     │ Diesel     │ SeaORM       │");
    println!("  ├──────────┼──────────┼────────────┼──────────────┤");
    println!("  │ 类型      │ SQL 工具 │ 同步 ORM   │ 异步 ORM     │");
    println!("  │ 异步      │ ✓ 原生   │ △ 有限     │ ✓ 原生       │");
    println!("  │ 查询方式  │ 原始 SQL │ DSL        │ Entity/DSL   │");
    println!("  │ 迁移      │ SQL 文件 │ SQL 文件   │ Rust 代码    │");
    println!("  │ 关联查询  │ 手动 SQL │ 类型安全   │ 类型安全     │");
    println!("  │ 学习成本  │ 低       │ 中高       │ 中           │");
    println!("  │ 生态成熟  │ 成熟     │ 非常成熟   │ 成熟         │");
    println!("  │ 适合场景  │ 简单项目 │ 复杂同步   │ 异步 Web 后端│");
    println!("  └──────────┴──────────┴────────────┴──────────────┘");
    println!();

    println!("选型建议：");
    println!("  • 简单项目 / 对 SQL 很熟悉       → SQLx");
    println!("  • 同步应用 / 极致编译时安全        → Diesel");
    println!("  • 异步 Web 后端 / 需要完整 ORM    → SeaORM");
    println!("  • 微服务 / 性能关键路径            → SQLx（最少抽象层）");
    println!();
}
