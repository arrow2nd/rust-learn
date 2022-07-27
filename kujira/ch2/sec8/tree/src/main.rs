// treeコマンドのクローン

use std::{env, path};

fn tree(target: &path::PathBuf, level: isize) {
    let files = target.read_dir().expect("存在しないパス");

    for entry in files {
        let path = entry.unwrap().path();

        // levelの分だけインデント
        for _ in 1..=level {
            print!("|  ");
        }

        // ファイル名を取得
        let filename = path.file_name().unwrap().to_string_lossy();

        // ディレクトリなら再帰的に表示
        if path.is_dir() {
            println!("|-- <{}>", filename);
            tree(&path, level + 1);
            continue;
        }

        println!("|-- {}", filename);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // カレントディレクトリを指定
    let mut target_dir = ".";

    // パスが指定されているなら置換
    if args.len() >= 2 {
        target_dir = &args[1];
    }

    let target = path::PathBuf::from(target_dir);

    println!("{}", target_dir);
    tree(&target, 0);
}
