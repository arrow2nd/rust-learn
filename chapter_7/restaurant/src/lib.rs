// あらゆる要素は標準で非公開（private）
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    mod serving {
        fn _serve_order() {}
        mod test {
            fn _fix_incorrect_order() {
                // 相対パスで親モジュールにアクセス
                super::_serve_order();
            }
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("apple"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//-------------------------------------
// use でモジュールをスコープに持ち込む
// pub use で外部に名前を公開（再公開）
pub use crate::front_of_house::hosting;
//-------------------------------------

//-------------------------------------
// asキーワードでリネームする
// use std::fmt::Result;
// use std::fmt::Result as IoResult;
//-------------------------------------

//-------------------------------------
// ネストしたパスで纏めて持ち込む

// use std::cmp::Ordering;
// use std::io;
// ↓
// use std::{cmp::Ordering, io};
//-------------------------------------

//-------------------------------------
// 片方がもう片方のサブパスである場合

// use std::io;
// use std::io::Write;
// ↓
// use std::io::{self, Write};
//-------------------------------------

//-------------------------------------
// glob演算子で全ての公開要素を持ち込む
// use std::collections::*;
//-------------------------------------

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    //---------------------------------

    // 公開・非公開のフィールドを持つ構造体
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries");

    println!("I'd like {} toast please", meal.toast);

    //---------------------------------

    // enumの列挙子を全て公開
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}
