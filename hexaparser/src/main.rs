use std::env;
use std::process;

use hexaparser;
use hexaparser::Config;
use hexaparser::run;

use std::ascii::AsciiExt;
extern crate hex;
use hex::ToHex;
use std::fmt::Write;

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



    let mut s = String::new();
    "Hello world!".write_hex(&mut s).unwrap();
    println!("{}", s);
    println!("{}", encode_hex("foobar".as_bytes()));

}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}


