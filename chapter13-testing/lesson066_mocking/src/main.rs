// ============================================================
// Lesson 066: Mock 测试（概念讲解）
// ============================================================
// 本课学习 Rust 中的 Mock 测试技术，包括：
// - 用 trait 实现依赖注入的手动 Mock
// - 测试替身概念：Stub、Mock、Fake、Spy、Dummy
// - 提及 mockall crate（注释说明，不引入依赖）
// - 通过 trait 抽象实现可测试的代码架构
//
// 【核心理念】
// Mock 测试的目的是在测试中替换掉外部依赖（数据库、网络、文件系统等），
// 使测试快速、可靠、可重复。
//
// Rust 中实现 Mock 的主要方式：
// 1. 定义 trait 描述依赖的接口
// 2. 生产代码使用 trait 而不是具体类型（依赖注入）
// 3. 测试中提供 trait 的 Mock 实现
//
// 这种方式不需要任何外部库，是 Rust 惯用的做法。
// ============================================================

use std::collections::HashMap;

// ============================================================
// 测试替身概念
// ============================================================
// 测试替身 (Test Double) 是一个通用术语，指在测试中替代真实对象的对象。
// 常见类型：
//
// 1. Dummy（假对象）
//    - 仅用于填充参数，不会被实际使用
//    - 例如：传入一个空的 Logger
//
// 2. Stub（桩）
//    - 返回预设的固定值，不记录调用信息
//    - 例如：总是返回固定温度的温度传感器
//
// 3. Fake（伪实现）
//    - 有实际的工作逻辑，但是简化版本
//    - 例如：内存中的 HashMap 代替真实数据库
//
// 4. Mock（模拟对象）
//    - 记录调用信息，可以验证是否被正确调用
//    - 例如：验证邮件发送函数被调用了几次，参数是什么
//
// 5. Spy（间谍）
//    - 包装真实对象，记录调用的同时执行真实逻辑
//    - Rust 中较少使用
// ============================================================

// ============================================================
// 示例 1：数据存储的 Mock（Fake 实现）
// ============================================================

/// 定义数据存储的 trait（接口抽象）
/// 生产代码和测试代码都依赖这个 trait，而不是具体实现
#[allow(dead_code)]
trait UserStore {
    fn save(&mut self, user: User) -> Result<(), String>;
    fn find_by_id(&self, id: u32) -> Option<&User>;
    fn find_by_email(&self, email: &str) -> Option<&User>;
    fn delete(&mut self, id: u32) -> Result<(), String>;
    fn count(&self) -> usize;
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    email: String,
}

/// 真实的数据库存储（生产环境使用）
/// 这里用注释模拟，实际会连接数据库
struct _RealDatabaseStore {
    // connection: DatabaseConnection,
}

// 真实实现（略，在生产环境中会操作真正的数据库）
// impl UserStore for RealDatabaseStore { ... }

/// Fake 实现：用内存 HashMap 模拟数据库
/// 这是一个有实际工作逻辑的简化版本
struct FakeUserStore {
    users: HashMap<u32, User>,
}

impl FakeUserStore {
    fn new() -> Self {
        FakeUserStore {
            users: HashMap::new(),
        }
    }
}

impl UserStore for FakeUserStore {
    fn save(&mut self, user: User) -> Result<(), String> {
        // 检查邮箱是否已存在
        if self.users.values().any(|u| u.email == user.email && u.id != user.id) {
            return Err(format!("邮箱 {} 已被注册", user.email));
        }
        self.users.insert(user.id, user);
        Ok(())
    }

    fn find_by_id(&self, id: u32) -> Option<&User> {
        self.users.get(&id)
    }

    fn find_by_email(&self, email: &str) -> Option<&User> {
        self.users.values().find(|u| u.email == email)
    }

    fn delete(&mut self, id: u32) -> Result<(), String> {
        self.users
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| format!("用户 {} 不存在", id))
    }

    fn count(&self) -> usize {
        self.users.len()
    }
}

// ============================================================
// 示例 2：通知服务的 Mock（记录调用）
// ============================================================

/// 通知服务 trait
trait NotificationService {
    fn send_email(&mut self, to: &str, subject: &str, body: &str) -> Result<(), String>;
    fn send_sms(&mut self, phone: &str, message: &str) -> Result<(), String>;
}

/// 真实的通知服务（生产环境）
struct _RealNotificationService;
// impl NotificationService for RealNotificationService {
//     // 实际发送邮件和短信...
// }

/// Mock 通知服务：记录所有调用，不实际发送
struct MockNotificationService {
    email_calls: Vec<(String, String, String)>, // (to, subject, body)
    sms_calls: Vec<(String, String)>,           // (phone, message)
    should_fail: bool,                          // 模拟失败情况
}

#[allow(dead_code)]
impl MockNotificationService {
    fn new() -> Self {
        MockNotificationService {
            email_calls: Vec::new(),
            sms_calls: Vec::new(),
            should_fail: false,
        }
    }

    /// 设置是否模拟失败
    fn set_should_fail(&mut self, fail: bool) {
        self.should_fail = fail;
    }

    /// 验证邮件被调用了多少次
    fn email_call_count(&self) -> usize {
        self.email_calls.len()
    }

    /// 验证短信被调用了多少次
    fn sms_call_count(&self) -> usize {
        self.sms_calls.len()
    }

    /// 获取最后一次邮件调用的参数
    fn last_email(&self) -> Option<&(String, String, String)> {
        self.email_calls.last()
    }

    /// 获取最后一次短信调用的参数
    fn last_sms(&self) -> Option<&(String, String)> {
        self.sms_calls.last()
    }
}

impl NotificationService for MockNotificationService {
    fn send_email(&mut self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        if self.should_fail {
            return Err("模拟：邮件发送失败".to_string());
        }
        self.email_calls
            .push((to.to_string(), subject.to_string(), body.to_string()));
        Ok(())
    }

    fn send_sms(&mut self, phone: &str, message: &str) -> Result<(), String> {
        if self.should_fail {
            return Err("模拟：短信发送失败".to_string());
        }
        self.sms_calls
            .push((phone.to_string(), message.to_string()));
        Ok(())
    }
}

// ============================================================
// 示例 3：Stub（返回固定值的简单替身）
// ============================================================

/// 天气服务 trait
trait WeatherService {
    fn get_temperature(&self, city: &str) -> Result<f64, String>;
    fn get_humidity(&self, city: &str) -> Result<f64, String>;
}

/// Stub 实现：总是返回固定值
struct StubWeatherService {
    temperature: f64,
    humidity: f64,
}

impl StubWeatherService {
    fn new(temperature: f64, humidity: f64) -> Self {
        StubWeatherService {
            temperature,
            humidity,
        }
    }
}

impl WeatherService for StubWeatherService {
    fn get_temperature(&self, _city: &str) -> Result<f64, String> {
        Ok(self.temperature)
    }

    fn get_humidity(&self, _city: &str) -> Result<f64, String> {
        Ok(self.humidity)
    }
}

// ============================================================
// 业务逻辑：使用 trait 抽象，支持依赖注入
// ============================================================

/// 用户注册服务 —— 依赖 UserStore 和 NotificationService
/// 通过泛型接受任何实现了对应 trait 的类型
struct RegistrationService<S: UserStore, N: NotificationService> {
    store: S,
    notifier: N,
}

impl<S: UserStore, N: NotificationService> RegistrationService<S, N> {
    fn new(store: S, notifier: N) -> Self {
        RegistrationService { store, notifier }
    }

    /// 注册新用户
    fn register(&mut self, id: u32, name: &str, email: &str) -> Result<(), String> {
        // 验证输入
        if name.is_empty() {
            return Err("用户名不能为空".to_string());
        }
        if !email.contains('@') {
            return Err("邮箱格式不正确".to_string());
        }

        // 检查邮箱是否已注册
        if self.store.find_by_email(email).is_some() {
            return Err(format!("邮箱 {} 已被注册", email));
        }

        // 保存用户
        let user = User {
            id,
            name: name.to_string(),
            email: email.to_string(),
        };
        self.store.save(user)?;

        // 发送欢迎邮件
        self.notifier.send_email(
            email,
            "欢迎注册",
            &format!("亲爱的 {}，欢迎加入！", name),
        )?;

        Ok(())
    }

    /// 获取用户总数
    fn user_count(&self) -> usize {
        self.store.count()
    }
}

/// 天气预报服务 —— 依赖 WeatherService
struct WeatherReport<W: WeatherService> {
    service: W,
}

impl<W: WeatherService> WeatherReport<W> {
    fn new(service: W) -> Self {
        WeatherReport { service }
    }

    /// 获取天气描述
    fn describe(&self, city: &str) -> String {
        let temp = self.service.get_temperature(city).unwrap_or(0.0);
        let humidity = self.service.get_humidity(city).unwrap_or(0.0);

        let temp_desc = if temp > 30.0 {
            "炎热"
        } else if temp > 20.0 {
            "温暖"
        } else if temp > 10.0 {
            "凉爽"
        } else {
            "寒冷"
        };

        let humidity_desc = if humidity > 80.0 {
            "潮湿"
        } else if humidity > 50.0 {
            "适中"
        } else {
            "干燥"
        };

        format!(
            "{}：温度 {:.1}°C（{}），湿度 {:.0}%（{}）",
            city, temp, temp_desc, humidity, humidity_desc
        )
    }

    /// 判断是否需要带伞
    fn need_umbrella(&self, city: &str) -> bool {
        self.service.get_humidity(city).unwrap_or(0.0) > 80.0
    }
}

fn main() {
    println!("=== Lesson 066: Mock 测试 ===\n");

    // ---- 演示 Fake 数据存储 ----
    println!("--- 1. Fake 数据存储演示 ---");
    {
        let mut store = FakeUserStore::new();
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        };
        store.save(user).unwrap();
        println!("保存用户后，总数: {}", store.count());

        if let Some(found) = store.find_by_id(1) {
            println!("按 ID 查找: {:?}", found);
        }
        if let Some(found) = store.find_by_email("alice@example.com") {
            println!("按邮箱查找: {:?}", found);
        }
    }

    // ---- 演示 Mock 通知服务 ----
    println!("\n--- 2. Mock 通知服务演示 ---");
    {
        let mut mock = MockNotificationService::new();
        mock.send_email("test@example.com", "测试", "这是测试邮件")
            .unwrap();
        mock.send_sms("13800138000", "验证码：1234").unwrap();

        println!("邮件调用次数: {}", mock.email_call_count());
        println!("短信调用次数: {}", mock.sms_call_count());
        println!("最后一封邮件: {:?}", mock.last_email());
        println!("最后一条短信: {:?}", mock.last_sms());
    }

    // ---- 演示注册服务 ----
    println!("\n--- 3. 注册服务（使用 Fake + Mock）---");
    {
        let store = FakeUserStore::new();
        let notifier = MockNotificationService::new();
        let mut service = RegistrationService::new(store, notifier);

        match service.register(1, "张三", "zhangsan@example.com") {
            Ok(()) => println!("注册成功！用户总数: {}", service.user_count()),
            Err(e) => println!("注册失败: {}", e),
        }

        match service.register(2, "李四", "lisi@example.com") {
            Ok(()) => println!("注册成功！用户总数: {}", service.user_count()),
            Err(e) => println!("注册失败: {}", e),
        }

        // 尝试重复邮箱
        match service.register(3, "王五", "zhangsan@example.com") {
            Ok(()) => println!("注册成功！"),
            Err(e) => println!("注册失败（预期）: {}", e),
        }
    }

    // ---- 演示天气预报（Stub）----
    println!("\n--- 4. 天气预报（使用 Stub）---");
    {
        let stub = StubWeatherService::new(25.0, 65.0);
        let report = WeatherReport::new(stub);
        println!("{}", report.describe("北京"));
        println!("需要带伞: {}", report.need_umbrella("北京"));

        let rainy_stub = StubWeatherService::new(18.0, 90.0);
        let rainy_report = WeatherReport::new(rainy_stub);
        println!("{}", rainy_report.describe("上海"));
        println!("需要带伞: {}", rainy_report.need_umbrella("上海"));
    }

    // ---- mockall crate 说明 ----
    println!("\n--- 5. mockall crate 简介 ---");
    println!("  mockall 是 Rust 最流行的 Mock 框架，提供：");
    println!("    - #[automock] 属性宏：自动为 trait 生成 Mock 类型");
    println!("    - 期望设置（expect_xxx）：指定方法应该被调用几次");
    println!("    - 参数匹配器（predicate）：灵活的参数验证");
    println!("    - 返回值设置（returning）：指定返回值或回调");
    println!("    - 调用顺序验证（Sequence）");
    println!();
    println!("  使用示例（需要添加依赖）：");
    println!("  Cargo.toml:");
    println!("    [dev-dependencies]");
    println!("    mockall = \"0.13\"");
    println!();

    // ============================================================
    // mockall 代码示例（注释形式）
    // ============================================================
    // ```rust
    // use mockall::automock;
    //
    // #[automock]
    // trait WeatherService {
    //     fn get_temperature(&self, city: &str) -> Result<f64, String>;
    //     fn get_humidity(&self, city: &str) -> Result<f64, String>;
    // }
    //
    // #[test]
    // fn test_weather_report_with_mockall() {
    //     let mut mock = MockWeatherService::new();
    //
    //     // 设置期望：get_temperature 被调用1次，参数为 "北京"，返回 25.0
    //     mock.expect_get_temperature()
    //         .with(mockall::predicate::eq("北京"))
    //         .times(1)
    //         .returning(|_| Ok(25.0));
    //
    //     mock.expect_get_humidity()
    //         .with(mockall::predicate::eq("北京"))
    //         .times(1)
    //         .returning(|_| Ok(65.0));
    //
    //     let report = WeatherReport::new(mock);
    //     let desc = report.describe("北京");
    //     assert!(desc.contains("温暖"));
    //
    //     // mock 被 drop 时会自动验证所有期望是否被满足
    // }
    // ```

    // ---- 最佳实践 ----
    println!("\n--- 6. Mock 测试最佳实践 ---");
    println!("  1. 用 trait 定义外部依赖的接口");
    println!("  2. 业务逻辑通过泛型或 trait object 接受依赖");
    println!("  3. 简单场景用手动 Mock，复杂场景用 mockall");
    println!("  4. Fake > Stub > Mock（优先使用简单的替身）");
    println!("  5. 不要 Mock 你不拥有的类型（先包装再 Mock）");
    println!("  6. Mock 应该反映真实行为的简化版");
    println!("  7. 测试行为，而不是实现细节");
    println!("  8. 避免过度 Mock —— 太多 Mock 意味着设计可能有问题");
}

// ============================================================
// 测试模块
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    // ---- 测试 FakeUserStore ----

    #[test]
    fn test_fake_store_save_and_find() {
        let mut store = FakeUserStore::new();
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@test.com".to_string(),
        };

        assert!(store.save(user.clone()).is_ok());
        assert_eq!(store.count(), 1);

        let found = store.find_by_id(1);
        assert_eq!(found, Some(&user));
    }

    #[test]
    fn test_fake_store_find_by_email() {
        let mut store = FakeUserStore::new();
        store
            .save(User {
                id: 1,
                name: "Bob".to_string(),
                email: "bob@test.com".to_string(),
            })
            .unwrap();

        let found = store.find_by_email("bob@test.com");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Bob");

        assert!(store.find_by_email("unknown@test.com").is_none());
    }

    #[test]
    fn test_fake_store_duplicate_email() {
        let mut store = FakeUserStore::new();
        store
            .save(User {
                id: 1,
                name: "Alice".to_string(),
                email: "same@test.com".to_string(),
            })
            .unwrap();

        let result = store.save(User {
            id: 2,
            name: "Bob".to_string(),
            email: "same@test.com".to_string(),
        });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("已被注册"));
    }

    #[test]
    fn test_fake_store_delete() {
        let mut store = FakeUserStore::new();
        store
            .save(User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@test.com".to_string(),
            })
            .unwrap();

        assert!(store.delete(1).is_ok());
        assert_eq!(store.count(), 0);
        assert!(store.delete(1).is_err()); // 删除不存在的用户
    }

    // ---- 测试 MockNotificationService ----

    #[test]
    fn test_mock_notification_records_calls() {
        let mut mock = MockNotificationService::new();

        mock.send_email("a@test.com", "标题1", "内容1").unwrap();
        mock.send_email("b@test.com", "标题2", "内容2").unwrap();
        mock.send_sms("13800000000", "验证码").unwrap();

        assert_eq!(mock.email_call_count(), 2);
        assert_eq!(mock.sms_call_count(), 1);

        let last_email = mock.last_email().unwrap();
        assert_eq!(last_email.0, "b@test.com");
        assert_eq!(last_email.1, "标题2");

        let last_sms = mock.last_sms().unwrap();
        assert_eq!(last_sms.0, "13800000000");
    }

    #[test]
    fn test_mock_notification_failure() {
        let mut mock = MockNotificationService::new();
        mock.set_should_fail(true);

        let result = mock.send_email("a@test.com", "标题", "内容");
        assert!(result.is_err());

        let result = mock.send_sms("13800000000", "消息");
        assert!(result.is_err());

        // 失败时不记录调用
        assert_eq!(mock.email_call_count(), 0);
    }

    // ---- 测试 RegistrationService（使用 Fake + Mock）----

    #[test]
    fn test_registration_success() {
        let store = FakeUserStore::new();
        let notifier = MockNotificationService::new();
        let mut service = RegistrationService::new(store, notifier);

        let result = service.register(1, "Alice", "alice@test.com");
        assert!(result.is_ok());
        assert_eq!(service.user_count(), 1);
    }

    #[test]
    fn test_registration_sends_email() {
        let store = FakeUserStore::new();
        let notifier = MockNotificationService::new();
        let mut service = RegistrationService::new(store, notifier);

        service.register(1, "Alice", "alice@test.com").unwrap();

        // 验证邮件被发送了（通过访问内部 notifier）
        assert_eq!(service.notifier.email_call_count(), 1);
        let email = service.notifier.last_email().unwrap();
        assert_eq!(email.0, "alice@test.com");
        assert_eq!(email.1, "欢迎注册");
        assert!(email.2.contains("Alice"));
    }

    #[test]
    fn test_registration_empty_name() {
        let store = FakeUserStore::new();
        let notifier = MockNotificationService::new();
        let mut service = RegistrationService::new(store, notifier);

        let result = service.register(1, "", "test@test.com");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "用户名不能为空");
        assert_eq!(service.user_count(), 0);
    }

    #[test]
    fn test_registration_invalid_email() {
        let store = FakeUserStore::new();
        let notifier = MockNotificationService::new();
        let mut service = RegistrationService::new(store, notifier);

        let result = service.register(1, "Alice", "invalid-email");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "邮箱格式不正确");
    }

    #[test]
    fn test_registration_duplicate_email() {
        let store = FakeUserStore::new();
        let notifier = MockNotificationService::new();
        let mut service = RegistrationService::new(store, notifier);

        service.register(1, "Alice", "same@test.com").unwrap();
        let result = service.register(2, "Bob", "same@test.com");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("已被注册"));
    }

    #[test]
    fn test_registration_notification_failure() {
        let store = FakeUserStore::new();
        let mut notifier = MockNotificationService::new();
        notifier.set_should_fail(true); // 模拟通知失败
        let mut service = RegistrationService::new(store, notifier);

        let result = service.register(1, "Alice", "alice@test.com");
        // 通知失败应该导致注册失败
        assert!(result.is_err());
    }

    // ---- 测试 WeatherReport（使用 Stub）----

    #[test]
    fn test_weather_hot_and_humid() {
        let stub = StubWeatherService::new(35.0, 85.0);
        let report = WeatherReport::new(stub);
        let desc = report.describe("测试城市");
        assert!(desc.contains("炎热"));
        assert!(desc.contains("潮湿"));
        assert!(report.need_umbrella("测试城市"));
    }

    #[test]
    fn test_weather_cold_and_dry() {
        let stub = StubWeatherService::new(5.0, 30.0);
        let report = WeatherReport::new(stub);
        let desc = report.describe("测试城市");
        assert!(desc.contains("寒冷"));
        assert!(desc.contains("干燥"));
        assert!(!report.need_umbrella("测试城市"));
    }

    #[test]
    fn test_weather_warm_and_moderate() {
        let stub = StubWeatherService::new(25.0, 60.0);
        let report = WeatherReport::new(stub);
        let desc = report.describe("北京");
        assert!(desc.contains("温暖"));
        assert!(desc.contains("适中"));
        assert!(!report.need_umbrella("北京"));
    }

    #[test]
    fn test_weather_cool() {
        let stub = StubWeatherService::new(15.0, 50.0);
        let report = WeatherReport::new(stub);
        let desc = report.describe("城市");
        assert!(desc.contains("凉爽"));
    }
}
