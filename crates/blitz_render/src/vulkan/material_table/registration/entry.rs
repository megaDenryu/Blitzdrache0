//! 束1つぶんの保持材質。触れるのは描画対象ごと材質スロットごとの材質の並びだけであり、束の出し入れとIDの発番規則には触れない。
//! 材質1件の形とその出入りは`held_material`が持つ。
//!
//! 不変条件: 内側の並びは`材質スロット素材一覧`の並びと同じである。呼び出し元はこの並びで材質IDを受け取り、自分が持つ材質スロット番号と対にする。並びが崩れると別の材質のIDをスロットへ結ぶ。

use crate::render_object_material::描画対象素材;
use crate::vulkan::material_table::material_id::大域材質ID;
use crate::vulkan::material_table::pack_input::梱包対象材質;

use super::held_material::保持材質;
use super::minting::安定IDの発番;

/// 1つの描画対象が持つ材質の、材質スロット素材一覧と同じ並びの大域材質ID。
pub(in crate::vulkan::material_table) type 描画対象別の材質ID = Vec<大域材質ID>;

pub(crate) struct 束の保持材質 {
    描画対象別: Vec<Vec<保持材質>>,
}

impl 束の保持材質 {
    pub(super) fn 取り込む(発番: &mut 安定IDの発番, 描画対象一覧: &[描画対象素材]) -> Self {
        let 描画対象別 = 描画対象一覧
            .iter()
            .map(|描画対象| {
                描画対象
                    .材質スロット素材一覧()
                    .一覧()
                    .iter()
                    .map(|スロット素材| 保持材質::素材から取り込む(発番, スロット素材.マテリアル()))
                    .collect()
            })
            .collect();
        Self { 描画対象別 }
    }

    pub(super) fn 描画対象別の材質id一覧(&self) -> Vec<描画対象別の材質ID> {
        self.描画対象別
            .iter()
            .map(|材質一覧| 材質一覧.iter().map(保持材質::材質id).collect())
            .collect()
    }

    pub(super) fn 梱包対象を並べる(&self) -> Vec<梱包対象材質<'_>> {
        self.描画対象別.iter().flatten().map(保持材質::梱包対象へ写す).collect()
    }
}
