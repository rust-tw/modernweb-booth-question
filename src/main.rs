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
        println!("{}", subject.question);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("請輸入一些文字");
        if guess.trim() == subject.answer {
            println!("答對了，你好棒 owo");
        } else {
            println!("叭叭，答錯了 OAO");
        }
    }
}
