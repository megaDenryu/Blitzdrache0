//! 独自のコード進行を保存・削除する2つの操作コマンドの型契約。
//!
//! 保存が追加と差し替えを兼ねるのは、試作の独自進行の登録が同じ名前を上書きする形だからである。
//! 名前が正本の鍵であるため、保存のコマンドは進行を丸ごと運び、実在するかを問わない。

use serde::{Deserialize, Serialize};

use super::reference_resolution::コマンドの指し先の解決係;
use crate::resource::validation_error::資源検証エラー;
use crate::resource::コード進行;

/// 独自の進行を保存するとは、同じ名前の進行があれば置き換え、無ければ足す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 独自の進行を保存する {
    pub 進行: コード進行,
}

/// 独自の進行を削除するとは、名指した独自の進行を一覧から取り除く操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 独自の進行を削除する {
    pub 名前: String,
}

impl 独自の進行を保存する {
    pub(super) fn 検証する(&self) -> Result<(), 資源検証エラー> {
        self.進行.検証する()
    }
}

impl 独自の進行を削除する {
    /// 実在しない名前の削除を拒むのは、消したつもりの進行が残っていることに送り手が気づけないためである。
    pub(super) fn 検証する(&self, 解決係: &コマンドの指し先の解決係<'_>) -> Result<(), 資源検証エラー> {
        解決係.独自の進行が実在することを確かめる(&self.名前)
    }
}
