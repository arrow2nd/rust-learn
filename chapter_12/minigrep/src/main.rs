extern crate minigrep;

use minigrep::Config;
use std::{env, process};

fn main() {
    // コマンドライン引数をパース
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("引数の解析時に問題が発生しました: {}", err);
        process::exit(1);
    });

    println!("query = {}", config.query);
    println!("filename = {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("アプリケーションエラー: {}", e);
        process::exit(1);
    }
}
