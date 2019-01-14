use std::env;
use std::fs;

fn main(){
    //first will be the name of the binary
    let args: Vec<String> = env::args().collect();

    let config = parse_args(&args);

    println!("args are:: {} and {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename).expect("something went wrong");
    println!("file contents:\n {}", contents);
}

struct Config{
    query: String,
    filename: String
}

fn parse_args(args: &[String]) -> Config{
    let query = args[1].clone();
    let filename= args[2].clone();
    Config { query, filename }
}