//! 実装とトレイトの本体の直下(波括弧の深さ1)にある関数の署名を読み、受け手と引数が自分の型への可変参照かを答える。
//! 入れ子の関数(メソッドの本体の中の `fn`)と、本体の中の別の `impl` の関数は深さが2以上であるため拾わない。
//! 引数の並びは、名前の後ろの型引数(`<F: FnOnce(&mut Self)>`)を読み飛ばしてから探す。署名は複数行にまたがってよい(本体の文字列は行を改行で繋いでいる)。
//! 引数1つが自分の型への可変参照かの規則は `parameter_form.rs` が持つ。

use super::declaration_brackets::{最上位で開いた括弧, 最上位のカンマで分ける, 見出しの括弧の深さ};
use super::line_matching::先頭の識別子;
use super::parameter_form::関数の引数;

/// 本体の直下の関数1つの、名前と引数の並び(括弧の中の表記を最上位のカンマで分けたもの)。
pub struct 関数の署名 {
    pub 名前: String,
    引数一覧: Vec<String>,
}

/// 自己変更を問う相手。実装の対象の型の名前(全称の実装なら型引数の名前、マクロの中ならメタ変数)と、その実装が可変参照を対象にするかの組である。
pub struct 自己変更を問う対象<'a> {
    pub 型名: &'a str,
    pub 可変参照を対象にするか: bool,
}

impl 自己変更を問う対象<'_> {
    /// その関数が対象の型へ自己変更を与えるか。可変参照を対象にする実装の関数は、受け手の形によらず与える。
    pub fn 自己変更を与えるか(&self, 関数: &関数の署名) -> bool {
        self.可変参照を対象にするか || 関数.自分の型への可変参照を持つか(self.型名)
    }
}

impl 関数の署名 {
    /// 受け手か、受け手でない引数のどれかが自分の型(`Self` か型名)への可変参照か。
    fn 自分の型への可変参照を持つか(&self, 型名: &str) -> bool {
        self.引数一覧.iter().any(|引数| 関数の引数(引数).自分の型への可変参照か(型名))
    }
}

/// 本体(開く `{` から始まる文字列)の直下に宣言された関数の署名の一覧。
pub fn 本体の直下の関数の署名一覧(本体: &str) -> Vec<関数の署名> {
    let mut 深さ = 0usize;
    let mut 直前: Option<char> = None;
    let mut 署名一覧 = Vec::new();
    for (位置, 文字) in 本体.char_indices() {
        match 文字 {
            '{' => 深さ += 1,
            '}' => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    break;
                }
            }
            'f' if 深さ == 1 && 直前.is_none_or(|前| !(前.is_alphanumeric() || 前 == '_')) => 署名一覧.extend(関数の署名を読む(&本体[位置..])),
            _ => {}
        }
        直前 = Some(文字);
    }
    署名一覧
}

/// `fn 名前<型引数>(引数の並び)` で始まる表記を読む。`fn(` の関数の型(名前を持たない)と、関数でない語は無しである。
/// 名前はマクロの本体の中のメタ変数(`fn $名前(...)`)でもよい。
pub fn 関数の署名を読む(残り: &str) -> Option<関数の署名> {
    let 名前の前 = 残り.strip_prefix("fn")?;
    if !名前の前.starts_with(char::is_whitespace) {
        return None;
    }
    let 名前の表記 = 名前の前.trim_start();
    let 名前 = match 名前の表記.strip_prefix('$') {
        Some(メタ変数) => format!("${}", 先頭の識別子(メタ変数)),
        None => 先頭の識別子(名前の表記),
    };
    if 名前.trim_start_matches('$').is_empty() {
        return None;
    }
    let mut 括弧 = 見出しの括弧の深さ::default();
    let mut 開き = None;
    for (位置, 文字) in 名前の前.char_indices() {
        match 括弧.一文字読む(文字) {
            最上位で開いた括弧::引数の丸括弧 => {
                開き = Some(位置);
                break;
            }
            最上位で開いた括弧::本体の波括弧 => return None,
            最上位で開いた括弧::無し if 文字 == ';' => return None,
            最上位で開いた括弧::無し => {}
        }
    }
    let 引数の並び = 丸括弧の中身(&名前の前[開き?..])?;
    let 引数一覧 = 最上位のカンマで分ける(引数の並び).into_iter().map(str::trim).filter(|引数| !引数.is_empty()).map(str::to_string).collect();
    Some(関数の署名 { 名前, 引数一覧 })
}

// `(` で始まる表記の、対になる `)` までの内側。
fn 丸括弧の中身(表記: &str) -> Option<&str> {
    let mut 深さ = 0usize;
    for (位置, 文字) in 表記.char_indices() {
        match 文字 {
            '(' => 深さ += 1,
            ')' => {
                深さ = 深さ.checked_sub(1)?;
                if 深さ == 0 {
                    return Some(&表記[1..位置]);
                }
            }
            _ => {}
        }
    }
    None
}
