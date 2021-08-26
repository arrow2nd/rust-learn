// デバッグ出力を利用したいので明示的に指定
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!("長方形の面積は、{} 平方ピクセルです！", area(&rect1));

    // デバッグ用に全フィールドの値を表示する
    println!("{:#?}", rect1);
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
