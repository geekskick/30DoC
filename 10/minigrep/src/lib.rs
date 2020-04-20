use std::fs;
use std::error::Error;

pub fn run(args: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&args.fname)?; // ? returns the error from the function
   
    let results = if args.case_sensitive{
        search(&args.search_for, &contents)
    }else{
        isearch(&args.search_for, &contents)
    };

    for line in results{
        println!("{}", line);
    }
    Ok(())

}

pub struct Config{
    search_for : String,
    fname : String,
    case_sensitive: bool,
}

impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("Not enough args");
        }
        let search_for = &args[1];
        let fname = &args[2];
        let case_sensitive = std::env::var("CASE_SENSITIVE").is_ok();
        Ok(Config{search_for:search_for.to_string(), fname:fname.to_string(), case_sensitive: case_sensitive})
    }
}

// <'a> is a lifetime generic so anything lifetimey is shared 
fn search<'a>(query : &str, file_contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in file_contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

fn isearch<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in file_contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "DuCt";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], 
        isearch(query, contents));
    }
}