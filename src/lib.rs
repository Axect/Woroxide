#[macro_use]
extern crate serde_derive;
extern crate cursive;
extern crate serde;
extern crate serde_json;

pub mod exam;
pub mod parser;

//use crate::parser::conv::smart_to_total_words;
use cursive::{
    traits::*,
    views::{Dialog, EditView, ListView, SelectView},
    Cursive,
};
use exam::exam_api::{
    Chapter::{All, Chap, Range},
    {Exam, Kind, Kind2},
};
//use std::fs::File;
//use std::io::{stdin, BufWriter};
//use std::process::exit;
//use std::rc::Rc;

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RECEIVE_PURPOSE {
    Memorize,
    Exam,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RECEIVE_KIND {
    Word,
    Mean,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RECEIVE_RANGE {
    Chap,
    Range,
    All,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RECEIVE_METHOD {
    Rand,
    Sequential,
}

#[derive(Debug, Copy, Clone)]
pub struct Phase1 {
    pub purpose: RECEIVE_PURPOSE,
    pub kind: RECEIVE_KIND,
    pub range: RECEIVE_RANGE,
    pub method: RECEIVE_METHOD,
}

#[derive(Debug, Copy, Clone)]
pub struct Phase2 {
    pub start: usize,
    pub end: usize,
    pub num: usize,
}

pub fn setting(s: &mut Cursive) {
    s.add_layer({
        Dialog::new().title("Woroxide").content(
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
                            s.focus_name("kind").unwrap();
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
                            s.focus_name("range").unwrap();
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
                            s.focus_name("method").unwrap();
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
                            let mut phase1: Phase1 = s.take_user_data().unwrap();
                            phase1.method = *item;
                            setting2(s, phase1)
                        })
                        .with_name("method"),
                ),
        )
    })
}

pub fn setting2(s: &mut Cursive, phase1: Phase1) {
    s.pop_layer();
    s.set_user_data(Phase2 {
        start: 0,
        end: 0,
        num: 0,
    });
    let kind = phase1.kind;
    let purpose = phase1.purpose;
    match phase1.range {
        RECEIVE_RANGE::Chap => match phase1.method {
            RECEIVE_METHOD::Sequential => {
                s.add_layer(
                    Dialog::new().title("Specify Chapter").content(
                        EditView::new()
                            .on_submit(move |s, txt| {
                                let c: usize = txt.parse().unwrap();
                                let chap = Chap(c);
                                let kind = match phase1.kind {
                                    RECEIVE_KIND::Word => Kind::Word,
                                    RECEIVE_KIND::Mean => Kind::Mean,
                                };
                                let exam = Exam::new(chap, kind, Kind2::Sequential);
                                begin_exam(s, exam, purpose)
                            })
                            .fixed_width(3),
                    ),
                );
            }
            RECEIVE_METHOD::Rand => s.add_layer(
                Dialog::new().title("Supplement").content(
                    ListView::new()
                        .child(
                            "Chapter: ",
                            EditView::new()
                                .on_submit(move |s, txt| {
                                    let ud: &mut Phase2 = s.user_data().unwrap();
                                    (*ud).start = txt.parse().unwrap();
                                    s.focus_name("number").unwrap();
                                })
                                .fixed_width(3),
                        )
                        .child(
                            "Number of Word: ",
                            EditView::new()
                                .on_submit(move |s, txt| {
                                    let ud: Phase2 = s.take_user_data().unwrap();
                                    let num: usize = txt.parse().unwrap();
                                    let kind = match kind {
                                        RECEIVE_KIND::Word => Kind::Word,
                                        RECEIVE_KIND::Mean => Kind::Mean,
                                    };
                                    let exam = Exam::new(Chap(ud.start), kind, Kind2::Random(num));
                                    begin_exam(s, exam, purpose)
                                })
                                .with_name("number")
                                .fixed_width(4),
                        ),
                ),
            ),
        },
        RECEIVE_RANGE::Range => match phase1.method {
            RECEIVE_METHOD::Rand => {
                s.add_layer(
                    Dialog::new().title("Supplement").content(
                        ListView::new()
                            .child(
                                "Start Chapter",
                                EditView::new()
                                    .on_submit(|s, txt| {
                                        let ud: &mut Phase2 = s.user_data().unwrap();
                                        (*ud).start = txt.parse().unwrap();
                                        s.focus_name("end").unwrap();
                                    })
                                    .fixed_width(3)
                                    .with_name("start"),
                            )
                            .child(
                                "End Chapter",
                                EditView::new()
                                    .on_submit(|s, txt| {
                                        let ud: &mut Phase2 = s.user_data().unwrap();
                                        (*ud).end = txt.parse().unwrap();
                                        s.focus_name("number").unwrap();
                                    })
                                    .fixed_width(3)
                                    .with_name("end"),
                            )
                            .child(
                                "Number of words",
                                EditView::new()
                                    .on_submit(move |s, txt| {
                                        let num: usize = txt.parse().unwrap();
                                        let kind = match kind {
                                            RECEIVE_KIND::Word => Kind::Word,
                                            RECEIVE_KIND::Mean => Kind::Mean,
                                        };
                                        let ud: Phase2 = s.take_user_data().unwrap();
                                        let exam = Exam::new(
                                            Range(ud.start, ud.end),
                                            kind,
                                            Kind2::Random(num),
                                        );
                                        begin_exam(s, exam, purpose)
                                    })
                                    .fixed_width(4)
                                    .with_name("number"),
                            ),
                    ),
                );
            }
            RECEIVE_METHOD::Sequential => {
                s.add_layer(
                    Dialog::new().title("Supplement").content(
                        ListView::new()
                            .child(
                                "Start Chapter",
                                EditView::new()
                                    .on_submit(|s, txt| {
                                        let ud: &mut Phase2 = s.user_data().unwrap();
                                        (*ud).start = txt.parse().unwrap();
                                        s.focus_name("end").unwrap();
                                    })
                                    .fixed_width(3)
                                    .with_name("start"),
                            )
                            .child(
                                "End Chapter",
                                EditView::new()
                                    .on_submit(move |s, txt| {
                                        let end: usize = txt.parse().unwrap();
                                        let kind = match kind {
                                            RECEIVE_KIND::Word => Kind::Word,
                                            RECEIVE_KIND::Mean => Kind::Mean,
                                        };
                                        let ud: Phase2 = s.take_user_data().unwrap();
                                        let exam = Exam::new(
                                            Range(ud.start, end),
                                            kind,
                                            Kind2::Sequential,
                                        );
                                        begin_exam(s, exam, purpose)
                                    })
                                    .fixed_width(3)
                                    .with_name("end"),
                            ),
                    ),
                );
            }
        },
        RECEIVE_RANGE::All => match phase1.method {
            RECEIVE_METHOD::Rand => {
                s.add_layer(
                    Dialog::new().title("Supplement").content(
                        ListView::new().child(
                            "Number of words",
                            EditView::new()
                                .on_submit(move |s, text| {
                                    let number: usize = text.parse().unwrap();
                                    let kind = match phase1.kind {
                                        RECEIVE_KIND::Word => Kind::Word,
                                        RECEIVE_KIND::Mean => Kind::Mean,
                                    };
                                    let exam = Exam::new(All, kind, Kind2::Random(number));
                                    begin_exam(s, exam, purpose)
                                })
                                .fixed_width(4)
                                .with_name("number"),
                        ),
                    ),
                );
            }
            RECEIVE_METHOD::Sequential => {
                let kind = match phase1.kind {
                    RECEIVE_KIND::Word => Kind::Word,
                    RECEIVE_KIND::Mean => Kind::Mean,
                };
                let exam = Exam::new(All, kind, Kind2::Sequential);
                begin_exam(s, exam, purpose)
            }
        },
    }
}

pub fn begin_exam(s: &mut Cursive, ex: Exam, purpose: RECEIVE_PURPOSE) {
    match purpose {
        RECEIVE_PURPOSE::Exam => ex.start_exam(s),
        RECEIVE_PURPOSE::Memorize => ex.start_memorize(s),
    }
}
