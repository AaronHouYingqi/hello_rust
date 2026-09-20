//! 学习目标：理解所有权转移、共享借用、可变借用和字符串切片。
//! 运行：cargo run --example ch02_ownership
//! 练习：实现返回第二个单词切片的函数，并思考没有第二个单词时如何返回结果。

fn main() {
    println!("第 02 章：所有权与借用");

    let original = String::from("Rust language");
    let owned = original;
    // String 的赋值转移了所有权；此后不能再使用 original。
    println!("所有权转移后：{owned}");

    // &String 可以自动转换为 &str。共享借用允许读取数据，无需复制字符串。
    let word = first_word(&owned);
    println!("共享借用得到首词：{word}；原字符串仍可读取：{owned}");

    let mut greeting = String::from("你好");
    {
        // 可变借用期间不能同时使用指向同一数据的其他引用。
        let borrowed = &mut greeting;
        borrowed.push_str("，Rust！");
    }
    println!("可变借用修改后：{greeting}");

    // str 使用 UTF-8；不要随意用字节下标切割中文，避免落在字符中间。
    let sentence = String::from("  你好，Rust 世界！  ");
    let word = first_word(&sentence);
    println!("字符串切片：{word}");
}

/// 返回第一个由空白分隔的单词，忽略开头的空白；没有单词时返回空切片。
///
/// 返回类型 &str 借用输入中的文本，无需分配新的 String。
/// split_whitespace 支持 Unicode 空白，并保持 UTF-8 字符边界完整。
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::first_word;

    #[test]
    fn empty_or_whitespace_input_has_no_word() {
        for text in ["", " ", "\t\n\r", "\u{3000}"] {
            assert_eq!(first_word(text), "");
        }
    }

    #[test]
    fn ignores_leading_and_trailing_whitespace() {
        assert_eq!(first_word("  Rust  "), "Rust");
        assert_eq!(first_word("\tRust\n"), "Rust");
    }

    #[test]
    fn returns_only_the_first_word() {
        assert_eq!(first_word("Rust"), "Rust");
        assert_eq!(first_word("learn Rust together"), "learn");
        assert_eq!(first_word("learn\tRust\ntogether"), "learn");
    }

    #[test]
    fn handles_chinese_text_and_unicode_whitespace() {
        let sentence = String::from("  你好，Rust 世界！  ");
        assert_eq!(first_word(&sentence), "你好，Rust");
        assert_eq!(first_word("\u{3000}学习\u{3000}Rust"), "学习");
    }
}
