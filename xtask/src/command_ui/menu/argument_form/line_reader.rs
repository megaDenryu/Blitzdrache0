//! フォームが入力の1行を受け取る口。担当するのは、行を1つ受け取るという操作の宣言と、
//! 標準入力から受け取る実装だけである。
//!
//! 口を型で切るのは、必須の引数を空で答えたときの聞き直しと、位置の引数を省いた後の飛ばしを、
//! 標準入力を使わずに試験できるようにするためである。

pub(in crate::command_ui::menu) trait 行の読み手 {
    /// 1行を受け取る。入力が閉じたときと読めなかったときは無しを返す。
    fn 一行受け取る(&mut self) -> Option<String>;
}

pub(in crate::command_ui::menu) struct 標準入力の読み手;

impl 標準入力の読み手 {
    pub(in crate::command_ui::menu) fn 生成する() -> Self {
        Self
    }
}

impl 行の読み手 for 標準入力の読み手 {
    fn 一行受け取る(&mut self) -> Option<String> {
        let mut 行 = String::new();
        match std::io::stdin().read_line(&mut 行) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(行),
        }
    }
}
