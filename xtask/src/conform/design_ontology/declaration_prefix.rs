//! 宣言と引数の頭に付く外側の属性(`#[...]`)と可視性(`pub`・`pub(crate)`・`pub(in パス)` 等)を読み飛ばす純粋な関数。
//! `impl`・`trait`・`macro_rules!`・`use`・`type` の見出しの行と関数の引数は、前に属性を同じ行で書けるため、その判定の前にこれを剥がす。
//! 属性は角括弧の対応を数えて閉じるところまでを1つとし、複数並んでもよい。行の中で閉じない属性(次の行へ続く属性)は剥がさず、属性の書き出しから返す。

/// 先頭の外側の属性をすべて剥がした残り(前の空白も除く)。
pub fn 先頭の属性を読み飛ばす(表記: &str) -> &str {
    let mut 残り = 表記.trim_start();
    while let Some(属性の中) = 残り.strip_prefix("#[") {
        let Some(閉じる位置) = 属性が閉じる位置(属性の中) else {
            return 残り;
        };
        残り = 属性の中[閉じる位置 + ']'.len_utf8()..].trim_start();
    }
    残り
}

/// 先頭の外側の属性と可視性を剥がした残り。`pub` の後ろが空白でも `(` でもなければ(`pub` で始まる識別子)、可視性として剥がさない。
pub fn 属性と可視性を読み飛ばす(行: &str) -> &str {
    let 残り = 先頭の属性を読み飛ばす(行);
    let Some(後ろ) = 残り.strip_prefix("pub") else {
        return 残り;
    };
    let 後ろ = match 後ろ.strip_prefix('(') {
        Some(括弧の中) => match 括弧の中.split_once(')') {
            Some((_, 閉じた後ろ)) => 閉じた後ろ,
            None => return 残り,
        },
        None => 後ろ,
    };
    if 後ろ.starts_with(char::is_whitespace) { 後ろ.trim_start() } else { 残り }
}

// `#[` の直後から数えて、その属性を閉じる `]` のバイト位置。
fn 属性が閉じる位置(属性の中: &str) -> Option<usize> {
    let mut 深さ = 1usize;
    for (位置, 文字) in 属性の中.char_indices() {
        match 文字 {
            '[' => 深さ += 1,
            ']' => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return Some(位置);
                }
            }
            _ => {}
        }
    }
    None
}
