//! 第 1 章：基础语法。
//!
//! 学习目标：理解变量与遮蔽、常见类型、函数，以及条件和循环。
//! 运行方式：cargo run --example ch01_basics
//!
//! 扩展练习：
//! 1. 修改学习时长数组，观察完成比例和分支输出的变化。
//! 2. 为 match 增加一个“超额完成”的分支。
//! 3. 编写函数，把分钟数转换为小时数（返回 f64）。

// 参数和返回值需要声明类型；末尾不加分号的表达式就是返回值。
fn weekly_target(minutes_per_day: u32, days: u32) -> u32 {
    minutes_per_day * days
}

fn main() {
    println!("=== 第 1 章：基础语法 ===");

    let course: &str = "Rust";
    let target_minutes: u32 = 20;
    // 再次使用 let 会遮蔽旧变量；这里创建了一个新的不可变变量。
    let target_minutes = target_minutes + 10;
    // mut 允许后续给同一个变量赋值。
    let mut studied_minutes: u32 = 0;

    // 元组可以组合不同类型，数组的元素类型相同且长度固定。
    let plan: (&str, u32) = (course, target_minutes);
    let (course_name, daily_target) = plan;
    let sessions: [u32; 3] = [10, 15, 5];
    println!("学习计划：{course_name}，每天 {daily_target} 分钟");

    for minutes in sessions {
        studied_minutes += minutes;
        println!("本次学习 {minutes} 分钟，累计 {studied_minutes} 分钟");
    }

    // as 在这里把整数转换为浮点数，使除法保留小数部分。
    let progress: f64 = studied_minutes as f64 / daily_target as f64;
    let finished: bool = studied_minutes >= daily_target;
    let marker: char = '学';
    // if 也是表达式，两条分支需要产生相同类型的值。
    let feedback = if finished {
        "目标已完成"
    } else {
        "继续加油"
    };
    println!("{marker}：完成比例 {:.0}%，{feedback}", progress * 100.0);

    // match 会检查所有可能的情况；_ 匹配前面未覆盖的值。
    let stage = match studied_minutes {
        0 => "尚未开始",
        1..=29 => "正在积累",
        _ => "已经达到本例的 30 分钟目标",
    };
    println!("学习阶段：{stage}");
    println!(
        "每周学习 5 天的目标：{} 分钟",
        weekly_target(daily_target, 5)
    );

    let mut countdown = 3;
    while countdown > 0 {
        println!("休息倒计时：{countdown}");
        countdown -= 1;
    }
}
