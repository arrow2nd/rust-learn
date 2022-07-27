// 指定されたテキストファイルを読み込んで表示

use std::{env, fs, process};

enum ErrCode {
    Arg = 1,
    Io,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // ファイル名の指定があるか確認
    if args.len() < 2 {
        eprintln!("Err: 入力ファイルを指定してください");
        process::exit(ErrCode::Arg as i32);
    }

    let filename = &args[1];
    let result = fs::read_to_string(filename);

    // 内容を表示
    match result {
        Ok(text) => println!("{}", text),
        Err(err) => {
            eprintln!("Err: {}", err);
            process::exit(ErrCode::Io as i32);
        }
    }
}
