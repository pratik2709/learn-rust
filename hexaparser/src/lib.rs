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


    let wbyl = words_by_line(&contents);

    for ww in &wbyl{
        for w in ww{
            actual_file.write("0xA0".as_bytes())?;
            actual_file.write(encode_hex(w.as_bytes()).as_bytes())?;
            actual_file.write("".as_bytes())?;
        }
        actual_file.write("0x0A".as_bytes())?;
    }

    let re = Regex::new("[\\p{Punct}\\p{IsPunctuation}]").unwrap();
    assert!(re.is_match("!"));

    Ok(())
}

fn words_by_line<'a>(stringBuffer:&'a str) -> Vec<Vec<& 'a str>>{
    stringBuffer.lines().map(|line|{
        line.split_whitespace().collect()
    }).collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    String::from("0x") + &s
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