fn main() {
    if_else(7);
    let_if();

    loop_while();
    loop_for();
    loop_range();
    loop_inf();
}

fn if_else(num: i32) {
    //-----------------------------
    // 条件式は bool型 のみ
    // 自動的に論理値に変換されない
    //-----------------------------
    // if文に紐付く一連のコードは「アーム」とも呼ばれる
    if num < 5 {
        println!("true!");
    } else {
        println!("false...");
    }
}

fn let_if() {
    let condition = true;
    //-----------------------------
    // 三項演算子がない！！！！！！！！！！！！！！！！
    // （ifが式なので）
    //-----------------------------
    let num = if condition { 5 } else { 6 };

    println!("num is {}", num);
}

fn loop_inf() {
    // いわゆる while (true)
    loop {
        println!("loop!");
    }
}

fn loop_while() {
    let mut num = 3;
    while num != 0 {
        println!("{}!", num);

        num = num - 1;
    }

    println!("done!!");
}

fn loop_for() {
    let a = [10, 20, 30, 40, 50];

    // for-in
    for elm in a.iter() {
        println!("value is {}", elm);
    }
}

fn loop_range() {
    for num in (1..4).rev() {
        println!("{}!", num);
    }

    println!("done!!");
}
