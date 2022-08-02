// 宝箱
trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }

    fn check(&self);
    fn get_key_no(&self) -> i32;
}

// 宝石箱
struct JewelryBox {
    price: i32,
    key_no: i32,
}

impl JewelryBox {
    fn new(price: i32, key_no: i32) -> Self {
        JewelryBox { price, key_no }
    }
}

impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("宝石箱！金貨を {} 枚入手した。", self.price);
    }

    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

// 罠
struct TrapBox {
    damage: i32,
}

impl TrapBox {
    fn new(damage: i32) -> Self {
        TrapBox { damage }
    }
}

impl TreasureBox for TrapBox {
    fn open(&self, _key_no: i32) -> bool {
        true
    }

    fn check(&self) {
        println!("罠だ！{} のダメージを負った。", self.damage);
    }

    fn get_key_no(&self) -> i32 {
        0
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        eprintln!("鍵が合わず宝箱を開けることができなかった。");
        return;
    }

    tbox.check();
}

fn main() {
    let box1 = JewelryBox::new(30, 1);
    let box2 = TrapBox::new(3);
    let box3 = JewelryBox::new(50, 2);

    let my_key_no = 1;

    open_box(&box1, my_key_no);
    open_box(&box2, my_key_no);
    open_box(&box3, my_key_no);
}
