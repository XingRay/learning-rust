// ============================================================
// Lesson 101: 命令模式 (Command Pattern)
// ============================================================
//
// 命令模式将请求封装为对象，使你可以：
//   - 将操作参数化
//   - 将操作放入队列
//   - 支持撤销（undo）和重做（redo）
//
// 本课以一个文本编辑器为例，演示完整的命令模式实现。

use std::fmt;

// ============================================================
// 1. Command Trait 定义
// ============================================================

/// 命令 trait：所有命令都必须实现 execute 和 undo
trait Command: fmt::Display {
    /// 执行命令，返回是否成功
    fn execute(&mut self, doc: &mut TextDocument) -> bool;
    /// 撤销命令
    fn undo(&mut self, doc: &mut TextDocument);
}

// ============================================================
// 2. 文本文档（命令的接收者 Receiver）
// ============================================================

/// 文本文档 —— 命令操作的目标
struct TextDocument {
    content: String,
    /// 光标位置（字节偏移）
    cursor: usize,
}

impl TextDocument {
    fn new() -> Self {
        TextDocument {
            content: String::new(),
            cursor: 0,
        }
    }

    /// 在光标位置插入文本
    fn insert_at(&mut self, pos: usize, text: &str) {
        let pos = pos.min(self.content.len());
        self.content.insert_str(pos, text);
        self.cursor = pos + text.len();
    }

    /// 删除指定范围的文本，返回被删除的内容
    fn delete_range(&mut self, start: usize, end: usize) -> String {
        let start = start.min(self.content.len());
        let end = end.min(self.content.len());
        if start >= end {
            return String::new();
        }
        let deleted: String = self.content[start..end].to_string();
        self.content.replace_range(start..end, "");
        self.cursor = start;
        deleted
    }

    /// 替换指定范围的文本，返回被替换的内容
    fn replace_range(&mut self, start: usize, end: usize, new_text: &str) -> String {
        let start = start.min(self.content.len());
        let end = end.min(self.content.len());
        let old_text: String = self.content[start..end].to_string();
        self.content.replace_range(start..end, new_text);
        self.cursor = start + new_text.len();
        old_text
    }

    /// 获取文档内容
    fn text(&self) -> &str {
        &self.content
    }

    /// 获取文档长度
    fn len(&self) -> usize {
        self.content.len()
    }
}

impl fmt::Display for TextDocument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\" (长度={}, 光标={})", self.content, self.content.len(), self.cursor)
    }
}

// ============================================================
// 3. 具体命令实现
// ============================================================

/// 插入文本命令
struct InsertCommand {
    position: usize,
    text: String,
}

impl InsertCommand {
    fn new(position: usize, text: &str) -> Self {
        InsertCommand {
            position,
            text: text.to_string(),
        }
    }
}

impl fmt::Display for InsertCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Insert({}, \"{}\")", self.position, self.text)
    }
}

impl Command for InsertCommand {
    fn execute(&mut self, doc: &mut TextDocument) -> bool {
        doc.insert_at(self.position, &self.text);
        true
    }

    fn undo(&mut self, doc: &mut TextDocument) {
        // 撤销插入 = 删除刚插入的文本
        doc.delete_range(self.position, self.position + self.text.len());
    }
}

/// 删除文本命令
struct DeleteCommand {
    start: usize,
    end: usize,
    /// 保存被删除的文本，用于撤销
    deleted_text: String,
}

impl DeleteCommand {
    fn new(start: usize, end: usize) -> Self {
        DeleteCommand {
            start,
            end,
            deleted_text: String::new(),
        }
    }
}

impl fmt::Display for DeleteCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Delete({}, {})", self.start, self.end)
    }
}

impl Command for DeleteCommand {
    fn execute(&mut self, doc: &mut TextDocument) -> bool {
        self.deleted_text = doc.delete_range(self.start, self.end);
        !self.deleted_text.is_empty()
    }

    fn undo(&mut self, doc: &mut TextDocument) {
        // 撤销删除 = 在原位置重新插入
        doc.insert_at(self.start, &self.deleted_text);
    }
}

/// 替换文本命令
struct ReplaceCommand {
    start: usize,
    end: usize,
    new_text: String,
    /// 保存被替换的文本，用于撤销
    old_text: String,
}

impl ReplaceCommand {
    fn new(start: usize, end: usize, new_text: &str) -> Self {
        ReplaceCommand {
            start,
            end,
            new_text: new_text.to_string(),
            old_text: String::new(),
        }
    }
}

impl fmt::Display for ReplaceCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Replace({}, {}, \"{}\")", self.start, self.end, self.new_text)
    }
}

impl Command for ReplaceCommand {
    fn execute(&mut self, doc: &mut TextDocument) -> bool {
        self.old_text = doc.replace_range(self.start, self.end, &self.new_text);
        true
    }

    fn undo(&mut self, doc: &mut TextDocument) {
        // 撤销替换 = 把新文本替换回旧文本
        doc.replace_range(self.start, self.start + self.new_text.len(), &self.old_text);
    }
}

/// 追加文本命令（在文档末尾追加）
struct AppendCommand {
    text: String,
    /// 追加前文档的长度，用于撤销
    prev_len: usize,
}

impl AppendCommand {
    fn new(text: &str) -> Self {
        AppendCommand {
            text: text.to_string(),
            prev_len: 0,
        }
    }
}

impl fmt::Display for AppendCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Append(\"{}\")", self.text)
    }
}

impl Command for AppendCommand {
    fn execute(&mut self, doc: &mut TextDocument) -> bool {
        self.prev_len = doc.len();
        doc.insert_at(doc.len(), &self.text);
        true
    }

    fn undo(&mut self, doc: &mut TextDocument) {
        doc.delete_range(self.prev_len, self.prev_len + self.text.len());
    }
}

// ============================================================
// 4. 命令管理器（支持撤销/重做）
// ============================================================

/// 编辑器 —— 管理命令队列和撤销/重做栈
struct Editor {
    document: TextDocument,
    /// 已执行的命令栈（用于撤销）
    undo_stack: Vec<Box<dyn Command>>,
    /// 已撤销的命令栈（用于重做）
    redo_stack: Vec<Box<dyn Command>>,
}

impl Editor {
    fn new() -> Self {
        Editor {
            document: TextDocument::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    /// 执行一个命令
    fn execute(&mut self, mut cmd: Box<dyn Command>) {
        let cmd_name = format!("{}", cmd);
        if cmd.execute(&mut self.document) {
            println!("  ✅ 执行: {} -> 文档: {}", cmd_name, self.document);
            self.undo_stack.push(cmd);
            // 执行新命令后，清空重做栈
            self.redo_stack.clear();
        } else {
            println!("  ❌ 执行失败: {}", cmd_name);
        }
    }

    /// 撤销最后一个命令
    fn undo(&mut self) {
        if let Some(mut cmd) = self.undo_stack.pop() {
            let cmd_name = format!("{}", cmd);
            cmd.undo(&mut self.document);
            println!("  ↩️  撤销: {} -> 文档: {}", cmd_name, self.document);
            self.redo_stack.push(cmd);
        } else {
            println!("  ⚠️  没有可撤销的操作");
        }
    }

    /// 重做最后一个撤销的命令
    fn redo(&mut self) {
        if let Some(mut cmd) = self.redo_stack.pop() {
            let cmd_name = format!("{}", cmd);
            cmd.execute(&mut self.document);
            println!("  ↪️  重做: {} -> 文档: {}", cmd_name, self.document);
            self.undo_stack.push(cmd);
        } else {
            println!("  ⚠️  没有可重做的操作");
        }
    }

    /// 获取文档内容
    fn text(&self) -> &str {
        self.document.text()
    }

    /// 显示状态
    fn status(&self) {
        println!(
            "  📄 文档: {} | 撤销栈: {} | 重做栈: {}",
            self.document,
            self.undo_stack.len(),
            self.redo_stack.len()
        );
    }
}

// ============================================================
// 5. 命令队列（批量执行）
// ============================================================

/// 宏命令 —— 将多个命令组合为一个
struct MacroCommand {
    name: String,
    commands: Vec<Box<dyn Command>>,
}

impl MacroCommand {
    fn new(name: &str) -> Self {
        MacroCommand {
            name: name.to_string(),
            commands: Vec::new(),
        }
    }

    fn add(&mut self, cmd: Box<dyn Command>) {
        self.commands.push(cmd);
    }
}

impl fmt::Display for MacroCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Macro(\"{}\" x{} cmds)", self.name, self.commands.len())
    }
}

impl Command for MacroCommand {
    fn execute(&mut self, doc: &mut TextDocument) -> bool {
        let mut all_ok = true;
        for cmd in &mut self.commands {
            if !cmd.execute(doc) {
                all_ok = false;
            }
        }
        all_ok
    }

    fn undo(&mut self, doc: &mut TextDocument) {
        // 逆序撤销所有子命令
        for cmd in self.commands.iter_mut().rev() {
            cmd.undo(doc);
        }
    }
}

// ============================================================
// main 函数
// ============================================================

fn main() {
    println!("=== Lesson 101: 命令模式 (Command Pattern) ===\n");

    // ---------------------------------------------------------
    // 1. 基本的命令执行与撤销
    // ---------------------------------------------------------
    println!("--- 1. 基本命令执行 ---");

    let mut editor = Editor::new();
    editor.status();

    // 追加文本
    editor.execute(Box::new(AppendCommand::new("Hello")));
    editor.execute(Box::new(AppendCommand::new(", World")));
    editor.execute(Box::new(AppendCommand::new("!")));

    println!("\n当前文本: \"{}\"", editor.text());

    println!();

    // ---------------------------------------------------------
    // 2. 撤销与重做
    // ---------------------------------------------------------
    println!("--- 2. 撤销与重做 ---");

    // 撤销最后一步
    editor.undo(); // 撤销 "!"
    println!("撤销后: \"{}\"", editor.text());

    editor.undo(); // 撤销 ", World"
    println!("再撤销: \"{}\"", editor.text());

    // 重做
    editor.redo(); // 重做 ", World"
    println!("重做后: \"{}\"", editor.text());

    editor.redo(); // 重做 "!"
    println!("再重做: \"{}\"", editor.text());

    // 尝试多次重做（没有更多可重做的）
    editor.redo();

    editor.status();

    println!();

    // ---------------------------------------------------------
    // 3. 插入、删除、替换命令
    // ---------------------------------------------------------
    println!("--- 3. 插入、删除、替换 ---");

    let mut editor2 = Editor::new();

    // 构建文本 "The quick brown fox"
    editor2.execute(Box::new(AppendCommand::new("The quick brown fox")));

    // 在位置 10 插入 "very "
    editor2.execute(Box::new(InsertCommand::new(10, "very ")));
    // 现在: "The quick very brown fox"

    // 删除 "very "（位置 10-15）
    editor2.execute(Box::new(DeleteCommand::new(10, 15)));
    // 现在: "The quick brown fox"

    // 替换 "brown" 为 "red"
    editor2.execute(Box::new(ReplaceCommand::new(10, 15, "red")));
    // 现在: "The quick red fox"

    println!("\n当前: \"{}\"", editor2.text());

    // 逐步撤销所有操作
    println!("\n逐步撤销:");
    editor2.undo(); // 撤销 Replace -> "The quick brown fox"
    editor2.undo(); // 撤销 Delete -> "The quick very brown fox"
    editor2.undo(); // 撤销 Insert -> "The quick brown fox"
    editor2.undo(); // 撤销 Append -> ""

    println!("全部撤销后: \"{}\"", editor2.text());

    // 全部重做
    println!("\n全部重做:");
    editor2.redo();
    editor2.redo();
    editor2.redo();
    editor2.redo();
    println!("全部重做后: \"{}\"", editor2.text());

    println!();

    // ---------------------------------------------------------
    // 4. 宏命令（批量操作）
    // ---------------------------------------------------------
    println!("--- 4. 宏命令（批量操作）---");

    let mut editor3 = Editor::new();

    // 创建一个宏命令：一次性构建完整文本
    let mut macro_cmd = MacroCommand::new("创建文档");
    macro_cmd.add(Box::new(AppendCommand::new("# 标题\n")));
    macro_cmd.add(Box::new(AppendCommand::new("\n")));
    macro_cmd.add(Box::new(AppendCommand::new("正文第一段。\n")));
    macro_cmd.add(Box::new(AppendCommand::new("正文第二段。\n")));

    editor3.execute(Box::new(macro_cmd));
    println!("\n宏命令执行后:");
    println!("---");
    println!("{}", editor3.text());
    println!("---");

    // 一次撤销整个宏命令
    editor3.undo();
    println!("宏命令撤销后: \"{}\"", editor3.text());

    // 重做宏命令
    editor3.redo();
    println!("宏命令重做后:");
    println!("{}", editor3.text());

    println!();

    // ---------------------------------------------------------
    // 5. 新命令会清空重做栈
    // ---------------------------------------------------------
    println!("--- 5. 新命令清空重做栈 ---");

    let mut editor4 = Editor::new();
    editor4.execute(Box::new(AppendCommand::new("AAA")));
    editor4.execute(Box::new(AppendCommand::new("BBB")));
    editor4.execute(Box::new(AppendCommand::new("CCC")));
    editor4.status();

    // 撤销两步
    editor4.undo(); // 撤销 CCC
    editor4.undo(); // 撤销 BBB
    editor4.status();

    // 执行新命令 —— 重做栈会被清空
    editor4.execute(Box::new(AppendCommand::new("XXX")));
    editor4.status();

    // 此时无法再重做 BBB 和 CCC
    editor4.redo(); // 没有可重做的

    println!("\n最终文本: \"{}\"", editor4.text());

    println!();

    // ---------------------------------------------------------
    // 6. 总结
    // ---------------------------------------------------------
    println!("--- 6. 命令模式总结 ---");
    println!("┌──────────────────┬─────────────────────────────────────┐");
    println!("│ 组件             │ 说明                                │");
    println!("├──────────────────┼─────────────────────────────────────┤");
    println!("│ Command trait    │ 定义 execute() 和 undo() 接口       │");
    println!("│ 具体命令         │ Insert/Delete/Replace/Append        │");
    println!("│ Receiver         │ TextDocument（被操作的对象）         │");
    println!("│ Invoker          │ Editor（管理命令栈）                 │");
    println!("│ MacroCommand     │ 组合多个命令为一个                   │");
    println!("│ Undo/Redo 栈     │ 两个栈实现撤销和重做                │");
    println!("└──────────────────┴─────────────────────────────────────┘");

    println!("\n=== 命令模式学习完成！===");
}
