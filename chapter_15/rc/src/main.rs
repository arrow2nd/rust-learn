enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a 生成後のカウント = {}", Rc::strong_count(&a));

    let _b = Cons(3, Rc::clone(&a));
    println!("b 生成後のカウント = {}", Rc::strong_count(&a));

    {
        let _c = Cons(3, Rc::clone(&a));
        println!("c 生成後のカウント = {}", Rc::strong_count(&a));
    }

    println!("c がスコープを抜けた後のカウント= {}", Rc::strong_count(&a));
}
