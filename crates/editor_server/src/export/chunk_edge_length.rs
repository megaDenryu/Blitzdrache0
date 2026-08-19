//! チャンク1つの一辺メートルの導出。世界の一辺メートル(f64)を軸あたりチャンク数で割り、
//! blitz_asset_compilerの`高さ格子諸元`が要求するf32へ狭める。標準ライブラリはf64からf32への
//! 型付き変換を持たず、`as`による縮小変換はリポジトリの規約が禁じているため、綴りの往復
//! (`crates/blitz_asset_compiler/src/loader/joint/value_read.rs`と同じ技法)で狭める。

use super::error::書き出しエラー;

pub(super) fn 求める(世界の一辺メートル: f64, 軸あたりチャンク数: u32) -> Result<f32, 書き出しエラー> {
    if 軸あたりチャンク数 == 0 {
        return Err(書き出しエラー::数値変換に失敗("軸あたりチャンク数が0である".to_string()));
    }
    let チャンク一辺メートル = 世界の一辺メートル / f64::from(軸あたりチャンク数);
    if !チャンク一辺メートル.is_finite() {
        return Err(書き出しエラー::数値変換に失敗(format!(
            "チャンク一辺メートルが有限でない: {チャンク一辺メートル}"
        )));
    }
    チャンク一辺メートル
        .to_string()
        .parse::<f32>()
        .ok()
        .filter(|値| 値.is_finite())
        .ok_or_else(|| 書き出しエラー::数値変換に失敗(format!("チャンク一辺メートルをf32へ狭められない: {チャンク一辺メートル}")))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn 一辺のメートルを軸あたりチャンク数で割る() {
        assert_eq!(求める(1024.0, 4).unwrap(), 256.0);
    }

    #[test]
    fn 軸あたりチャンク数0は拒む() {
        assert!(求める(1024.0, 0).is_err());
    }
}
