//! 型の表記を、空白と改行とコメントの違いを消した字句の並びへ写した値。受け取るのは字句の木の字句の並びか、読み口が読んだ表記の文字列、返すのはこの値か、表記を字句へ分けられなかったときの無しである。
//! 見出しの中身の突き合わせ(`header_content_reconciliation.rs`)が、読み口の読んだ対象の型の表記と型の別名の右辺を、字句の木から取り出した表記と表記の全体で比べるために使う。
//! 名前だけを比べると、パスの最後の名前が両側で一致したまま表記の途中を取り違えた読み(型引数の定数式の中の `where` で表記を切る形)を見落とすためである。
//! 字句は1つの空白で区切り、括弧の群は開き括弧と中の並びと閉じ括弧を並べる。文字列と文字のリテラルは並びに入れない。読み口が読むコードだけの行は、それらを落とした後の行であるためである。数のリテラルは残す。
//! 読み口の表記を字句へ分けられないのは、表記が括弧の途中で切れているときであり、その読みは字句の木と必ず食い違う。

use std::str::FromStr;

use proc_macro2::{Delimiter, TokenStream, TokenTree};

use super::super::identifier_boundary::識別子の文字か;

/// 空白とコメントの違いを消した字句の並び。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct 正規化した字句の並び(String);

impl 正規化した字句の並び {
    /// 字句の木の字句の並びから。
    pub fn 字句から作る(字句一覧: &[TokenTree]) -> Self {
        Self(字句一覧.iter().flat_map(語の並び).collect::<Vec<_>>().join(" "))
    }

    /// 読み口が読んだ表記の文字列から。表記を字句へ分けられなければ無い。
    pub fn 表記から作る(表記: &str) -> Option<Self> {
        let 木 = TokenStream::from_str(表記).ok()?;
        Some(Self::字句から作る(&木.into_iter().collect::<Vec<_>>()))
    }

    /// 並びのどこかにマクロの呼び出し(識別子の後ろに `!` と括弧の群が続く並び)があるか。群の中の字句も同じ並びへ開いて持つため、深さによらず当たる。
    pub fn マクロの呼び出しを含むか(&self) -> bool {
        let 語一覧: Vec<&str> = self.0.split(' ').collect();
        語一覧.windows(3).any(|語| matches!(語, [名前, "!", "(" | "[" | "{"] if 名前.starts_with(識別子の文字か)))
    }

    /// 並びのどこかに可変参照の型の書き出し(`&` の後ろに寿命を挟んでよく `mut` が続く並び)があるか。群の中の字句も同じ並びへ開いて持つため、深さによらず当たる。
    /// 関数の型と `Fn` 系のトレイトの引数の丸括弧(`fn(&mut 規則)`・`FnMut(&mut 規則)`)の中は数えない。引数の位置の可変参照は、その関数の値を持つ側へ書き換える相手を与えないためである。戻り値の型(`fn() -> &'static mut 規則`)は数える。
    pub fn 関数の引数の外に可変参照を含むか(&self) -> bool {
        let 語一覧 = 関数の引数を除いた語一覧(&self.0);
        (0..語一覧.len()).any(|位置| matches!(&語一覧[位置..], ["&", "mut", ..] | ["&", "'", _, "mut", ..]))
    }

    /// 違反の説明に書く表記。
    pub fn 説明の表記(&self) -> &str {
        &self.0
    }
}

// 引数の丸括弧を持つ関数の型と `Fn` 系のトレイトの名前。
const 引数の丸括弧を持つ名前一覧: [&str; 4] = ["fn", "Fn", "FnMut", "FnOnce"];

// 並びの語のうち、引数の丸括弧を持つ名前の直後の丸括弧を、対応する閉じ括弧まで除いた語の一覧。丸括弧の語は群からしか生まれないため、開きと閉じの数は必ず釣り合う。
fn 関数の引数を除いた語一覧(並び: &str) -> Vec<&str> {
    let 語一覧: Vec<&str> = 並び.split(' ').collect();
    let mut 残した = Vec::new();
    let mut 位置 = 0;
    while let Some(語) = 語一覧.get(位置) {
        残した.push(*語);
        位置 = if 引数の丸括弧を持つ名前一覧.contains(語) && 語一覧.get(位置 + 1) == Some(&"(") {
            閉じ括弧の次(&語一覧, 位置 + 1)
        } else {
            位置 + 1
        };
    }
    残した
}

// 開きの添字の `(` に対応する `)` の次の添字。
fn 閉じ括弧の次(語一覧: &[&str], 開き: usize) -> usize {
    let mut 深さ = 0usize;
    for (位置, 語) in 語一覧.iter().enumerate().skip(開き) {
        match *語 {
            "(" => 深さ += 1,
            ")" => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return 位置 + 1;
                }
            }
            _ => {}
        }
    }
    語一覧.len()
}

// 1つの字句を、並びに書く語の列へ写す。群は開き括弧・中の語・閉じ括弧の順である。
fn 語の並び(字句: &TokenTree) -> Vec<String> {
    match 字句 {
        TokenTree::Group(群) => {
            let (開き, 閉じ) = match 群.delimiter() {
                Delimiter::Parenthesis => ("(", ")"),
                Delimiter::Bracket => ("[", "]"),
                Delimiter::Brace => ("{", "}"),
                Delimiter::None => ("", ""),
            };
            let 中: Vec<String> = 群.stream().into_iter().flat_map(|中の字句| 語の並び(&中の字句)).collect();
            [開き.to_string()].into_iter().chain(中).chain([閉じ.to_string()]).filter(|語| !語.is_empty()).collect()
        }
        TokenTree::Ident(識別子) => vec![識別子.to_string()],
        TokenTree::Punct(記号) => vec![記号.as_char().to_string()],
        TokenTree::Literal(リテラル) => {
            let 表記 = リテラル.to_string();
            if 表記.starts_with(|文字: char| 文字.is_ascii_digit()) { vec![表記] } else { Vec::new() }
        }
    }
}
