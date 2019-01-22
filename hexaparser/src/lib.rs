extern crate regex;
use std::error::Error;
use std::fs;
use std::ascii;

use std::path::Path;
use std::fs::File;
use std::io::Write;

use regex::Regex;




pub fn run(config: Config) ->  Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("file contents:\n {}", contents);

    let actualPath = Path::new("output.txt");
    let mut actual_file = match File::create(&actualPath) {
        Err(why) => panic!("Something went wrong while writing file {}", why.description()),
        Ok(actual_file) => actual_file,
    };


    match actual_file.write_all(encode_hex(contents.as_bytes()).as_bytes()){
        Err(why) => panic!("cannot write {}", why.description()),
        Ok(actual_file) => println!("Written")
    }

    let re = Regex::new("[\\p{Punct}\\p{IsPunctuation}]").unwrap();
    assert!(re.is_match("A"));

    Ok(())
}

//fn words_by_line(stringBuffer:&str){
//    stringBuffer.lines().map(|line|{
//        line.split_whitespace().collect()
//    }).collect()
//}

pub fn encode_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("arguments not complete");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}