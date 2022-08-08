use std::{sync::mpsc, thread, time::Duration};

// 1秒毎にメッセージを送信
fn sleep_sender(name: &str, sender: mpsc::Sender<String>) {
    for i in 1..=5 {
        let msg = format!("{}: i={}", name, i);
        sender.send(msg).unwrap();
        thread::sleep(Duration::from_millis(1000));
    }

    sender.send("quit".to_string()).unwrap();
}

fn main() {
    // チャンネルを用意
    // MEMO: スレッド間でのデータのやりとりに使用
    let (s, r) = mpsc::channel::<String>();

    // スレッド1を作成
    let sender = s.clone();
    thread::spawn(|| {
        sleep_sender("スレッド1", sender);
    });

    // スレッド2を作成
    let sender = s.clone();
    thread::spawn(|| {
        sleep_sender("スレッド2", sender);
    });

    // スレッドからメッセージを受ける
    loop {
        let buf = r.recv().unwrap();
        println!("[receive] {}", buf);

        if buf == "quit" {
            break;
        }
    }
}
