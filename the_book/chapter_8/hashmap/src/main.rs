use std::collections::HashMap;

fn main() {
    test_a();
    test_b();
    test_c();
    test_d();
    test_e();
    test_f();
}

// ハッシュマップの生成・挿入
fn test_a() {
    let mut scores = HashMap::new();

    // キーと値を挿入
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}

fn test_b() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    // collectメソッドを使ってハッシュマップに変換
    // 型注釈の <_, _> は型推論を利用
    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();

    println!("{:?}", scores);
}

// 所有権
fn test_c() {
    let f_name = String::from("favorite color");
    let f_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(f_name, f_value);

    // 所有権のある値はムーブされる
    // println!("{}", f_name);

    println!("{:?}", map);
}

// 値にアクセスする
fn test_d() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // forでキーと値のペアを走査
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    // Someに包まれて返ってくる
    println!("{:?}", score);
}

// 値を更新する
fn test_e() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Blue: 10 は上書きされる
    println!("{:?}", scores);

    // キーに値が存在しない場合にのみ追加
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // {"Blue": 25, "Yellow": 50}
    println!("{:?}", scores);
}

// 古い値に基づいて値を更新する
fn test_f() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // 可変参照が返る
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
