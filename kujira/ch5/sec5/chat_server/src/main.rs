use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // サーバのアドレスとポート番号
    let server_addr = "127.0.0.1:8888";

    // スレッド間準備
    let (tx, rx) = mpsc::channel::<String>();

    // クライアント一覧
    let mut clients: Vec<TcpStream> = Vec::new();

    // サーバを起動
    let server = TcpListener::bind(server_addr).expect("サーバの起動に失敗");
    server.set_nonblocking(true).expect("利用不可");
    println!("{}でサーバを起動しました", server_addr);

    loop {
        // クライアントの接続待ち受け
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続されました: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        // スレッド間通信待ち受け
        if let Ok(msg) = rx.try_recv() {
            println!("全員に送信: {}", msg.trim());
            clients = send_all(clients, &msg);
        }

        thread::sleep(Duration::from_millis(100));
    }
}

// クライアントの受信用スレッドを開始
fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);

    thread::spawn(move || loop {
        // メッセージを待つ
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            // 受信内容をメインスレッドに送信
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }

        thread::sleep(Duration::from_millis(100));
    });
}

// 全てのクライアントに送信
fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];

    for mut socket in clients.into_iter() {
        // 文字列をバイトに変換して送信
        let bytes = String::from(s).into_bytes();

        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {}", e);
            continue;
        }

        // 所有権を回収
        collector.push(socket);
    }

    // 所有権を戻す
    collector
}
