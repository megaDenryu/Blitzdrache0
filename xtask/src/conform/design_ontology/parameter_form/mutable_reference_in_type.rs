//! 受け手でない引数の型の表記の中に、自分の型(`Self` か型名)への可変参照が現れるかを答える規則。受け手の `self: T` が `T` のどこかに可変参照を含めば数えるのと対称にする。
//! 数えるのは、型の表記のどこかにある `&mut X`・`&'a mut X` で、`X` の中に自分の型の名前が識別子として現れるものである(`Option<&mut Self>`・`&mut [Self]`・`&mut &mut Self`・`(&mut Self, u8)`)。
//! 自分の型の名前はマクロのメタ変数(`$型`)でもよい。
//! 関数の型とトレイト境界の中(`fn(&mut Self)`・`FnOnce(&mut Self)`・`impl FnMut(&mut Self)`・`Box<dyn Fn(&mut Self)>`)は数えない。その引数が可変参照を受け取る処理であって、可変参照そのものではないためである。
//! 除く範囲は、`fn`・`Fn`・`FnMut`・`FnOnce`・`impl`・`dyn` の語から、その語を囲む括弧の中の要素の終わり(同じ深さのカンマか、囲みを閉じる括弧)までである。

use super::super::identifier_boundary::識別子の文字か;
use super::super::line_matching::可変参照の参照先;

const 除く範囲を始める語一覧: [&str; 6] = ["fn", "Fn", "FnMut", "FnOnce", "impl", "dyn"];

/// 受け手でない引数の型の表記(`Option<&mut Self>`・`&mut crate::a::規則` 等)。
#[repr(transparent)]
pub struct 引数の型の表記<'a>(pub &'a str);

impl 引数の型の表記<'_> {
    /// 関数の型とトレイト境界の外に、自分の型への可変参照があるか。
    pub fn 自分の型への可変参照を含むか(&self, 型名: &str) -> bool {
        let 見る部分 = 関数の型と境界を除く(self.0);
        見る部分
            .match_indices('&')
            .any(|(位置, _)| 可変参照の参照先(&見る部分[位置..]).is_some_and(|参照先| 識別子の一覧(&参照先[..要素の終わり(参照先)]).any(|名前| 名前 == "Self" || 名前 == 型名)))
    }
}

// 除く範囲を始める語から要素の終わりまでを落とした表記。
fn 関数の型と境界を除く(表記: &str) -> String {
    let mut 残した表記 = String::new();
    let mut 残り = 表記;
    while let Some(始まり) = 除く範囲の始まり(残り) {
        残した表記.push_str(&残り[..始まり]);
        let 後ろ = &残り[始まり..];
        残り = &後ろ[要素の終わり(後ろ)..];
    }
    残した表記.push_str(残り);
    残した表記
}

// 表記の中で、除く範囲を始める語が識別子として最初に現れる位置。
fn 除く範囲の始まり(表記: &str) -> Option<usize> {
    let mut 位置 = 0;
    for 名前 in 識別子の一覧(表記) {
        let 開始 = 位置 + 表記[位置..].find(名前)?;
        if 除く範囲を始める語一覧.contains(&名前) {
            return Some(開始);
        }
        位置 = 開始 + 名前.len();
    }
    None
}

// 表記の先頭から数えて、今の要素が終わる位置。同じ深さのカンマか、囲みを閉じる括弧の手前である。`->` の `>` は数えない。
fn 要素の終わり(表記: &str) -> usize {
    let mut 深さ = 0usize;
    let mut 直前 = None;
    for (位置, 文字) in 表記.char_indices() {
        match 文字 {
            '(' | '[' | '<' | '{' => 深さ += 1,
            '>' if 直前 == Some('-') => {}
            ')' | ']' | '>' | '}' if 深さ == 0 => return 位置,
            ')' | ']' | '>' | '}' => 深さ -= 1,
            ',' if 深さ == 0 => return 位置,
            _ => {}
        }
        直前 = Some(文字);
    }
    表記.len()
}

// 表記の中の識別子(英数字・下線・非ASCIIの文字の並び。マクロのメタ変数の頭の `$` を含む)を順に返す。
// `$` を区切りとして落とすと、マクロの中の実装 `impl $型` の引数 `&mut $型` の参照先が `型` になり、対象の型名 `$型` に照らせない。
fn 識別子の一覧(表記: &str) -> impl Iterator<Item = &str> {
    表記.split(|文字: char| !(識別子の文字か(文字) || 文字 == '$')).filter(|名前| !名前.is_empty())
}
