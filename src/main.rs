use std::io;

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
            println!("問題：{}", subject.question);
            let mut guess = String::new();
            println!("請輸入答案：");
            io::stdin().read_line(&mut guess)
                .expect("請輸入一些文字");
            if guess.trim() == subject.answer {
                println!("答對了，你好棒 owo");
                break;
            } else {
                println!("叭叭，答錯了 OAO");
            }
        }
    }
}
