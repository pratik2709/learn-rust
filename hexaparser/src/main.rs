use std::env;
use std::process;

use hexaparser;
use hexaparser::Config;
use hexaparser::run;
//
//use std::fmt::Write;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Write;

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

    println!("{}", encode_hex("foobar".as_bytes()));

    let actualPath = Path::new("output.txt");
    let mut actual_file = match File::create(&actualPath) {
        Err(why) => panic!("Something went wrong while writing file {}", why.description()),
        Ok(actual_file) => actual_file,
    };

    match actual_file.write_all(encode_hex("hello".as_bytes()).as_bytes()){
        Err(why) => panic!("cannot write {}", why.description()),
        Ok(actual_file) => println!("Written")
    }

}

pub fn encode_hex(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}


