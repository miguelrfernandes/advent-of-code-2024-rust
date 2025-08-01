use std::error::Error;
use std::fs;

pub struct Config {
    pub input_file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let input_file_path = args[1].clone();
        
        Ok(Config { input_file_path }) 
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.input_file_path)?;

    println!("{}", contents);

    Ok(())
}
