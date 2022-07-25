// コインの組み合わせ

fn main() {
    let expect_price = 3950;

    for c500 in 0..=10 {
        for c100 in 0..=3 {
            for c50 in 0..=10 {
                let total = 500 * c500 + 100 * c100 + 50 * c50;

                if total == expect_price {
                    println!("500円 * {}, 100円 * {}, 50円 * {} = {}", c500, c100, c50, expect_price);
                }
            }
        }
    }
}
