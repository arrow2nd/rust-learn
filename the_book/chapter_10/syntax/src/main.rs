// 比較演算子を使うためトレイト境界に PartialOrd を指定
// Copyトレイトを実装する型のみを使って呼び出したいので Copy も追加
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let num_list = vec![34, 50, 25, 100, 85];
    let result = largest(&num_list);
    println!("1番大きい数値は {} です！", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("1番大きい文字は {} です！", result);
}
