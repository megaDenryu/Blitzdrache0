//! `impl` の見出しの表記を、実装の種類(固有かトレイトの実装か)と実装が対象にする型と型引数の名前へ分けて読んだ値。
//! 受け取るのは見出しの表記(`impl` から本体を開く `{` まで。前に `unsafe` があってもよい)、返すのはこの値か、`impl` の見出しでないときの無しである。
//! 実装が対象にする型の読み方は、子のモジュール `impl_syntax/target_type.rs` が持つ。

mod target_type;

use super::declaration_brackets::{最上位で開いた括弧, 最上位のカンマで分ける, 見出しの括弧の深さ};
use super::identifier_boundary::識別子として現れる位置一覧;
use super::line_matching::{implの予約語より後ろ, 先頭の型引数を分ける, 先頭の識別子};
pub use target_type::実装の対象の型;

/// 実装の種類。トレイトの実装はトレイトを書いた位置の表記(`変更`・`crate::a::変更<u8>`)を持つ。
pub enum 実装の種類 {
    固有の実装,
    トレイトの実装 { トレイトの表記: String },
}

pub struct 実装の見出しの構文 {
    pub 種類: 実装の種類,
    pub 対象: 実装の対象の型,
    型引数の名前一覧: Vec<String>, // 寿命と定数を除いた型の引数の名前(`impl<'a, T: Clone, const N: usize>` なら `T` だけ)
}

impl 実装の見出しの構文 {
    pub fn 読む(表記: &str) -> Option<Self> {
        let (型引数, 残り) = 先頭の型引数を分ける(implの予約語より後ろ(表記)?.trim_start());
        let 宣言 = 本体と境界より前(残り);
        let (種類, 対象の表記) = match 最上位のforの位置(宣言) {
            Some(位置) => (
                実装の種類::トレイトの実装 {
                    トレイトの表記: 宣言[..位置].trim().to_string(),
                },
                &宣言[位置 + "for".len()..],
            ),
            None => (実装の種類::固有の実装, 宣言),
        };
        Some(Self {
            種類,
            対象: 実装の対象の型::表記から読む(対象の表記),
            型引数の名前一覧: 型の引数の名前一覧(型引数),
        })
    }

    /// 対象の型が実装自身の型引数である全称の実装(`impl<T: 境界> トレイト for T`・`for &mut T`)か。検査器は境界を評価できず、対象の型を具体の型へ結び付けられない。
    pub fn 全称の実装か(&self) -> bool {
        self.型引数の名前一覧.iter().any(|名前| 名前 == self.対象.名前())
    }
}

// 見出しの表記から、`where` の境界と本体を開く `{` を除いた宣言の部分。型引数の中の定数式の波括弧は本体と取り違えない。
// `where` は空白の一致でなく識別子の境界で探す。`規則<T>where T: 境界` のように前に空白の無い形も境界として読むためである。
fn 本体と境界より前(残り: &str) -> &str {
    let 残り = 識別子として現れる位置一覧(残り, "where").first().map_or(残り, |位置| &残り[..*位置]);
    let mut 括弧 = 見出しの括弧の深さ::default();
    let 終わり = 残り.char_indices().find(|(_, 文字)| 括弧.一文字読む(*文字) == 最上位で開いた括弧::本体の波括弧).map_or(残り.len(), |(位置, _)| 位置);
    残り[..終わり].trim()
}

// どの括弧の中でもない位置に識別子の境界で現れる `for` の開始位置。トレイトの型引数(`変更<fn(u8) -> u8>`)の中を探さない。
// 空白の一致(` for `)で探さないのは、`for(規則)`・`for&mut 規則` を固有の実装と読み違えるためである。高階の寿命の束縛(`dyn for<'a> Fn(&'a u8)`)の `for` は除く。
fn 最上位のforの位置(宣言: &str) -> Option<usize> {
    let 現れ一覧 = 識別子として現れる位置一覧(宣言, "for");
    let mut 括弧 = 見出しの括弧の深さ::default();
    for (位置, 文字) in 宣言.char_indices() {
        if 括弧.最上位か() && 現れ一覧.contains(&位置) && !高階の寿命の束縛か(&宣言[位置 + "for".len()..]) {
            return Some(位置);
        }
        括弧.一文字読む(文字);
    }
    None
}

// `for` の後ろが高階の寿命の束縛の山括弧(`<'a>`・`<>`)か。実装の対象の型は山括弧で始まれない(関連型の射影は正規形が禁じる)ため、`<` の後ろが寿命か閉じ括弧なら束縛である。
fn 高階の寿命の束縛か(後ろ: &str) -> bool {
    後ろ.trim_start().strip_prefix('<').is_some_and(|山括弧の中| 山括弧の中.trim_start().starts_with(['\'', '>']))
}

fn 型の引数の名前一覧(型引数: &str) -> Vec<String> {
    最上位のカンマで分ける(型引数)
        .into_iter()
        .map(str::trim)
        .filter(|引数| !引数.starts_with('\'') && !引数.starts_with("const "))
        .map(先頭の識別子)
        .filter(|名前| !名前.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{実装の種類, 実装の見出しの構文};

    // 見出しを読んだトレイトの表記と対象の型の表記。固有の実装ならトレイトの表記は無い。
    fn 読んだ組(見出し: &str) -> Option<(Option<String>, String)> {
        let 構文 = 実装の見出しの構文::読む(見出し)?;
        let トレイト = match 構文.種類 {
            実装の種類::固有の実装 => None,
            実装の種類::トレイトの実装 { トレイトの表記 } => Some(トレイトの表記),
        };
        Some((トレイト, 構文.対象.表記().to_string()))
    }

    #[test]
    fn 空白を挟まないforとwhereを識別子の境界で読む() {
        assert_eq!(読んだ組("impl 初期化 for(規則) {"), Some((Some("初期化".to_string()), "規則".to_string())));
        assert_eq!(読んだ組("impl 初期化 for&mut 規則 {"), Some((Some("初期化".to_string()), "規則".to_string())));
        assert!(実装の見出しの構文::読む("impl 初期化 for&mut 規則 {").is_some_and(|構文| 構文.対象.可変参照か));
        assert_eq!(読んだ組("impl<T> 初期化 for 規則<T>where T: Clone {"), Some((Some("初期化".to_string()), "規則<T>".to_string())));
    }

    #[test]
    fn 名前の中のforと高階の寿命の束縛はトレイトの実装の区切りと読まない() {
        assert_eq!(読んだ組("impl forward {"), Some((None, "forward".to_string())));
        assert_eq!(読んだ組("impl dyn for<'a> Fn(&'a u8) {"), Some((None, "dyn for<'a> Fn(&'a u8)".to_string())));
    }
}
