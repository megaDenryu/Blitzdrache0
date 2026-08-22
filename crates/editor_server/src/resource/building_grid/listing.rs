//! 建物一覧の型契約。エクスプローラーが建物の木を描くために要る最小の項目だけを持つ。
//!
//! 外形や部品の一覧を載せないのは、それらを建物外形カタログが既に配っているためである。一覧に写しを載せると、
//! 格子を保存した直後に一覧とカタログが違うことを言う瞬間ができる。

use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

use super::super::building_definition_id::建物定義ID;

/// 建物の格子の一覧項目とは、保存済みの建物1件を一覧に並べるための名乗りのことである。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub struct 建物の格子の一覧項目 {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub 識別子: 建物定義ID,
    pub 表示名: String,
    pub 升目の数: u32,
}
