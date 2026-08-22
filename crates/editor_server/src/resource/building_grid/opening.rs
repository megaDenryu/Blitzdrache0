//! 升目の側面のはめ口へ何を入れるかの型契約。壁の種類と、壁の外面へ付ける飾りをここが持つ。
//!
//! 「壁を入れない」を欄の不在でなく枝で表すのは、壁を消した保存物と欄を書き忘れた保存物を読む側が
//! 区別できるようにするためである。飾りを壁の枝の中へ入れ子にするのは、壁の無い面へ飾りだけが浮く
//! 宣言を型で書けなくするためである。
//! 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断8」

use serde::{Deserialize, Serialize};
#[cfg(feature = "typescript")]
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
pub enum 壁の種類 {
    平壁,
    窓壁,
    扉枠付きの壁,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[serde(tag = "種類", content = "値")]
pub enum 壁の外面の飾り {
    付けない,
    出窓を差し込む,
    煙突を立てる { 段数: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[serde(tag = "種類", content = "値")]
pub enum はめ口の値 {
    入れない,
    壁を入れる {
        壁の種類: 壁の種類,
        外面の飾り: 壁の外面の飾り,
    },
}
