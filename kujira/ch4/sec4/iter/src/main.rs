// 素数を返す8ビットイテレータ
struct PrimeIter {
    n: u8,
}

impl PrimeIter {
    fn new() -> Self {
        PrimeIter { n: 1 }
    }

    // 素数かどうか
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 {
                return false;
            }
        }

        return true;
    }
}

// イテレータを実装
impl Iterator for PrimeIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // 素数を検索して返す
        loop {
            self.n += 1;

            // 8ビットの上限を超えたら検索終了
            if self.n == std::u8::MAX {
                return None;
            }

            if self.is_prime() {
                return Some(self.n);
            }
        }
    }
}

fn main() {
    let prime_iter = PrimeIter::new();

    for n in prime_iter {
        print!("{}, ", n);
    }

    println!("");
}
