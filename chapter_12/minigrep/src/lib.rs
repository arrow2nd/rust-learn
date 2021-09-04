use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("引数の数が足りません");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ファイルを開く
    let mut f = File::open(config.filename).expect("ファイルが見つかりません");

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("text:\n{}", contents);

    Ok(())
}
