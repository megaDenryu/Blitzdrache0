//! 布素材の生成が課す、バイト列の長さと布粒子添字の範囲の検証。受け取るのは名前と実際の値と期待、返すのは型付きエラーである。

use super::布素材エラー;

pub(super) fn 長さを検証する(名前: &'static str, 実際: usize, 期待: usize) -> Result<(), 布素材エラー> {
    if 実際 != 期待 {
        return Err(布素材エラー::バイト列長不一致 { 名前, 期待, 実際 });
    }
    Ok(())
}

pub(super) fn 添字を検証する(名前: &'static str, 添字: u32, 粒子数: u32) -> Result<(), 布素材エラー> {
    if 添字 >= 粒子数 {
        return Err(布素材エラー::布粒子添字範囲外 { 名前, 添字, 粒子数 });
    }
    Ok(())
}

pub(super) fn 四バイトを読む(バイト列: &[u8], 開始: usize) -> [u8; 4] {
    [バイト列[開始], バイト列[開始 + 1], バイト列[開始 + 2], バイト列[開始 + 3]]
}

pub(super) fn 添字をusizeへ変換する(値: u32) -> usize {
    usize::try_from(値).unwrap_or_else(|_| panic!("添字{値}がusizeに収まらない"))
}
