// mod print;

use std::io;
use colored::*;
use crate::print;


pub fn read_line(prompt: &str, allow_empty: bool) -> String {
    let mut input = String::new();
    loop {
        // inputの前にプロンプトを表示
        print::input_prompt(prompt, None);
        io::stdin()  // 標準入力を取得
        .read_line(&mut input)
        .expect("Failed to read line");
        // カーソルの点滅を削除
        print::overwrite_prompt(prompt, None, &input);
        if !allow_empty && input.trim().is_empty() {
            print::warning("空入力は許容されていません");
            continue;
        } else {
            break;
        }
    }
    input.trim().to_string()
}

pub fn confirm(prompt_content: &str) -> bool {
    loop {
        let mut input = String::new();
        let example_prompt = format!("({}/{})", "y".green(), "N".red());
        print::input_prompt(prompt_content, Some(&example_prompt));
        io::stdin()  // 標準入力を取得
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();
        if input.to_lowercase() == "y" || input.to_lowercase() == "yes" {
            // カーソルの点滅の削除と色付きの文字のグレーアウト化
            let example_prompt = format!("({}/{})", "y".green(), "N".dimmed());
            print::overwrite_prompt(prompt_content, Some(&example_prompt),&input);
            return true;
        } else if input.to_lowercase() == "n" || input.to_lowercase() == "no" {
            // カーソルの点滅の削除と色付きの文字のグレーアウト化
            let example_prompt = format!("({}/{})", "y".dimmed(), "N".red());
            print::overwrite_prompt(prompt_content, Some(&example_prompt),&input);
            return false;
        };
        // カーソルの点滅の削除と色付きの文字のグレーアウト化
        let example_prompt = format!("({}/{})", "y".dimmed(), "N".dimmed());
        print::overwrite_prompt(prompt_content, Some(&example_prompt),&input);
    };
}
