use image::{GenericImage, GenericImageView, Rgba};

fn main() {
    // コマンドライン引数
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: img_filter <imageFile>");
        return;
    }

    // 入出力ファイルを指定
    let in_file = args[1].clone();
    let out_file = format!("{}_out.png", in_file);
    println!("input: {}", in_file);
    println!("output: {}", out_file);

    // 画像ファイル読み込み
    let mut img = image::open(in_file).expect("画像ファイルが読み込めません");

    // 幅高さを取得
    let (w, h) = img.dimensions();

    for y in 0..h {
        for x in 0..w {
            // ピクセルデータを取得
            let p: Rgba<u8> = img.get_pixel(x, y);

            // ネガポジ反転
            let p = Rgba([255 - p[0], 255 - p[1], 255 - p[2], p[3]]);

            // 変更したピクセルを設定
            img.put_pixel(x, y, p);
        }
    }

    // 書出し
    img.save(out_file).unwrap();
}
