// 英和辞書ツール

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let dic_path = "ejdic-hand/ejdict-hand-utf8.txt";
    let arg: Vec<String> = std::env::args().collect();

    if arg.len() < 2 {
        println!("[Usage] eiwa <word>");
        return;
    }

    let word = &arg[1];

    // 辞書ファイルを開く
    let fp = match File::open(dic_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Err: {}", e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(fp);

    // 1行ずつ一致するものがあるか確認
    for l in reader.lines() {
        let line = l.unwrap();

        if line.find(word) == None {
            continue;
        }

        println!("{}", line);
    }
}
