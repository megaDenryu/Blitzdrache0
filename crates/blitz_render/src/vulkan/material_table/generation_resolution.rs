//! 1つの資源表世代の中で材質IDを引いた結果。担当するのは、GPUのレコード添字と正規化済みの材質変種キーが
//! 必ず同じ材質から同時に決まることを1つの値で示すことである。
//!
//! 2つを別々の表で持たないのは、描画発行が「どのレコードを塗るか」と「どのパイプラインで塗るか」を同時に要るためであり、
//! 別々に引くと片方だけが古い世代のものになりうるからである。変種キーはGPUのバイト列へは載せない
//! (パイプラインの選択はCPU側の判断であり、GPUへ二重の正本を作らない)。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」

use crate::vulkan::material_variant::材質変種キー;

use super::record_index::材質レコード添字;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 世代内材質解決 {
    レコード添字: 材質レコード添字,
    変種キー: 材質変種キー,
}

impl 世代内材質解決 {
    pub(in crate::vulkan::material_table) const fn 生成する(
        レコード添字: 材質レコード添字, 変種キー: 材質変種キー
    ) -> Self {
        Self {
            レコード添字, 変種キー
        }
    }

    pub(crate) const fn レコード添字(self) -> 材質レコード添字 {
        self.レコード添字
    }

    pub(crate) const fn 変種キー(self) -> 材質変種キー {
        self.変種キー
    }
}
