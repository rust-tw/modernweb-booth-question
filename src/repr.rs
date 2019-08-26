use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Subject {
    pub question: String,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    #[serde(default)]
    pub right: bool,
}

fn default_mark() -> String {
    ".".to_owned()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Addition {
    #[serde(default = "default_mark")]
    pub mark: String,
    #[serde(default)]
    pub items: Vec<String>,
}

impl Default for Addition {
    fn default() -> Self {
        Addition {
            mark: default_mark(),
            items: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Modernweb {
    #[serde(default)]
    pub addition: Addition,
    pub subjects: Vec<Subject>,
}

impl Subject {
    pub fn right_answers(&self) -> HashSet<usize> {
        self.answers
            .iter()
            .enumerate()
            .filter_map(|(ix, item)| if item.right { Some(ix) } else { None })
            .collect()
    }
}
