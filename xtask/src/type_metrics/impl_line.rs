//! implブロックの開始行から、実装対象として書かれた経路の綴りを読み取る。トレイト実装は `for` の後ろが対象。
//!
//! `crate::far::設定`のように段が書かれていれば段ごと返す。段を落として型名だけにすると、同じ名前の定義が
//! 複数あるときにどの定義のimplかを言えなくなる。段の意味を解くのは`type_path`が行う。

const 経路の終端文字: [char; 4] = ['<', '{', ' ', '('];

pub fn 実装対象の経路の綴り(行: &str) -> Option<String> {
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
    let 綴り = 経路を切り出す(対象);
    if 綴り.is_empty() { None } else { Some(綴り) }
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
                    return 文字列.get(位置 + 1..).unwrap_or_default().trim_start();
                }
            }
            _ => {}
        }
    }
    文字列
}

fn 経路を切り出す(文字列: &str) -> String {
    文字列.trim_start().chars().take_while(|文字| !経路の終端文字.contains(文字)).collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 固有implの対象を読み取る() {
        assert_eq!(実装対象の経路の綴り("impl レンダラー {").unwrap(), "レンダラー");
    }

    #[test]
    fn トレイト実装はforの後ろを対象にする() {
        assert_eq!(実装対象の経路の綴り("impl fmt::Display for 違反 {").unwrap(), "違反");
    }

    #[test]
    fn 段を書いた実装対象は段ごと読み取る() {
        assert_eq!(実装対象の経路の綴り("impl crate::far::設定 {").unwrap(), "crate::far::設定");
    }

    #[test]
    fn ジェネリクス引数を飛ばして対象を読み取る() {
        assert_eq!(実装対象の経路の綴り("impl<'a, T: Into<u32>> 台帳<T> {").unwrap(), "台帳");
    }

    #[test]
    fn implで始まる識別子は対象外() {
        assert!(実装対象の経路の綴り("implicit = 1;").is_none());
    }
}
