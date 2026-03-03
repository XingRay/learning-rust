// ============================================================
// Lesson 040: Box<T> 堆分配
// ============================================================
// Box<T> 是 Rust 中最简单的智能指针，它将数据分配在堆上，
// 而 Box 本身（一个指针）存放在栈上。
//
// 常见使用场景：
// 1. 编译时大小未知的类型（如递归类型）
// 2. 大量数据需要转移所有权但避免拷贝
// 3. trait 对象（Box<dyn Trait>）

fn main() {
    println!("=== Lesson 040: Box<T> 堆分配 ===\n");

    // -------------------------------------------------------
    // 1. Box::new 基本用法
    // -------------------------------------------------------
    println!("--- 1. Box::new 基本用法 ---");

    // 将一个 i32 值分配到堆上
    let boxed_value = Box::new(42);
    println!("boxed_value = {}", boxed_value);

    // Box 实现了 Deref，可以像普通引用一样使用
    let sum = *boxed_value + 8;
    println!("*boxed_value + 8 = {}", sum);

    // 将字符串数据装箱
    let boxed_string = Box::new(String::from("Hello, Box!"));
    println!("boxed_string = {}", boxed_string);
    println!("boxed_string 长度 = {}", boxed_string.len()); // 自动解引用调用 String::len

    println!();

    // -------------------------------------------------------
    // 2. 堆 vs 栈分配
    // -------------------------------------------------------
    println!("--- 2. 堆 vs 栈分配 ---");

    // 栈分配：数据直接存储在栈帧中，速度快但大小必须在编译时已知
    let stack_value: i32 = 100;

    // 堆分配：数据存储在堆上，Box 指针（通常 8 字节）存储在栈上
    let heap_value: Box<i32> = Box::new(100);

    println!("栈上的值: {}", stack_value);
    println!("堆上的值: {}", heap_value);

    // 打印指针地址来观察堆/栈的区别
    println!(
        "stack_value 的地址（栈）: {:p}",
        &stack_value as *const i32
    );
    println!(
        "heap_value 指向的地址（堆）: {:p}",
        &*heap_value as *const i32
    );
    println!(
        "heap_value 自身的地址（栈）: {:p}",
        &heap_value as *const Box<i32>
    );

    // 大数组示例：栈空间有限，大数据适合放在堆上
    let large_array = Box::new([0u8; 1_000_000]); // 1MB 数据放在堆上
    println!("大数组（堆上）长度: {} 字节", large_array.len());

    println!();

    // -------------------------------------------------------
    // 3. 递归类型 — 链表示例
    // -------------------------------------------------------
    println!("--- 3. 递归类型（链表）---");

    // 如果不使用 Box，以下定义会导致编译错误：
    //   enum List { Cons(i32, List), Nil }
    // 原因：编译器无法确定 List 的大小（无限递归嵌套）
    //
    // 使用 Box<List> 后，Cons 变体的大小 = i32 (4字节) + Box 指针 (8字节)

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    // 构建链表: 1 -> 2 -> 3 -> Nil
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("链表: {:?}", list);

    // 遍历链表
    fn print_list(list: &List) {
        match list {
            Cons(value, next) => {
                print!("{} -> ", value);
                print_list(next);
            }
            Nil => println!("Nil"),
        }
    }

    print!("遍历: ");
    print_list(&list);

    // 计算链表长度
    fn list_length(list: &List) -> usize {
        match list {
            Cons(_, next) => 1 + list_length(next),
            Nil => 0,
        }
    }

    println!("链表长度: {}", list_length(&list));

    println!();

    // -------------------------------------------------------
    // 4. Box<dyn Trait> — trait 对象
    // -------------------------------------------------------
    println!("--- 4. Box<dyn Trait> --- ");

    // 定义一个 trait
    trait Animal {
        fn name(&self) -> &str;
        fn speak(&self) -> String;
    }

    struct Dog {
        name: String,
    }
    impl Animal for Dog {
        fn name(&self) -> &str {
            &self.name
        }
        fn speak(&self) -> String {
            format!("{}：汪汪！", self.name)
        }
    }

    struct Cat {
        name: String,
    }
    impl Animal for Cat {
        fn name(&self) -> &str {
            &self.name
        }
        fn speak(&self) -> String {
            format!("{}：喵喵！", self.name)
        }
    }

    // Box<dyn Animal> 可以存储任何实现了 Animal trait 的类型
    // 这叫做"动态分发"（dynamic dispatch）
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: String::from("旺财"),
        }),
        Box::new(Cat {
            name: String::from("咪咪"),
        }),
        Box::new(Dog {
            name: String::from("大黄"),
        }),
    ];

    for animal in &animals {
        println!("{}", animal.speak());
    }

    // 函数返回 Box<dyn Trait>
    fn create_animal(kind: &str) -> Box<dyn Animal> {
        match kind {
            "dog" => Box::new(Dog {
                name: String::from("小白"),
            }),
            "cat" => Box::new(Cat {
                name: String::from("小花"),
            }),
            _ => Box::new(Dog {
                name: String::from("默认狗"),
            }),
        }
    }

    let animal = create_animal("cat");
    println!("工厂创建: {}", animal.speak());

    println!();

    // -------------------------------------------------------
    // 5. Box 的 Deref 行为
    // -------------------------------------------------------
    println!("--- 5. Box 的 Deref 行为 ---");

    // Box<T> 实现了 Deref<Target = T>，所以可以自动解引用
    let boxed_str = Box::new(String::from("Rust 智能指针"));

    // 自动解引用：Box<String> -> String -> str
    // 函数接受 &str，但传入 &Box<String> 也能工作
    fn print_str(s: &str) {
        println!("字符串内容: {}", s);
    }

    print_str(&boxed_str); // 自动解引用链: &Box<String> -> &String -> &str

    // 手动解引用
    let inner: &String = &*boxed_str;
    println!("手动解引用: {}", inner);

    // Box 拥有所有权，离开作用域时自动释放堆内存
    {
        let temp_box = Box::new(vec![1, 2, 3, 4, 5]);
        println!("临时 Box 中的 Vec: {:?}", temp_box);
        // temp_box 在此处被 drop，堆上的 Vec 和其中的数据一同被释放
    }

    // 解引用移动（move out of Box）
    let boxed_vec = Box::new(vec![10, 20, 30]);
    let unboxed_vec: Vec<i32> = *boxed_vec; // 将数据从堆移到栈（所有权转移）
    println!("从 Box 中取出的 Vec: {:?}", unboxed_vec);
    // 注意：此时 boxed_vec 已经被移动，不能再使用

    println!();

    // -------------------------------------------------------
    // 6. 综合示例：用 Box 构建二叉树
    // -------------------------------------------------------
    println!("--- 6. 综合示例：二叉树 ---");

    #[derive(Debug)]
    enum BinaryTree {
        Leaf(i32),
        Node {
            value: i32,
            left: Box<BinaryTree>,
            right: Box<BinaryTree>,
        },
    }

    impl BinaryTree {
        /// 计算树中所有值的总和
        fn sum(&self) -> i32 {
            match self {
                BinaryTree::Leaf(v) => *v,
                BinaryTree::Node { value, left, right } => value + left.sum() + right.sum(),
            }
        }

        /// 计算树的深度
        fn depth(&self) -> usize {
            match self {
                BinaryTree::Leaf(_) => 1,
                BinaryTree::Node { left, right, .. } => {
                    1 + left.depth().max(right.depth())
                }
            }
        }
    }

    //       10
    //      /  \
    //     5    15
    //    / \   / \
    //   2   7 12  20
    let tree = BinaryTree::Node {
        value: 10,
        left: Box::new(BinaryTree::Node {
            value: 5,
            left: Box::new(BinaryTree::Leaf(2)),
            right: Box::new(BinaryTree::Leaf(7)),
        }),
        right: Box::new(BinaryTree::Node {
            value: 15,
            left: Box::new(BinaryTree::Leaf(12)),
            right: Box::new(BinaryTree::Leaf(20)),
        }),
    };

    println!("二叉树: {:?}", tree);
    println!("所有值的总和: {}", tree.sum()); // 10+5+2+7+15+12+20 = 71
    println!("树的深度: {}", tree.depth()); // 3

    println!("\n=== Lesson 040 完成 ===");
}
