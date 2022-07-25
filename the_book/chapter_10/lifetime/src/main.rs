use std::fmt::Display;

fn main() {
    // 静的ライフタイム .. プログラム全体の期間生きる
    let _s: &'static str = "hogehoge";

    test_a();
    test_b();
}

fn test_a() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("最長の文字列は、{}です", result);
}

// ライフタイム注釈記法 ... '名前 (慣習として 'a を使うことが多い)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }
}

fn test_b() {
    let novel = String::from("Call me Ishmael. Some yaers ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i);
}

// ライフタイム省略
fn _first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// まとめ
fn _longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display, // Displayトレイトを実装する全ての型
{
    println!("アナウンス！ {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
