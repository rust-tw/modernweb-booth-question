mod ui;

use crate::repr::Subject;
use std::{collections::HashSet, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Message {
    Empty,
    Pass,
    Fail,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QuestionKind {
    SingleChoose,
    MultipleChoose,
}

impl fmt::Display for QuestionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SingleChoose => f.write_str("單選"),
            Self::MultipleChoose => f.write_str("多選"),
        }
    }
}

pub struct Question {
    index: usize,
    title: String,
    options: Vec<String>,
    answer: HashSet<usize>,
}

impl Question {
    pub fn from_subject(index: usize, subject: Subject) -> Self {
        let answer = subject.right_answers();
        let Subject { question, answers } = subject;
        Question {
            index,
            title: question,
            options: answers.into_iter().map(|x| x.text).collect(),
            answer,
        }
    }

    fn kind(&self) -> QuestionKind {
        if self.answer.len() == 1 {
            QuestionKind::SingleChoose
        } else {
            QuestionKind::MultipleChoose
        }
    }

    fn is_single_choose(&self) -> bool {
        self.kind() == QuestionKind::SingleChoose
    }

    fn option_count(&self) -> usize {
        self.options.len()
    }
}

pub struct Game<'a> {
    term: &'a mut termion::raw::RawTerminal<std::io::Stdout>,
    total: usize,
    current: usize,
    selected: HashSet<usize>,
    question: Option<Question>,
    message: Message,
    mark: String,
    items: Vec<String>,
    unique: bool,
}

impl<'a> Game<'a> {
    pub fn new(
        term: &'a mut termion::raw::RawTerminal<std::io::Stdout>,
        total: usize,
        mark: String,
        items: Vec<String>,
    ) -> Self {
        let unique = items
            .iter()
            .map(|x| x[0..1].to_ascii_lowercase())
            .collect::<HashSet<_>>()
            .len()
            == items.len();
        Game {
            term,
            total,
            mark,
            items,
            unique,
            current: 0,
            selected: HashSet::new(),
            question: None,
            message: Message::Empty,
        }
    }

    pub fn update_question(&mut self, question: Question) {
        self.question = Some(question);
        self.message = Message::Empty;
        self.current = 0;
        self.selected.clear();
    }

    pub fn move_prev(&mut self) {
        let question = self.question.as_ref().unwrap();
        if self.current == 0 {
            self.current = question.option_count() - 1;
        } else {
            self.current -= 1;
        }
    }

    pub fn move_next(&mut self) {
        let question = self.question.as_ref().unwrap();
        self.current += 1;
        if self.current == question.option_count() {
            self.current = 0;
        }
    }

    pub fn toggle_item(&mut self, mut c: char) {
        let question = self.question.as_ref().unwrap();
        let is_single_choose = question.is_single_choose();
        if !self.unique || question.option_count() > self.items.len() {
            return;
        }

        c = c.to_ascii_lowercase();
        if let Some(n) = self
            .items
            .iter()
            .position(|x| x.to_ascii_lowercase().starts_with(c))
        {
            self.toggle_index(n);
        }

        if is_single_choose {
            self.commit();
        }
    }

    pub fn toggle_current(&mut self) {
        self.toggle_index(self.current);
    }

    fn toggle_index(&mut self, n: usize) {
        if self.selected.contains(&n) {
            self.selected.remove(&n);
        } else {
            self.selected.insert(n);
        }
    }

    pub fn commit(&mut self) -> bool {
        let question = self.question.as_ref().unwrap();
        if question.is_single_choose() && self.selected.len() == 0 {
            self.selected.insert(self.current);
        }
        if self.selected == question.answer {
            self.message = Message::Pass;
        } else {
            self.message = Message::Fail;
        }
        if question.is_single_choose() {
            self.selected.clear();
        }
        self.message == Message::Pass
    }
}
