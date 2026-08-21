//! 出力ルート直下の全消去と、現行対象外だけの削除。

use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use super::生成の出力ルート;
use crate::generation_ledger::生成台帳エラー;
use blitz_engine::実行時アセットの公開完了印;

impl 生成の出力ルート {
    pub(crate) fn 直下をすべて削除する(&self) -> Result<(), 生成台帳エラー> {
        self.直下を走査して削除する(|_| true)
    }

    pub(crate) fn 指定外の直下を削除する(&self, 保持する名前: &HashSet<OsString>) -> Result<(), 生成台帳エラー> {
        self.直下を走査して削除する(|名前| 名前 != 実行時アセットの公開完了印::ファイル名() && !保持する名前.contains(名前))
    }

    fn 直下を走査して削除する(&self, 削除するか: impl Fn(&OsStr) -> bool) -> Result<(), 生成台帳エラー> {
        let 一覧 = match std::fs::read_dir(&self.0) {
            Ok(一覧) => 一覧,
            Err(誤り) if 誤り.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(誤り) => {
                return Err(生成台帳エラー::出力の削除に失敗した {
                    パス: self.0.clone(),
                    事由: 誤り.to_string(),
                });
            }
        };
        for 項目 in 一覧 {
            let 項目 = 項目.map_err(|誤り| 生成台帳エラー::出力の削除に失敗した {
                パス: self.0.clone(),
                事由: 誤り.to_string(),
            })?;
            if 削除するか(&項目.file_name()) {
                Self::削除する(項目.path())?;
            }
        }
        Ok(())
    }

    fn 削除する(パス: PathBuf) -> Result<(), 生成台帳エラー> {
        let 結果 = if パス.is_dir() {
            std::fs::remove_dir_all(&パス)
        } else {
            std::fs::remove_file(&パス)
        };
        結果.map_err(|誤り| 生成台帳エラー::出力の削除に失敗した {
            パス,
            事由: 誤り.to_string(),
        })
    }
}
