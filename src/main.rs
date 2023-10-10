use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err|{ 
        eprintln!("Error on parsing arguments: {err}");
        process::exit(1); // kills the process
    });


    if let Err(e) = run(config) { 
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
