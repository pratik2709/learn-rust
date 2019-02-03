use std::env;
use std::process;

use hexaparser;
use hexaparser::Config;
use hexaparser::run;

fn main() {
    //first will be the name of the binary

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Something went seriously wrong:: {}", err);
        process::exit(1);
    });


    println!("filename to read is:: {}", config.filename);

    if let Err(e) = run(config){
        println!("App error {}", e);
        process::exit(1);
    }

}



