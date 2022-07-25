fn _serve_order() {}

mod test {
    fn _fix_incorrect_order() {
        // 相対パスで親モジュールにアクセス
        super::_serve_order();
    }
}
