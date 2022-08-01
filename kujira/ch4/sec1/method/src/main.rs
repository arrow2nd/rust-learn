// BMI判定
struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange {
            min,
            max,
            label: label.to_string(),
        }
    }

    // 範囲内かテスト
    fn test(&self, v: f64) -> bool {
        (self.min <= v) && (self.max > v)
    }
}

// 身長と体重
struct Body {
    height_cm: f64,
    weight_kg: f64,
    name: String,
}

impl Body {
    fn new(height_cm: f64, weight_kg: f64, name: &str) -> Self {
        Body {
            height_cm,
            weight_kg,
            name: name.to_string(),
        }
    }

    // BMIを計算
    fn calc_bmi(&self) -> f64 {
        self.weight_kg / (self.height_cm / 100.0).powf(2.0)
    }

    // 肥満度を表示
    fn print_result(&self) {
        let mut result = String::from("不明");

        // BMI
        let bmi = self.calc_bmi();
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "低体重"),
            BmiRange::new(18.5, 25.0, "普通体重"),
            BmiRange::new(25.0, 30.0, "肥満1度"),
            BmiRange::new(30.0, 35.0, "肥満2度"),
            BmiRange::new(35.0, 40.0, "肥満3度"),
            BmiRange::new(40.0, 99.9, "肥満4度"),
        ];

        // 判定
        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }

        println!("{}さん : BMI = {:.1} 判定 = {}", self.name, bmi, result);
    }
}

fn main() {
    let body = Body::new(153.0, 47.0, "A");
    body.print_result();

    let body = Body::new(163.0, 55.0, "F");
    body.print_result();

    let body = Body::new(162.0, 51.0, "M");
    body.print_result();
}
