use colored::*;
use unicode_width::UnicodeWidthChar;
use std::io::{self, Write};
use indexmap::IndexMap;

// pub fn line(line_content = "-": &str) {x
//     println!("{}", line_content);
// }

pub fn hint(content: &str) {
    println!("{} {}", "[Hint]".magenta(), content);
}

pub fn info(content: &str) {
    println!("{} {}", "[Info]".cyan(), content);
}

pub fn success(content: &str) {
    println!("{} {}", "[Success]".green(), content);
}

pub fn warning(content: &str) {
    println!("{} {}", "[Warning]".yellow(), content);
}

pub fn title(content: &str) {
    let content = format!("[{}]", content);
    println!();
    println!("{}", "=".repeat(30));
    println!("{}", content.cyan());
    println!("{}", "=".repeat(30));
    println!();
}

pub fn list(list_content: IndexMap<String, String>) {
    // Keyの中から最大の文字数を取得
    // let max_key_length = list_content.keys().map(|key| key.len()).max().unwrap();
    let mut max_key_length = 0;
    let mut key_length_vec: Vec<usize> = Vec::new();
    // Keyをひとつづつ取り出して、最大文字数を取得
    for key in list_content.keys(){
        let mut count = 0;
        for char in key.chars() {
            match char.width() {
                Some(width) => count += width,
                None => count += 1,
            }
        }
        key_length_vec.push(count);
        if count > max_key_length {
            max_key_length = count;
        }
    }

    // 空白行
    println!();
    // valueの最大文字数を取得
    let max_value_length = list_content.values().map(|value| value.len()).max().unwrap();
    // max_key_length + 3(スペースと縦棒) + valueの最大数 + 2(余裕を持たせるため)の横線を表示
    println!("{}", "=".repeat(max_key_length + 3 + max_value_length + 2));
    // Keyの最大文字数に合わせて左寄せで空白で埋める
    for (key, value) in list_content {
        let space_width = max_key_length - key_length_vec.remove(0);
        println!("{}{space} | {}", key, value, space = " ".repeat(space_width));
    }
    // max_key_length + 3(スペースと縦棒) + valueの最大数 + 2(余裕を持たせるため)の横線を表示
    println!("{}", "=".repeat(max_key_length + 3 + max_value_length + 2));
    println!();
}

pub fn input_prompt(prompt: &str, without_underline_prompt: Option<&str>) {
    let without_underline_prompt = if let Some(prompt) = without_underline_prompt {
        format!("{} ", prompt)
    } else {
        "".to_string()
    };
    print!("{} {}{} ", prompt.underline(), without_underline_prompt, ">>".blink().yellow());
    io::stdout().flush().unwrap();
}

pub fn overwrite_prompt(prompt: &str, without_underline_prompt: Option<&str>, input: &str) {
    let without_underline_prompt = if let Some(prompt) = without_underline_prompt {
        format!("{} ", prompt)
    } else {
        "".to_string()
    };
    const ANSI_CLEAR: &str = "\x1B[A\r\x1B[2K";
    println!("{}{} {}{} {}", ANSI_CLEAR, prompt.underline(), without_underline_prompt, ">>".dimmed(), input.trim());
}
