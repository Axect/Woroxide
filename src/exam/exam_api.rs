extern crate cursive;
extern crate num_rational;

use crate::parser::word::{TotalWords, Word};
use cursive::{
    traits::*,
    views::{Button, Dialog, EditView, LinearLayout},
    Cursive,
};
use num_rational::Rational;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::stdin;
use std::process::exit;
use Chapter::{All, Chap, Range};
use cursive::views::{TextView, DummyView, OnEventView, ListView};
use cursive::event::Key;
use std::rc::Rc;

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
        s.set_user_data((exam_list, score_num));

        input_mean(s, word, score_num, score_denom);
        // s.add_layer(
        //     Dialog::around(
        //         OnEventView::new(
        //             LinearLayout::vertical()
        //                 .child(
        //                     TextView::new(word).with_name("word")
        //                 )
        //                 .child(
        //                     ListView::new()
        //                         .child(
        //                             "Mean: ",
        //                             EditView::new()
        //                                 .on_submit(move |s, txt| {
        //                                     let score_denom = score_denom.clone();
        //                                     let trial = txt.to_string();
        //                                     let l = trial.len();
        //                                     let (next, correct, score_num, message) = s.with_user_data(|(list, score): &mut (Vec<Word>, usize)| {
        //                                         let curr = list.remove(0);
        //                                         let next = match list.get(0) {
        //                                             None => None,
        //                                             Some(value) => Some(value.clone()),
        //                                         };
        //                                         let (correct, message) = if curr.match_with_mean(trial.to_string()) {
        //                                             *score += 1;
        //                                             (true, format!("Score: {}/{}", score, score_denom))
        //                                         } else {
        //                                             (false, "Incorrect!".to_string())
        //                                         };
        //                                         (next, correct, *score, message)
        //                                     }).unwrap();
        //
        //                                     match next {
        //                                         None => {
        //                                             s.pop_layer();
        //                                             s.add_layer(
        //                                                 Dialog::new()
        //                                                     .title("Total Score")
        //                                                     .content(
        //                                                         TextView::new(message)
        //                                                     )
        //                                                     .button("Ok", |s| s.quit())
        //                                             );
        //                                         },
        //                                         Some(curr) => {
        //                                             let word = curr.get_word();
        //                                             let mean = curr.get_mean();
        //                                             s.call_on_name("word", |view: &mut TextView| {
        //                                                 view.set_content(word);
        //                                             });
        //                                             s.call_on_name("score", |view: &mut TextView| {
        //                                                 view.set_content(format!("Score: {}/{}", score_num, score_denom));
        //                                             });
        //                                             s.refresh();
        //                                         }
        //                                     }
        //                                 })
        //                                 .fixed_width(20)
        //                                 .with_name("mean")
        //                         )
        //                 )
        //                 .child(
        //                     TextView::new(format!("Score: {}/{}", score_num, score_denom)).with_name("score")
        //                 )
        //         )
        //     )
        //         .title("Exam")
        // );
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

        let mut score_num = 0usize;
        let curr = &exam_list[0];
        let word = curr.get_word();
        let mean = curr.get_mean();
        let index = 0usize;

        s.set_user_data(exam_list);

        s.add_layer(
            Dialog::around(
                OnEventView::new(
                    LinearLayout::vertical()
                        .child(
                            TextView::new(word).with_name("word")
                        )
                        .child(
                            TextView::new(mean).with_name("mean")
                        )
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
                    })
            )
                .title("Exam")
        );

        // match self.kind {
        //     Kind::Word => {
        //         println!("Please enter the correct meaning of given word");
        //         println!();
        //         for i in 0..exam_list.len() {
        //             let word = &exam_list[i];
        //             println!("> {}", word.get_word());
        //
        //             let mut trial = String::new();
        //             match stdin().read_line(&mut trial) {
        //                 Ok(_) => {
        //                     if word.match_with_mean(trial) == true {
        //                         score_num += 1;
        //                         println!("Correct! score is: {}/{}", score_num, exam_list.len());
        //                         println!("");
        //                     } else {
        //                         println!("Incorrect!");
        //                         println!("");
        //                     }
        //                 }
        //                 Err(error) => {
        //                     println!("{}", error);
        //                     exit(1);
        //                 }
        //             }
        //         }
        //     }
        //     Kind::Mean => {
        //         println!("Please enter the correct word of given meanings");
        //         println!();
        //         for i in 0..exam_list.len() {
        //             let word = &exam_list[i];
        //             println!("{:?}", word.get_mean());
        //             print!("> ");
        //             let mut trial = String::new();
        //             match stdin().read_line(&mut trial) {
        //                 Ok(_) => {
        //                     if word.match_with_word(trial) {
        //                         score_num += 1;
        //                         println!("Correct! score is: {}/{}", score_num, exam_list.len());
        //                         println!("");
        //                     } else {
        //                         println!("Incorrect!");
        //                         println!("");
        //                     }
        //                 }
        //                 Err(error) => {
        //                     println!("{}", error);
        //                     exit(1);
        //                 }
        //             }
        //         }
        //     }
        // }
        //
        // println!("Total score is: {}/{}", score_num, exam_list.len());
    }

    //pub fn start_memorize(&self) {
    //    let mut memorize_list = self.words.clone();

    //    match self.kind2 {
    //        Kind2::Random(n) => {
    //            memorize_list.shuffle(&mut thread_rng());
    //            memorize_list = memorize_list.into_iter().take(n).collect();
    //        }
    //        _ => (),
    //    }

    //    match self.kind {
    //        Kind::Word => {
    //            println!("Think the correct meaning of given word");
    //            println!();
    //            for i in 0..memorize_list.len() {
    //                let word = &memorize_list[i];
    //                println!("> {}", word.get_word());

    //                let mut trial = String::new();
    //                match stdin().read_line(&mut trial) {
    //                    Ok(_) => {
    //                        let mean = word.get_mean();
    //                        println!("{}", mean);
    //                        println!();
    //                    }
    //                    Err(error) => {
    //                        println!("{}", error);
    //                        exit(1);
    //                    }
    //                }
    //            }
    //        }
    //        Kind::Mean => {
    //            println!("Think the correct word of given meanings");
    //            println!();
    //            for i in 0..memorize_list.len() {
    //                let word = &memorize_list[i];
    //                println!("{:?}", word.get_mean());
    //                print!("> ");
    //                let mut trial = String::new();
    //                match stdin().read_line(&mut trial) {
    //                    Ok(_) => {
    //                        let word = word.get_word();
    //                        println!("{}", word);
    //                        println!();
    //                    }
    //                    Err(error) => {
    //                        println!("{}", error);
    //                        exit(1);
    //                    }
    //                }
    //            }
    //        }
    //    }

    //    println!("Finish memorizing!");
    //}
}
fn input_mean(s: &mut Cursive, word: String, score_num: usize, score_denom: usize) {
    let mut score_num = score_num;
    s.add_layer(
        Dialog::around(
            OnEventView::new(
                LinearLayout::vertical()
                    .child(
                        TextView::new(word).with_name("word")
                    )
                    .child(
                        ListView::new()
                            .child(
                                "Mean: ",
                                EditView::new()
                                    .on_submit(move |s, txt| {
                                        let score_denom = score_denom.clone();
                                        let trial = txt.to_string();
                                        let l = trial.len();
                                        let (next, correct, score_num, message) = s.with_user_data(|(list, score): &mut (Vec<Word>, usize)| {
                                            let curr = list.remove(0);
                                            let next = match list.get(0) {
                                                None => None,
                                                Some(value) => Some(value.clone()),
                                            };
                                            let (correct, message) = if curr.match_with_mean(trial.to_string()) {
                                                *score += 1;
                                                (true, format!("Score: {}/{}", score, score_denom))
                                            } else {
                                                (false, "Incorrect!".to_string())
                                            };
                                            (next, correct, *score, message)
                                        }).unwrap();

                                        match next {
                                            None => {
                                                s.pop_layer();
                                                s.add_layer(
                                                    Dialog::new()
                                                        .title("Total Score")
                                                        .content(
                                                            TextView::new(message)
                                                        )
                                                        .button("Ok", |s| s.quit())
                                                );
                                            },
                                            Some(curr) => {
                                                s.pop_layer();
                                                let word = curr.get_word();
                                                input_mean(s, word, score_num, score_denom)
                                            }
                                        }
                                    })
                                    .fixed_width(20)
                            )
                    )
                    .child(
                        TextView::new(format!("Score: {}/{}", score_num, score_denom)).with_name("score")
                    )
            )
        )
            .title("Exam")
    );
}