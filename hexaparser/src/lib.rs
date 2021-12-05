extern crate regex;

use regex::Regex;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

include!("config.rs");

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("file contents:\n {}", contents);

    let actual_path = Path::new("output.txt");
    let mut actual_file = match File::create(&actual_path) {
        Err(why) => panic!("Something went wrong while writing file {}", why.description()),
        Ok(actual_file) => actual_file,
    };


    let line_by_line_word_reader = words_by_line(&contents);

    for words in &line_by_line_word_reader {
        for word in words {
            actual_file.write("0xA0".as_bytes())?;
            actual_file.write(encode_hex(word.as_bytes()).as_bytes())?;
            actual_file.write("".as_bytes())?;
        }
        actual_file.write("0x0A".as_bytes())?;
    }

    let re = Regex::new("[\\p{Punct}\\p{IsPunctuation}]").unwrap();
    assert!(re.is_match("!"));

    Ok(())
}

fn words_by_line<'a>(string_buffer: &'a str) -> Vec<Vec<&'a str>> {
    string_buffer.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut encoded_string = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        write!(&mut encoded_string, "{:02x}", byte);
    }
    String::from("0x") + &encoded_string
}

