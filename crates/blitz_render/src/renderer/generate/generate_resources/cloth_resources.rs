//! 布資源の組み立て(判断52〜54)。動く目標を持つ布はスキン付きシーン前提(目標の確定がスキン済み頂点を読む)で、
//! 目標の更新対応のスキン頂点添字はここで頂点数と照合する。

use ash::vk;

use crate::cloth_material::布素材;
use crate::cloth_shader_set::布シェーダー一式;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::descriptor::シーンセットレイアウト一式;
use crate::vulkan::skinning::スキニング一式;
use crate::vulkan::transfer::ステージング経由の転送係;

#[allow(clippy::too_many_arguments)]
pub(super) fn 組み立てる(
    転送係: ステージング経由の転送係<'_>,
    シーンカラー形式: vk::Format,
    セットレイアウト: &シーンセットレイアウト一式,
    布: Option<&布素材>,
    シェーダー: &布シェーダー一式,
    スキニング: Option<&スキニング一式>,
) -> Result<Option<vulkan::cloth::布一式>, レンダラーエラー> {
    let Some(素材) = 布 else {
        return Ok(None);
    };
    // 更新対応0件の吊るし布(判断56)はスキン不要。動く目標を持つ布(マント)はスキン必須で添字を検証する。
    let スキン済みbuffer = if 素材.目標の更新対応一覧.is_empty() {
        None
    } else {
        let Some(スキニング) = スキニング else {
            return Err(crate::error::布エラー::スキン必須.into());
        };
        for 対応 in &素材.目標の更新対応一覧 {
            if 対応[1] >= スキニング.頂点数 {
                return Err(crate::error::布エラー::アタッチ先範囲外 {
                    添字: 対応[1],
                    頂点数: スキニング.頂点数,
                }
                .into());
            }
        }
        Some(スキニング.出力バッファ())
    };
    Ok(Some(vulkan::cloth::布一式::生成する(
        転送係,
        シーンカラー形式,
        セットレイアウト,
        素材,
        シェーダー,
        スキン済みbuffer,
    )?))
}
