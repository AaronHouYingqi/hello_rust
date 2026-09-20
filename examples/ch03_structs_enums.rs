//! 第 3 章：结构体与枚举。
//!
//! 学习目标：用 struct 保存相关数据，用 impl 定义方法，
//! 用 enum 和 match 表达状态，并用 Option 表达可能不存在的备注。
//! 运行方式：cargo run --example ch03_structs_enums
//!
//! 扩展练习：
//! 1. 为任务添加难度字段，并在输出中展示。
//! 2. 增加携带暂停原因的 Paused(String) 状态，补全 match。
//! 3. 添加清除备注的方法，将备注恢复为 None。

enum TaskStatus {
    Todo,
    // 不同枚举变体可以携带不同的数据，这里记录已学习的分钟数。
    InProgress(u32),
    Done,
}

struct LearningTask {
    title: String,
    status: TaskStatus,
    note: Option<String>,
}

impl LearningTask {
    // 没有 self 参数的关联函数通过 LearningTask::new 调用。
    fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
            status: TaskStatus::Todo,
            note: None,
        }
    }

    // &mut self 允许方法修改当前实例。
    fn start(&mut self, minutes: u32) {
        self.status = TaskStatus::InProgress(minutes);
    }

    fn complete(&mut self, note: &str) {
        self.status = TaskStatus::Done;
        self.note = Some(note.to_owned());
    }

    // &self 只借用实例，因此展示后仍能继续使用这个任务。
    fn show(&self) {
        println!("任务：{}", self.title);
        match &self.status {
            TaskStatus::Todo => println!("状态：尚未开始"),
            TaskStatus::InProgress(minutes) => println!("状态：学习中，已投入 {minutes} 分钟"),
            TaskStatus::Done => println!("状态：已完成"),
        }

        // Option 要求显式处理“有值”和“没有值”两种情况。
        match &self.note {
            Some(note) => println!("备注：{note}"),
            None => println!("备注：暂无"),
        }
        println!();
    }
}

fn main() {
    println!("=== 第 3 章：结构体与枚举 ===");

    let mut task = LearningTask::new("掌握结构体与枚举");
    task.show();

    task.start(20);
    task.show();

    task.complete("已能用 match 处理每一种任务状态");
    task.show();
}
