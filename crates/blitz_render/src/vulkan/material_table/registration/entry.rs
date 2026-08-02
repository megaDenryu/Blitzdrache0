//! 束1つぶんの保持材質。触れるのは描画対象ごと材質スロットごとの材質と、そのCPU側の画素だけであり、
//! 束の出し入れとIDの発番規則には触れない。
//!
//! 不変条件: 内側の並びは`材質スロット素材一覧`の並びと同じである。呼び出し元はこの並びで材質IDを受け取り、
//! 自分が持つ材質スロット番号と対にする。並びが崩れると別の材質のIDをスロットへ結ぶ。

use crate::render_object_material::描画対象素材;
use crate::texture_material::テクスチャ素材;

use crate::vulkan::material_table::image_id::画像ID;
use crate::vulkan::material_table::material_id::大域材質ID;
use crate::vulkan::material_table::pack_input::梱包対象材質;
use crate::vulkan::material_table::texture_id::テクスチャID;
use crate::vulkan::material_table::texture_role::材質テクスチャ役割;
use crate::vulkan::material_table::texture_spec::テクスチャ指定;

use super::minting::安定IDの発番;

/// 1つの描画対象が持つ材質の、材質スロット素材一覧と同じ並びの大域材質ID。
pub(in crate::vulkan::material_table) type 描画対象別の材質ID = Vec<大域材質ID>;

struct 保持テクスチャ {
    テクスチャid: テクスチャID,
    画像id: 画像ID,
    素材: テクスチャ素材,
}

struct 保持材質 {
    材質id: 大域材質ID,
    ベースカラー係数: [f32; 4],
    金属度係数: f32,
    粗さ係数: f32,
    /// `材質テクスチャ役割::全役割`の配列添字の並びで持つ。
    役割別テクスチャ: [Option<保持テクスチャ>; 3],
}

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
                    .map(|スロット素材| 一件を取り込む(発番, スロット素材.マテリアル()))
                    .collect()
            })
            .collect();
        Self { 描画対象別 }
    }

    pub(super) fn 描画対象別の材質id一覧(&self) -> Vec<描画対象別の材質ID> {
        self.描画対象別
            .iter()
            .map(|材質一覧| 材質一覧.iter().map(|材質| 材質.材質id).collect())
            .collect()
    }

    pub(super) fn 梱包対象を並べる(&self) -> Vec<梱包対象材質<'_>> {
        self.描画対象別.iter().flatten().map(梱包対象へ写す).collect()
    }
}

fn 一件を取り込む(発番: &mut 安定IDの発番, マテリアル: &crate::material::マテリアル素材) -> 保持材質 {
    let 役割別テクスチャ = std::array::from_fn(|添字| {
        マテリアル.役割のテクスチャ(材質テクスチャ役割::全役割[添字]).map(|素材| 保持テクスチャ {
            テクスチャid: 発番.テクスチャidを発番する(),
            画像id: 発番.画像idを引き当てる(素材),
            素材: 素材.clone(),
        })
    });
    保持材質 {
        材質id: 発番.材質idを発番する(),
        ベースカラー係数: マテリアル.ベースカラー係数(),
        金属度係数: マテリアル.金属度係数(),
        粗さ係数: マテリアル.粗さ係数(),
        役割別テクスチャ,
    }
}

fn 梱包対象へ写す(材質: &保持材質) -> 梱包対象材質<'_> {
    let 役割別指定 = std::array::from_fn(|添字| {
        材質.役割別テクスチャ[添字]
            .as_ref()
            .map(|保持| テクスチャ指定::生成する(保持.テクスチャid, 保持.画像id, &保持.素材))
    });
    梱包対象材質::生成する(材質.材質id, 材質.ベースカラー係数, 材質.金属度係数, 材質.粗さ係数, 役割別指定)
}
