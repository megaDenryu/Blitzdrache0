//! implブロックの開始行から、実装対象の型名を読み取る。トレイト実装は `for` の後ろが対象。

const 型名の終端文字: [char; 4] = ['<', '{', ' ', '('];

pub fn 実装対象の型名(行: &str) -> Option<String> {
    let 整形 = 行.trim();
    if 整形.starts_with("//") {
        return None;
    }
    let 残り = 整形.strip_prefix("impl")?;
    if !残り.starts_with(char::is_whitespace) && !残り.starts_with('<') {
        return None;
    }
    let 型部 = ジェネリクス引数を飛ばす(残り.trim_start());
    let 対象 = 型部.rsplit_once(" for ").map_or(型部, |(_, 後ろ)| 後ろ);
    let 型名 = 型名を切り出す(対象);
    if 型名.is_empty() { None } else { Some(型名) }
}

/// `impl<'a, T: Into<U>>` の山括弧を釣り合いで数えて読み飛ばす。
/// `Fn() -> u32` のような境界内の矢印は閉じ括弧と区別できないため、飽和減算で破綻を避ける。
fn ジェネリクス引数を飛ばす(文字列: &str) -> &str {
    if !文字列.starts_with('<') {
        return 文字列;
    }
    let mut 深さ: usize = 0;
    for (位置, 文字) in 文字列.char_indices() {
        match 文字 {
            '<' => 深さ += 1,
            '>' => {
                深さ = 深さ.saturating_sub(1);
                if 深さ == 0 {
                    return 文字列[位置 + 1..].trim_start();
                }
            }
            _ => {}
        }
    }
    文字列
}

fn 型名を切り出す(文字列: &str) -> String {
    let 先頭: String = 文字列.trim_start().chars().take_while(|文字| !型名の終端文字.contains(文字)).collect();
    match 先頭.rfind("::") {
        Some(位置) => 先頭[位置 + 2..].to_string(),
        None => 先頭,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 固有implの対象を読み取る() {
        assert_eq!(実装対象の型名("impl レンダラー {").unwrap(), "レンダラー");
    }

    #[test]
    fn トレイト実装はforの後ろを対象にする() {
        assert_eq!(実装対象の型名("impl fmt::Display for 違反 {").unwrap(), "違反");
    }

    #[test]
    fn ジェネリクス引数を飛ばして対象を読み取る() {
        assert_eq!(実装対象の型名("impl<'a, T: Into<u32>> 台帳<T> {").unwrap(), "台帳");
    }

    #[test]
    fn implで始まる識別子は対象外() {
        assert!(実装対象の型名("implicit = 1;").is_none());
    }
}
