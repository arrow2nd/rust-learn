// テキストファイル内の数値を合計して出力

use std::{env, fs, process};

fn main() {
    let arg = env::args();
    let mut total = 0.0f64;

    for (i, name) in arg.enumerate() {
        // コマンド部分を読み飛ばす
        if i == 0 {
            continue;
        }

        let text = match fs::read_to_string(name) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Err: {}", e);
                process::exit(1);
            }
        };

        for line in text.split('\n') {
            let n: f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => 0.0,
            };

            total += n;
        }
    }

    // 結果
    println!("合計 = {}", total);
}
