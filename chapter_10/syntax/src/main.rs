fn largest<T>(list: &[T]) -> T {
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
