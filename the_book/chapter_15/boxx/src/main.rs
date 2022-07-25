enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    box_test();
    list_test();
}

fn box_test() {
    let b = Box::new(5);
    println!("b = {}", b);
}

fn list_test() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
