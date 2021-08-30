// あらゆる要素は標準で非公開（private）
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn sert_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
        fn cook_order() {}

        mod test {
            fn fix_incorrect_order() {
                // 相対パスで親モジュールにアクセス
                super::serve_order();
            }
        }
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apple"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス
    front_of_house::hosting::add_to_waitlist();

    //---------------------------------

    // 公開・非公開のフィールドを持つ構造体
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit = String::from("blueberries");

    println!("I'd like {} toast please", meal.toast);

    //---------------------------------

    // enumの列挙子を全て公開
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
