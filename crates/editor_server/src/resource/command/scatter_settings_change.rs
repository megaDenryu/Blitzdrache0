//! 散布設定を変更する操作コマンドの型契約。対象チャンクの散布の設定を丸ごと差し替える。

use serde::{Deserialize, Serialize};

use crate::resource::chunk_coordinate::チャンク座標;
use crate::resource::scatter_settings::散布の設定;

/// 散布設定を変更するとは、対象チャンクの散布の設定(最小間隔・乱数の種)を
/// 新しい値へ差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 散布設定を変更する {
    pub チャンク座標: チャンク座標,
    pub 新しい設定: 散布の設定,
}
