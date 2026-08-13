//! 世界ソースの置き場に共通する清掃。チャンク目録は必ず現行対象として残す。

use std::collections::HashSet;
use std::ffi::OsString;

use super::{チャンク目録ソースのファイル名, 世界のソースディレクトリ};
use crate::generation_ledger::生成台帳エラー;

impl 世界のソースディレクトリ {
    pub(in crate::asset_layout) fn 生成物一式を削除する(&self) -> Result<(), 生成台帳エラー> {
        self.0.直下をすべて削除する()
    }

    pub(in crate::asset_layout) fn 指定外の直下を削除する(
        &self,
        追加で保持する名前: HashSet<OsString>,
    ) -> Result<(), 生成台帳エラー> {
        let mut 保持する名前 = 追加で保持する名前;
        保持する名前.insert(OsString::from(チャンク目録ソースのファイル名));
        self.0.指定外の直下を削除する(&保持する名前)
    }
}
