use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
};

#[derive(Debug, Default, Deserialize)]
pub struct Subject {
    question: String,
    answer: String,
}

const QUESTION_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/question.json");

fn main() {
    let f = File::open(QUESTION_PATH).expect("question.json not found");
    let subjects: Vec<Subject> = serde_json::from_reader(f).unwrap();
    let subjects = subjects
        .into_iter()
        .map(|Subject { question, answer }| {
            quote::quote! {
                Subject {
                    question: #question .to_owned(),
                    answer: #answer .to_owned(),
                }
            }
        })
        .collect::<Vec<_>>();
    let code = quote::quote! {
        static SUBJECTS: Lazy<Vec<Subject>> = Lazy::new(|| vec![
            #(#subjects ,)*
        ]);
    };
    fs::write(
        &format!("{}/subjects.rs", env::var("OUT_DIR").unwrap()),
        code.to_string(),
    )
    .unwrap();
}
