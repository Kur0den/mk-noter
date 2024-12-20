// mod print;

use std::io;

use colored::*;

use crate::print;




pub fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    // inputの前にプロンプトを表示
    print::input_prompt(prompt);
    io::stdin()  // 標準入力を取得
        .read_line(&mut input)
        .expect("Failed to read line");
    // カーソルの点滅を削除
    print::overwrite_prompt(prompt, &input);
    input.trim().to_string()
}

pub fn confirm(prompt_content: &str) -> bool {
    loop {
        let mut input = String::new();
        let prompt = format!("{} ({}/{})", prompt_content, "y".green(), "N".red());
        print::input_prompt(&prompt);
        io::stdin()  // 標準入力を取得
            .read_line(&mut input)
            .expect("Failed to read line");
        // カーソルの点滅の削除と色付きの文字のグレーアウト化
        let prompt = format!("{} ({}/{})", prompt_content, "y".dimmed(), "N".dimmed());
        print::overwrite_prompt(&prompt, &input);
        input = input.trim().to_string();
        if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
            return true;
        } else if input.to_lowercase() == "n" || input.to_lowercase() == "no" {
            return false;
        };
    };
}
