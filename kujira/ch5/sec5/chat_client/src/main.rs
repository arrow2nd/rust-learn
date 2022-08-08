use std::io::{stdin, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let server_addr = "127.0.0.1:8888";

    // サーバに接続
    let mut socket = TcpStream::connect(server_addr).expect("サーバに接続できません");
    socket.set_nonblocking(true).expect("利用不可");
    println!("{}と接続しました", server_addr);

    // 受信用のスレッドを開始
    start_thread(socket.try_clone().unwrap());

    // ユーザ名の入力
    let user = input("ユーザ名");
    println!("ユーザ: {} / メッセージを入力", user);

    // 入力を全てサーバに送信
    loop {
        let msg = input("");
        let msg = format!("{} > {}\n", user, msg);
        let buf = msg.as_bytes();

        socket.write_all(buf).unwrap();
    }
}

// スレッドを開始してサーバからメッセージを受け取る
fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);

    thread::spawn(move || loop {
        // サーバから受信
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("[received] {}", buf.trim());
            }
        }

        thread::sleep(Duration::from_millis(100));
    });
}

// 標準入力から文字列を受け取る
fn input(msg: &str) -> String {
    if msg != "" {
        println!("{}", msg);
    }

    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("標準入力エラー");

    String::from(buf.trim())
}
