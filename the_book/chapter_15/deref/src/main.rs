use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y1 = &x;
    let y2 = Box::new(x); // Boxを参照のように使う
    let y3 = MyBox(x);

    assert_eq!(5, x);
    assert_eq!(5, *y1);
    assert_eq!(5, *y2);
    assert_eq!(5, *y3);

    // 暗黙的な参照外し型強制
    let m = MyBox(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
