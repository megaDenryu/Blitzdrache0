//! 描画段階資源の生成。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに生成した資源をその場で逆順に破棄する。生成の途中経過を外へ出さないため、
//! 部分的に生成された器は呼び出し元から見えない。
//! フレーム構成しだいで作る段階の資源は`optional_stages`が担う。シーンと影の本体のパイプラインはパイプライン台帳が持つため、ここでは作らない。

mod optional_stages;

use ash::vk;

use super::描画段階資源;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム構成;
use crate::shader_bundle::シェーダー束;
use crate::vulkan::descriptor::シーンセットレイアウト一式;
use crate::vulkan::tracked_device::GPUデバイス;

/// 器を組み立てるのに要る材料一式。
pub(in crate::renderer) struct 生成要求<'a> {
    pub(in crate::renderer) device: &'a GPUデバイス,
    pub(in crate::renderer) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    /// シーン段階の色アタッチメントの形式(ポスト処理があればHDR中間画像、無ければスワップチェーン)。
    /// 影段階は深度だけへ書くため色形式を要らない。
    pub(in crate::renderer) シーンカラー形式: vk::Format,
    pub(in crate::renderer) セットレイアウト: &'a シーンセットレイアウト一式,
    pub(in crate::renderer) シェーダー: &'a シェーダー束,
    pub(in crate::renderer) 構成: フレーム構成,
}

pub(super) fn 生成する(要求: 生成要求<'_>) -> Result<描画段階資源, レンダラーエラー> {
    let 任意 = optional_stages::組み立てる(&要求)?;
    Ok(描画段階資源 {
        空: 任意.空,
        大気のベイク済み画像: 任意.大気のベイク済み画像,
        布シャドウ: 任意.布シャドウ,
    })
}
