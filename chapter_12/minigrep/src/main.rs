extern crate minigrep;

use minigrep::Config;
use std::{env, process};

fn main() {
    // コマンドライン引数をパース
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("引数の解析時に問題が発生しました: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("アプリケーションエラー: {}", e);
        process::exit(1);
    }
}
