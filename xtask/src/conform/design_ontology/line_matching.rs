//! コードだけの行の中の文字列の照合。依存を持たない純粋な関数だけを置く。受け取るのは行(または行の一覧)と探す文字列、返すのは照合の結果である。

use std::ffi::OsStr;
use std::path::{Component, Path};

use super::declaration_prefix::先頭の属性を読み飛ばす;
use super::identifier_boundary::識別子の文字か;

/// 先頭から識別子の文字(英数字・下線・非ASCIIの文字)が続く限りを返す。
pub fn 先頭の識別子(残り: &str) -> String {
    残り.chars().take_while(|文字| 識別子の文字か(*文字)).collect()
}

/// `語` が行の中に、前が行頭・空白・`)` で、後ろが識別子の続きでない形で現れるか。
pub fn 語として現れるか(行: &str, 語: &str) -> bool {
    行.match_indices(語).any(|(位置, _)| {
        let 前 = 行[..位置].chars().next_back().is_none_or(|文字| 文字.is_whitespace() || 文字 == ')');
        let 後 = 先頭の識別子(&行[位置 + 語.len()..]).is_empty();
        前 && 後
    })
}

/// 開始の行から波括弧が閉じる行(終端を含む)を返す。開始の行に開き括弧が無ければ開始の行である。
pub fn 波括弧が閉じる行(行一覧: &[String], 開始: usize) -> usize {
    let mut 深さ = 0usize;
    for (添字, 行) in 行一覧.iter().enumerate().skip(開始) {
        深さ += 行.matches('{').count();
        深さ = 深さ.saturating_sub(行.matches('}').count());
        if 深さ == 0 {
            return 添字;
        }
    }
    行一覧.len().saturating_sub(1)
}

/// パスの `crates` の直下のディレクトリの名前(クレートの名前)。`crates` の下に無いパスは空である。
pub fn クレート名(パス: &Path) -> &OsStr {
    let mut 部品一覧 = パス.components();
    部品一覧.find(|部品| matches!(部品, Component::Normal(名前) if *名前 == "crates"));
    match 部品一覧.next() {
        Some(Component::Normal(名前)) => 名前,
        _ => OsStr::new(""),
    }
}

/// `impl` の予約語より後ろの表記。前に同じ行で書いた属性(`#[allow(x)] impl`)と `unsafe` があれば読み飛ばす。`impl` の見出しの行でなければ無い。`impl` の見出しの読み口はすべてこの1つを共有する。
pub fn implの予約語より後ろ(行: &str) -> Option<&str> {
    let 行 = 先頭の属性を読み飛ばす(行);
    let 行 = 行.strip_prefix("unsafe").filter(|残り| 残り.starts_with(char::is_whitespace)).map_or(行, str::trim_start);
    let 残り = 行.strip_prefix("impl")?;
    (残り.starts_with(char::is_whitespace) || 残り.starts_with('<')).then_some(残り)
}

/// 先頭の型引数(`<...>`)の内側と、それより後ろの表記。先頭が `<` でなければ内側は空である。入れ子の山括弧を数え、`->` の `>` は数えない。閉じなければどちらも空である。
pub fn 先頭の型引数を分ける(表記: &str) -> (&str, &str) {
    if !表記.starts_with('<') {
        return ("", 表記);
    }
    let mut 深さ = 0usize;
    let mut 直前 = None;
    for (位置, 文字) in 表記.char_indices() {
        match 文字 {
            '<' => 深さ += 1,
            '>' if 直前 != Some('-') => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return (&表記[1..位置], &表記[位置 + 1..]);
                }
            }
            _ => {}
        }
        直前 = Some(文字);
    }
    ("", "")
}

/// 型やトレイトのパスの最後の要素の名前(`crate::a::規則<T>` なら `規則`、`FnOnce(u8)` なら `FnOnce`)。
pub fn パスの最後の名前(表記: &str) -> &str {
    let パス = 表記.split(['<', '(']).next().unwrap_or_default();
    パス.rsplit("::").next().unwrap_or_default().trim()
}

/// 可変参照の型(`&mut X`・`&'a mut X`)の参照先 `X` の表記。可変参照の型でなければ無い。
pub fn 可変参照の参照先(表記: &str) -> Option<&str> {
    寿命より後ろ(表記)?.strip_prefix("mut").filter(|後ろ| 後ろ.starts_with(char::is_whitespace)).map(str::trim_start)
}

/// 参照の型(`&X`・`&'a X`・`&mut X`・`&'a mut X`)の参照先 `X` の表記。参照の型でなければ無い。
pub fn 参照の参照先(表記: &str) -> Option<&str> {
    可変参照の参照先(表記).or_else(|| 寿命より後ろ(表記))
}

// 参照の型の `&` と寿命より後ろ(`mut X` か `X`)。参照の型でなければ無い。
fn 寿命より後ろ(表記: &str) -> Option<&str> {
    let 残り = 表記.trim_start().strip_prefix('&')?.trim_start();
    Some(match 残り.strip_prefix('\'') {
        Some(寿命) => 寿命.trim_start_matches(識別子の文字か).trim_start(),
        None => 残り,
    })
}
