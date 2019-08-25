extern crate termion;

use std::io;
use termion::{color, style};

struct Subject {
    question: String,
    answer: String
}

fn main() {
    let subjects = vec![
        Subject {
            question: String::from("測試"),
            answer: String::from("襪嗚")
        }
    ];
    for subject in &subjects {
        loop {
            println!("{}{}問題：{}{}", color::Fg(color::Cyan), style::Bold, subject.question, style::Reset);
            let mut guess = String::new();
            println!("{}請輸入答案：{}", color::Fg(color::LightBlack), style::Reset);
            io::stdin().read_line(&mut guess)
                .expect("請輸入一些文字");
            if guess.trim() == subject.answer {
                println!("{}答對了，你好棒 owo{}", color::Fg(color::Yellow), style::Reset);
                break;
            } else {
                println!("{}叭叭，答錯了 OAO{}", color::Fg(color::LightRed), style::Reset);
            }
        }
    }
}
