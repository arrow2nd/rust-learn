#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    // モジュール外の定義を利用
    use super::*;

    #[test]
    fn eq_test() {
        assert_eq!(2 * 4, 8);
    }

    #[test]
    fn ne_test2() {
        assert_ne!(2 * 3, 7);
    }

    #[test]
    fn array_int_test() {
        let a = [1, 2, 3];
        let b = [1, 2, 3];

        assert_eq!(a, b);
    }

    #[test]
    fn array_str_test() {
        let c = ["abc".to_string(), "ABC".to_string()];
        let d = [String::from("abc"), String::from("ABC")];

        assert_eq!(c, d);
    }

    #[test]
    fn vec_test() {
        let a = vec!["abc", "ABC"];
        let mut b: Vec<&str> = Vec::new();
        b.push("abc");
        b.push("ABC");

        assert_eq!(a, b);
    }

    #[test]
    fn struct_test() {
        let a = GItem {
            name: "りんご".to_string(),
            price: 200,
        };

        let b = GItem {
            name: String::from("りんご"),
            price: 200,
        };

        assert_eq!(a.name, b.name);
        assert_eq!(a.price, b.price);

        // NOTE: 構造体定義前に PartialEq 属性を指定しているので、
        // 各フィールドを自動で比較できる
        assert_eq!(a, b);
    }
}
