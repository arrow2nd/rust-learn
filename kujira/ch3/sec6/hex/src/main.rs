fn hex_dump(s: &str) {
    // 1バイトずつ表示
    for (i, c) in s.bytes().enumerate() {
        // アドレスを表示
        if i % 16 == 0 {
            print!("{:08x}|", i);
        }

        // 4桁ごとに区切り文字を挿入
        if i % 4 == 3 {
            print!("{:02x}|", c);
        } else {
            print!("{:02x} ", c);
        }

        // 15バイト毎に改行
        if i % 16 == 15 {
            println!("");
        }
    }

    println!("");
}

fn main() {
    hex_dump("#Shiny 空にカケラをばらまいて");
}
