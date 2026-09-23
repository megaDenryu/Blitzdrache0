//! ソースの1行が型の別名の宣言(`type 別名<..> = 右辺;`)かを読む純粋な関数。受け取るのはコードだけの1行、返すのは別名と右辺の表記の組か、別名の宣言でないときの無しである。
//! 読む位置を限らないのは、名前の閉包(`marker_name_closure.rs`)が、関数の本体の中の `type` もマクロの本体の中の `type` も同じ辺として数えるためである。
//! 位置で絞ると、検査器が読まない位置に置いた別名が名前の閉包から漏れる。読む位置を広く取ることで漏れた辺が無くなり、増える辺は違反の側へ倒す近似になる。
//! 実装とトレイトの本体の中の関連型(`type Output = Self;`)も同じ形であるため辺になるが、右辺の名前がマーカーの名前と一致したときにだけ効き、そのときは違反の側へ倒れる。

use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::line_matching::先頭の識別子;

/// 型の別名の宣言1件。別名と、右辺の表記(末尾の `;` と前後の空白を除いたもの)の組である。
pub struct 型の別名の宣言 {
    pub 別名: String,
    pub 右辺: String,
}

/// 1行が `type 別名<..> = 右辺;` なら、その別名と右辺。別名の宣言でなければ無い。
pub fn 型の別名の宣言を読む(行: &str) -> Option<型の別名の宣言> {
    let 後ろ = 属性と可視性を読み飛ばす(行).strip_prefix("type")?;
    if !後ろ.starts_with(char::is_whitespace) {
        return None;
    }
    let 名前の表記 = 後ろ.trim_start();
    let 別名 = 先頭の識別子(名前の表記);
    let 右辺 = 名前の表記.split_once('=')?.1.split(';').next().unwrap_or_default().trim();
    (!別名.is_empty() && !右辺.is_empty()).then(|| 型の別名の宣言 { 別名, 右辺: 右辺.to_string() })
}

#[cfg(test)]
mod tests {
    use super::型の別名の宣言を読む;

    #[test]
    fn 属性と可視性を剥がして別名と右辺を読む() {
        let 読んだ組 = 型の別名の宣言を読む("pub(crate) type 短い名前<T> = crate::a::規則<T>;").map(|宣言| (宣言.別名, 宣言.右辺));
        assert_eq!(読んだ組, Some(("短い名前".to_string(), "crate::a::規則<T>".to_string())));
        assert!(型の別名の宣言を読む("typedef 何か = 別の何か;").is_none());
        assert!(型の別名の宣言を読む("type 途中まで<T>").is_none());
    }
}
