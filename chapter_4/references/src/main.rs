fn main() {
    ref_test_a();
    ref_test_b();
}

//----------------------------------

fn ref_test_a() {
    let s1 = String::from("Hello");

    // s1 の参照を渡す
    let len = calc_length(&s1);

    println!("length of '{}' is {}", s1, len);
}

fn calc_length(s: &String) -> usize {
    s.len()
    // s はスコープ外になるが、参照であり、所有権が無いので何も起きない
    // 引数に参照を取ることを「借用」と呼ぶ
}

//----------------------------------

fn ref_test_b() {
    let mut s = String::from("Hello");

    ref_change(&mut s);

    println!("{}", s);
}

fn ref_change(ms: &mut String) {
    ms.push_str(", World!");
}

//----------------------------------

fn _dangling() {
    let ref2nothing = _dangle();

    println!("{}", ref2nothing);
}

fn _dangle() -> String {
    let s = String::from("hello");

    // &s ダングリング参照になってしまうのでコンパイルできない
    s
}
