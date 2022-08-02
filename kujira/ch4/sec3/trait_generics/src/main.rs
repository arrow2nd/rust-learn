// ジェネリクスにトレイトを指定 ... トレイト境界
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// 複数のトレイトを指定
// NOTE: 同時に同じ変数を2度使用するので Copy トレイトの実装が必要
fn x2<T: std::ops::Add<Output = T> + Copy>(a: T) -> T {
    a + a
}

// whereを使って記述
fn x3<T>(a: T) -> T
where
    T: std::ops::Add<Output = T> + Copy,
{
    a + a + a
}

fn main() {
    println!("{}", add(10, 10));
    println!("{}", x2(10));
    println!("{}", x3(10));
}
