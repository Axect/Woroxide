#[macro_use]
extern crate serde_derive;

extern crate serde_json;
use serde_json::{Result, Value};
use std::fs;
use std::fs::File;
use std::path::Path;

mod parser;

use parser::word::Words;

fn main() {
    let json_file_path = Path::new("word/word.json");
    let json_file = File::open(json_file_path).expect("file not found");
    let test: Words =
        serde_json::from_reader(json_file).expect("error while reading json");
    for v in test.get_words().into_iter() {
        println!("{:#?}", v);
    }
}