//! 場所巡りの世界の現行ソースだけを残す清掃。

use std::collections::HashSet;
use std::ffi::OsString;

use blitz_engine::チャンク座標;

use super::{場所巡りの世界のソースディレクトリ, 目印のバイナリファイル名, 種を書き出すファイル名};
use crate::generation_ledger::生成台帳エラー;
use crate::source_asset_paths::目印の柱のファイル名;

impl 場所巡りの世界のソースディレクトリ {
    pub fn 生成物一式を削除する(&self) -> Result<(), 生成台帳エラー> {
        self.0.生成物一式を削除する()
    }

    pub fn 現行対象外を削除する(&self, 座標一覧: &[チャンク座標]) -> Result<(), 生成台帳エラー> {
        let mut 保持する名前 = HashSet::from([
            OsString::from(目印のバイナリファイル名),
            OsString::from(目印の柱のファイル名),
            OsString::from(種を書き出すファイル名),
        ]);
        保持する名前.extend(座標一覧.iter().map(|座標| OsString::from(Self::高さ格子の相対ファイル名(*座標))));
        self.0.指定外の直下を削除する(保持する名前)
    }
}
