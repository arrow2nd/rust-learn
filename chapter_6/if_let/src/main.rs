fn main() {
    let some_u8_value = Some(0u8);

    // Some(3) にマッチしたときのみ処理を行う
    match some_u8_value {
        Some(3) => println!("three!"),
        _ => (),
    }

    // if let 構文で表現
    if let Some(3) = some_u8_value {
        println!("three!")
    }
}
