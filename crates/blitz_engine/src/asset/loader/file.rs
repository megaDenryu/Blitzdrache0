//! glTFファイルの基準ディレクトリからの相対URI読み込み。バッファ・画像の
//! 外部ファイル参照の両方から使う共通処理。

use std::path::Path;

use crate::asset::error::アセットエラー;

/// 基準ディレクトリからの相対パスとしてバイト列を読む。
///
/// 前提: `uri`は`data:`スキームでないことを呼び出し側が確認済み
/// （data URIは未対応として個別の型付きエラーで扱う）。
pub(super) fn 外部ファイルを読む(基準ディレクトリ: &Path, uri: &str) -> Result<Vec<u8>, アセットエラー> {
    let パス = 基準ディレクトリ.join(uri);
    std::fs::read(&パス)
        .map_err(|誤り| アセットエラー::ファイル読込失敗(format!("{}: {誤り}", パス.display())))
}
