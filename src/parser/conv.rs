use crate::parser::word::{TotalWords, Word, Words};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn smart_to_total_words() -> TotalWords {
    let mut words_vec: Vec<Words> = Vec::new();
    for i in 1..22 {
        let words = smart_to_words(i);
        words_vec.push(words);
    }

    TotalWords::new(words_vec)
}

pub fn smart_to_words(num: usize) -> Words {
    let chap = num - 1;
    let word_file = File::open(&format!("word/word_smart/word{}.txt", chap))
        .expect(&format!("Can't open word{}.txt", chap));
    let mean_file = File::open(&format!("word/word_smart/mean{}.txt", chap))
        .expect(&format!("Can't open mean{}.txt", chap));
    let word_reader = BufReader::new(word_file);
    let mean_reader = BufReader::new(mean_file);

    let mut word_vec: Vec<Word> = Vec::new();
    for (word, mean) in word_reader.lines().zip(mean_reader.lines()) {
        match (word, mean) {
            (Ok(w), Ok(m)) => {
                let word = Word::new(w, m);
                word_vec.push(word);
            }
            _ => assert!(false, "Can't parse word & mean"),
        }
    }

    Words::new(num, word_vec)
}
