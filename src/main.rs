use std::{env, io};
use std::{thread, time};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use serde::{Deserialize, Serialize};
use termion::{color, style};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug, Serialize, Deserialize)]
struct Subject {
    question: String,
    answers: Vec<Answer>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Answer {
    text: String,
    right: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Addition {
    mark: Option<String>,
    items: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Modernweb {
    addition: Option<Addition>,
    subjects: Vec<Subject>,
}

impl Addition {
    pub fn item_text(&self, ix: usize) -> String {
        if let Some(items) = &self.items {
            return items.iter().enumerate()
                .find(|(i, _)| &ix == i)
                .map_or((ix + 1).to_string(), |(_, t)| t.to_string());
        }
        (ix + 1).to_string()
    }

    pub fn mark(&self) -> String {
        self.mark.clone().map_or(".".to_string(), |v| v)
    }
}

impl Modernweb {
    pub fn addition(&self) -> &Option<Addition> {
        &self.addition
    }
}

impl Subject {
    pub fn right_answers(&self) -> Vec<usize> {
        self.answers.iter().enumerate()
            .filter(|(_, item)| item.right.is_some() && item.right == Some(true))
            .map(|(ix, _)| ix)
            .collect()
    }

    pub fn multi_answer(&self) -> bool {
        self.right_answers().len() > 1
    }
}

macro_rules! rprintln {
    () => (print!("\r\n"));
    ($($arg:tt)*) => ({
        print!($($arg)*);
        rprintln!();
    })
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let toml_file = args.get(1).map_or("src/question.toml".to_string(), |v| v.to_string());

    let question_text = std::fs::read_to_string(Path::new(&toml_file[..])).expect("Failed to reading file: question.toml");
    let modernweb: Modernweb = toml::from_str(&question_text[..]).expect("question.toml format fail");
    let subjects = &modernweb.subjects;
    let length = subjects.len();

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();

    for (count, subject) in subjects.iter().enumerate() {
        let answer_len = subject.answers.len();
        let mut pass = None;
        let mut current = None;
        let mut selected: HashSet<usize> = HashSet::new();
        loop {
            draw_question(&modernweb, subject, count, length, &current, &selected, pass);

            for c in io::stdin().keys() {
                write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
                match c.unwrap() {
                    Key::Char(' ') => {
                        if let Some(ix) = current {
                            let multi_answer = subject.multi_answer();
                            if multi_answer {
                                if selected.contains(&ix) {
                                    selected.remove(&ix);
                                } else {
                                    selected.insert(ix);
                                }
                            } else {
                                selected.clear();
                                selected.insert(ix);
                            }
                            if !multi_answer {
                                pass = Some(check_pass(&subject.right_answers(), &selected));
                            }
                            draw_question(&modernweb, subject, count, length, &current, &selected, pass);
                            if let Some(true) = pass { break; }
                        }
                    }
                    Key::Char('\n') => {
                        if selected.is_empty() {
                            pass = Some(false);
                            draw_question(&modernweb, subject, count, length, &current, &selected, pass);
                            continue;
                        }

                        pass = Some(check_pass(&subject.right_answers(), &selected));
                        draw_question(&modernweb, subject, count, length, &current, &selected, pass);
                        if let Some(true) = pass { break; }
                    }
                    Key::Ctrl('c') => return,
                    Key::Up => {
                        pass = None;
                        current = Some(cloc_position(&current, answer_len, 1));
                        draw_question(&modernweb, subject, count, length, &current, &selected, pass);
                    }
                    Key::Down => {
                        pass = None;
                        current = Some(cloc_position(&current, answer_len, 2));
                        draw_question(&modernweb, subject, count, length, &current, &selected, pass);
                    }
                    _ => {}
                }
            }

            let ten_millis = time::Duration::from_millis(300);
            let now = time::Instant::now();

            thread::sleep(ten_millis);
            write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
            break;
        }
    }

    rprintln!(
        "{}你全部都答對了耶，可以領個小禮物 OwO{}",
        color::Fg(color::LightYellow),
        style::Reset
    );
}

fn check_pass(right_answers: &Vec<usize>, selected: &HashSet<usize>) -> bool {
    if selected.is_empty() { return false; }
    if selected.len() != right_answers.len() { return false; }

    let mut right = true;
    for x in selected {
        if !right_answers.contains(x) {
            right = false;
        }
    }
    right
}

fn cloc_position(current: &Option<usize>, len: usize, direct: u8) -> usize {
    match direct {
        // up
        1 => {
            if current == &None { return len - 1; }
            let c = current.unwrap();
            if c == 0 { return len - 1; }
            c - 1
        }
        // down
        2 => {
            if current == &None { return 0; }
            let next = current.unwrap() + 1;
            if next == len { return 0; }
            next
        }
        _ => 0
    }
}

fn draw_question(modernweb: &Modernweb, subject: &Subject, count: usize, length: usize, current: &Option<usize>, selected: &HashSet<usize>, pass: Option<bool>) {
    let answer_type_text = if subject.multi_answer() { "多選" } else { "單選" };
    rprintln!(
        "{}{}{}/{} ({}) 問題：{}{}",
        color::Fg(color::Cyan),
        style::Bold,
        count + 1,
        length,
        answer_type_text,
        subject.question,
        style::Reset
    );
    for (ix, answer) in subject.answers.iter().enumerate() {
        let mark_current = if let Some(i) = current { if i == &ix { "o" } else { " " } } else { " " };
        let mark_selected = if selected.iter().find(|&&item| item == ix).is_some() { "√" } else { " " };
        let item_text = if let Some(addition) = modernweb.addition() { addition.item_text(ix) } else { (ix + 1).to_string() };
        let mark = if let Some(addition) = modernweb.addition() { addition.mark() } else { ".".to_string() };
        rprintln!(
            "{}{}{} {} {}{} {}",
            color::Fg(color::Blue),
            item_text,
            mark,
            answer.text,
            color::Fg(color::Red),
            mark_current,
            mark_selected
        )
    }
    rprintln!();
    let note_text = "空  格： 選取答案\r\n方向鍵： 移動游標";
    let multi_note_text = if subject.multi_answer() { "\r\n換行鍵： 確認答案" } else { "" };
    rprintln!("{}{}{}", color::Fg(color::Green), note_text, multi_note_text);

    match pass {
        Some(true) => {
            rprintln!(
                "{}答對了，你好棒 owo\n{}",
                color::Fg(color::Yellow),
                style::Reset
            );
        }
        Some(false) => {
            rprintln!(
                "{}叭叭，答錯了 OAO\n{}",
                color::Fg(color::LightRed),
                style::Reset
            );
        }
        None => {}
    }
}
