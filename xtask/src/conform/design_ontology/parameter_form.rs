//! 関数の引数1つ(引数の並びを最上位のカンマで分けた1つの表記)が、自分の型への可変参照かを答える規則。
//! 可変と判定する受け手は `&mut self`・`&'a mut self` と、`self: T`・`mut self: T` で `T` が可変参照を含むものである。値で受ける `self`・`mut self` は元の値を書き換えないため数えない。
//! 受け手でない引数は、型のどこかに自分の型への可変参照が現れるもの(`&mut Self`・`Option<&mut Self>`・`&mut [Self]`・`(&mut Self, u8)` 等)を数える。関連関数が型自身のAPIとして自己変更を与えるためである。
//! 関数の型とトレイト境界の中の可変参照(`fn(&mut Self)`・`impl FnOnce(&mut Self)`)は数えない(規則は子のモジュール `parameter_form/mutable_reference_in_type.rs`)。型引数の境界(`F: FnOnce(&mut Self)`)は引数の並びの外にあるため読まない。
//! 引数の型は、最上位(丸括弧・波括弧・角括弧・山括弧の外)の単独の `:` より後ろである。構造体のパターン(`規則 { 値: x }: &mut 規則`)の中の `:` を区切りと取り違えないためである。
//! 引数の頭の属性(`#[allow(unused_mut)] &mut self`)は、判定の前に剥がす。剥がさないと受け手の形を読めず、可変の受け手を黙って通すためである。

mod mutable_reference_in_type;

use super::declaration_brackets::見出しの括弧の深さ;
use super::declaration_prefix::先頭の属性を読み飛ばす;
use super::line_matching::{参照の参照先, 可変参照の参照先};
use mutable_reference_in_type::引数の型の表記;
pub use mutable_reference_in_type::要素の終わり;

/// 関数の引数1つの表記(`&mut self`・`対象: &mut Self` 等)。
#[repr(transparent)]
pub struct 関数の引数<'a>(pub &'a str);

impl 関数の引数<'_> {
    /// この引数が自分の型(`Self` か型名)への可変参照か。頭の属性を剥がしてから、受け手なら受け手の規則、そうでなければ引数の型の規則で答える。
    pub fn 自分の型への可変参照か(&self, 型名: &str) -> bool {
        関数の引数(先頭の属性を読み飛ばす(self.0)).属性の無い引数が自分の型への可変参照か(型名)
    }

    fn 属性の無い引数が自分の型への可変参照か(&self, 型名: &str) -> bool {
        match self.受け手が可変か() {
            Some(可変か) => 可変か,
            None => self.型().is_some_and(|型| 引数の型の表記(型).自分の型への可変参照を含むか(型名)),
        }
    }

    // 受け手(`self` の形の引数)なら可変か。受け手でなければ無い。
    fn 受け手が可変か(&self) -> Option<bool> {
        let 引数 = self.0.trim();
        if 参照の参照先(引数) == Some("self") {
            return Some(可変参照の参照先(引数).is_some());
        }
        let 値 = 引数.strip_prefix("mut ").map_or(引数, str::trim_start);
        let 後ろ = 値.strip_prefix("self")?;
        if 後ろ.trim().is_empty() {
            return Some(false);
        }
        let 型 = 後ろ.trim_start().strip_prefix(':')?;
        Some(型.match_indices('&').any(|(位置, _)| 可変参照の参照先(&型[位置..]).is_some()))
    }

    // 受け手でない引数の型(最上位の単独の `:` より後ろ)。パスの `::` を区切りと取り違えない。
    fn 型(&self) -> Option<&str> {
        let 引数 = self.0;
        let mut 括弧 = 見出しの括弧の深さ::default();
        let 位置 = 引数.char_indices().find_map(|(位置, 文字)| {
            let 区切りか = 文字 == ':' && 括弧.最上位か() && !引数[位置 + 1..].starts_with(':') && !引数[..位置].ends_with(':');
            括弧.括弧として数える(文字);
            区切りか.then_some(位置)
        })?;
        Some(引数[位置 + 1..].trim())
    }
}
