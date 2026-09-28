//! 構造体の名前付きのフィールドの1行(`pub 名前: 型,`)から、名前と型の表記を読む。依存を持たない純粋な関数だけを置く。
//! 受け取るのはコードだけの行、返すのは名前と型の表記の組か、フィールドとして読めないという答えである。

// `pub 名前: 型,` の行から、名前と型の表記を読む。閉じ括弧の行と、名前と型の組でない行は読めないとして無い。
pub(super) fn フィールドの名前と型の表記(行: &str) -> Option<(&str, &str)> {
    let (名前, 型の表記) = 可視性の前置きを落とす(行).split_once(':')?;
    let 型の表記 = 型の表記.trim().trim_end_matches(',').trim();
    (!名前.trim().is_empty() && !型の表記.is_empty() && !型の表記.starts_with(':')).then_some((名前.trim(), 型の表記))
}

// フィールドの行の頭の可視性(`pub`・`pub(crate)`・`pub(super)`・`pub(in パス)`)を落とす。括弧の付いた可視性は、閉じ括弧までを落とす。
// 括弧の中のパス(`pub(in crate::x)` の `::`)を名前と型の区切りの `:` と読まないため、区切りを探す前に落とす。
fn 可視性の前置きを落とす(行: &str) -> &str {
    if let Some((_, 後ろ)) = 行.strip_prefix("pub(").and_then(|括弧の中から| 括弧の中から.split_once(')')) {
        return 後ろ.trim_start();
    }
    行.strip_prefix("pub ").unwrap_or(行)
}
