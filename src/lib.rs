use colored::*;
use once_cell::sync::Lazy;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
pub struct Subject {
    question: String,
    answer: String,
}

include!(concat!(env!("OUT_DIR"), "/subjects.rs"));

#[derive(Serialize)]
pub struct AnswerResult {
    pub correct: bool,
    pub message: String,
}

#[wasm_bindgen]
pub struct Game {
    current: usize,
    length: usize,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(length: usize) -> Self {
        Game { length, current: 0 }
    }

    #[wasm_bindgen]
    pub fn next_question(&mut self) -> bool {
        self.current += 1;
        self.current < SUBJECTS.len()
    }

    #[wasm_bindgen]
    pub fn render(&self) -> String {
        let question = format!(
            "{}/{} 問題：{}\r",
            self.current + 1,
            self.length,
            SUBJECTS[self.current].question
        );

        format!(
            "{}\r\n{}",
            question.cyan().bold(),
            "請輸入答案：".bright_black()
        )
    }

    #[wasm_bindgen]
    pub fn input(&self, guess: String) -> JsValue {
        let res = if guess.trim().to_lowercase() == SUBJECTS[self.current].answer {
            AnswerResult {
                correct: true,
                message: format!("{}", "答對了，你好棒 owo\r\n\r".yellow()),
            }
        } else {
            AnswerResult {
                correct: false,
                message: format!("{}", "叭叭，答錯了 OAO\r\n\r".bright_red()),
            }
        };
        JsValue::from_serde(&res).unwrap()
    }

    #[wasm_bindgen]
    pub fn end(&self) -> String {
        format!(
            "{}\r\n{}\r",
            "你全部都答對了耶，歡迎拍下過關畫面，到南休息區「Code Dojo 技術傳道場」Mozilla 攤位領個小禮物 OwO".bright_yellow(),
            "我們是 Rust Taiwan，歡迎加入我們，我們的群組：http://t.me/rust_tw".bright_yellow(),
        )
    }
}
