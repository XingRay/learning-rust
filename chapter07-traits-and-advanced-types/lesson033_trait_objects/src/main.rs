#![allow(dead_code)]

/// # Lesson 033 - Trait 对象
///
/// Trait 对象是 Rust 实现动态多态（运行时多态）的方式。
///
/// ## 学习目标
/// - 理解 dyn Trait 语法
/// - 掌握 Box<dyn Trait> 的使用
/// - 区分动态分发与静态分发
/// - 了解 trait 对象安全规则（object safety）
/// - 学会使用 Vec<Box<dyn Trait>> 构建异构集合
///
/// ## 运行方式
/// ```bash
/// cargo run -p lesson033_trait_objects
/// ```

// =============================================================
// Lesson 033: Trait 对象 - 动态分发与运行时多态
// =============================================================

use std::fmt;

// ---------------------------------------------------------
// 1. 定义用于演示的 Trait 和类型
// ---------------------------------------------------------

/// 可绘制的 trait —— 任何图形都能绘制
trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

/// 圆形
struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("  🔵 绘制圆形 (半径: {:.1})", self.radius);
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &str {
        "圆形"
    }
}

/// 矩形
struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("  🟦 绘制矩形 ({}x{})", self.width, self.height);
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn name(&self) -> &str {
        "矩形"
    }
}

/// 三角形
struct Triangle {
    base: f64,
    height: f64,
}

impl Drawable for Triangle {
    fn draw(&self) {
        println!("  🔺 绘制三角形 (底: {:.1}, 高: {:.1})", self.base, self.height);
    }

    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }

    fn name(&self) -> &str {
        "三角形"
    }
}

// ---------------------------------------------------------
// 2. 静态分发 vs 动态分发
// ---------------------------------------------------------

/// 静态分发（编译时确定类型）
/// 编译器会为每种具体类型生成专门的函数版本（单态化 monomorphization）
/// 优点：性能好（零成本抽象），可以内联优化
/// 缺点：会生成更多机器代码（代码膨胀）
fn draw_static(shape: &impl Drawable) {
    print!("[静态分发] ");
    shape.draw();
}

/// 动态分发（运行时确定类型）
/// 通过虚函数表（vtable）在运行时查找要调用的方法
/// 优点：代码紧凑，灵活
/// 缺点：有微小的运行时开销（指针间接访问）
fn draw_dynamic(shape: &dyn Drawable) {
    print!("[动态分发] ");
    shape.draw();
}

// ---------------------------------------------------------
// 3. Box<dyn Trait> —— 堆上的 trait 对象
// ---------------------------------------------------------

/// 画布 —— 持有一组不同类型的图形
struct Canvas {
    // Vec<Box<dyn Drawable>> 是异构集合
    // 每个元素可以是不同的具体类型（Circle, Rectangle, Triangle 等）
    shapes: Vec<Box<dyn Drawable>>,
    name: String,
}

impl Canvas {
    fn new(name: &str) -> Self {
        Canvas {
            shapes: Vec::new(),
            name: name.to_string(),
        }
    }

    /// 添加任意实现了 Drawable 的图形
    fn add_shape(&mut self, shape: Box<dyn Drawable>) {
        self.shapes.push(shape);
    }

    /// 绘制所有图形
    fn draw_all(&self) {
        println!("📋 画布「{}」中的图形:", self.name);
        for (i, shape) in self.shapes.iter().enumerate() {
            print!("  {}. ", i + 1);
            shape.draw();
        }
    }

    /// 计算所有图形的总面积
    fn total_area(&self) -> f64 {
        self.shapes.iter().map(|s| s.area()).sum()
    }

    /// 获取图形数量
    fn count(&self) -> usize {
        self.shapes.len()
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "画布「{}」: {} 个图形, 总面积 {:.2}",
            self.name,
            self.count(),
            self.total_area()
        )
    }
}

// ---------------------------------------------------------
// 4. Trait 对象安全（Object Safety）
// ---------------------------------------------------------
// 并非所有 trait 都可以作为 trait 对象使用。
// 只有"对象安全"的 trait 才能创建 dyn Trait。
//
// 对象安全的规则：
// 1. trait 中的方法不能返回 Self 类型
// 2. trait 中的方法不能有泛型参数
// 3. trait 不能有 Self: Sized 约束
//
// 这些限制的原因是：使用 dyn Trait 时，编译器不知道具体类型，
// 无法确定 Self 的大小或泛型参数的具体类型。

/// ✅ 对象安全的 trait
trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
}

/// ❌ 不是对象安全的 trait（返回 Self）
/// 不能用作 dyn NotObjectSafe
trait Cloneable: Sized {
    fn clone_self(&self) -> Self; // 返回 Self —— 不对象安全
}

/// ❌ 不是对象安全的 trait（泛型方法）
trait GenericMethod {
    fn process<T>(&self, item: T); // 泛型参数 —— 不对象安全
}

// 但可以通过一些技巧"部分"使用这些 trait：
// 对于 Clone，标准库提供了 Clone trait，但不能直接用 dyn Clone
// 可以创建辅助 trait 来绕过：

/// 把 Clone 包装成对象安全的版本
trait CloneBox {
    fn clone_box(&self) -> Box<dyn Animal>;
}

/// 为所有同时实现 Animal + Clone 的类型实现 CloneBox
impl<T: 'static + Animal + Clone> CloneBox for T {
    fn clone_box(&self) -> Box<dyn Animal> {
        Box::new(self.clone())
    }
}

// ---------------------------------------------------------
// 具体的动物类型
// ---------------------------------------------------------

#[derive(Clone)]
struct Dog {
    name: String,
    breed: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("汪汪！我是{}({})", self.name, self.breed)
    }
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
struct Cat {
    name: String,
    indoor: bool,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        let location = if self.indoor { "室内" } else { "室外" };
        format!("喵～我是{}，我是{}猫", self.name, location)
    }
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone)]
struct Bird {
    name: String,
    can_fly: bool,
}

impl Animal for Bird {
    fn speak(&self) -> String {
        if self.can_fly {
            format!("叽叽喳喳～我是{}，我会飞！", self.name)
        } else {
            format!("咕咕～我是{}，我不会飞", self.name)
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
}

// ---------------------------------------------------------
// 5. 返回 trait 对象的工厂函数
// ---------------------------------------------------------

/// 根据类型名创建动物 —— 返回 Box<dyn Animal>
fn create_animal(animal_type: &str, name: &str) -> Box<dyn Animal> {
    match animal_type {
        "dog" => Box::new(Dog {
            name: name.to_string(),
            breed: "柴犬".to_string(),
        }),
        "cat" => Box::new(Cat {
            name: name.to_string(),
            indoor: true,
        }),
        "bird" => Box::new(Bird {
            name: name.to_string(),
            can_fly: true,
        }),
        _ => Box::new(Dog {
            name: name.to_string(),
            breed: "未知".to_string(),
        }),
    }
}

// ---------------------------------------------------------
// 6. dyn Trait 与引用
// ---------------------------------------------------------

/// 使用 &dyn Trait 引用（不需要堆分配）
fn describe_animal(animal: &dyn Animal) {
    println!("  🐾 {}: {}", animal.name(), animal.speak());
}

/// 使用 Vec<&dyn Trait> —— 借用版的异构集合
fn describe_all(animals: &[&dyn Animal]) {
    println!("所有动物:");
    for animal in animals {
        describe_animal(*animal);
    }
}

fn main() {
    println!("=== Lesson 033: Trait 对象 ===\n");

    // ---------------------------------------------------------
    // 演示 1: 静态分发 vs 动态分发
    // ---------------------------------------------------------
    println!("--- 1. 静态分发 vs 动态分发 ---");

    let circle = Circle { radius: 5.0 };
    let rect = Rectangle {
        width: 4.0,
        height: 6.0,
    };

    // 静态分发：编译时确定调用哪个 draw
    draw_static(&circle);
    draw_static(&rect);

    // 动态分发：运行时通过 vtable 查找 draw
    draw_dynamic(&circle);
    draw_dynamic(&rect);

    println!("\n📝 两者区别:");
    println!("   静态分发: 编译器生成专门代码，性能最优，代码量大");
    println!("   动态分发: 通过虚函数表查找，微小开销，代码紧凑");

    // ---------------------------------------------------------
    // 演示 2: Box<dyn Trait> 与异构集合
    // ---------------------------------------------------------
    println!("\n--- 2. Box<dyn Trait> 异构集合 ---");

    let mut canvas = Canvas::new("我的画布");

    // 添加不同类型的图形到同一个 Vec 中
    canvas.add_shape(Box::new(Circle { radius: 3.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 4.0,
        height: 5.0,
    }));
    canvas.add_shape(Box::new(Triangle {
        base: 6.0,
        height: 4.0,
    }));
    canvas.add_shape(Box::new(Circle { radius: 1.5 }));

    canvas.draw_all();
    println!("\n{}", canvas);

    // 列出每个图形的面积
    println!("\n各图形面积:");
    for shape in &canvas.shapes {
        println!("  {} 的面积: {:.2}", shape.name(), shape.area());
    }

    // ---------------------------------------------------------
    // 演示 3: 工厂模式与 trait 对象
    // ---------------------------------------------------------
    println!("\n--- 3. 工厂模式 ---");

    // 使用工厂函数创建不同类型的动物
    let animals: Vec<Box<dyn Animal>> = vec![
        create_animal("dog", "小白"),
        create_animal("cat", "咪咪"),
        create_animal("bird", "翠翠"),
        create_animal("dog", "大黄"),
    ];

    println!("动物园:");
    for animal in &animals {
        println!("  🐾 {}", animal.speak());
    }

    // ---------------------------------------------------------
    // 演示 4: &dyn Trait 引用
    // ---------------------------------------------------------
    println!("\n--- 4. &dyn Trait 引用 ---");

    let dog = Dog {
        name: "旺财".to_string(),
        breed: "金毛".to_string(),
    };
    let cat = Cat {
        name: "小花".to_string(),
        indoor: false,
    };
    let bird = Bird {
        name: "鹦鹉".to_string(),
        can_fly: true,
    };

    // 使用 &dyn Animal 引用，无需堆分配
    describe_animal(&dog);
    describe_animal(&cat);

    // Vec<&dyn Trait> —— 借用的异构集合
    let animal_refs: Vec<&dyn Animal> = vec![&dog, &cat, &bird];
    describe_all(&animal_refs);

    // ---------------------------------------------------------
    // 演示 5: Trait 对象安全
    // ---------------------------------------------------------
    println!("\n--- 5. Trait 对象安全规则 ---");

    println!("✅ 对象安全的 trait 可以用作 dyn Trait:");
    println!("   - 方法不返回 Self");
    println!("   - 方法没有泛型参数");
    println!("   - 没有 Self: Sized 约束");
    println!();
    println!("❌ 不对象安全的常见情况:");
    println!("   - fn clone(&self) -> Self  (Clone trait)");
    println!("   - fn process<T>(&self, item: T)  (泛型方法)");
    println!("   - trait Foo: Sized {{}}  (Sized 约束)");
    println!();
    println!("💡 绕过方法:");
    println!("   - 用辅助 trait 包装（如 CloneBox 模式）");
    println!("   - 用枚举代替 trait 对象（当类型集合已知时）");

    // ---------------------------------------------------------
    // 演示 6: 用枚举代替 trait 对象（当类型集合已知时）
    // ---------------------------------------------------------
    println!("\n--- 6. 枚举 vs Trait 对象 ---");

    // 当所有可能的类型在编译时已知时，枚举可能是更好的选择
    enum Shape {
        Circle(f64),          // 半径
        Rectangle(f64, f64),  // 宽, 高
        Triangle(f64, f64),   // 底, 高
    }

    impl Shape {
        fn area(&self) -> f64 {
            match self {
                Shape::Circle(r) => std::f64::consts::PI * r * r,
                Shape::Rectangle(w, h) => w * h,
                Shape::Triangle(b, h) => 0.5 * b * h,
            }
        }

        fn name(&self) -> &str {
            match self {
                Shape::Circle(_) => "圆形",
                Shape::Rectangle(_, _) => "矩形",
                Shape::Triangle(_, _) => "三角形",
            }
        }
    }

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle(6.0, 3.0),
    ];

    println!("使用枚举的图形集合:");
    for shape in &shapes {
        println!("  {} 面积: {:.2}", shape.name(), shape.area());
    }

    println!("\n📝 枚举 vs Trait 对象:");
    println!("   枚举: 类型集合封闭，性能好，添加新类型需修改枚举");
    println!("   Trait 对象: 类型集合开放，可扩展，有运行时开销");

    println!("\n🎉 恭喜！你已经完成了 Trait 对象的学习！");
}
