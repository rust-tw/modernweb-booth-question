use std::io;

struct Subject {
    question: String,
    answer: String,
}

fn main() {
    let subjects: Vec<Subject> = vec![
        Subject {
            question: String::from("Rust 1.0 在哪一年釋出？\n1. 2012 年 2. 2013 年 3. 2014 年 4. 2015 年"),
            answer: String::from("4")
        },
        Subject {
            question: String::from("Rust.tw 的聚會時間在什麼時候？\n1. 每個月的最後一個星期六晚上七點半 2. 每個月的最後一個星期六晚上八點半 3. 每個月的最後一個星期六晚上九點半 4. 每個月的最後一個星期六晚上十點半"),
            answer: String::from("1")
        },
        Subject {
            question: String::from("以 Rust 開發的瀏覽器引擎叫什麼名字？\n1. Doge 2. Servo 3. Gecko 4. Wow"),
            answer: String::from("2")
        },
        Subject {
            question: String::from("2016年stackoverflow most loved language 是？\n1. Rust 2. Python 3. JavaScript 4. Kotlin 5. php"),
            answer: String::from("1")
        },
        Subject {
            question: String::from("2017年stackoverflow most loved language 是？\n1. Python 2. Rust 3. JavaScript 4. Kotlin 5. php"),
            answer: String::from("2")
        },
        Subject {
            question: String::from("2018年stackoverflow most loved language 是？\n1. JavaScript 2. Python 3. Rust 4. Kotlin 5. php"),
            answer: String::from("3")
        },
        Subject {
            question: String::from("2019年stackoverflow most loved language 是？\n1. Kotlin 2. Python 3. JavaScript 4. Rust 5. php"),
            answer: String::from("4")
        },
        Subject {
            question: String::from("Facebook 用 Rust 開發的區塊鏈數位貨幣叫做？\n1. LibreOffice 2. Doge 3. Libra 4. Facebook Coin"),
            answer: String::from("3")
        }
    ];

    let length = subjects.len();
    for (count, subject) in subjects.iter().enumerate() {
        loop {
            println!("{}/{} 問題：{}", count + 1, length, subject.question);

            let mut guess = String::new();
            println!("請輸入答案：");

            io::stdin().read_line(&mut guess).expect("請輸入一些文字");

            if guess.trim().to_lowercase() == subject.answer {
                println!("答對了，你好棒 owo\n");
                break;
            } else {
                println!("叭叭，答錯了 OAO\n");
            }
        }
    }

    println!("你全部都答對了耶，歡迎拍下過關畫面，到南休息區「Code Dojo 技術傳道場」Mozilla 攤位領個小禮物 OwO");

    println!("我們是 Rust Taiwan，歡迎加入我們，我們的群組：http://t.me/rust_tw");
}
