extern crate cursive;

use crate::parser::word::{TotalWords, Word};
use cursive::{
    align::HAlign,
    event::Key,
    theme::Effect,
    traits::*,
    views::{Dialog, EditView, ListView, OnEventView, TextView},
    Cursive,
};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::process::exit;
use Chapter::{All, Chap, Range};

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
    Random(usize),
    Sequential,
}

#[derive(Debug)]
pub struct Exam {
    pub kind: Kind,
    pub kind2: Kind2,
    pub words: Vec<Word>,
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
                let words = w.get_word_vec();

                Exam { kind, kind2, words }
            }
            Range(i, f) => {
                let total_words = TotalWords::from_file("word/word.json");
                let mut w_vec: Vec<Word> = Vec::new();
                for k in i..f + 1 {
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
                    words: w_vec,
                }
            }
            All => {
                let total_words = TotalWords::from_file("word/word.json");
                let words_total = total_words.get_words_vec();
                let mut w_vec: Vec<Word> = Vec::new();
                words_total
                    .into_iter()
                    .for_each(|w| w_vec.extend(w.get_word_vec()));

                Exam {
                    kind,
                    kind2,
                    words: w_vec,
                }
            }
        }
    }

    pub fn start_exam(&self, s: &mut Cursive) {
        s.pop_layer();
        let mut exam_list = self.words.clone();

        match self.kind2 {
            Kind2::Random(n) => {
                exam_list.shuffle(&mut thread_rng());
                exam_list = exam_list.into_iter().take(n).collect();
            }
            _ => (),
        }

        let score_num = 0usize;
        let score_denom = exam_list.len();
        let curr = &exam_list[0];
        let word = curr.get_word();
        let mean = curr.get_mean();

        s.set_user_data((exam_list, score_num));

        match self.kind {
            Kind::Word => input_mean(s, word, score_num, score_denom),
            Kind::Mean => input_word(s, mean, score_num, score_denom),
        }
    }

    pub fn start_memorize(&self, s: &mut Cursive) {
        s.pop_layer();
        let mut exam_list = self.words.clone();

        match self.kind2 {
            Kind2::Random(n) => {
                exam_list.shuffle(&mut thread_rng());
                exam_list = exam_list.into_iter().take(n).collect();
            }
            _ => (),
        }

        let curr = &exam_list[0];
        let word = curr.get_word();
        let mean = curr.get_mean();

        s.set_user_data(exam_list);

        s.add_layer(
            Dialog::around(
                OnEventView::new(
                    ListView::new()
                        .delimiter()
                        .child(
                            "",
                            TextView::new(word)
                                .effect(Effect::Italic)
                                .h_align(HAlign::Center)
                                .with_name("word"),
                        )
                        .delimiter()
                        .delimiter()
                        .child(
                            "",
                            TextView::new(mean)
                                .h_align(HAlign::Center)
                                .with_name("mean")
                                .fixed_width(100),
                        )
                        .fixed_height(10),
                )
                .on_event(Key::Enter, |s| {
                    let curr_opt = s.with_user_data(|list: &mut Vec<Word>| list.pop()).unwrap();
                    match curr_opt {
                        None => s.quit(),
                        Some(curr) => {
                            let word = curr.get_word();
                            let mean = curr.get_mean();
                            s.call_on_name("word", |view: &mut TextView| {
                                view.set_content(word);
                            });
                            s.call_on_name("mean", |view: &mut TextView| {
                                view.set_content(mean);
                            });
                        }
                    }
                }),
            )
            .title("Memorize"),
        );
    }
}

fn input_mean(s: &mut Cursive, word: String, score_num: usize, score_denom: usize) {
    s.add_layer(
        Dialog::around(OnEventView::new(
            ListView::new()
                .delimiter()
                .child(
                    "",
                    TextView::new(word)
                        .effect(Effect::Italic)
                        .h_align(HAlign::Center)
                        .with_name("word"),
                )
                .delimiter()
                .delimiter()
                .child(
                    "",
                    ListView::new().child(
                        "Mean: ",
                        EditView::new()
                            .on_submit(move |s, txt| {
                                let score_denom = score_denom.clone();
                                let trial = txt.to_string();
                                let (next, score_num, message) = s
                                    .with_user_data(|(list, score): &mut (Vec<Word>, usize)| {
                                        let curr = list.remove(0);
                                        let next = match list.get(0) {
                                            None => None,
                                            Some(value) => Some(value.clone()),
                                        };
                                        let message = if curr.match_with_mean(trial.to_string()) {
                                            *score += 1;
                                            format!("Score: {}/{}", score, score_denom)
                                        } else {
                                            "Incorrect!".to_string()
                                        };
                                        (next, *score, message)
                                    })
                                    .unwrap();

                                match next {
                                    None => {
                                        s.pop_layer();
                                        s.add_layer(
                                            Dialog::new()
                                                .title("Total Score")
                                                .content(TextView::new(message))
                                                .button("Ok", |s| s.quit()),
                                        );
                                    }
                                    Some(curr) => {
                                        s.pop_layer();
                                        let word = curr.get_word();
                                        input_mean(s, word, score_num, score_denom)
                                    }
                                }
                            })
                            .fixed_width(30),
                    ),
                )
                .child(
                    "",
                    TextView::new(format!("Score: {}/{}", score_num, score_denom))
                        .with_name("score"),
                ),
        ))
        .title("Exam")
        .fixed_height(20),
    );
}

fn input_word(s: &mut Cursive, mean: String, score_num: usize, score_denom: usize) {
    s.add_layer(
        Dialog::around(OnEventView::new(
            ListView::new()
                .delimiter()
                .child(
                    "",
                    TextView::new(mean)
                        .effect(Effect::Italic)
                        .h_align(HAlign::Center)
                        .with_name("mean")
                        .fixed_height(5)
                        .fixed_width(100),
                )
                .delimiter()
                .delimiter()
                .child(
                    "",
                    ListView::new().child(
                        "Word: ",
                        EditView::new()
                            .on_submit(move |s, txt| {
                                let score_denom = score_denom.clone();
                                let trial = txt.to_string();
                                let (next, score_num, message) = s
                                    .with_user_data(|(list, score): &mut (Vec<Word>, usize)| {
                                        let curr = list.remove(0);
                                        let next = match list.get(0) {
                                            None => None,
                                            Some(value) => Some(value.clone()),
                                        };
                                        let message = if curr.match_with_word(trial.to_string()) {
                                            *score += 1;
                                            format!("Score: {}/{}", score, score_denom)
                                        } else {
                                            "Incorrect!".to_string()
                                        };
                                        (next, *score, message)
                                    })
                                    .unwrap();

                                match next {
                                    None => {
                                        s.pop_layer();
                                        s.add_layer(
                                            Dialog::new()
                                                .title("Total Score")
                                                .content(TextView::new(message))
                                                .button("Ok", |s| s.quit()),
                                        );
                                    }
                                    Some(curr) => {
                                        s.pop_layer();
                                        let mean = curr.get_mean();
                                        input_word(s, mean, score_num, score_denom)
                                    }
                                }
                            })
                            .fixed_width(100),
                    ),
                )
                .delimiter()
                .child(
                    "",
                    TextView::new(format!("Score: {}/{}", score_num, score_denom))
                        .with_name("score"),
                ),
        ))
        .title("Exam")
        .fixed_height(20),
    );
}
