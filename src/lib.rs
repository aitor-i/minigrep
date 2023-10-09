use std::{fs, env} ;
use std::error::Error;

pub fn run(config: Config)-> Result<(), Box<dyn Error>>{ 

    let contents = fs::read_to_string(&config.file_path)?;

    let search_result = if config.ignore_case { 
        search(&config.query, &contents)
    } else { 
        search_case_sensitive(&config.query, &contents)
    };


    println!("Found: {}", search_result[0]);

    return Ok(());
}

pub struct Config { 
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool
}

impl Config { 
    pub fn build (args: &[String]) ->Result<Config, &'static str> { 
        if args.len() < 3 { 
            return Err("Not enought arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})

    }
}

pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str> { 
    let mut results= Vec::new() as Vec<&'a str>;
    let query = query.to_lowercase();

    for line in contents.lines() { 
        if line.to_lowercase().contains(&query){ 
            results.push(line.trim());
        }
    }
    results
}

pub fn search_case_sensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> { 
    let mut results= Vec::new() as Vec<&'a str>;

    for line in contents.lines() { 
        if line.contains(&query){ 
            results.push(line.trim());
        }
    }
    results
}
# [cfg(test)]
mod tests{ 

    use super::*;

    #[test]
    fn one_result(){ 
        let query = "Still";
        let contents = "\
            I show less than I have to stay tasteful
            Still catching hate cause they’re hateful
            Too much food out the gate it’s a plateful
            Now your girls full of tate and she’s grateful";

        assert_eq!(
            vec!["Still catching hate cause they’re hateful"],
            search(query, contents)
        );
    }


    #[test]
    fn case_insensitive() { 
        
        let query = "still";
        let contents = "\
            I show less than I have to stay tasteful
            Still catching hate cause they’re hateful
            Too much food out the gate it’s a plateful
            Now your girls full of tate and she’s grateful";

        assert_eq!(
            vec!["Still catching hate cause they’re hateful"],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() { 
        
        let query = "still";
        let contents = "\
            I show less than I have to stay tasteful
            Still catching hate cause they’re hateful
            Too much food out the gate it’s a plateful
            Now your girls full of tate and she’s grateful";

        assert_eq!(
            vec![] as Vec<&str>,
            search_case_sensitive(query, contents)
        );
    }
}
