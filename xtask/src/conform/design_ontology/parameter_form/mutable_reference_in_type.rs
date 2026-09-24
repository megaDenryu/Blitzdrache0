//! 受け手でない引数の型の表記の中に、自分の型(`Self` か型名)への可変参照が現れるかを答える規則。受け手の `self: T` が `T` のどこかに可変参照を含めば数えるのと対称にする。
//! 数えるのは、型の表記のどこかにある `&mut X`・`&'a mut X` で、`X` の中に自分の型の名前が識別子として現れるものである(`Option<&mut Self>`・`&mut [Self]`・`&mut &mut Self`・`(&mut Self, u8)`)。
//! 自分の型の名前はマクロのメタ変数(`$型`)でもよい。
//! 関数の型と `Fn` 系のトレイトの引数の丸括弧の中(`fn(&mut Self)`・`FnOnce(&mut Self)`・`impl FnMut(&mut Self)`・`Box<dyn Fn(&mut Self)>`)は数えない。その引数が可変参照を受け取る処理であって、可変参照そのものではないためである。
//! 除く範囲は、関数の型では `fn` の語から、その語を囲む括弧の中の要素の終わり(同じ深さのカンマか、囲みを閉じる括弧)までであり(戻り値の型を含む)、`Fn`・`FnMut`・`FnOnce` では語から直後の引数の丸括弧を閉じるまでである。
//! `impl` と `dyn` の後ろは除かない。`impl Iterator<Item = &'a mut Self>` と `dyn Iterator<Item = &mut Self>` は、字面に自分の型への可変参照を持つためである。

use super::super::identifier_boundary::識別子の文字か;
use super::super::line_matching::可変参照の参照先;

const 関数の型を始める語: &str = "fn";
const 引数の丸括弧を持つトレイトの名前一覧: [&str; 3] = ["Fn", "FnMut", "FnOnce"];

/// 受け手でない引数の型の表記(`Option<&mut Self>`・`&mut crate::a::規則` 等)。
#[repr(transparent)]
pub struct 引数の型の表記<'a>(pub &'a str);

impl 引数の型の表記<'_> {
    /// 関数の型と `Fn` 系のトレイトの引数の丸括弧の外に、自分の型への可変参照があるか。
    pub fn 自分の型への可変参照を含むか(&self, 型名: &str) -> bool {
        let 見る部分 = 関数の型と境界を除く(self.0);
        見る部分
            .match_indices('&')
            .any(|(位置, _)| 可変参照の参照先(&見る部分[位置..]).is_some_and(|参照先| 識別子の一覧(&参照先[..要素の終わり(参照先)]).any(|名前| 名前 == "Self" || 名前 == 型名)))
    }
}

// 除く範囲を落とした表記。
fn 関数の型と境界を除く(表記: &str) -> String {
    let mut 残した表記 = String::new();
    let mut 残り = 表記;
    while let Some((始まり, 終わり)) = 除く範囲(残り) {
        残した表記.push_str(&残り[..始まり]);
        残り = &残り[終わり..];
    }
    残した表記.push_str(残り);
    残した表記
}

// 表記の中で、除く範囲を始める語が識別子として最初に現れる位置から、除く範囲の終わりまで。
fn 除く範囲(表記: &str) -> Option<(usize, usize)> {
    let mut 位置 = 0;
    for 名前 in 識別子の一覧(表記) {
        let 開始 = 位置 + 表記[位置..].find(名前)?;
        let 語の後ろ = 開始 + 名前.len();
        if 名前 == 関数の型を始める語 {
            return Some((開始, 開始 + 要素の終わり(&表記[開始..])));
        }
        if 引数の丸括弧を持つトレイトの名前一覧.contains(&名前) {
            return Some((開始, 語の後ろ + 頭の丸括弧の群の長さ(&表記[語の後ろ..])));
        }
        位置 = 語の後ろ;
    }
    None
}

// 表記の頭(空白を読み飛ばした後)の丸括弧の群を閉じる `)` までの長さ。頭が `(` でなければ0、閉じていなければ表記の全体である。
fn 頭の丸括弧の群の長さ(表記: &str) -> usize {
    let 空白 = 表記.len() - 表記.trim_start().len();
    if !表記[空白..].starts_with('(') {
        return 0;
    }
    let mut 深さ = 0usize;
    for (位置, 文字) in 表記[空白..].char_indices() {
        match 文字 {
            '(' => 深さ += 1,
            ')' if 深さ <= 1 => return 空白 + 位置 + ')'.len_utf8(),
            ')' => 深さ -= 1,
            _ => {}
        }
    }
    表記.len()
}

/// 表記の先頭から数えて、今の要素が終わる位置。同じ深さのカンマか、囲みを閉じる括弧の手前である。`->` の `>` は数えない。関数の署名の境界が、引数の `impl Trait` と `dyn Trait` の範囲を切り出すのにも使う。
pub fn 要素の終わり(表記: &str) -> usize {
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
