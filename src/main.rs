#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod parser;
mod exam;

use parser::word;
use exam::exam_api::{Exam, Kind, Kind2};
use exam::exam_api::Chapter::{Chap, Range, All};
use std::io::stdin;
use std::process::exit;

#[allow(non_snake_case)]
fn main() {
    let mut kind1 = String::new();
    let mut kind2 = String::new();
    let mut chap = String::new();
    let mut chap_start = String::new();
    let mut chap_end = String::new();
    let mut kind0 = String::new();

    let mut Chapter = Chap(1);
    let mut Kind_1 = Kind::Word;
    let mut Kind_2 = Kind2::Random;

    println!("Woroxide Ver 0.0.1");
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
                    Ok(_) => Chapter = Range(chap_start.parse().unwrap(), chap_end.parse().unwrap()),
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