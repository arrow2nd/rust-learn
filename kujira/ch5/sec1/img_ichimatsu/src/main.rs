use image;

fn main() {
    // ピクセルの色
    let white = image::Rgb::<u8>([255, 255, 255]);
    let red = image::Rgb::<u8>([255, 90, 90]);

    // 1マスのサイズ
    let w = 64;

    // 市松模様を書く
    let draw = |x, y| {
        let (xi, yi) = (x / w, y / w);

        match (xi % 2, yi % 2) {
            (0, 0) | (1, 1) => white,
            (1, 0) | (0, 1) => red,
            (_, _) => panic!("error"),
        }
    };

    // 生成
    let img = image::ImageBuffer::from_fn(512, 512, draw);

    // 書出し
    img.save("ichimatsu.png").unwrap();
}
