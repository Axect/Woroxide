#[macro_use]
extern crate serde_derive;
extern crate cursive;
extern crate serde;
extern crate serde_json;

mod exam;
mod parser;

use crate::parser::conv::smart_to_total_words;
use cursive::{
    traits::*,
    views::{Button, Dialog, EditView, ListView, SelectView, TextView},
    Cursive,
};
use exam::exam_api::{
    Chapter,
    Chapter::{All, Chap, Range},
    {Exam, Kind, Kind2},
};
use std::fs::File;
use std::io::{stdin, BufWriter};
use std::process::exit;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RECEIVE_PURPOSE {
    Memorize,
    Exam,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RECEIVE_KIND {
    Word,
    Mean,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RECEIVE_RANGE {
    Chap,
    Range,
    All,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum RECEIVE_METHOD {
    Rand,
    Sequential,
}

#[derive(Debug, Copy, Clone)]
struct Phase1 {
    purpose: RECEIVE_PURPOSE,
    kind: RECEIVE_KIND,
    range: RECEIVE_RANGE,
    method: RECEIVE_METHOD,
}

#[derive(Debug, Copy, Clone)]
struct Phase2 {
    start: usize,
    end: usize,
    num: usize,
}

fn main() {
    // Cursive
    let mut siv = Cursive::default();
    let default_phase1 = Phase1 {
        purpose: RECEIVE_PURPOSE::Memorize,
        kind: RECEIVE_KIND::Word,
        range: RECEIVE_RANGE::Chap,
        method: RECEIVE_METHOD::Rand
    };
    siv.set_user_data(default_phase1);

    // Start
    //start(&mut siv);
    setting(&mut siv);

    siv.run();

    //println!("What's your purpose?");
    //println!("1. Memorize, 2. Execute exam");
    //let mut purpose = String::new();
    //match stdin().read_line(&mut purpose) {
    //    Ok(_) => (),
    //    Err(e) => {
    //        println!("{}", e);
    //        exit(1);
    //    }
    //}

    //let purpose: usize = purpose.trim().parse().expect("Can't convert purpose to usize");
    //match purpose {
    //    1 => memorize(),
    //    2 => execute_exam(),
    //    3 => write_json(),
    //    _ => {
    //        println!("Wrong argument!");
    //        exit(1);
    //    }
    //}
}

fn setting(s: &mut Cursive) {
    s.add_layer({
        Dialog::new()
            .title("Woroxide")
            .content(
                ListView::new()
                    .child(
                        "Purpose",
                        SelectView::<RECEIVE_PURPOSE>::new()
                            .popup()
                            .item("Memorize", RECEIVE_PURPOSE::Memorize)
                            .item("Exam", RECEIVE_PURPOSE::Exam)
                            .on_submit(|s, item| {
                                let ud: &mut Phase1 = s.user_data().unwrap();
                                (*ud).purpose = *item;
                                s.focus_name("kind");
                            })
                            .with_name("purpose"),
                    )
                    .child(
                        "Kind of Test",
                        SelectView::<RECEIVE_KIND>::new()
                            .popup()
                            .item("Word", RECEIVE_KIND::Word)
                            .item("Mean", RECEIVE_KIND::Mean)
                            .on_submit(|s, item| {
                                let ud: &mut Phase1 = s.user_data().unwrap();
                                (*ud).kind = *item;
                                s.focus_name("range");
                            })
                            .with_name("kind"),
                    )
                    .child(
                        "Range of Test",
                        SelectView::<RECEIVE_RANGE>::new()
                            .popup()
                            .item("Specific Chapter", RECEIVE_RANGE::Chap)
                            .item("Range", RECEIVE_RANGE::Range)
                            .item("All", RECEIVE_RANGE::All)
                            .on_submit(|s, item| {
                                let ud: &mut Phase1 = s.user_data().unwrap();
                                (*ud).range = *item;
                                s.focus_name("method");
                            })
                            .with_name("range"),
                    )
                    .child(
                        "Method of Test",
                        SelectView::<RECEIVE_METHOD>::new()
                            .popup()
                            .item("Random", RECEIVE_METHOD::Rand)
                            .item("Sequential", RECEIVE_METHOD::Sequential)
                            .on_submit(|s, item| {
                                let ud: &mut Phase1 = s.user_data().unwrap();
                                (*ud).method = *item;
                            })
                            .with_name("method"),
                    )
            )
            .button("Ok", |s| {
                let phase1 = s.take_user_data().unwrap();
                setting2(s, phase1)
            })
    })
}

fn setting2(s: &mut Cursive, phase1: Phase1) {
    s.pop_layer();
    s.set_user_data(
        Phase2 {
            start: 0,
            end: 0,
            num: 0
        }
    );
    match phase1.range {
        RECEIVE_RANGE::Chap => {
            s.add_layer(
                Dialog::new()
                    .title("Specify Chapter")
                    .content(
                        EditView::new()
                            .on_submit(move |s, txt| {
                                let c: usize = txt.parse().unwrap();
                                let chap = Chap(c);
                                let kind = match phase1.kind {
                                    RECEIVE_KIND::Word => Kind::Word,
                                    RECEIVE_KIND::Mean => Kind::Mean,
                                };
                                let method = match phase1.method {
                                    RECEIVE_METHOD::Sequential => Kind2::Sequential,
                                    RECEIVE_METHOD::Rand => Kind2::Random(0),
                                };
                                let exam = Exam::new(chap, kind, method);
                                begin_exam(s, exam)
                            })
                            .fixed_width(3)
                    )
            );
        }
        RECEIVE_RANGE::Range => {
            match phase1.method {
                RECEIVE_METHOD::Rand => {
                    s.add_layer(
                        Dialog::new()
                            .title("Supplement")
                            .content(
                                ListView::new()
                                    .child(
                                        "Start Chapter",
                                        EditView::new()
                                            .on_submit(|s, txt| {
                                                let ud: &mut Phase2 = s.user_data().unwrap();
                                                (*ud).start = txt.parse().unwrap();
                                                s.focus_name("end");
                                            })
                                            .fixed_width(3)
                                            .with_name("start")
                                    )
                                    .child(
                                        "End Chapter",
                                        EditView::new()
                                            .on_submit(|s, txt| {
                                                let ud: &mut Phase2 = s.user_data().unwrap();
                                                (*ud).end = txt.parse().unwrap();
                                                s.focus_name("number");
                                            })
                                            .fixed_width(3)
                                            .with_name("end")
                                    )
                                    .child(
                                        "Number of words",
                                        EditView::new()
                                            .on_submit(|s, txt| {
                                                let ud: &mut Phase2 = s.user_data().unwrap();
                                                (*ud).num = txt.parse().unwrap();
                                            })
                                            .fixed_width(4)
                                            .with_name("number")
                                    )
                            )
                            .button(
                                "OK",
                                move |s| {
                                    let kind = match phase1.kind {
                                        RECEIVE_KIND::Word => Kind::Word,
                                        RECEIVE_KIND::Mean => Kind::Mean,
                                    };
                                    let ud: Phase2 = s.take_user_data().unwrap();
                                    let exam = Exam::new(Range(ud.start, ud.end), kind, Kind2::Random(ud.num));
                                    begin_exam(s, exam)
                                }
                            )
                    );
                }
                RECEIVE_METHOD::Sequential => {
                    s.add_layer(
                        Dialog::new()
                            .title("Supplement")
                            .content(
                                ListView::new()
                                    .child(
                                        "Start Chapter",
                                        EditView::new()
                                            .on_submit(|s, txt| {
                                                let ud: &mut Phase2 = s.user_data().unwrap();
                                                (*ud).start = txt.parse().unwrap();
                                                s.focus_name("end");
                                            })
                                            .fixed_width(3)
                                            .with_name("start")
                                    )
                                    .child(
                                        "End Chapter",
                                        EditView::new()
                                            .on_submit(|s, txt| {
                                                let ud: &mut Phase2 = s.user_data().unwrap();
                                                (*ud).end = txt.parse().unwrap();
                                            })
                                            .fixed_width(3)
                                            .with_name("end")
                                    )
                            )
                            .button(
                                "Ok",
                                move |s| {
                                    let kind = match phase1.kind {
                                        RECEIVE_KIND::Word => Kind::Word,
                                        RECEIVE_KIND::Mean => Kind::Mean,
                                    };
                                    let ud: Phase2 = s.take_user_data().unwrap();
                                    let exam = Exam::new(Range(ud.start, ud.end), kind, Kind2::Sequential);
                                    begin_exam(s, exam)
                                }
                            )
                    );
                }
            }
        }
        RECEIVE_RANGE::All => {
            match phase1.method {
                RECEIVE_METHOD::Rand => {
                    s.add_layer(
                        Dialog::new()
                            .title("Supplement")
                            .content(
                                ListView::new()
                                    .child(
                                        "Number of words",
                                        EditView::new()
                                            .on_submit(move |s, text| {
                                                let number: usize = text.parse().unwrap();
                                                let kind = match phase1.kind {
                                                    RECEIVE_KIND::Word => Kind::Word,
                                                    RECEIVE_KIND::Mean => Kind::Mean,
                                                };
                                                let exam = Exam::new(All, kind, Kind2::Random(number));
                                                begin_exam(s, exam)
                                            })
                                            .fixed_width(4)
                                            .with_name("number")
                                    )
                            )
                    );
                }
                RECEIVE_METHOD::Sequential => {
                    let kind = match phase1.kind {
                        RECEIVE_KIND::Word => Kind::Word,
                        RECEIVE_KIND::Mean => Kind::Mean,
                    };
                    let exam = Exam::new(All, kind, Kind2::Sequential);
                    begin_exam(s, exam)
                }
            }
        }
    }
}

/// Start Woroxide
fn start(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("What's your purpose?")
            .title("Woroxide")
            .button("Memorize", memorize)
            .button("Exam", execute_exam)
            .button("Quit", Cursive::quit),
    );
}

fn write_json() {
    let whole = smart_to_total_words();
    let file = File::create("word/word.json").expect("Can't create file");
    let json_writer = BufWriter::new(file);
    serde_json::to_writer_pretty(json_writer, &whole).expect("hmm");
}

#[allow(non_snake_case)]
fn execute_exam(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("What kinds of test do you want?")
            .title("Set kind of test")
            .button("Specific chapter", specific_chapter)
            .button("Range of chapters", range_chapter)
            .button("All chapters", all_chapter)
            .button("Back", start),
    );
}

fn specific_chapter(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::new().title("Enter the chapter").content(
            EditView::new()
                .on_submit(input_chapter)
                .with_name("chapter")
                .fixed_width(10),
        ), //.button("Ok", |s| {
           //    let chapter = s.call_on_name("chapter", |view: &mut EditView| {
           //        view.get_content()
           //    }).unwrap();

           //    let chap_usize = chapter.trim().parse::<usize>().unwrap();
           //    kind1(s, Chap(chap_usize))
           //})
    );
}

fn input_chapter(s: &mut Cursive, chap: &str) {
    if chap.is_empty() {
        s.add_layer(Dialog::info("Please enter a chapter"));
    } else {
        let chap_usize = chap.trim().parse::<usize>().unwrap();
        kind1(s, Chap(chap_usize))
    }
}

fn range_chapter(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::new().title("Enter the start chapter").content(
            EditView::new()
                .on_submit(input_start_chapter)
                .with_name("start chap")
                .fixed_width(10),
        ), //.button("Ok", |s| {
           //    let start = s.call_on_name("start chap", |view: &mut EditView| {
           //        view.get_content()
           //    }).unwrap();

           //    let start_usize = start.trim().parse::<usize>().unwrap();
           //    input_end_chapter(s, start_usize)
           //})
    );
}

fn input_start_chapter(s: &mut Cursive, chap: &str) {
    if chap.is_empty() {
        s.add_layer(Dialog::info("Please enter a chapter"));
    } else {
        let chap_usize = chap.trim().parse::<usize>().unwrap();
        input_end_chapter(s, chap_usize)
    }
}

fn input_end_chapter(s: &mut Cursive, start_chap: usize) {
    s.pop_layer();
    s.add_layer(
        Dialog::new().title("Enter an end chapter").content(
            EditView::new()
                .on_submit(move |s, chap| {
                    let end_chap = chap.trim().parse::<usize>().unwrap();
                    let range = Range(start_chap, end_chap);
                    kind1(s, range)
                })
                .with_name("end chap")
                .fixed_width(10),
        ),
    );
}

fn all_chapter(s: &mut Cursive) {
    s.pop_layer();
    kind1(s, All)
}

fn kind1(s: &mut Cursive, chap: Chapter) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("What do you want to test?")
            .title("Set Kind1")
            .button("Word", move |s| kind2(s, chap, Kind::Word))
            .button("Mean", move |s| kind2(s, chap, Kind::Mean)),
    );
}

fn kind2(s: &mut Cursive, chap: Chapter, k1: Kind) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Choose method for your test")
            .title("Set Kind2")
            .button("Random", move |s| {
                s.pop_layer();
                s.add_layer(
                    Dialog::around(
                        EditView::new()
                            .on_submit(move |s, txt| input_random(s, txt, chap, k1))
                            .fixed_width(3),
                    )
                    .title("Enter the number of words"),
                );
            })
            .button("Sequential", move |s| {
                let exam = Exam::new(chap, k1, Kind2::Sequential);
                begin_exam(s, exam);
            }),
    );
}

fn input_random(s: &mut Cursive, txt: &str, chap: Chapter, k1: Kind) {
    s.pop_layer();
    let num = txt.parse().unwrap();
    let exam = Exam::new(chap, k1, Kind2::Random(num));
    begin_exam(s, exam)
}

fn begin_exam(s: &mut Cursive, ex: Exam) {
    s.pop_layer();
    s.add_layer(
        Dialog::info(format!("{:?}", ex.words))
    );
}

fn memorize(s: &mut Cursive) {
    unimplemented!()
}

//#[allow(non_snake_case)]
//fn memorize(s: &mut Cursive) {
//    let mut kind1 = String::new();
//    let mut kind2 = String::new();
//    let mut chap = String::new();
//    let mut chap_start = String::new();
//    let mut chap_end = String::new();
//    let mut kind0 = String::new();
//
//    let Chapter;
//    let Kind_1;
//    let Kind_2;
//
//    println!("Woroxide Ver 0.2.0");
//    println!("");
//    println!("> What kinds of memorize do you want?");
//    println!("> 1. Specific chapter\t2. Range of chapters\t3. All chapters");
//    match stdin().read_line(&mut kind0) {
//        Ok(_) => (),
//        Err(error) => {
//            println!("{}", error);
//            exit(1);
//        }
//    }
//
//    if kind0.trim() == "1" {
//        println!("");
//        println!("> What chapter do you want to memorize?");
//        match stdin().read_line(&mut chap) {
//            Ok(_) => Chapter = Chap(chap.trim().parse().unwrap()),
//            Err(error) => {
//                println!("{}", error);
//                exit(1);
//            }
//        }
//    } else if kind0.trim() == "2" {
//        println!("");
//        println!("> Input start chapter");
//        match stdin().read_line(&mut chap_start) {
//            Ok(_) => {
//                println!("> Input last chapter");
//                match stdin().read_line(&mut chap_end) {
//                    Ok(_) => {
//                        Chapter = Range(chap_start.parse().unwrap(), chap_end.parse().unwrap())
//                    }
//                    Err(error) => {
//                        println!("{}", error);
//                        exit(1);
//                    }
//                }
//            }
//            Err(error) => {
//                println!("{}", error);
//                exit(1);
//            }
//        }
//    } else {
//        Chapter = All;
//    }
//
//    println!("");
//    println!("> What do you want to memorize?");
//    println!("> 1. Word\t2. Mean");
//    match stdin().read_line(&mut kind1) {
//        Ok(_) => (),
//        Err(error) => {
//            println!("{}", error);
//            exit(1);
//        }
//    }
//
//    if kind1.trim() == "1" {
//        Kind_1 = Kind::Word;
//    } else {
//        Kind_1 = Kind::Mean;
//    }
//
//    println!("");
//    println!("> What kinds of memorize format do you want?");
//    println!("> 1. Random\t2. Sequential");
//    match stdin().read_line(&mut kind2) {
//        Ok(_) => (),
//        Err(error) => {
//            println!("{}", error);
//            exit(1);
//        }
//    }
//
//    if kind2.trim() == "1" {
//        Kind_2 = Kind2::Random;
//    } else {
//        Kind_2 = Kind2::Sequential;
//    }
//
//    println!("");
//
//    let exam = Exam::new(Chapter, Kind_1, Kind_2);
//    exam.start_memorize();
//}
