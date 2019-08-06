#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod parser;
mod exam;

use parser::word;
use exam::exam_api::{Exam, Kind, Kind2};
use exam::exam_api::Chapter::Chap;
use std::io::stdin;

fn main() {
    let exam = Exam::new(Chap(1), Kind::Word, Kind2::Random);
    exam.start_exam();
}