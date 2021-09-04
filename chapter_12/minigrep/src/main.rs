use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("query = {}", query);
    println!("filename = {}", filename);

    // ファイルを開く
    let mut f = File::open(filename).expect("ファイルが見つかりません");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("ファイルの読み込み中に問題がありました");

    println!("text:\n{}", contents);
}
