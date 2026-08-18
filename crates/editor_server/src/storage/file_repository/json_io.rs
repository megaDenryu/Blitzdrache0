//! 構造化データ(JSON)とバイナリ格子の共通の読み書き工程。`世界`・`チャンク`の両モジュールが
//! 解決済みのパスを渡して使う、ファイル保管庫内部の私有ヘルパー(呼び出し連鎖の中の独立した工程。
//! 参照: CLAUDE.md「切り出しの根拠義務」第5項)。

use std::path::Path;

use serde::{Serialize, de::DeserializeOwned};

use super::atomic_save::一時ファイル経由で書き込む;
use crate::storage::{保存要求エラー, 読み込みエラー};

pub(super) fn json構造体を読む<T: DeserializeOwned>(パス: &Path) -> Result<Option<T>, 読み込みエラー> {
    if !パス.is_file() {
        return Ok(None);
    }
    let 本文 = std::fs::read_to_string(パス)?;
    let 値 = serde_json::from_str(&本文)?;
    Ok(Some(値))
}

pub(super) fn json構造体を保存する<T: Serialize>(パス: &Path, データ: &T) -> Result<(), 保存要求エラー> {
    let 本文 = serde_json::to_string_pretty(データ)?;
    一時ファイル経由で書き込む(パス, 本文.as_bytes())?;
    Ok(())
}

pub(super) fn バイナリを読む(パス: &Path) -> Result<Option<Vec<u8>>, 読み込みエラー> {
    if !パス.is_file() {
        return Ok(None);
    }
    Ok(Some(std::fs::read(パス)?))
}

pub(super) fn バイナリを保存する(パス: &Path, バイト列: &[u8]) -> Result<(), 保存要求エラー> {
    一時ファイル経由で書き込む(パス, バイト列)?;
    Ok(())
}
