use std::fs::File;
use std::io::{self, Read};

fn main() {
    let _r1 = read_username_from_file_a();
    let _r2 = read_username_from_file_b();
}

// エラーを委譲する（呼びだし元に返す）
fn read_username_from_file_a() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        // File::open のエラーを返す
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // ファイルの中身を読み出して結果を返す
    // （match は式なので return する必要はない）
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?演算子を使用して実装
fn read_username_from_file_b() -> Result<String, io::Error> {
    // ? :  Result が Ok なら Ok の中身が式から返ってきてプログラムは継続、
    //      Err なら Err の中身が return を使った時と同様に呼び出し元に返却される
    //      （戻り値がResult型の関数内でのみ使用可能）
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}
