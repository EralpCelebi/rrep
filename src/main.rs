extern crate termion;

mod config;
mod filehandler;
mod search;

use termion::color;
use std::env::args;
use std::process;

fn main() {
    // ###############################
    let arg: Vec<String> = args().collect();
    let conf = config::parse_config(&arg).unwrap_or_else(|_| {
        eprintln!("Usage: rrep <querry> <file>");
        process::exit(1);
    });
    // ###############################
    let data: String = filehandler::fopen(conf.clone()).unwrap_or_else(|_| {
        eprintln!("Couldn't open spesified file.");
        process::exit(2);
    });
    // ###############################
    let tokens: Vec<search::Word> = search::search(conf.clone(), &data).unwrap_or_else(|_| {
        eprintln!("No tokens found!");
        process::exit(3);
    });
    // ###############################
    // ! OPTIMISE THIS
    let mut lines: Vec<String> = Vec::new();

    for line in data.lines() {
        lines.push(String::from(line));
    }

    for (i,line) in lines.iter_mut().enumerate() {
        for word in &tokens {
            if word.line == i {
                let a: String = format!("{}{}{}{}{}",
                &line[0..word.ch],
                color::Fg(color::Red),
                &conf.query,
                color::Fg(color::Reset),
                &line[word.ch + &conf.query.len()..]);
                (*line) = a;
            }
        }
    }

    for line in lines {
        println!("{}",line);
    }

    // ###############################
}
