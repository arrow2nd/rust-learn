macro_rules! じいや {
    (
        可変変数 $a:ident を初期値 $v:tt で宣言いたします
    ) => {
        let mut $a = $v;
    };

    (
        $a:ident に $v:tt を加算しますの
    ) => {
        $a += $v;
    };

    (
        変数 $i:ident を $from:tt から $to:tt までの間ループいたしますわ
        $block:block
    ) => {{
        for $i in $from..=$to
            $block
    }};

    (
        $v:tt を表示してくださいまし
    ) => {
        println!("{}", $v);
    }
}

fn main() {
    じいや! {
        可変変数 total を初期値 0 で宣言いたします
    }

    じいや! {
        変数 i を 0 から 10 までの間ループいたしますわ {
            じいや! { total に i を加算しますの }
        }
    }

    じいや! {
        total を表示してくださいまし
    }
}
