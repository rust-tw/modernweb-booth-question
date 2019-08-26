use super::{Game, Message, QuestionKind};
use std::{borrow::Cow, fmt, io::Write};
use termion::{color, style};

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::Fail => f.write_fmt(format_args!(
                "{}叭叭，答錯了 OAO\n{}",
                color::Fg(color::LightRed),
                style::Reset,
            )),
            Message::Pass => f.write_fmt(format_args!(
                "{}答對了，你好棒 owo\n{}",
                color::Fg(color::Yellow),
                style::Reset,
            )),
            Message::Empty => Ok(()),
        }
    }
}

impl QuestionKind {
    fn keyboard_guide(&self) -> &'static str {
        match self {
            QuestionKind::SingleChoose => "換行鍵： 選取答案\r\n方向鍵： 移動游標",
            QuestionKind::MultipleChoose => {
                "空白鍵： 選取答案\r\n方向鍵： 移動游標\r\n換行鍵： 確認答案"
            }
        }
    }

    fn display_guide(&self) -> DisplayGuide {
        DisplayGuide(*self)
    }
}

struct DisplayGuide(QuestionKind);

impl fmt::Display for DisplayGuide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}{}",
            color::Fg(color::Green),
            self.0.keyboard_guide()
        ))
    }
}

impl Game<'_> {
    pub fn render(&mut self) {
        let question = self.question.as_ref().unwrap();
        write!(
            self.term,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        rprintln!(
            "{}{}{}/{} ({})問題：{}{}",
            color::Fg(color::Cyan),
            style::Bold,
            question.index + 1,
            self.total,
            question.kind(),
            question.title,
            style::Reset
        );
        for (ix, answer) in question.options.iter().enumerate() {
            let mark_current = if self.current == ix { "o" } else { " " };
            let mark_selected = if self.selected.contains(&ix) {
                "√"
            } else {
                " "
            };
            let item_text = self
                .items
                .get(ix)
                .map_or_else(|| Cow::Owned((ix + 1).to_string()), |x| Cow::Borrowed(x));
            let mark = &self.mark;

            rprintln!(
                "{}{}{} {} {}{} {}",
                color::Fg(color::Blue),
                item_text,
                mark,
                answer,
                color::Fg(color::Red),
                mark_current,
                mark_selected
            )
        }

        rprintln!("{}", self.message);
        rprintln!("{}", question.kind().display_guide());
        self.term.flush().unwrap();
    }
}
