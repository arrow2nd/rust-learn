// アンケート結果の集計

use std::collections::HashMap;

const KEYS: [&str; 3] = ["A", "B", "C"];
const DATA: &str = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C,D";

fn main() {
    let mut aggregation = HashMap::new();

    for k in KEYS {
        aggregation.insert(k, 0);
    }

    // 集計
    for w in DATA.split(',') {
        if aggregation.get(w) == None {
            continue;
        }

        // NOTE: インクリメント・デクリメント演算子は無い
        aggregation.insert(w, aggregation[w] + 1);
    }

    // 結果
    for k in KEYS {
        // 2桁右寄せで表示
        println!("{}: {:>2}", k, aggregation[k]);
    }
}
