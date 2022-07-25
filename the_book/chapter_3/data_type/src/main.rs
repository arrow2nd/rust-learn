fn main() {
    scalar_int();
    scalar_float();
    calc();
    boolean();
    char();
    tuple();
    array();
}

fn scalar_int() {
    // 符号付き整数
    let int8: i8 = -2;
    // 符号なし整数
    let uint8: u8 = 2;

    // 整数リテラル
    let x = 0xff;
    let o = 0o77;
    let b = 0b1111_0000;

    println!("int8 = {}\nuint8 = {}", int8, uint8);
    println!("x = {}\no = {}\nb = {}\n", x, o, b);
}

fn scalar_float() {
    // 標準で64bit（倍精度浮動小数点数）
    // （f32とほぼ同スピードだから、だそうです）
    let float_a = 2.0;
    let float_b: f32 = 3.0;

    println!("float_a = {}\nfloat_b = {}\n", float_a, float_b);
}

fn calc() {
    let sum = 5 + 10;
    let deff = 765 - 283;
    let prod = 12 * 12;
    let quot = 23 / 7;
    let rem = 23 % 7;

    println!("5 + 10 = {}", sum);
    println!("765 - 283 = {}", deff);
    println!("12 * 12 = {}", prod);
    println!("23 / 7 = {} ... {}\n", quot, rem);
}

fn boolean() {
    let flg_a = true;
    let flg_b: bool = false;

    println!("flg_a = {}", flg_a);
    println!("flg_b = {}\n", flg_b);
}

fn char() {
    let c = 'c';
    let z: char = 'Z';

    // 「Rustのchar型は、ユニコードのスカラー値を表します」
    let crab_emoji = '🦀';

    println!("c = {}\nz = {}", c, z);
    println!("emoji = {}\n", crab_emoji);
}

fn tuple() {
    let tup = (500, 6.4, 1);

    // アクセスしたい値の番号を指定して直接アクセス
    println!("tup = ({}, {}, {})", tup.0, tup.1, tup.2);

    // パターンマッチングを使用して分解
    let (x, y, z) = tup;

    println!("(x, y, z) = ({}, {}, {})\n", x, y, z);
}

fn array() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("first = {}\nsecond = {}", first, second);
}
