fn main() {
    str_move();
    str_clone();
    int_copy();

    copy_test();
    copy_test_b();
}

//-----------------------------------

fn str_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 は s2 にムーブされたので s1 は使用できない
    // println!("s1 = {}, s2 = {}", s1, s2);
    println!("s2 = {}", s2);
}

fn str_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // ヒープ上のデータがコピーされるのでどちらも使用できる
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn int_copy() {
    let x = 5;
    let y = x;

    // i32 は既知のサイズを持っておりまた、Copy の型なのでコピーされる
    // 故に x, y 両方とも使用できる
    println!("x = {}, y = {}", x, y);
}

//-----------------------------------c

fn copy_test() {
    let str = String::from("foooo!!");

    take_ownership(str);

    // take_ownership にムーブされたので使用不可
    // println!("{}", str);

    let x = 5;

    makes_copy(x);

    // xもムーブされるものの、i32 は Copy なので使用可能
    println!("{}", x);
}

fn take_ownership(s: String) {
    println!("{}", s);
    // s がスコープを抜けると drop が呼ばれメモリが開放される
}

fn makes_copy(i: i32) {
    println!("{}", i);
    // i はスコープを抜けても特に何もない
}

//-----------------------------------

fn copy_test_b() {
    // gives_ownership は戻り値を s1 にムーブする
    let s1 = gives_ownership();

    let s2 = String::from("hello!");

    // s2 は takes_and_gives_back にムーブされる
    // 戻り値は s3 にムーブされる
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}, s3 = {}", s1, s3);

    // s1, s3 はスコープを抜けるのでドロップされる
    // s2 もスコープを抜けるが、ムーブされているので何も起きない
}

fn gives_ownership() -> String {
    let str = String::from("hello!!");

    // 呼び出し元関数にムーブされる
    str
}

fn takes_and_gives_back(s: String) -> String {
    // 呼び出し元関数にムーブされる
    s
}

// 所有権がめちゃめちゃに明確になってて面白い
