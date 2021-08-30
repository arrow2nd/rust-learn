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
