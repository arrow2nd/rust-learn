#[derive(Debug)]
enum UsState {
    Alabama,
    _Alaska,
}

enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six = {:?}, none = {:?}", six, none);

    let some_u8_val = 0u8;
    match some_u8_val {
        1 => println!("one"),
        7 => println!("seven"),
        _ => (), // 全ての値にマッチ（ここでは何もしない）
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
