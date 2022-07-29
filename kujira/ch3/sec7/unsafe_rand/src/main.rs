use std::time::{SystemTime, UNIX_EPOCH};

// ミュータブルなグローバル変数（unsafe）
static mut SEED: u32 = 0;

unsafe fn rand_xorshift(start: u32, end: u32) -> u32 {
    // 初期化
    if SEED == 0 {
        let epoc = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        SEED = epoc.as_millis() as u32;
    }

    // 生成
    SEED ^= SEED << 13;
    SEED ^= SEED >> 17;
    SEED ^= SEED << 5;

    return SEED % (end - start + 1) + start;
}

fn main() {
    for i in 1..=100 {
        unsafe {
            let v = rand_xorshift(1, 6);
            println!("{:>2}: {}", i, v);
        }
    }
}
