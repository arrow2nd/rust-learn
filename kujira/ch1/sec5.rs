// for と if で3の倍数と3のつく整数に A を表示する

fn main() {
    for i in 1..51 {
        if i % 3 == 0 || i % 10 == 3 || (i >= 30 && i <= 39) {
            println!("A");
            continue;
        }

        println!("{}", i);
    }
}
