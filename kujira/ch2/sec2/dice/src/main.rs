// サイコロを振る
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for i in 0..5 {
        let dice = rng.gen_range(1..=6);
        println!("{}: {}", i, dice);
    }
}
