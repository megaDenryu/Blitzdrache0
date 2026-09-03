//! 見下ろし図の下書きを編集し、下書きと高さ場・材質重みを相互に導く8つの操作コマンドの型契約。
//! 下書きの編集を画面だけの状態にせず編集コマンドの枝にするのは、AIが人と同じ操作を画面なしで積めるようにし、
//! 取り消しで線の追加と生成が別々に戻るようにするためである。適用と差し戻しの実装はTS側の編集モデルが持つ。
//! 参照: `_doc/設計/見下ろし図による地形編集.md`「判断2」

use serde::{Deserialize, Serialize};

use crate::resource::chunk_coordinate::チャンク座標;
use crate::resource::coarse_cell_paint::粗マスの塗り;
use crate::resource::contour_line::等高線;

/// 等高線を追加するとは、対象チャンクの下書きへ等高線を1本足す操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 等高線を追加する {
    pub チャンク座標: チャンク座標,
    pub 等高線: 等高線,
}

/// 等高線を変更するとは、対象チャンクの下書きの添字の等高線を丸ごと差し替える操作コマンドのことである。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 等高線を変更する {
    pub チャンク座標: チャンク座標,
    pub 添字: u32,
    pub 等高線: 等高線,
}

/// 等高線を削除するとは、対象チャンクの下書きから添字の等高線を消す操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 等高線を削除する {
    pub チャンク座標: チャンク座標,
    pub 添字: u32,
}

/// 粗マスを塗るとは、対象チャンクの下書きの複数の粗マスへ高さ・層・またはその両方を一括で置く操作コマンドのことである。
/// 高さも層も無い項目はその粗マスの塗りを消す。一辺が下書きの一辺と違うときは塗りを空にしてから置く。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 粗マスを塗る {
    pub チャンク座標: チャンク座標,
    pub 粗マスの一辺の升目数: u32,
    pub 塗り一覧: Vec<粗マスの塗り>,
}

/// 等高線から高さ場を生成するとは、対象チャンクの等高線一覧を拘束にして高さ格子を丸ごと作り直す操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 等高線から高さ場を生成する {
    pub チャンク座標: チャンク座標,
}

/// 粗マスから地形を生成するとは、対象チャンクの粗マスの塗りから高さ格子と材質重み格子を作り直す操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 粗マスから地形を生成する {
    pub チャンク座標: チャンク座標,
}

/// 高さ場から等高線を導くとは、対象チャンクの高さ格子から一定間隔の等高線一覧を導いて下書きの等高線を置き換える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 高さ場から等高線を導く {
    pub チャンク座標: チャンク座標,
    pub 間隔メートル: f64,
}

/// 高さ場から粗マスを導くとは、対象チャンクの高さ格子と材質重みから粗マスの塗りを導いて下書きを置き換える操作コマンドのことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct 高さ場から粗マスを導く {
    pub チャンク座標: チャンク座標,
    pub 粗マスの一辺の升目数: u32,
}
