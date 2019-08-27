use std::fs::File;
use std::io;

use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Subject {
    question: String,
    answer: String,
}

fn main() {
    let questions_file =
        File::open("src/question.json").expect("Failed to reading file: question.json.");
    let subjects: Vec<Subject> = serde_json::from_reader(questions_file).unwrap();

    let length = subjects.len();
    for (count, subject) in subjects.iter().enumerate() {
        loop {
            let question = format!("{}/{} 問題：{}", count + 1, length, subject.question);
            println!("{}", question.cyan().bold());

            let mut guess = String::new();
            println!("{}", "請輸入答案：".bright_black());

            io::stdin().read_line(&mut guess).expect("請輸入一些文字");

            if guess.trim().to_lowercase() == subject.answer {
                println!("{}", "答對了，你好棒 owo\n".yellow());
                break;
            } else {
                println!("{}", "叭叭，答錯了 OAO\n".bright_red());
            }
        }
    }

    println!(
        "{}",
        "你全部都答對了耶，歡迎拍下過關畫面，到南休息區「Code Dojo 技術傳道場」Mozilla 攤位領個小禮物 OwO".bright_yellow()
    );

    println!(
        "{}",
        "我們是 Rust Taiwan，歡迎加入我們，我們的群組：http://t.me/rust_tw"
            .bright_yellow()
    );
}
