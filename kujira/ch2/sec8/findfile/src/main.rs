// ディレクトリを再帰的に検索

use std::{env, path};

fn findfile(target: &path::PathBuf, keyword: &str) {
    let files = target.read_dir().expect("存在しないパス");

    for dir_entry in files {
        let path = dir_entry.unwrap().path();

        // ディレクトリならサブディレクトリを再帰的に検索
        if path.is_dir() {
            findfile(&path, keyword);
            continue;
        }

        // ファイル名を文字列に変換
        // NOTE: to_string_lossy() ... Unicodeで表現できない文字を U+FFFD に置き換えるメソッド
        let filename = path.file_name().unwrap().to_string_lossy();

        // キーワードを含まないなら次へ
        if filename.find(keyword) == None {
            continue;
        }

        println!("{}", path.to_string_lossy());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("[Usage] findfile <path> <keyword>");
        return;
    }

    let target_dir = &args[1];
    let keyword = &args[2];

    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}
