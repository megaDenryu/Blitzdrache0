//! `impl` の見出しの表記を、実装の種類(固有かトレイトの実装か)と実装が対象にする型と型引数の名前へ分けて読んだ値。
//! 受け取るのは見出しの表記(`impl` から本体を開く `{` まで。前に `unsafe` があってもよい)、返すのはこの値か、`impl` の見出しでないときの無しである。
//! 実装が対象にする型の読み方は、子のモジュール `impl_syntax/target_type.rs` が持つ。

mod target_type;

use super::declaration_brackets::{最上位で開いた括弧, 最上位のカンマで分ける, 見出しの括弧の深さ};
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
                &宣言[位置 + " for ".len()..],
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
fn 本体と境界より前(残り: &str) -> &str {
    let 残り = 残り.split_once(" where ").map_or(残り, |(宣言, _)| 宣言);
    let mut 括弧 = 見出しの括弧の深さ::default();
    let 終わり = 残り.char_indices().find(|(_, 文字)| 括弧.一文字読む(*文字) == 最上位で開いた括弧::本体の波括弧).map_or(残り.len(), |(位置, _)| 位置);
    残り[..終わり].trim()
}

// どの括弧の中でもない位置の ` for ` の開始位置。トレイトの型引数(`変更<fn(u8) -> u8>`)の中を探さない。
fn 最上位のforの位置(宣言: &str) -> Option<usize> {
    let mut 括弧 = 見出しの括弧の深さ::default();
    for (位置, 文字) in 宣言.char_indices() {
        if 括弧.最上位か() && 宣言[位置..].starts_with(" for ") {
            return Some(位置);
        }
        括弧.一文字読む(文字);
    }
    None
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
