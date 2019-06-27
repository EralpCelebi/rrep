use super::config;
use std::io::ErrorKind;

#[derive(Debug,PartialEq,Clone,Copy)]
pub struct Word {
    pub line: usize,
    pub ch  : usize
}

impl Word {
    pub fn new(line: usize, ch: usize) -> Word {
        Word {
            line,
            ch
        }
    }
}

pub fn search(conf: config::Config, data: &str) -> Result<Vec<Word>,ErrorKind> {
    let mut tokens: Vec<Word> = Vec::new();
    let mut toks: String = String::new();
    
    for (linei, line) in data.lines().enumerate() {
        toks.clear();
        for (chari, character) in line.chars().enumerate() {
            toks.push(character);
            if toks.contains(&conf.query) {
                tokens.push(Word::new(linei,(chari + 1)-&conf.query.len()));
                toks.clear();
            }
        }
    }

    if tokens.is_empty() {
        Err(ErrorKind::NotFound)
    } else {
        Ok(tokens)
    }
}