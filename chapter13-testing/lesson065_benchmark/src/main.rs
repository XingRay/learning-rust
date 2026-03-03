// ============================================================
// Lesson 065: 性能基准测试（概念讲解）
// ============================================================
// 本课学习 Rust 中的性能基准测试，包括：
// - std::time::Instant 手动计时
// - 基准测试的基本概念和注意事项
// - criterion crate 简介（注释说明，不引入依赖）
// - std::hint::black_box 防止编译器优化
// - 性能测试最佳实践
//
// 【Rust 基准测试生态】
// 1. Nightly Rust 内置的 #[bench] 属性（不稳定特性）
// 2. criterion crate（最流行的第三方基准测试框架）
// 3. 手动使用 std::time::Instant 计时（本课重点）
//
// 由于我们不引入外部依赖，本课使用标准库进行手动计时，
// 并用注释详细说明 criterion 的使用方式。
// ============================================================

use std::hint::black_box;
use std::time::{Duration, Instant};

// ============================================================
// 被测试的函数：不同的排序和计算算法
// ============================================================

/// 冒泡排序 - O(n²)
fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// 插入排序 - O(n²)，但对近乎有序的数组更快
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

/// 迭代法计算斐波那契数
fn fibonacci_iterative(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

/// 递归法计算斐波那契数（故意低效，用于对比）
fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

/// 计算素数个数（试除法）
fn count_primes(limit: u32) -> u32 {
    let mut count = 0;
    for n in 2..=limit {
        if is_prime(n) {
            count += 1;
        }
    }
    count
}

/// 判断是否为素数
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// 字符串拼接 - 使用 String::push_str
fn string_concat_push(n: usize) -> String {
    let mut result = String::new();
    for i in 0..n {
        result.push_str(&i.to_string());
    }
    result
}

/// 字符串拼接 - 使用 format! 和 join
fn string_concat_join(n: usize) -> String {
    (0..n).map(|i| i.to_string()).collect::<Vec<_>>().join("")
}

// ============================================================
// 手动基准测试工具
// ============================================================

/// 运行一个函数多次并返回平均耗时
fn benchmark<F, R>(name: &str, iterations: u32, mut f: F) -> Duration
where
    F: FnMut() -> R,
{
    // 预热阶段（warmup）：让 CPU 缓存和分支预测器稳定
    for _ in 0..3 {
        // black_box 防止编译器把整个调用优化掉
        black_box(f());
    }

    // 正式测量
    let start = Instant::now();
    for _ in 0..iterations {
        // black_box 包裹返回值，防止编译器发现结果未使用而跳过计算
        black_box(f());
    }
    let total = start.elapsed();
    let avg = total / iterations;

    println!(
        "  {:<35} | 迭代次数: {:>6} | 总耗时: {:>10.3?} | 平均: {:>10.3?}",
        name, iterations, total, avg
    );

    avg
}

/// 对比两个函数的性能
fn compare_benchmark<F1, F2, R1, R2>(
    name1: &str,
    mut f1: F1,
    name2: &str,
    mut f2: F2,
    iterations: u32,
) where
    F1: FnMut() -> R1,
    F2: FnMut() -> R2,
{
    // 预热
    for _ in 0..3 {
        black_box(f1());
        black_box(f2());
    }

    // 测量 f1
    let start1 = Instant::now();
    for _ in 0..iterations {
        black_box(f1());
    }
    let time1 = start1.elapsed();

    // 测量 f2
    let start2 = Instant::now();
    for _ in 0..iterations {
        black_box(f2());
    }
    let time2 = start2.elapsed();

    let avg1 = time1 / iterations;
    let avg2 = time2 / iterations;

    println!("  {:<30} 平均: {:>10.3?}", name1, avg1);
    println!("  {:<30} 平均: {:>10.3?}", name2, avg2);

    if avg1 < avg2 {
        let ratio = avg2.as_nanos() as f64 / avg1.as_nanos().max(1) as f64;
        println!("  → {} 更快，约 {:.2}x", name1, ratio);
    } else if avg2 < avg1 {
        let ratio = avg1.as_nanos() as f64 / avg2.as_nanos().max(1) as f64;
        println!("  → {} 更快，约 {:.2}x", name2, ratio);
    } else {
        println!("  → 两者速度相当");
    }
}

fn main() {
    println!("=== Lesson 065: 性能基准测试 ===\n");

    // ============================================================
    // 1. std::time::Instant 基本用法
    // ============================================================
    println!("--- 1. Instant 基本计时 ---");
    {
        let start = Instant::now();

        // 执行一些工作
        let mut sum: u64 = 0;
        for i in 0..1_000_000 {
            sum += black_box(i);
        }

        let elapsed = start.elapsed();
        println!("  累加 1,000,000 次，结果: {}, 耗时: {:?}", sum, elapsed);
        println!("  毫秒: {} ms", elapsed.as_millis());
        println!("  微秒: {} µs", elapsed.as_micros());
        println!("  纳秒: {} ns", elapsed.as_nanos());
    }

    // ============================================================
    // 2. black_box 的作用
    // ============================================================
    println!("\n--- 2. std::hint::black_box 的作用 ---");
    println!("  black_box(value) 告诉编译器：");
    println!("    - 这个值有「副作用」，不要优化掉对它的计算");
    println!("    - 不要内联或删除相关的计算过程");
    println!("  没有 black_box，编译器可能会：");
    println!("    - 发现返回值没被使用，直接跳过整个函数调用");
    println!("    - 在编译期算出结果，运行时什么都不做");
    println!("    - 导致测量结果为 0 纳秒，完全不准确");
    {
        // 示例：没有 black_box 可能被优化掉
        let start = Instant::now();
        let result = black_box(fibonacci_iterative(black_box(30)));
        let elapsed = start.elapsed();
        println!("  fibonacci_iterative(30) = {}, 耗时: {:?}", result, elapsed);
    }

    // ============================================================
    // 3. 排序算法基准测试
    // ============================================================
    println!("\n--- 3. 排序算法基准测试 ---");
    {
        let size = 500;
        let iterations = 100;

        println!("  数组大小: {}, 迭代次数: {}\n", size, iterations);

        // 生成测试数据（每次迭代使用同样的乱序数据）
        let base_data: Vec<i32> = {
            // 简单的伪随机生成
            let mut data = Vec::with_capacity(size);
            let mut seed: u64 = 42;
            for _ in 0..size {
                seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
                data.push((seed >> 33) as i32);
            }
            data
        };

        benchmark("冒泡排序", iterations, || {
            let mut data = base_data.clone();
            bubble_sort(&mut data);
            data
        });

        benchmark("插入排序", iterations, || {
            let mut data = base_data.clone();
            insertion_sort(&mut data);
            data
        });

        benchmark("标准库排序 (sort)", iterations, || {
            let mut data = base_data.clone();
            data.sort();
            data
        });

        benchmark("标准库排序 (sort_unstable)", iterations, || {
            let mut data = base_data.clone();
            data.sort_unstable();
            data
        });
    }

    // ============================================================
    // 4. 斐波那契：迭代 vs 递归
    // ============================================================
    println!("\n--- 4. 斐波那契算法对比 ---");
    {
        let n = 25;
        let iterations = 100;
        println!("  计算 fibonacci({}), 迭代次数: {}\n", n, iterations);

        compare_benchmark(
            "迭代法",
            || fibonacci_iterative(black_box(n)),
            "递归法",
            || fibonacci_recursive(black_box(n)),
            iterations,
        );
    }

    // ============================================================
    // 5. 字符串拼接方式对比
    // ============================================================
    println!("\n--- 5. 字符串拼接方式对比 ---");
    {
        let n = 1000;
        let iterations = 100;
        println!("  拼接 {} 个数字, 迭代次数: {}\n", n, iterations);

        compare_benchmark(
            "push_str 方式",
            || string_concat_push(black_box(n)),
            "join 方式",
            || string_concat_join(black_box(n)),
            iterations,
        );
    }

    // ============================================================
    // 6. 素数计算
    // ============================================================
    println!("\n--- 6. 素数计算基准测试 ---");
    {
        let limit = 10_000;
        let iterations = 10;
        println!("  计算 {} 以内的素数, 迭代次数: {}\n", limit, iterations);

        benchmark("count_primes(10000)", iterations, || {
            count_primes(black_box(limit))
        });

        let count = count_primes(limit);
        println!("  {} 以内共有 {} 个素数", limit, count);
    }

    // ============================================================
    // 7. criterion crate 说明（注释）
    // ============================================================
    println!("\n--- 7. criterion crate 简介 ---");
    println!("  criterion 是 Rust 最流行的基准测试框架，提供：");
    println!("    - 统计学分析（平均值、标准差、置信区间）");
    println!("    - 自动确定合适的迭代次数");
    println!("    - HTML 报告和图表生成");
    println!("    - 性能回归检测（与历史结果对比）");
    println!("    - 参数化基准测试");
    println!();
    println!("  使用 criterion 的步骤：");
    println!("  1. Cargo.toml 添加：");
    println!("     [dev-dependencies]");
    println!("     criterion = {{ version = \"0.5\", features = [\"html_reports\"] }}");
    println!("     [[bench]]");
    println!("     name = \"my_benchmark\"");
    println!("     harness = false");
    println!("  2. 创建 benches/my_benchmark.rs");
    println!("  3. 运行：cargo bench");
    println!();

    // ============================================================
    // criterion 代码示例（注释形式）
    // ============================================================
    // ```rust
    // // benches/my_benchmark.rs
    // use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
    //
    // fn bench_fibonacci(c: &mut Criterion) {
    //     // 简单基准测试
    //     c.bench_function("fibonacci_iterative_20", |b| {
    //         b.iter(|| fibonacci_iterative(criterion::black_box(20)));
    //     });
    //
    //     // 参数化基准测试（不同输入大小）
    //     let mut group = c.benchmark_group("fibonacci");
    //     for n in [10, 20, 30, 40].iter() {
    //         group.bench_with_input(BenchmarkId::new("iterative", n), n, |b, &n| {
    //             b.iter(|| fibonacci_iterative(criterion::black_box(n)));
    //         });
    //     }
    //     group.finish();
    // }
    //
    // fn bench_sorting(c: &mut Criterion) {
    //     let data: Vec<i32> = (0..1000).rev().collect();
    //
    //     let mut group = c.benchmark_group("sorting");
    //     group.bench_function("bubble_sort", |b| {
    //         b.iter(|| {
    //             let mut d = data.clone();
    //             bubble_sort(&mut d);
    //         });
    //     });
    //     group.bench_function("insertion_sort", |b| {
    //         b.iter(|| {
    //             let mut d = data.clone();
    //             insertion_sort(&mut d);
    //         });
    //     });
    //     group.finish();
    // }
    //
    // criterion_group!(benches, bench_fibonacci, bench_sorting);
    // criterion_main!(benches);
    // ```

    // ============================================================
    // 8. 性能测试最佳实践
    // ============================================================
    println!("\n--- 8. 性能测试最佳实践 ---");
    println!("  1. 始终使用 black_box 防止编译器过度优化");
    println!("  2. 预热（warmup）：正式测量前先运行几次");
    println!("  3. 多次迭代取平均值，减少随机误差");
    println!("  4. 使用 Release 模式：cargo bench 或 cargo run --release");
    println!("  5. 关闭其他程序，减少系统负载干扰");
    println!("  6. 注意内存分配的影响（clone、Vec::new 等）");
    println!("  7. 分离「setup 代码」和「被测代码」");
    println!("  8. 测试有代表性的输入数据");
    println!("  9. 正式基准测试优先使用 criterion crate");
    println!(" 10. 不要过早优化——先正确，再优化！");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 3, 8, 1, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![5, 3, 8, 1, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8]);
    }

    #[test]
    fn test_fibonacci_iterative() {
        assert_eq!(fibonacci_iterative(0), 0);
        assert_eq!(fibonacci_iterative(1), 1);
        assert_eq!(fibonacci_iterative(10), 55);
    }

    #[test]
    fn test_fibonacci_recursive() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(10), 55);
    }

    #[test]
    fn test_count_primes() {
        assert_eq!(count_primes(10), 4);   // 2, 3, 5, 7
        assert_eq!(count_primes(100), 25);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(97));
    }

    #[test]
    fn test_string_concat_results_equal() {
        // 两种拼接方式结果应该相同
        let result1 = string_concat_push(20);
        let result2 = string_concat_join(20);
        assert_eq!(result1, result2);
    }
}
