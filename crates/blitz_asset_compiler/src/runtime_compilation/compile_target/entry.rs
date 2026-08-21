//! コンパイル対象1件: 何を焼いてどこへ書き出すかを持つ型。出力パスを外から組ませないのは、拡張子の綴りが
//! 呼び出し側へ散ると、書く側と読む側で食い違ったときに「生成物が無いので焼き直す」という無言の劣化になるためである。
//!
//! 自分の生成物を書き出すのも揃っているかを答えるのもこの型である。生値のパスがこの型より外へ出ない。

use std::path::PathBuf;

use crate::実行時形式の出力ルート;
use blitz_engine::{アセットID, チャンク座標};

use super::super::chunk_ledger::台帳での扱い;
use super::super::source_kind::ソース種別;

pub(crate) struct コンパイル対象 {
    pub(crate) id: アセットID,
    pub(crate) 所有チャンク: チャンク座標,
    pub(crate) 種別: ソース種別,
    出力パス: PathBuf,
    pub(crate) 台帳での扱い: 台帳での扱い,
}

impl コンパイル対象 {
    /// 出力パスは出力ルートが安定IDから導く。呼び出し側が拡張子の綴りを組まないため、綴りの写しが散らない。
    pub(crate) fn 生成する(
        id: アセットID,
        所有チャンク: チャンク座標,
        種別: ソース種別,
        出力ルート: &実行時形式の出力ルート,
        台帳での扱い: 台帳での扱い,
    ) -> Self {
        Self {
            出力パス: 出力ルート.アセットの出力パス(&id),
            id,
            所有チャンク,
            種別,
            台帳での扱い,
        }
    }

    pub(crate) fn 生成物が揃っているか(&self) -> bool {
        self.出力パス.is_file()
    }

    /// この対象の生成物をファイルへ書く唯一の口。書いた先の名前を返すのは、カタログが実行時パスとして持つためである。
    pub(crate) fn 生成物を書き出す(&self, バイト列: &[u8]) -> Result<PathBuf, String> {
        std::fs::write(&self.出力パス, バイト列).map_err(|誤り| format!("{}を書き出せない: {誤り}", self.出力パス.display()))?;
        let ファイル名 = self.出力パス.file_name().ok_or_else(|| "生成物のファイル名が無い".to_string())?;
        println!("[compile_assets] {}: {}バイト", self.出力パス.display(), バイト列.len());
        Ok(PathBuf::from(ファイル名))
    }

    pub(crate) fn 表示の綴り(&self) -> std::path::Display<'_> {
        self.出力パス.display()
    }
}
