//! 粒子トイの資源の組み立て。粒子はシーンと同じアタッチメント(ポスト有効時はHDR中間画像)へ追記描画するため、描画先の色形式をシーンと揃える(判断39)。
//! 注意: 粒子シェーダーと粒子素材は両方揃うか両方無いかのどちらかでなければならない。片方だけの状態は呼び出し元の配線ミスであり、`粒子入力不一致`で失敗させる。
//! 材料に転送環境とフレームシェーダー定数を取るため、基礎資源の組み立て後に呼ぶ。

use ash::vk;

use super::base_resources::基礎資源;
use super::request::生成要求;
use crate::error::レンダラーエラー;
use crate::vulkan;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::depth::深度形式;

pub(super) fn 組み立てる(
    要求: &生成要求<'_>,
    確保係: &GPU資源の確保係<'_>,
    基礎: &基礎資源,
    シーンカラー形式: vk::Format,
) -> Result<Option<vulkan::particles::粒子リソース一式>, レンダラーエラー> {
    match (要求.シェーダー.粒子.as_ref(), 要求.粒子素材) {
        (Some(シェーダー), Some(素材)) => Ok(Some(vulkan::particles::粒子リソース一式::生成する(
            確保係,
            &基礎.転送環境,
            シーンカラー形式,
            深度形式,
            &基礎.シェーダー定数,
            シェーダー,
            素材,
        )?)),
        (None, None) => Ok(None),
        (Some(_), None) | (None, Some(_)) => Err(crate::error::生成要求不一致エラー::粒子入力不一致.into()),
    }
}
