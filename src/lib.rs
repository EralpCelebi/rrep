mod config;
mod search;

#[cfg(test)] 
mod tests {
    use super::*;
    #[test]
    pub fn test_parse_config() {
        let virtual_argv: [String;3] = [String::from("rrep"), String::from("hey"), String::from("file.txt")];
        
        let test_answer = config::Config::new("hey", "file.txt");
        
        let conf = config::parse_config(&virtual_argv).unwrap();

        assert!(conf == test_answer);
    }

    #[test]
    pub fn test_search() {
        let test_conf = config::Config::new("quick", "dont_care.txt");
        let data = "The quick brown fox jumped over the fence.";

        let actual_answer = search::Word::new(0,4);

        let test_answer = &search::search(test_conf, &data).unwrap()[0];

        assert_eq!(actual_answer, *test_answer);

    }
}