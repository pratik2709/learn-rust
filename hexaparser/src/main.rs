use std::env;
use std::process;

use hexaparser;
use hexaparser::Config;
use hexaparser::run;

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



