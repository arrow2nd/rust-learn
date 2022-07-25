// シーザー暗号（A~Z）

fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as i16;

    let convert = |c| (((c - a + shift + 26) % 26 + a) as u8) as char;
    let enc = |c| if 'A' <= c && 'Z' >= c { convert(c as i16) } else { c };

    return text.chars().map(|c| enc(c)).collect();
}

fn main() {
    let enc = encrypt("HELLO", 3);
    let dec = encrypt(&enc, -3);

    println!("{} -> {}", enc, dec);
}
