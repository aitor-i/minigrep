use std::{fs, result};
use std::error::Error;

pub fn run(config: Config)-> Result<(), Box<dyn Error>>{ 

    let contents = fs::read_to_string(&config.file_path)?;

    println!("With text: \n{}", &contents);

    return Ok(());
}

pub struct Config { 
    pub query: String,
    pub file_path: String
}

impl Config { 
    pub fn build (args: &[String]) ->Result<Config, &'static str> { 
        if args.len() < 3 { 
            return Err("Not enought arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})

    }
}

pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str> { 
    let mut results= Vec::new() as Vec<&'a str>;

    for line in contents.lines() { 
        if line.contains(query){ 
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
}
