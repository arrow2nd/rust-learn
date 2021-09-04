use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // コマンドライン引数をパース
    let args: Vec<String> = env::args().collect();
    let config = parse_config(&args);

    println!("query = {}", config.query);
    println!("filename = {}", config.filename);

    // ファイルを開く
    let mut f = File::open(config.filename).expect("ファイルが見つかりません");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("ファイルの読み込み中に問題がありました");

    println!("text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
