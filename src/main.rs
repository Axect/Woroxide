#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod exam;
mod parser;

use exam::exam_api::Chapter::{All, Chap, Range};
use exam::exam_api::{Exam, Kind, Kind2};
use std::io::stdin;
use std::process::exit;
use std::fs::File;
use std::io::{BufWriter};
use crate::parser::conv::smart_to_total_words;

fn main() {
    println!("What's your purpose?");
    println!("1. Memorize, 2. Execute exam");
    let mut purpose = String::new();
    match stdin().read_line(&mut purpose) {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    let purpose: usize = purpose.trim().parse().expect("Can't convert purpose to usize");
    match purpose {
        1 => memorize(),
        2 => execute_exam(),
        3 => write_json(),
        _ => {
            println!("Wrong argument!");
            exit(1);
        }
    }
}

fn write_json() {
    let whole = smart_to_total_words();
    let file = File::create("word/word.json").expect("Can't create file");
    let json_writer = BufWriter::new(file);
    serde_json::to_writer_pretty(json_writer, &whole).expect("hmm");
}

#[allow(non_snake_case)]
fn execute_exam() {
    let mut kind1 = String::new();
    let mut kind2 = String::new();
    let mut chap = String::new();
    let mut chap_start = String::new();
    let mut chap_end = String::new();
    let mut kind0 = String::new();

    let Chapter;
    let Kind_1;
    let Kind_2;

    println!("Woroxide Ver 0.2.0");
    println!("");
    println!("> What kinds of test do you want?");
    println!("> 1. Specific chapter\t2. Range of chapters\t3. All chapters");
    match stdin().read_line(&mut kind0) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }

    if kind0.trim() == "1" {
        println!("");
        println!("> What chapter do you want to test?");
        match stdin().read_line(&mut chap) {
            Ok(_) => Chapter = Chap(chap.trim().parse().unwrap()),
            Err(error) => {
                println!("{}", error);
                exit(1);
            }
        }
    } else if kind0.trim() == "2" {
        println!("");
        println!("> Input start chapter");
        match stdin().read_line(&mut chap_start) {
            Ok(_) => {
                println!("> Input last chapter");
                match stdin().read_line(&mut chap_end) {
                    Ok(_) => {
                        Chapter = Range(chap_start.trim().parse().unwrap(), chap_end.trim().parse().unwrap())
                    }
                    Err(error) => {
                        println!("{}", error);
                        exit(1);
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
                exit(1);
            }
        }
    } else {
        Chapter = All;
    }

    println!("");
    println!("> What do you want to test?");
    println!("> 1. Word\t2. Mean");
    match stdin().read_line(&mut kind1) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }

    if kind1.trim() == "1" {
        Kind_1 = Kind::Word;
    } else {
        Kind_1 = Kind::Mean;
    }

    println!("");
    println!("> What kinds of test format do you want?");
    println!("> 1. Random\t2. Sequential");
    match stdin().read_line(&mut kind2) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }

    if kind2.trim() == "1" {
        Kind_2 = Kind2::Random;
    } else {
        Kind_2 = Kind2::Sequential;
    }

    println!("");

    let exam = Exam::new(Chapter, Kind_1, Kind_2);
    exam.start_exam();
}

#[allow(non_snake_case)]
fn memorize() {
    let mut kind1 = String::new();
    let mut kind2 = String::new();
    let mut chap = String::new();
    let mut chap_start = String::new();
    let mut chap_end = String::new();
    let mut kind0 = String::new();

    let Chapter;
    let Kind_1;
    let Kind_2;

    println!("Woroxide Ver 0.2.0");
    println!("");
    println!("> What kinds of memorize do you want?");
    println!("> 1. Specific chapter\t2. Range of chapters\t3. All chapters");
    match stdin().read_line(&mut kind0) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }

    if kind0.trim() == "1" {
        println!("");
        println!("> What chapter do you want to memorize?");
        match stdin().read_line(&mut chap) {
            Ok(_) => Chapter = Chap(chap.trim().parse().unwrap()),
            Err(error) => {
                println!("{}", error);
                exit(1);
            }
        }
    } else if kind0.trim() == "2" {
        println!("");
        println!("> Input start chapter");
        match stdin().read_line(&mut chap_start) {
            Ok(_) => {
                println!("> Input last chapter");
                match stdin().read_line(&mut chap_end) {
                    Ok(_) => {
                        Chapter = Range(chap_start.parse().unwrap(), chap_end.parse().unwrap())
                    }
                    Err(error) => {
                        println!("{}", error);
                        exit(1);
                    }
                }
            }
            Err(error) => {
                println!("{}", error);
                exit(1);
            }
        }
    } else {
        Chapter = All;
    }

    println!("");
    println!("> What do you want to memorize?");
    println!("> 1. Word\t2. Mean");
    match stdin().read_line(&mut kind1) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }

    if kind1.trim() == "1" {
        Kind_1 = Kind::Word;
    } else {
        Kind_1 = Kind::Mean;
    }

    println!("");
    println!("> What kinds of memorize format do you want?");
    println!("> 1. Random\t2. Sequential");
    match stdin().read_line(&mut kind2) {
        Ok(_) => (),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    }

    if kind2.trim() == "1" {
        Kind_2 = Kind2::Random;
    } else {
        Kind_2 = Kind2::Sequential;
    }

    println!("");

    let exam = Exam::new(Chapter, Kind_1, Kind_2);
    exam.start_memorize();
}
