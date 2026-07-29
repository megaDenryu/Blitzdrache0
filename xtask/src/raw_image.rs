//! ダンプされた読み戻し画像を読み込む工程。空の検収が複数あり、どれも同じ2ファイルの対を読むため1箇所に置く。
//! 受け取るのはダンプのベース名、返すのは寸法とRGBA8のバイト列である。
//! 書き出し側は`<ベース名>.raw`(RGBA8連結)と`<ベース名>.size`(幅 高さ)の2ファイルへ分けるため、読み手も対で扱う。

use std::path::{Path, PathBuf};

pub fn 読み込む(ダンプ先: &Path) -> Result<(usize, usize, Vec<u8>), String> {
    let 寸法パス = PathBuf::from(ダンプ先).with_extension("size");
    let 寸法 = std::fs::read_to_string(&寸法パス).map_err(|誤り| format!("{}を読めない: {誤り}", 寸法パス.display()))?;
    let mut 要素 = 寸法.split_whitespace();
    let 幅 = 数値を読む(要素.next())?;
    let 高さ = 数値を読む(要素.next())?;
    let 画像パス = PathBuf::from(ダンプ先).with_extension("raw");
    let rgba8 = std::fs::read(&画像パス).map_err(|誤り| format!("{}を読めない: {誤り}", 画像パス.display()))?;
    if rgba8.len() != 幅 * 高さ * 4 {
        return Err(format!("寸法とRGBA8長が違う: {}と{}", 幅 * 高さ * 4, rgba8.len()));
    }
    Ok((幅, 高さ, rgba8))
}

fn 数値を読む(語: Option<&str>) -> Result<usize, String> {
    語.ok_or_else(|| "読み戻し画像の寸法が足りない".to_string())?
        .parse()
        .map_err(|誤り| format!("読み戻し画像の寸法を数として読めない: {誤り}"))
}
