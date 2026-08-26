//! 楽器と打楽器の種類の型契約。
//!
//! 合成の技法そのものは汎用の音声ライブラリが持ち、その技法を特定の音色として名づけたこの一覧は
//! エディター側にある(参照: `_doc/設計/楽曲エディター.md`「判断1」)。

use serde::{Deserialize, Serialize};

/// 楽器とは、トラックの音をブラウザが鳴らすときに選ぶ音色の名前のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum 楽器 {
    グランドピアノ,
    弦楽合奏,
    フルート,
    矩形波の主旋律,
    アコースティックギター,
    エレクトリックピアノ,
    ウッドベース,
    エレキベース,
    三角波のベース,
    生ドラム,
    矩形波と雑音のドラム,
}

/// 打楽器の種類とは、打楽器のトラックの1行が受け持つ打撃音の区別のことである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum 打楽器の種類 {
    バスドラム,
    スネア,
    ハイハット,
}
