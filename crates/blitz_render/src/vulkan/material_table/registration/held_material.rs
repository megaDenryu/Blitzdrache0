//! 束が保つ材質1件の形と、その出入り。担当するのは、レンダラーへ渡された材質素材から発番済みのIDを添えた1件を作ることと、
//! その1件を梱包工程が読む形へ写すことである。触れるのはこの1件のフィールドだけであり、束の並びには触れない。
//!
//! 不変条件: 役割別テクスチャの並びは`材質テクスチャ役割::全役割`の配列添字と同じである。並びが崩れると、
//! 法線の枠へベースカラーの画素が入っても型が通る。

use crate::material::マテリアル素材;
use crate::material::地表層の数;
use crate::texture_material::テクスチャ素材;
use crate::vulkan::material_table::image_id::画像ID;
use crate::vulkan::material_table::material_id::大域材質ID;
use crate::vulkan::material_table::pack_input::梱包対象材質;
use crate::vulkan::material_table::texture_id::テクスチャID;
use crate::vulkan::material_table::texture_role::{役割の数, 材質テクスチャ役割};
use crate::vulkan::material_table::texture_spec::テクスチャ指定;
use crate::vulkan::material_variant::シェーディングモデル種別;

use super::minting::安定IDの発番;

struct 保持テクスチャ {
    テクスチャid: テクスチャID,
    画像id: 画像ID,
    素材: テクスチャ素材,
}

pub(super) struct 保持材質 {
    材質id: 大域材質ID,
    シェーディングモデル: シェーディングモデル種別,
    ベースカラー係数: [f32; 4],
    金属度係数: f32,
    粗さ係数: f32,
    役割別テクスチャ: [Option<保持テクスチャ>; 役割の数], // `材質テクスチャ役割::全役割`の配列添字の並びで持つ
    地表の層ごとのタイル倍率: [f32; 地表層の数],
}

impl 保持材質 {
    pub(super) fn 素材から取り込む(発番: &mut 安定IDの発番, マテリアル: &マテリアル素材) -> Self {
        let 役割別テクスチャ = std::array::from_fn(|添字| {
            マテリアル.役割のテクスチャ(材質テクスチャ役割::全役割[添字]).map(|素材| 保持テクスチャ {
                テクスチャid: 発番.テクスチャidを発番する(),
                画像id: 発番.画像idを引き当てる(素材),
                素材: 素材.clone(),
            })
        });
        Self {
            材質id: 発番.材質idを発番する(),
            シェーディングモデル: マテリアル.シェーディングモデル種別(),
            ベースカラー係数: マテリアル.ベースカラー係数(),
            金属度係数: マテリアル.金属度係数(),
            粗さ係数: マテリアル.粗さ係数(),
            役割別テクスチャ,
            地表の層ごとのタイル倍率: マテリアル.地表の層ごとのタイル倍率(),
        }
    }

    pub(super) fn 材質id(&self) -> 大域材質ID {
        self.材質id
    }

    pub(super) fn 梱包対象へ写す(&self) -> 梱包対象材質<'_> {
        let 役割別指定 = std::array::from_fn(|添字| {
            self.役割別テクスチャ[添字]
                .as_ref()
                .map(|保持| テクスチャ指定::生成する(保持.テクスチャid, 保持.画像id, &保持.素材))
        });
        梱包対象材質::生成する(
            self.材質id,
            self.シェーディングモデル,
            self.ベースカラー係数,
            self.金属度係数,
            self.粗さ係数,
            役割別指定,
            self.地表の層ごとのタイル倍率,
        )
    }
}
