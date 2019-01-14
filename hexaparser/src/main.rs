use std::env;
use std::fs;

fn main(){
    //first will be the name of the binary
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_args(&args);

    let contents = fs::read_to_string(filename).expect("something went wrong");
    println!("file contents:\n {}", contents);
}

fn parse_args(args: &[String]) -> (&str, &str){
    let query: &str = &args[1];
    let filename: &str = &args[2];
    (query, filename)
}