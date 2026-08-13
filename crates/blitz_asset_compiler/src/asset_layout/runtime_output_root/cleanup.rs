//! 実行時形式の現行対象だけを残す清掃。

use std::collections::HashSet;
use std::ffi::OsString;

use blitz_engine::アセットID;

use super::{実行時カタログのファイル名, 実行時形式の出力ルート, 実行時目録のファイル名};
use crate::generation_ledger::生成台帳エラー;

impl 実行時形式の出力ルート {
    pub fn 生成物一式を削除する(&self) -> Result<(), 生成台帳エラー> {
        self.0.直下をすべて削除する()
    }

    pub fn 現行対象外を削除する(&self, id一覧: &[アセットID]) -> Result<(), 生成台帳エラー> {
        let mut 保持する名前 = HashSet::from([OsString::from(実行時カタログのファイル名), OsString::from(実行時目録のファイル名)]);
        保持する名前.extend(id一覧.iter().map(|id| OsString::from(Self::アセットのファイル名(id))));
        self.0.指定外の直下を削除する(&保持する名前)
    }
}
