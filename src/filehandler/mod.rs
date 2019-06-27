use std::fs::read_to_string;
use super::config;

pub fn fopen(conf: config::Config) -> Result<String, Box<dyn std::error::Error> > {
    let s: String = read_to_string(conf.fname)?;
    Ok(s)
}