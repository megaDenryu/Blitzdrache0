//! ソースの1行が型の別名の宣言(`type 別名<..> = 右辺;`)かを読む純粋な関数。受け取るのはコードだけの1行、返すのは別名と右辺の組か、別名の宣言でないか、読み切れないかである。
//! 読む位置を限らないのは、名前の閉包(`marker_name_closure.rs`)が、関数の本体の中の `type` もマクロの本体の中の `type` も同じ辺として数えるためである。
//! 位置で絞ると、検査器が読まない位置に置いた別名が名前の閉包から漏れる。読む位置を広く取ることで漏れた辺が無くなり、増える辺は違反の側へ倒す近似になる。
//! 実装とトレイトの本体の中の関連型(`type Output = Self;`)も同じ形であるため辺になるが、右辺の名前がマーカーの名前と一致したときにだけ効き、そのときは違反の側へ倒れる。
//! 右辺を持たない関連型の宣言(`type Output;`・`type 状態: M状態 + PartialEq;`)は辺を作らないため、別名の宣言でないと答える。境界の中の等式(`type A: 甲<X = Y>;`)を右辺と読み違えないよう、`=` はどの括弧の中でもない位置のものだけを探す。
//! 1行では読み切れない綴り(同じ行に `;` が無い・型引数の山括弧が同じ行で閉じない)は黙って飛ばさず、読み切れないと答える。読み口の全域性の検査(`readable_form_assertion.rs`)がそれを違反にする。

use super::declaration_brackets::見出しの括弧の深さ;
use super::declaration_prefix::属性と可視性を読み飛ばす;
use super::declaration_reading_outcome::{宣言を読んだ結末, 読めない宣言};
use super::line_matching::{先頭の型引数を分ける, 先頭の識別子};

/// 型の別名の宣言1件。別名と、右辺の表記(末尾の `;` と前後の空白を除いたもの)の組である。
pub struct 型の別名の宣言 {
    pub 別名: String,
    pub 右辺: String,
}

/// 1行が `type 別名<..> = 右辺;` なら、その別名と右辺。
pub fn 型の別名の宣言を読む(行: &str) -> 宣言を読んだ結末<型の別名の宣言> {
    let Some(後ろ) = 属性と可視性を読み飛ばす(行).strip_prefix("type").filter(|後ろ| 後ろ.starts_with(char::is_whitespace)) else {
        return 宣言を読んだ結末::その宣言でない;
    };
    let 名前の表記 = 後ろ.trim_start();
    let 別名 = 先頭の識別子(名前の表記);
    if 別名.is_empty() {
        return 読み切れない(行, "`type` の後ろに別名の識別子が無い");
    }
    let 型引数から後ろ = 名前の表記.get(別名.len()..).unwrap_or_default().trim_start();
    let (型引数, 型引数より後ろ) = 先頭の型引数を分ける(型引数から後ろ);
    if 型引数から後ろ.starts_with('<') && 型引数.is_empty() && 型引数より後ろ.is_empty() {
        return 読み切れない(行, "別名の型引数の山括弧が同じ行の中で閉じていない");
    }
    let Some((宣言, _)) = 型引数より後ろ.split_once(';') else {
        return 読み切れない(行, "`type` の宣言が同じ行の中で `;` に届かない");
    };
    match 最上位の等号より後ろ(宣言) {
        None => 宣言を読んだ結末::その宣言でない,
        Some(右辺) if 右辺.trim().is_empty() => 読み切れない(行, "`=` の右辺が空である"),
        Some(右辺) => 宣言を読んだ結末::読めた(型の別名の宣言 { 別名, 右辺: 右辺.trim().to_string() }),
    }
}

fn 読み切れない(行: &str, 理由: &'static str) -> 宣言を読んだ結末<型の別名の宣言> {
    宣言を読んだ結末::読めない(読めない宣言 { 綴り: 行.trim().to_string(), 理由 })
}

// どの括弧の中でもない位置の最初の `=` より後ろ。境界の中の等式(`甲<X = Y>`)では分けない。等号がなければ無い。
fn 最上位の等号より後ろ(宣言: &str) -> Option<&str> {
    let mut 括弧 = 見出しの括弧の深さ::default();
    for (位置, 文字) in 宣言.char_indices() {
        if 文字 == '=' && 括弧.最上位か() {
            return 宣言.get(位置 + 1..);
        }
        括弧.括弧として数える(文字);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::型の別名の宣言を読む;

    fn 読んだ組(行: &str) -> Option<(String, String)> {
        型の別名の宣言を読む(行).読めた値().map(|宣言| (宣言.別名, 宣言.右辺))
    }

    #[test]
    fn 属性と可視性と型引数の既定値を剥がして別名と右辺を読む() {
        assert_eq!(読んだ組("pub(crate) type 短い名前<T> = crate::a::規則<T>;"), Some(("短い名前".to_string(), "crate::a::規則<T>".to_string())));
        assert_eq!(読んだ組("type 既定値付き<T = u8> = crate::a::規則<T>;"), Some(("既定値付き".to_string(), "crate::a::規則<T>".to_string())));
        assert_eq!(読んだ組("type 関数の型 = fn(&mut 規則) -> u8;"), Some(("関数の型".to_string(), "fn(&mut 規則) -> u8".to_string())));
    }

    #[test]
    fn 右辺を持たない関連型の宣言と別の予約語は別名の宣言でない() {
        assert!(型の別名の宣言を読む("typedef 何か = 別の何か;").読めなかった宣言().is_none());
        assert!(読んだ組("typedef 何か = 別の何か;").is_none());
        assert!(読んだ組("type 状態: M状態 + PartialEq;").is_none());
        assert!(読んだ組("type 境界の等式: 甲<X = Y>;").is_none());
    }

    #[test]
    fn 同じ1行で読み切れない綴りは読み切れないと答える() {
        assert!(型の別名の宣言を読む("type 途中まで<T>").読めなかった宣言().is_some());
        assert!(型の別名の宣言を読む("type 閉じない<T = 規則").読めなかった宣言().is_some());
        assert!(型の別名の宣言を読む("type 空の右辺 = ;").読めなかった宣言().is_some());
    }
}
