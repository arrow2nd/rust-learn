// 定数は型を指定する必要がある
const FAV_NUMBER: u32 = 123_456;

fn main() {
    var();
    shadowing();
    apply_shadowing(); // スネークケース推奨...？
}

fn var() {
    // 標準で変数は不変
    // let x = 5;

    // 指定すれば可変にもできる
    let mut x = 5;
    println!("The value of x is: {}", x);

    // 書き換えられる！
    x = 6;
    println!("The value of x is: {}", x);

    println!("My favorite number is: {}", FAV_NUMBER)
}

fn shadowing() {
    // 同じ名前の変数が宣言可能
    // 新しい変数は前に定義した変数を「覆い隠す」
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // x : 12
    println!("The value of x is : {}", x);
}

fn apply_shadowing() {
    // 型が異なっていても容認される
    let spaces = "     ";
    let spaces = spaces.len();

    // 型が異なるので怒られる
    // let mut spaces = "     ";
    // spaces = spaces.len();

    println!("len : {}", spaces)
}
