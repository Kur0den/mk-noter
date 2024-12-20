use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};

// pub fn line(line_content = "-": &str) {
//     println!("{}", line_content);
// }

pub fn hint(content: &str) {
    println!("{} {}", "[Hint]".magenta(), content);
}

pub fn info(content: &str) {
    println!("{} {}", "[Info]".cyan(), content);
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

pub fn list(list_content: HashMap<String, String>) {
    // Keyの中から最大の文字数を取得
    let max_key_length = list_content.keys().map(|key| key.len()).max().unwrap();

    // Keyの最大文字数に合わせて左寄せで空白で埋める
    for (key, value) in list_content {
        println!("{:<width$} | {}", key, value, width = max_key_length);
    }
}

pub fn input_prompt(prompt: &str) {
    print!("{}{} ", prompt, ">>".blink().yellow());
    io::stdout().flush().unwrap();
}

pub fn overwrite_prompt(prompt: &str, input: &str) {
    const ANSI_CLEAR: &str = "\x1B[A\r\x1B[2K";
    println!("{}{}{} {}", ANSI_CLEAR, prompt, ">>".dimmed(), input.trim());
}
