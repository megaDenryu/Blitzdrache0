//! 関数の引数1つ(引数の並びを最上位のカンマで分けた1つの表記)が、自分の型への可変参照かを答える規則。
//! 可変と判定する受け手は `&mut self`・`&'a mut self` と、`self: T`・`mut self: T` で `T` が可変参照を含むものである。値で受ける `self`・`mut self` は元の値を書き換えないため数えない。
//! 受け手でない引数は、型が `&mut Self`・`&'a mut Self`・`&mut 型名`・`&mut 型名<..>` のものを数える。関連関数が型自身のAPIとして自己変更を与えるためである。
//! 型引数の中や関数の型の中の可変参照(`F: FnOnce(&mut Self)`・`fn(&mut Self)`)は、その引数の型そのものが可変参照でないため数えない。
//! 引数の頭の属性(`#[allow(unused_mut)] &mut self`)は、判定の前に剥がす。剥がさないと受け手の形を読めず、可変の受け手を黙って通すためである。

use super::declaration_prefix::先頭の属性を読み飛ばす;
use super::line_matching::{パスの最後の名前, 参照の参照先, 可変参照の参照先};

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
            None => self.型().and_then(可変参照の参照先).map(パスの最後の名前).is_some_and(|名前| 名前 == "Self" || 名前 == 型名),
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

    // 受け手でない引数の型(最初の単独の `:` より後ろ)。パスの `::` を区切りと取り違えない。
    fn 型(&self) -> Option<&str> {
        let 引数 = self.0;
        let (位置, _) = 引数.char_indices().find(|&(位置, 文字)| 文字 == ':' && !引数[位置 + 1..].starts_with(':') && !引数[..位置].ends_with(':'))?;
        Some(引数[位置 + 1..].trim())
    }
}
