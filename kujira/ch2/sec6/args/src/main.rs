fn main() {
    let args = std::env::args();
    let mut total = 0.0;

    // コマンドライン引数を合計
    for (i, a) in args.enumerate() {
        // 先頭はコマンドなので読み飛ばす
        if i == 0 {
            continue;
        }

        let num: f64 = match a.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };

        total += num;
    }

    // 結果
    println!("合計 = {}", total);
}
