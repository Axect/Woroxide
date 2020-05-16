extern crate woroxide;
extern crate serde_json;

use std::fs;
use std::fs::File;
use std::io::BufWriter;
use woroxide::parser::word::TotalWords;
use woroxide::parser::conv::{toeic_to_words_vec, smart_to_total_words};

fn main() {
    //let mut word_smart = smart_to_words_vec();
    let path = "word/word.json";
    let mut total_word: TotalWords = if fs::metadata(path).is_ok() {
        TotalWords::from_file(path)
    } else {
        smart_to_total_words()
    };
    let toeic = toeic_to_words_vec();

    total_word.total.extend(toeic);
    
    let file = if fs::metadata(path).is_ok() {
        File::create(path).expect("Can't write file")
    } else {
        fs::rename(path, "word/word_backup.json").expect("Can't remove file");
        File::create(path).expect("Can't write file")
    };
    let json_writer = BufWriter::new(file);
    serde_json::to_writer_pretty(json_writer, &total_word).expect("Can't write json");
}
