// 買い物の比較

fn main() {
    let price = 98000.0;

    let a_shipping_fee = 1200.0;
    let a_discount = 0.8;
    let b_discount = 0.9;

    println!("A社 = {} 円", price * a_discount + a_shipping_fee);
    println!("B社 = {} 円", price * b_discount);
}
