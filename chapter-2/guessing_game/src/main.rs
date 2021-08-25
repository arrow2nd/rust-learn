use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を当ててください！");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("{}", secret_num);

    loop {
        println!("予想する数を入力してね");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("行の読み込みに失敗");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("あなたの予想 : {}", guess);

        // 2値比較
        // （switch文...?）
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("小さすぎ..."),
            Ordering::Greater => println!("大きすぎ！！"),
            Ordering::Equal => {
                println!("正解です！ 🦀 🦀 🦀");
                break;
            }
        }
    }
}
