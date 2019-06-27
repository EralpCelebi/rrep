use std::io::ErrorKind;

#[derive(Debug,Clone,PartialEq)]
pub struct Config {
    pub query: String,
    pub fname:  String
}

impl Config {
    pub fn new(query: &str, fname: &str) -> Config {
        Config {
            query: String::from(query),
            fname : String::from(fname)
        }
    }
}

pub fn parse_config(arg: &[String]) -> Result<Config,ErrorKind> {
    
    if arg.len() != 3 {
        return Err(ErrorKind::InvalidInput);
    }

    let query = arg[1].clone();
    let fname  = arg[2].clone();

    Ok(Config::new(&query, &fname))
}