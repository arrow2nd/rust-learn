fn main() {
    vec_test_a();
    vec_test_b();
    vec_test_c();
}

fn vec_test_a() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);
    v1.push(3);

    // マクロも用意されてる
    let v2 = vec![1, 2, 3];

    println!("v1 = {:?}", v1);
    println!("v2 = {:?}", v2);
}

fn vec_test_b() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("{:?}", third);
}

fn vec_test_c() {
    let mut v = vec![100, 32, 57];

    // 参照を走査して50ずつ加算
    for i in &mut v {
        *i += 50;
    }

    // ベクタの全要素を出力
    for i in &v {
        println!("{}", i);
    }
}
