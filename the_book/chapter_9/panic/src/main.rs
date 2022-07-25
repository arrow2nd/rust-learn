use std::fs::File;
use std::io::ErrorKind;

fn main() {
    test_a();
    test_b();
}

// エラーにマッチさせる
fn test_a() {
    let _f = File::open("hello.txt");

    let _f = match _f {
        Ok(file) => file,
        // ファイルが存在しないなら作成
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            // 作成に失敗
            Err(e) => {
                panic!("ファイルを作成できませんでした: {:?}", e);
            }
        },
        Err(error) => {
            panic!("ファイルを開けませんでした: {:?}", error);
        }
    };
}

// エラー時にパニックするショートカット
fn test_b() {
    // Result が Ok なら中身を返す
    // let f = File::open("hello.txt").unwrap();

    // こっちはエラーメッセージが指定できる
    let _f = File::open("hello.txt").expect("hello.txt が開けませんでした");
}
