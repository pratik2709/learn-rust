use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    //first will be the name of the binary
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Something went seriously wrong {}", err);
        process::exit(1);
    });



    println!("args are:: {} and {}", config.query, config.filename);

    if let Err(e) = run(config){
        println!("App error {}", e);
        process::exit(1);
    }
}

fn run(config: Config) ->  Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("file contents:\n {}", contents);
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("arguments not complete");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

