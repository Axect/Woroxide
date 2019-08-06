use crate::parser::word::{Word, TotalWords};
use Chapter::{Chap, Range, All};
use std::process::exit;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::stdin;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Chapter {
    Chap(usize),
    Range(usize, usize),
    All,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Kind {
    Word,
    Mean,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub enum Kind2 {
    Random,
    Sequential
}

#[derive(Debug)]
pub struct Exam {
    kind: Kind,
    kind2: Kind2,
    words: Vec<Word>
}

impl Exam {
    pub fn new(chap: Chapter, kind: Kind, kind2: Kind2) -> Self {
        match chap {
            Chap(c) => {
                let total_words = TotalWords::from_file("word/word.json");
                let w = match total_words.get_specific_words(c) {
                    Some(w) => w,
                    None => exit(1),
                };
                
                Exam {
                    kind,
                    kind2,
                    words: w.get_word_vec(),
                }
            },
            Range(i, f) => {
                let total_words = TotalWords::from_file("word/word.json");
                let mut w_vec: Vec<Word> = Vec::new();
                for k in i .. f+1 {
                    match total_words.get_specific_words(k) {
                        None => exit(1),
                        Some(w) => {
                            let words = w.get_word_vec();
                            w_vec.extend(words);
                        }
                    }
                }
                
                Exam {
                    kind,
                    kind2,
                    words: w_vec
                }
            },
            All => {
                let total_words = TotalWords::from_file("word/word.json");
                let words_total = total_words.get_words_vec();
                let mut w_vec: Vec<Word> = Vec::new();
                words_total.into_iter()
                    .for_each(|w| w_vec.extend(w.get_word_vec()));
                
                Exam {
                    kind,
                    kind2,
                    words: w_vec
                }
            }
        }
    }

    pub fn start_exam(&self) {
        let mut score = 0usize;
        let step = 100 / self.words.len();
        let total_score = step * self.words.len();
        let mut exam_list = self.words.clone();

        match self.kind2 {
            Kind2::Random => {
                exam_list.shuffle(&mut thread_rng());
            },
            _ => (),
        }

        match self.kind {
            Kind::Word => {
                println!("Please enter the correct meaning of given word");
                println!();
                for i in 0 .. exam_list.len() {
                    let word = &exam_list[i];
                    println!("> {}", word.get_word());

                    let mut trial = String::new();
                    match stdin().read_line(&mut trial) {
                        Ok(_) => {
                            if word.match_with_mean(trial) == true {
                                score += step;
                                println!("Correct! score is: {}", score);
                                println!("");
                            } else {
                                println!("Incorrect!");
                                println!("");
                            }
                        },
                        Err(error) => {
                            println!("{}", error);
                            exit(1);
                        }
                    }
                }
            },
            Kind::Mean => {
                println!("Please enter the correct word of given meanings");
                println!();
                for i in 0 .. exam_list.len() {
                    let word = &exam_list[i];
                    println!("{:?}", word.get_mean());
                    print!("> ");
                    let mut trial = String::new();
                    match stdin().read_line(&mut trial) {
                        Ok(_) => {
                            if word.match_with_word(trial) {
                                score += step;
                                println!("Correct! score is: {}", score);
                                println!("");
                            } else {
                                println!("Incorrect!");
                                println!("");
                            }
                        },
                        Err(error) => {
                            println!("{}", error);
                            exit(1);
                        }
                    }
                }
            }
        }

        println!("Total score is: {}/{}", score, total_score);
    }
}