// ============================================================
// Lesson 044: Weak<T> 引用 — 解决循环引用问题
// ============================================================
// Weak<T> 是 Rc<T> 的弱引用版本，不增加 strong_count。
// 当所有 Rc（强引用）被 drop 后，即使还有 Weak 存在，数据也会被释放。
//
// 主要用途：打破 Rc<T> 的循环引用，避免内存泄漏。
// 典型场景：树结构中子节点指向父节点。

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("=== Lesson 044: Weak 引用 ===\n");

    // -------------------------------------------------------
    // 1. 循环引用问题 — Rc 导致内存泄漏
    // -------------------------------------------------------
    println!("--- 1. 循环引用问题 ---");

    // 展示循环引用如何导致内存泄漏
    // （以下代码会创建泄漏，但不会导致程序崩溃）

    #[derive(Debug)]
    enum LeakyList {
        Cons(i32, RefCell<Rc<LeakyList>>),
        Nil,
    }

    impl LeakyList {
        fn tail(&self) -> Option<&RefCell<Rc<LeakyList>>> {
            match self {
                LeakyList::Cons(_, next) => Some(next),
                LeakyList::Nil => None,
            }
        }
    }

    // 构造循环引用来演示问题
    let a = Rc::new(LeakyList::Cons(1, RefCell::new(Rc::new(LeakyList::Nil))));
    println!("a 初始引用计数: {}", Rc::strong_count(&a));

    let b = Rc::new(LeakyList::Cons(2, RefCell::new(Rc::clone(&a))));
    println!("b 创建后:");
    println!("  a 引用计数: {}", Rc::strong_count(&a)); // 2
    println!("  b 引用计数: {}", Rc::strong_count(&b)); // 1

    // 制造循环引用：让 a 的 tail 指向 b
    if let Some(tail) = a.tail() {
        *tail.borrow_mut() = Rc::clone(&b);
    }

    println!("循环引用建立后:");
    println!("  a 引用计数: {}", Rc::strong_count(&a)); // 2
    println!("  b 引用计数: {}", Rc::strong_count(&b)); // 2

    // ⚠️ 此时 a 和 b 互相引用：
    //   a -> b -> a -> b -> ...（无限循环）
    //
    // 即使 a 和 b 离开作用域，它们的引用计数也只会从 2 变成 1
    // 永远不会归零，内存永远不会被释放！
    //
    // 如果取消下面的注释，会导致栈溢出（无限递归打印）：
    // println!("a: {:?}", a);

    println!("\n⚠️ 循环引用已创建！这些内存将永远不会被释放（内存泄漏）");
    println!("解决方案：使用 Weak<T> 打破循环\n");

    // -------------------------------------------------------
    // 2. Weak::new 与 Rc::downgrade
    // -------------------------------------------------------
    println!("--- 2. Weak::new 与 Rc::downgrade ---");

    // 创建一个空的 Weak（不指向任何数据）
    let empty_weak: Weak<i32> = Weak::new();
    println!("空 Weak upgrade: {:?}", empty_weak.upgrade()); // None

    // 从 Rc 创建 Weak（降级）
    let strong = Rc::new(String::from("Rust 很棒"));
    let weak = Rc::downgrade(&strong);

    println!("strong 值: {}", strong);
    println!("strong_count: {}", Rc::strong_count(&strong)); // 1
    println!("weak_count:   {}", Rc::weak_count(&strong)); // 1

    // 创建更多的弱引用
    let weak2 = Rc::downgrade(&strong);
    let weak3 = Weak::clone(&weak); // 也可以 clone Weak
    println!("三个弱引用后:");
    println!("  strong_count: {}", Rc::strong_count(&strong)); // 1（不变！）
    println!("  weak_count:   {}", Rc::weak_count(&strong)); // 3

    drop(weak2);
    drop(weak3);

    println!();

    // -------------------------------------------------------
    // 3. upgrade 方法 — 返回 Option<Rc<T>>
    // -------------------------------------------------------
    println!("--- 3. upgrade 方法 ---");

    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    let weak_ref = Rc::downgrade(&data);

    // upgrade() 尝试将 Weak 升级为 Rc
    // 如果数据仍然存在，返回 Some(Rc<T>)
    // 如果数据已被释放，返回 None
    match weak_ref.upgrade() {
        Some(rc) => println!("升级成功，数据: {:?}", rc),
        None => println!("数据已被释放"),
    }

    println!("drop 前 strong_count: {}", Rc::strong_count(&data));

    // 释放唯一的强引用
    drop(data);

    // 数据已释放，upgrade 返回 None
    match weak_ref.upgrade() {
        Some(rc) => println!("升级成功，数据: {:?}", rc),
        None => println!("升级失败：数据已被释放（所有强引用都已 drop）"),
    }

    println!();

    // 更详细的生命周期演示
    println!("--- 生命周期详细演示 ---");

    let weak_ref;
    {
        let rc = Rc::new("临时数据");
        weak_ref = Rc::downgrade(&rc);

        println!("作用域内 upgrade: {:?}", weak_ref.upgrade()); // Some("临时数据")
        println!("  strong_count: {}", Rc::strong_count(&rc)); // 注意 upgrade 也会临时增加
        // rc 在这里被 drop
    }

    println!("作用域外 upgrade: {:?}", weak_ref.upgrade()); // None

    println!();

    // -------------------------------------------------------
    // 4. 树结构中父子节点的 Weak 示例
    // -------------------------------------------------------
    println!("--- 4. 树结构：Weak 解决父子引用 ---");

    // 树节点：
    //   - children 使用 Rc（父节点拥有子节点）
    //   - parent 使用 Weak（子节点不拥有父节点，避免循环引用）

    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        parent: RefCell<Weak<TreeNode>>,       // 弱引用指向父节点
        children: RefCell<Vec<Rc<TreeNode>>>,   // 强引用拥有子节点
    }

    impl TreeNode {
        fn new(value: i32) -> Rc<TreeNode> {
            Rc::new(TreeNode {
                value,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            })
        }

        fn add_child(parent: &Rc<TreeNode>, child: &Rc<TreeNode>) {
            // 将子节点添加到父节点的 children 列表
            parent.children.borrow_mut().push(Rc::clone(child));
            // 设置子节点的 parent 为弱引用
            *child.parent.borrow_mut() = Rc::downgrade(parent);
        }

        fn parent_value(&self) -> Option<i32> {
            // 尝试获取父节点的值
            self.parent
                .borrow()
                .upgrade()
                .map(|parent| parent.value)
        }
    }

    // 构建树：
    //       root(1)
    //      /       \
    //   child_a(2)  child_b(3)
    //     |
    //   grandchild(4)

    let root = TreeNode::new(1);
    let child_a = TreeNode::new(2);
    let child_b = TreeNode::new(3);
    let grandchild = TreeNode::new(4);

    TreeNode::add_child(&root, &child_a);
    TreeNode::add_child(&root, &child_b);
    TreeNode::add_child(&child_a, &grandchild);

    println!("root 的值: {}", root.value);
    println!("root 的子节点数: {}", root.children.borrow().len());

    println!(
        "child_a 的父节点值: {:?}",
        child_a.parent_value()
    ); // Some(1)
    println!(
        "child_b 的父节点值: {:?}",
        child_b.parent_value()
    ); // Some(1)
    println!(
        "grandchild 的父节点值: {:?}",
        grandchild.parent_value()
    ); // Some(2)
    println!(
        "root 的父节点值: {:?}",
        root.parent_value()
    ); // None

    // 查看引用计数
    println!("\n引用计数详情:");
    println!(
        "  root       - strong: {}, weak: {}",
        Rc::strong_count(&root),
        Rc::weak_count(&root)
    );
    println!(
        "  child_a    - strong: {}, weak: {}",
        Rc::strong_count(&child_a),
        Rc::weak_count(&child_a)
    );
    println!(
        "  child_b    - strong: {}, weak: {}",
        Rc::strong_count(&child_b),
        Rc::weak_count(&child_b)
    );
    println!(
        "  grandchild - strong: {}, weak: {}",
        Rc::strong_count(&grandchild),
        Rc::weak_count(&grandchild)
    );

    println!();

    // -------------------------------------------------------
    // 5. 更完整的树操作示例
    // -------------------------------------------------------
    println!("--- 5. 完整的树操作 ---");

    impl TreeNode {
        /// 打印从当前节点到根节点的路径
        fn path_to_root(node: &Rc<TreeNode>) -> Vec<i32> {
            let mut path = vec![node.value];
            let mut current_parent = node.parent.borrow().clone();

            while let Some(parent_rc) = current_parent.upgrade() {
                path.push(parent_rc.value);
                current_parent = parent_rc.parent.borrow().clone();
            }

            path
        }

        /// 计算子树中所有节点值的和
        fn subtree_sum(node: &Rc<TreeNode>) -> i32 {
            let children_sum: i32 = node
                .children
                .borrow()
                .iter()
                .map(|child| TreeNode::subtree_sum(child))
                .sum();
            node.value + children_sum
        }

        /// 获取所有叶子节点的值
        fn leaf_values(node: &Rc<TreeNode>) -> Vec<i32> {
            let children = node.children.borrow();
            if children.is_empty() {
                vec![node.value]
            } else {
                children
                    .iter()
                    .flat_map(|child| TreeNode::leaf_values(child))
                    .collect()
            }
        }
    }

    println!(
        "grandchild 到根的路径: {:?}",
        TreeNode::path_to_root(&grandchild)
    ); // [4, 2, 1]
    println!(
        "child_a 到根的路径: {:?}",
        TreeNode::path_to_root(&child_a)
    ); // [2, 1]
    println!(
        "root 到根的路径: {:?}",
        TreeNode::path_to_root(&root)
    ); // [1]

    println!("整棵树的值总和: {}", TreeNode::subtree_sum(&root)); // 1+2+3+4 = 10
    println!("child_a 子树的值总和: {}", TreeNode::subtree_sum(&child_a)); // 2+4 = 6

    println!("叶子节点: {:?}", TreeNode::leaf_values(&root)); // [4, 3]

    println!();

    // -------------------------------------------------------
    // 6. Weak 的正确释放验证
    // -------------------------------------------------------
    println!("--- 6. Weak 的正确释放验证 ---");

    // 演示使用 Weak 后，节点能被正确释放（对比循环引用的泄漏）

    #[derive(Debug)]
    struct TrackedNode {
        name: String,
        parent: RefCell<Weak<TrackedNode>>,
        children: RefCell<Vec<Rc<TrackedNode>>>,
    }

    impl TrackedNode {
        fn new(name: &str) -> Rc<TrackedNode> {
            println!("  [+] 创建节点: {}", name);
            Rc::new(TrackedNode {
                name: String::from(name),
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![]),
            })
        }
    }

    impl Drop for TrackedNode {
        fn drop(&mut self) {
            println!("  [-] 释放节点: {}", self.name);
        }
    }

    println!("创建树结构:");
    {
        let parent = TrackedNode::new("父节点");
        let child1 = TrackedNode::new("子节点1");
        let child2 = TrackedNode::new("子节点2");

        // 建立父子关系（使用 Weak 引用父节点）
        parent.children.borrow_mut().push(Rc::clone(&child1));
        parent.children.borrow_mut().push(Rc::clone(&child2));
        *child1.parent.borrow_mut() = Rc::downgrade(&parent);
        *child2.parent.borrow_mut() = Rc::downgrade(&parent);

        println!("  树结构已建立");
        println!(
            "  parent strong_count: {}",
            Rc::strong_count(&parent)
        );
        println!("  parent weak_count: {}", Rc::weak_count(&parent));

        println!("离开作用域，所有节点应该被释放:");
        // 释放顺序：child2, child1, parent（逆序）
        // 因为使用了 Weak，没有循环引用，所有节点都能正确释放！
    }
    println!("  ✅ 所有节点已正确释放，没有内存泄漏！");

    println!();

    // -------------------------------------------------------
    // 7. 总结
    // -------------------------------------------------------
    println!("--- 7. 总结 ---");
    println!("┌──────────────┬────────────┬────────────┐");
    println!("│ 特性         │ Rc<T>      │ Weak<T>    │");
    println!("├──────────────┼────────────┼────────────┤");
    println!("│ 影响释放     │ ✅ 是      │ ❌ 否      │");
    println!("│ 增加计数     │ strong +1  │ weak +1    │");
    println!("│ 访问数据     │ 直接访问   │ 需 upgrade │");
    println!("│ 数据保证存在 │ ✅ 是      │ ❌ 可能无  │");
    println!("│ 用途         │ 共享所有权 │ 打破循环   │");
    println!("└──────────────┴────────────┴────────────┘");
    println!();
    println!("核心原则：");
    println!("  - 强引用（Rc）表示所有权关系");
    println!("  - 弱引用（Weak）表示非所有权的观察关系");
    println!("  - 父 -> 子：用 Rc（父拥有子）");
    println!("  - 子 -> 父：用 Weak（子不拥有父）");

    println!("\n=== Lesson 044 完成 ===");
}
