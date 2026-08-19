//! 出力世界名: `POST /api/書き出し/ソースアセット`が受け取る、書き出し先ディレクトリの名前を表す値オブジェクト。
//!
//! blitz_asset_compilerの`世界のディレクトリ名`は`&'static str`しか受け付けないため
//! (コンパイル時に決まる固定の世界名を前提にした型であり、実行時にHTTPの要求本文から届く綴りを
//! 渡す口を持たない)、実行時の綴りを受け取るこの型を editor_server 側に置く。検証の観点は
//! blitz_asset_compilerの`世界のディレクトリ名`が課すもの(区切り・場所を指す綴り・使えない記号・
//! 制御文字・末尾の点や空白の禁止)と同じ意図であり、ディレクトリ名として安全な1語であることを守る。

use super::error::書き出しエラー;

const 既定の世界名: &str = "editor_world";
const 使えない記号: [char; 7] = [':', '*', '?', '"', '<', '>', '|'];

/// 出力世界名とは、`assets/<この名前>/`へ書き出す先のディレクトリ名のことである。
pub struct 出力世界名(String);

impl 出力世界名 {
    /// `要求本文の世界名`が`None`または空文字なら既定の"editor_world"を使う。
    pub fn 生成する(要求本文の世界名: Option<String>) -> Result<Self, 書き出しエラー> {
        let 候補 = 要求本文の世界名
            .filter(|名前| !名前.is_empty())
            .unwrap_or_else(|| 既定の世界名.to_string());
        世界名の綴りを検証する(&候補)?;
        Ok(Self(候補))
    }

    pub fn 綴り(&self) -> &str {
        &self.0
    }
}

fn 世界名の綴りを検証する(名前: &str) -> Result<(), 書き出しエラー> {
    if 名前.contains(['/', '\\']) {
        return Err(書き出しエラー::世界名が不正(format!("{名前}: 区切り文字を含む")));
    }
    if 名前 == "." || 名前 == ".." {
        return Err(書き出しエラー::世界名が不正(format!("{名前}: 場所を指す綴りである")));
    }
    if let Some(記号) = 名前.chars().find(|文字| 使えない記号.contains(文字)) {
        return Err(書き出しエラー::世界名が不正(format!("{名前}: 使えない記号{記号}を含む")));
    }
    if 名前.chars().any(char::is_control) {
        return Err(書き出しエラー::世界名が不正(format!("{名前:?}: 制御文字を含む")));
    }
    if 名前.ends_with('.') || 名前.ends_with(' ') {
        return Err(書き出しエラー::世界名が不正(format!("{名前:?}: 点か空白で終わる")));
    }
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 無指定なら既定の世界名になる() {
        assert_eq!(出力世界名::生成する(None).unwrap().綴り(), "editor_world");
    }

    #[test]
    fn 空文字も既定の世界名になる() {
        assert_eq!(出力世界名::生成する(Some(String::new())).unwrap().綴り(), "editor_world");
    }

    #[test]
    fn 通常の綴りはそのまま受理する() {
        assert_eq!(出力世界名::生成する(Some("my_world".to_string())).unwrap().綴り(), "my_world");
    }

    #[test]
    fn 区切り文字を含む綴りは拒む() {
        assert!(出力世界名::生成する(Some("a/b".to_string())).is_err());
    }

    #[test]
    fn 上の場所を指す綴りは拒む() {
        assert!(出力世界名::生成する(Some("..".to_string())).is_err());
    }
}
