fn main() {
    def_str();
    update_str();
    access_str();
}

fn def_str() {
    let s = String::from("test text");

    let data = "test text";
    let data_str = data.to_string();

    println!("{}", s);
    println!("{}", data_str);
}

fn update_str() {
    // push_strで連結
    let mut ps = String::from("foo");
    ps.push_str("bar");
    println!("{}", ps);

    // pushで連結
    let mut p = String::from("lo");
    p.push('l');
    println!("{}", p);

    // +演算子で連結
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1はムーブされる s2は参照を使っているのでムーブされない
    println!("{}", s3);

    // format!マクロで連結
    let f1 = String::from("tic");
    let f2 = String::from("tac");
    let f3 = String::from("toe");
    let f4 = format!("{}-{}-{}", f1, f2, f3);
    println!("{}", f4);
}

fn access_str() {
    let s1 = String::from("hello");
    // 文字列への添字アクセスはできない！
    // let h = s1[0];

    for c in s1.chars() {
        println!("{}", c);
    }
}
