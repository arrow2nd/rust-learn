// 構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// タプル構造体
struct Color(i32, i32, i32);

fn main() {
    let user1 = build_user(String::from("test@test.com"), String::from("test"));

    // 構造体更新記法（JSのスプレット構文っぽい）
    let user2 = User {
        email: String::from("another@test.com"),
        username: String::from("another"),
        ..user1
    };

    println!("[ user2 ]");
    println!("name = {}", user2.username);
    println!("email = {}", user2.email);
    println!("sign_in_count = {}", user2.sign_in_count);
    println!("active = {}\n", user2.active);

    let black = Color(0, 0, 0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
}

// フィールド初期化省略記法
fn build_user(email: String, username: String) -> User {
    // 仮引数名とフィールド名が同じ場合、以下のように書ける
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
