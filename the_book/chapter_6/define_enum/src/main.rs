// 列挙子にデータを紐づける
enum IpAddrKind {
    V4(u8, u8, u8, u8), // データ型と量は異なっても可
    V6(String),
}

fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _six = IpAddrKind::V6(String::from("::1"));

    enum_test_b();
    none_test();
}

enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // メソッドを定義
        println!("call!");
    }
}

fn enum_test_b() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

fn none_test() {
    let _x: i8 = 5;
    // nullはないけど Option<T> で値の存在・不在を表すことができる
    let _y: Option<i8> = Some(5);

    // エラー！
    // let sum = x + y;
}
