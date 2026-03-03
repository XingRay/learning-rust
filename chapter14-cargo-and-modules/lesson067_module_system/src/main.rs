#![allow(dead_code)]
// ============================================================
// Lesson 067: 模块系统 (Module System)
// ============================================================
// Rust 的模块系统用于组织代码、控制可见性、避免命名冲突。
// 核心概念：
// - mod 关键字定义模块
// - pub 控制可见性
// - 模块树和路径（绝对路径 / 相对路径）
// ============================================================

// ============================================================
// 1. 使用 mod 定义内联模块
// ============================================================
// 模块默认是私有的，模块内的所有项（函数、结构体、枚举等）也默认是私有的。
// 使用 pub 关键字可以将项标记为公有。

mod math {
    // 公有函数：外部可以访问
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    // 私有函数：只有 math 模块内部可以访问
    fn _secret_formula(x: i32) -> i32 {
        x * 42 + 7
    }

    // 私有函数可以被同模块的公有函数调用
    pub fn magic_number(x: i32) -> i32 {
        _secret_formula(x)
    }
}

// ============================================================
// 2. 嵌套模块
// ============================================================
// 模块可以嵌套定义，形成层级结构（模块树）

mod animals {
    // 嵌套模块：dogs
    pub mod dogs {
        pub fn bark() -> &'static str {
            "汪汪汪！"
        }

        pub fn description() -> String {
            // 在嵌套模块中调用同级函数
            format!("狗会叫: {}", bark())
        }
    }

    // 嵌套模块：cats
    pub mod cats {
        pub fn meow() -> &'static str {
            "喵喵喵！"
        }

        // 调用兄弟模块中的函数，需要使用路径
        pub fn compare_with_dog() -> String {
            // 使用 super 回到父模块，再进入兄弟模块
            let dog_sound = super::dogs::bark();
            format!("猫叫: {}，狗叫: {}", meow(), dog_sound)
        }
    }

    // 父模块中的公有函数
    pub fn all_sounds() -> String {
        // 父模块可以访问子模块的公有项
        format!("{} {}", dogs::bark(), cats::meow())
    }
}

// ============================================================
// 3. pub 可见性详解
// ============================================================

mod restaurant {
    // 公有结构体
    pub struct Breakfast {
        pub toast: String,       // 公有字段：外部可以直接访问
        seasonal_fruit: String,  // 私有字段：外部不能直接访问
    }

    impl Breakfast {
        // 必须提供构造函数，因为有私有字段不能在外部直接构造
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("桃子"),
            }
        }

        // 提供访问私有字段的方法
        pub fn fruit(&self) -> &str {
            &self.seasonal_fruit
        }
    }

    // 公有枚举：所有变体自动公有
    // （与结构体不同！结构体字段默认私有，枚举变体默认公有）
    pub enum Appetizer {
        Soup,
        Salad,
        Bread,
    }

    // 模块中的私有模块
    mod kitchen {
        pub fn prepare_meal() -> String {
            String::from("厨房正在准备餐食...")
        }
    }

    // 公有函数可以访问同模块中的私有模块
    pub fn serve() -> String {
        kitchen::prepare_meal()
    }
}

// ============================================================
// 4. 模块树和路径
// ============================================================
// Rust 中的模块形成一棵树，crate 根（src/main.rs 或 src/lib.rs）是树的根节点。
//
// 访问模块中的项有三种路径方式：
//   (1) 绝对路径：以 crate 关键字开头，从 crate 根开始
//       crate::module_a::function_a()
//   (2) 相对路径：从当前模块开始
//       module_a::function_a()
//   (3) super：引用父模块（类似文件系统的 ..）
//       super::function_b()
//   (4) self：引用当前模块（类似文件系统的 .）
//       self::function_c()

mod network {
    pub mod server {
        pub fn start() -> String {
            String::from("服务器启动")
        }

        pub fn status() -> String {
            // 使用 self:: 引用当前模块（可选，通常省略）
            let msg = self::start();
            format!("状态: {} 完成", msg)
        }
    }

    pub mod client {
        pub fn connect() -> String {
            // 使用 super:: 引用父模块 network，再访问兄弟模块 server
            let server_status = super::server::start();
            format!("客户端连接 -> {}", server_status)
        }

        pub fn connect_absolute() -> String {
            // 使用绝对路径 crate:: 从根开始
            let server_status = crate::network::server::start();
            format!("客户端连接(绝对路径) -> {}", server_status)
        }
    }

    // network 模块自身的函数
    pub fn info() -> String {
        // 相对路径访问子模块
        let s = server::start();
        let c = client::connect();
        format!("网络信息: [{}] [{}]", s, c)
    }
}

// ============================================================
// 5. 文件模块 (mod.rs) —— 注释说明
// ============================================================
// 在实际项目中，模块通常不是内联定义的，而是放在单独的文件中。
//
// 方式一：文件即模块
//   src/
//   ├── main.rs         // 在 main.rs 中写 `mod math;`
//   └── math.rs         // math 模块的实现
//
// 方式二：目录 + mod.rs（适合有子模块的情况）
//   src/
//   ├── main.rs         // 在 main.rs 中写 `mod network;`
//   └── network/
//       ├── mod.rs      // network 模块的入口，可在此声明子模块
//       ├── server.rs   // network::server 子模块
//       └── client.rs   // network::client 子模块
//
// 方式三（Rust 2018+ 推荐）：文件 + 同名目录
//   src/
//   ├── main.rs         // 在 main.rs 中写 `mod network;`
//   ├── network.rs      // network 模块的入口
//   └── network/
//       ├── server.rs   // network::server 子模块
//       └── client.rs   // network::client 子模块
//
// 注意：方式二和方式三不能同时存在（不能既有 network.rs 又有 network/mod.rs）
//
// 本课程为了简洁，所有模块都采用内联方式定义在 main.rs 中。
// 实际项目中推荐将大模块拆分为独立文件。

// ============================================================
// 6. 更复杂的模块树示例
// ============================================================

mod company {
    pub mod engineering {
        pub mod frontend {
            pub fn build_ui() -> &'static str {
                "前端: 构建 UI"
            }
        }

        pub mod backend {
            pub fn build_api() -> &'static str {
                "后端: 构建 API"
            }

            pub fn call_frontend() -> String {
                // 使用 super 两次回到 engineering，再进入 frontend
                let ui = super::frontend::build_ui();
                format!("后端调用 -> {}", ui)
            }

            pub fn call_with_absolute() -> String {
                // 使用绝对路径从 crate 根开始
                let ui = crate::company::engineering::frontend::build_ui();
                format!("后端调用(绝对路径) -> {}", ui)
            }
        }
    }

    pub mod hr {
        pub fn hire() -> &'static str {
            "HR: 招聘新员工"
        }

        pub fn hire_engineer() -> String {
            // 使用 super 回到 company，再进入 engineering
            let api = super::engineering::backend::build_api();
            format!("{}, 为 [{}] 招聘", hire(), api)
        }
    }
}

fn main() {
    println!("=== Lesson 067: 模块系统 ===\n");

    // ---- 1. 基本模块使用 ----
    println!("--- 1. 基本模块使用 ---");
    println!("3 + 5 = {}", math::add(3, 5));
    println!("10 - 3 = {}", math::subtract(10, 3));
    println!("magic_number(1) = {}", math::magic_number(1));
    // math::_secret_formula(1); // 编译错误！私有函数不能从外部访问
    println!();

    // ---- 2. 嵌套模块 ----
    println!("--- 2. 嵌套模块 ---");
    println!("狗: {}", animals::dogs::bark());
    println!("猫: {}", animals::cats::meow());
    println!("{}", animals::dogs::description());
    println!("{}", animals::cats::compare_with_dog());
    println!("所有声音: {}", animals::all_sounds());
    println!();

    // ---- 3. pub 可见性 ----
    println!("--- 3. pub 可见性 ---");
    let mut meal = restaurant::Breakfast::summer("全麦面包");
    // 可以修改公有字段
    meal.toast = String::from("白面包");
    println!("面包: {}", meal.toast);
    // meal.seasonal_fruit = String::from("蓝莓"); // 编译错误！私有字段
    println!("水果: {}", meal.fruit());

    // 枚举变体自动公有
    let appetizer = restaurant::Appetizer::Soup;
    match appetizer {
        restaurant::Appetizer::Soup => println!("开胃菜: 汤"),
        restaurant::Appetizer::Salad => println!("开胃菜: 沙拉"),
        restaurant::Appetizer::Bread => println!("开胃菜: 面包"),
    }
    println!("上菜: {}", restaurant::serve());
    println!();

    // ---- 4. 模块路径演示 ----
    println!("--- 4. 模块路径 ---");
    // 绝对路径
    println!("[绝对] {}", crate::network::server::start());
    // 相对路径
    println!("[相对] {}", network::server::start());
    // self 和 super 示例
    println!("[self]  {}", network::server::status());
    println!("[super] {}", network::client::connect());
    println!("[crate] {}", network::client::connect_absolute());
    println!("{}", network::info());
    println!();

    // ---- 5. 复杂模块树 ----
    println!("--- 5. 复杂模块树 ---");
    println!("{}", company::engineering::frontend::build_ui());
    println!("{}", company::engineering::backend::build_api());
    println!("{}", company::engineering::backend::call_frontend());
    println!("{}", company::engineering::backend::call_with_absolute());
    println!("{}", company::hr::hire());
    println!("{}", company::hr::hire_engineer());
    println!();

    // ---- 总结 ----
    println!("=== 总结 ===");
    println!("1. mod 定义模块，模块可以嵌套");
    println!("2. 模块内的项默认私有，pub 标记为公有");
    println!("3. 结构体字段默认私有，枚举变体自动公有");
    println!("4. 路径：crate::（绝对）、self::（当前）、super::（父级）");
    println!("5. 文件模块：单文件 / mod.rs 目录 / 同名文件+目录");
}
