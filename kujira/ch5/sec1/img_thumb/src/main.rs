use image::{self, imageops, GenericImageView};

fn main() {
    // リサイズサイズ
    let size = 128;

    // コマンドライン引数
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: img_thumb <imageFile>");
        return;
    }

    // 入力・出力ファイルを指定
    let in_file = String::from(&args[1]);
    let out_file = format!("{}_thumb.png", in_file);
    println!("input: {}", in_file);
    println!("output: {}", out_file);

    // 画像ファイル読み込み
    let mut img = image::open(in_file).expect("画像ファイルが読み込めません");

    // サイズを取得
    // MEMO: `.dimensions()` の戻り値はタプル
    let dim = img.dimensions();

    // 正方形にクロップ
    let width = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img_croped = imageops::crop(
        &mut img,
        (dim.0 - width) / 2,
        (dim.1 - width) / 2,
        width,
        width,
    )
    .to_image();

    // リサイズ
    // MEMO: `imageops:Lanczos3` はフィルタの指定
    let img_resized = imageops::resize(&mut img_croped, size, size, imageops::Lanczos3);

    // 書出し
    img_resized.save(out_file).unwrap();
}
