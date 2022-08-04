fn main() {
    for i in 1..100 {
        let s = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _ => i.to_string(),
        };

        println!("{}", s)
    }
}
