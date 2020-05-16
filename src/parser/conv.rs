extern crate puruda;

use crate::parser::word::{TotalWords, Word, Words};
use std::fs::File;
use std::io::{BufRead, BufReader};
use puruda::*;
use rand::prelude::*;

#[allow(dead_code)]
pub fn smart_to_total_words() -> TotalWords {
    let mut words_vec: Vec<Words> = Vec::new();
    for i in 1..22 {
        let words = smart_to_words(i);
        words_vec.push(words);
    }

    TotalWords::new(words_vec)
}

#[allow(dead_code)]
pub fn smart_to_words_vec() -> Vec<Words> {
    let mut words_vec: Vec<Words> = Vec::new();
    for i in 1..22 {
        let words = smart_to_words(i);
        words_vec.push(words);
    }

    words_vec
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

pub fn toeic_to_words_vec() -> Vec<Words> {
    let col: Col2<Vec<String>, Vec<String>> = Col2::read_csv("word/toeic_327.csv", ',').expect("Can't read csv");
    let words = col.c1();
    let means = col.c2();
    
    let mut word_vec: Vec<Word> = Vec::new();
    for i in 0 .. words.len() {
        let word = Word::new(words[i].clone(), means[i].clone());
        word_vec.push(word);
    }

    word_vec.shuffle(&mut thread_rng());

    let mut words_vec: Vec<Words> = Vec::new();
    let mut word_iter = word_vec.into_iter();

    for i in 30 .. 38 {
        let mut w_vec: Vec<Word> = Vec::new();
        for _j in 0 .. 40 {
            w_vec.push(word_iter.next().unwrap());
        }
        let ws = Words::new(i, w_vec);
        words_vec.push(ws);
    }
    let w_vec = word_iter.collect::<Vec<Word>>();
    let ws = Words::new(38, w_vec);
    words_vec.push(ws);

    words_vec
}
