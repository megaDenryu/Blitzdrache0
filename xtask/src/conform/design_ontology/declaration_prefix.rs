//! 宣言と引数の頭に付く属性(外側の `#[...]` と、本体の先頭に書ける内側の `#![...]`)と可視性(`pub`・`pub(crate)`・`pub(in パス)` 等)を読み飛ばす純粋な関数。
//! `impl`・`trait`・`macro_rules!`・`use`・`type` の見出しの行と関数の引数は、前に属性を同じ行で書けるため、その判定の前にこれを剥がす。
//! 属性は角括弧の対応を数えて閉じるところまでを1つとし、複数並んでもよい。行の中で閉じない属性(次の行へ続く属性)は剥がさず、属性の書き出しから返す。
//! 宣言と `use` が条件付きの属性(`#[cfg(..)]`・`#[cfg_attr(..)]`)を持つかも答える。直前の属性は、複数の行にまたがる属性と、属性と宣言の間のコメントの行を越えて読む。名前の結び付けは cfg を評価せず、条件付きの宣言と取り込みを同時に見える候補として扱うためである。

/// 先頭の属性(外側の `#[..]` と内側の `#![..]`)をすべて剥がした残り(前の空白も除く)。
pub fn 先頭の属性を読み飛ばす(表記: &str) -> &str {
    let mut 残り = 表記.trim_start();
    while let Some(属性の中) = 残り.strip_prefix("#[").or_else(|| 残り.strip_prefix("#![")) {
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

/// 添字の行の宣言か `use` が、同じ行の頭か直前に続く属性に、条件付きの属性(名前に `cfg` を含む属性)を持つか。
/// 直前に続く属性は、複数の行にまたがる1つの属性(`#[cfg(all(` から `))]` まで)も角括弧の対応を数えて1つとして読み、属性と宣言の間の空の行(コメントと `///` の行はコードだけの行では空になる)は読み飛ばす。
pub fn 条件付きの属性を持つか(行一覧: &[String], 添字: usize) -> bool {
    let 条件付きか = |属性: &str| 属性.contains("cfg");
    let 行 = 行一覧.get(添字).map_or("", String::as_str);
    let 同じ行の属性 = 行.strip_suffix(先頭の属性を読み飛ばす(行)).unwrap_or_default();
    条件付きか(同じ行の属性) || 条件付きか(&直前に続く属性の表記(行一覧, 添字))
}

/// 直前に続く属性を探して遡る、空でない行の数の上限。これより長い属性の並びは保証範囲の外である。
const 属性を遡る行の上限: usize = 16;

// 添字の行の直前に続く、属性だけから成る行の並びを空白で繋いだ表記。前の項目の終わり(`;`・`{`・`}` で終わる行)で止まり、繋いだ表記が閉じた属性だけになる最も前の行から繋ぐ。
fn 直前に続く属性の表記(行一覧: &[String], 添字: usize) -> String {
    let 前の行一覧 = 行一覧.get(..添字).unwrap_or_default();
    let mut 始まり = 添字;
    let 空でない前の行一覧 = 前の行一覧.iter().enumerate().rev().filter(|(_, 行)| !行.trim().is_empty()).take(属性を遡る行の上限);
    for (前, 前の行) in 空でない前の行一覧 {
        if 前の行.trim_end().ends_with([';', '{', '}']) {
            break;
        }
        let 繋いだ表記 = 前の行一覧.get(前..).unwrap_or_default().join(" ");
        if 先頭の属性を読み飛ばす(&繋いだ表記).trim().is_empty() {
            始まり = 前;
        } else if 前の行.trim_start().starts_with("#[") {
            break;
        }
    }
    前の行一覧.get(始まり..).unwrap_or_default().join(" ")
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
