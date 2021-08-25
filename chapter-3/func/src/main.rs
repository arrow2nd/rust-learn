fn main() {
    println!("Hello, world!");

    // スネークケースが慣習らしい
    another_func(2, 8);

    test_a();

    let val = five();
    println!("return value is {}", val);
}

fn another_func(x: i32, y: i32) {
    println!("x = {}\ny = {}\n", x, y);
}

fn test_a() {
    // Rustでは代入が代入値を返さないので、
    // x = y = 6 のようなものは実行できない。
    let _x = 5;

    let y = {
        let x = 3;
        //「式」なので行末にセミコロンを含まない。
        // セミコロンが付くと「文」になり、値を返さなくなる。
        x + 1
    };

    // y = 4
    println!("y = {}\n", y);
}

// 戻り値のある関数
fn five() -> i32 {
    // returnで返すこともできるが
    // 多くの関数は最後に書かれた式を暗黙的に返す。
    5
}
