use std::time::{SystemTime, UNIX_EPOCH};

fn rand_init() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32
}

fn rand_xorshift(seed: &mut u32, start: u32, end: u32) -> u32 {
    // NOTE: 乱数生成毎にシードを更新する必要があるので、seedはmutable
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;

    return *seed % (end - start + 1) + start;
}

fn main() {
    let mut seed = rand_init();

    for i in 1..=100 {
        let v = rand_xorshift(&mut seed, 1, 6);
        println!("{:>2}: {}", i, v);
    }
}
