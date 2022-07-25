// 元号早見表

fn main() {
    let showa = 1926;
    let heisei = 1989;
    let reiwa = 2019;

    for year in showa..2027 {
        print!("西暦 {} 年 ... ", year);

        if year == reiwa {
            println!("令和元年");
        } else if year > reiwa {
            println!("令和 {} 年", year - reiwa + 1);
        } else if year == heisei {
            println!("平成元年");
        } else if year > heisei {
            println!("平成 {} 年", year - heisei + 1);
        } else if year == showa {
            println!("昭和元年");
        } else {
            println!("昭和 {} 年", year - showa + 1);
        }
    }
}
