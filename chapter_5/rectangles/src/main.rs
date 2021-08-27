// デバッグ出力を利用したいので明示的に指定
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// 構造体上にメソッドを定義（Goっぽい）
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 関連関数（selfを引数に取らない）
    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect::square(12);

    println!("rect1の面積は、{} 平方ピクセルです！", rect1.area());

    println!("rect1 > rect2 : {}", rect1.can_hold(&rect2));
    println!("rect1 > rect3 : {}", rect1.can_hold(&rect3));

    // デバッグ用に全フィールドの値を表示する
    // println!("{:#?}", rect1);
}
