pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let filename = match args.next() {
            Some(filename) => filename,
            None => {
                return Err("Did not get a file name");
            }
        };
        Ok(Config { filename })
    }
}