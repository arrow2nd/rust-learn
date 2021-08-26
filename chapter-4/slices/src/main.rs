fn main() {
    let s1 = String::from("Hello World");
    // String のスライスを渡す（無くても動く気がした）
    println!("first word is {}", first_word(&s1[..]));

    let str_literal = "Hello World";
    // 文字列リテラルは「スライス」なのでそのままで OK
    println!("first word is {}", first_word(&str_literal));
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // 0 ~ i までの文字列スライスを返す
            return &s[0..i];
        }
    }

    &s[..]
}

// Golang みたい
