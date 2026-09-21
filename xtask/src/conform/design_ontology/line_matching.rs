//! コードだけの行の中の文字列の照合。依存を持たない純粋な関数だけを置く。受け取るのは行(または行の一覧)と探す文字列、返すのは照合の結果である。

use std::ffi::OsStr;
use std::path::{Component, Path};

/// 先頭から識別子の文字(英数字・下線・非ASCIIの文字)が続く限りを返す。
pub fn 先頭の識別子(残り: &str) -> String {
    残り.chars().take_while(|文字| 文字.is_alphanumeric() || *文字 == '_').collect()
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

/// `impl 型名 {`・`impl<T> 型名<T> {` の行か。` for ` を含む行はトレイト実装であり、含めない。
pub fn 固有のimplの宣言か(行: &str, 型名: &str) -> bool {
    let 残り = 行.trim_start().strip_prefix("impl").unwrap_or_default();
    let 残り = if 残り.starts_with('<') { 残り.find('>').map_or("", |位置| &残り[位置 + 1..]) } else { 残り };
    !行.contains(" for ") && 先頭の識別子(残り.trim_start()) == 型名
}

/// トレイトの実装の行(`impl トレイト for 型`・`impl<T> トレイト for 型<T>`)の、トレイトを書く位置(前後の空白を除いたもの)。トレイトの実装の行でなければ無い。
pub fn トレイト実装の行のトレイトの位置(行: &str) -> Option<&str> {
    let 残り = 行.trim_start().strip_prefix("impl")?;
    if 残り.chars().next().is_some_and(|文字| 文字.is_alphanumeric() || 文字 == '_') {
        return None;
    }
    let 残り = if 残り.starts_with('<') { 残り.find('>').map_or("", |位置| &残り[位置 + 1..]) } else { 残り };
    let 終わり = 残り.find(" for ")?;
    Some(残り[..終わり].trim())
}

/// トレイトの実装の行が対象にする型の名前(` for ` の後ろの識別子。`impl 表示 for 位置<T> {` なら `位置`)。トレイトの実装の行でなければ無い。
pub fn トレイト実装の行の対象の型名(行: &str) -> Option<String> {
    トレイト実装の行のトレイトの位置(行)?;
    let 位置 = 行.find(" for ")?;
    let 型名 = 先頭の識別子(行[位置 + " for ".len()..].trim_start());
    (!型名.is_empty()).then_some(型名)
}

/// トレイトの実装の行が対象にする型の表記そのもの(` for ` の後ろから本体の `{` または `where` の前まで。`impl<T, E> M結果 for std::result::Result<T, E> {` なら `std::result::Result<T, E>`)。トレイトの実装の行でなければ無い。
pub fn トレイト実装の行の対象の型の表記(行: &str) -> Option<&str> {
    トレイト実装の行のトレイトの位置(行)?;
    let 位置 = 行.find(" for ")?;
    let 残り = &行[位置 + " for ".len()..];
    let 終わり = 残り.find('{').or_else(|| 残り.find(" where ")).unwrap_or(残り.len());
    Some(残り[..終わり].trim())
}

/// その型に属する `impl` の宣言の行か。固有の `impl 型名 {` と、その型を対象にするトレイトの実装 `impl トレイト for 型名 {` の両方を数える。
pub fn 型に属するimplの宣言か(行: &str, 型名: &str) -> bool {
    固有のimplの宣言か(行, 型名) || トレイト実装の行の対象の型名(行).is_some_and(|対象| 対象 == 型名)
}
