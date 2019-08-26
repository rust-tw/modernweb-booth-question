#[macro_use]
mod macros;
mod game;
mod repr;

use game::{Game, Question};
use repr::{Addition, Modernweb};
use std::{env, fs, io, thread, time::Duration};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode, style};

fn main() {
    let toml_file = env::args()
        .nth(1)
        .map_or("src/question.toml".to_string(), |v| v.to_string());

    let question_text =
        fs::read_to_string(&toml_file).expect("Failed to reading file: question.toml");
    let modernweb = toml::from_str(&question_text).expect("question.toml format fail");
    let Modernweb { subjects, addition } = modernweb;
    let Addition { mark, items } = addition;

    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut game = Game::new(&mut stdout, subjects.len(), mark, items);

    for (count, subject) in subjects.into_iter().enumerate() {
        game.update_question(Question::from_subject(count, subject));
        game.render();
        for c in io::stdin().keys() {
            match c.unwrap() {
                Key::Char(' ') => {
                    game.toggle_current();
                }
                Key::Char('\n') => {
                    if game.commit() {
                        game.render();
                        thread::sleep(Duration::from_millis(700));
                        break;
                    }
                }
                Key::Char(c) => {
                    game.toggle_item(c);
                }
                Key::Ctrl('c') => return,
                Key::Up => {
                    game.move_prev();
                }
                Key::Down => {
                    game.move_next();
                }
                _ => {}
            }
            game.render();
        }
    }

    rprintln!(
        "{}你全部都答對了耶，可以領個小禮物 OwO{}",
        color::Fg(color::LightYellow),
        style::Reset
    );
}
