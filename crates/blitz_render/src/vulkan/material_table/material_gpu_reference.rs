//! フレームの描画準備が材質IDを解決した結果。担当するのは、レコード添字が「どの資源表世代の中での位置か」を必ず伴わせることである。
//!
//! 注意: この参照を永続する描画束へ保存してはならない。世代を作り直すとレコード列が並び直り、
//! 旧い添字が新しい表の別の材質を指す。束は大域材質IDを持ち、フレームごとに束縛する世代へ対して解決し直す。
//! 生の添字を取り出せるのは、束縛する世代と一致することを`資源表世代`が確かめた後だけである。
//! 参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」

use super::generation_id::資源表世代ID;
use super::generation_resolution::世代内材質解決;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct 材質GPU参照 {
    世代id: 資源表世代ID,
    解決: 世代内材質解決,
}

impl 材質GPU参照 {
    pub(in crate::vulkan::material_table) const fn 生成する(世代id: 資源表世代ID, 解決: 世代内材質解決) -> Self {
        Self { 世代id, 解決 }
    }

    pub(crate) const fn 世代id(self) -> 資源表世代ID {
        self.世代id
    }

    /// 束縛する世代との一致を確かめずに中身だけを読ませないため、世代の検査を通す口とは別に公開しない。
    pub(in crate::vulkan::material_table) const fn 解決(self) -> 世代内材質解決 {
        self.解決
    }
}
