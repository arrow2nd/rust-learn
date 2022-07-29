use encoding_rs;
use std::fs;
use std::fs::File;
use std::io::Write;

fn save(filename: &str, text: &str) {
    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(text);
    let buf = enc.into_owned();

    // 書き出し
    let mut file = File::create(filename).expect("書出しに失敗");
    file.write(&buf[..]).expect("書込みに失敗");
}

fn load(filename: &str) -> String {
    let buf = fs::read(filename).expect("読込みに失敗");
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);

    return dec.into_owned();
}

fn main() {
    let filename = "test_sjis.txt";

    // 読込み
    save(filename, "猫も杓子も");

    // 書出し
    let s = load(filename);
    println!("{}", s);
}
