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
            question: String::from("Rust 1.0 在哪一年釋出？"),
            answer: String::from("2015年")
        },
        Subject {
            question: String::from("Rust.tw 的聚會時間在什麼時候？"),
            answer: String::from("每個月的最後一個星期六晚上七點半")
        },
        Subject {
            question: String::from("以 Rust 開發的瀏覽器引擎叫什麼名字？"),
            answer: String::from("Servo")
        },
        Subject {
            question: String::from("2016年stackoverflow most loved language 是？"),
            answer: String::from("Rust")
        },
        Subject {
            question: String::from("2017年stackoverflow most loved language 是？"),
            answer: String::from("Rust")
        },
        Subject {
            question: String::from("2018年stackoverflow most loved language 是？"),
            answer: String::from("Rust")
        },
        Subject {
            question: String::from("2019年stackoverflow most loved language 是？"),
            answer: String::from("Rust")
        },
        Subject {
            question: String::from("為了安全性，Rust不支援哪種OOP語法？"),
            answer: String::from("inheritance")
        },
    ];
    let mut count = 0;
    let length = subjects.len();
    for subject in &subjects {
        count += 1;
        loop {
            println!("{}{}{}/{} 問題：{}{}", color::Fg(color::Cyan), style::Bold, count, length, subject.question, style::Reset);
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
    println!("{}你全部都答對了耶，可以領個小禮物 OwO{}", color::Fg(color::LightYellow), style::Reset);
}
