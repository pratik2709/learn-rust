use std::env;
use std::fs;

fn main(){
    //first will be the name of the binary
    let args: Vec<String> = env::args().collect();

    let query: &str = &args[1];
    let filename: &str = &args[2];
    println!("Searching for {:?}", query);
    println!("In filename {:?}", filename);

    let contents = fs::read_to_string(filename);
    println!("file contents: {:?}", contents);
}