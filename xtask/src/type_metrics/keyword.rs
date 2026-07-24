//! 宣言行の先頭に付く可視性・実行様式の修飾子を取り除く。型名とメソッドの判定を
//! 修飾子の有無から独立させるために、行頭の正規化だけをここに閉じている。

const 修飾子一覧: [&str; 6] = ["pub(crate)", "pub(super)", "pub", "unsafe", "async", "const"];

pub fn 修飾子を取り除く(行: &str) -> &str {
    let mut 残り = 行.trim_start();
    loop {
        let 次候補 = 修飾子一覧
            .iter()
            .find_map(|修飾子| 残り.strip_prefix(*修飾子).filter(|次| 次.starts_with(char::is_whitespace)));
        match 次候補 {
            Some(次) => 残り = 次.trim_start(),
            None => return 残り,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 可視性と実行様式を続けて剥がす() {
        assert_eq!(修飾子を取り除く("    pub async unsafe fn 起動する()"), "fn 起動する()");
        assert_eq!(修飾子を取り除く("pub(crate) struct 台帳 {"), "struct 台帳 {");
    }

    #[test]
    fn 修飾子で始まる識別子は剥がさない() {
        assert_eq!(修飾子を取り除く("publish(値)"), "publish(値)");
    }
}
