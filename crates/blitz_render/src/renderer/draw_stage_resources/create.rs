//! 描画段階資源の生成。呼ばれるのはレンダラー生成時の1回だけであり、以降のフレームは参照しかしない。
//! 途中で失敗したら、それまでに生成した資源をその場で逆順に破棄する。生成の途中経過を外へ出さないため、
//! 部分的に生成された器は呼び出し元から見えない。
//! 常に作るシーンとシャドウをここが、フレーム構成しだいで作る段階の資源を`optional_stages`が担う。

mod optional_stages;

use ash::vk;

use super::描画段階資源;
use crate::error::レンダラーエラー;
use crate::frame_composition::フレーム構成;
use crate::shader_bundle::シェーダー束;
use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::uniform::フレームユニフォーム一式;

/// 器を組み立てるのに要る材料一式。
pub(in crate::renderer) struct 生成要求<'a> {
    pub(in crate::renderer) device: &'a GPUデバイス,
    pub(in crate::renderer) メモリプロパティ: &'a vk::PhysicalDeviceMemoryProperties,
    /// シーン段階の色アタッチメントの形式(ポスト処理があればHDR中間画像、無ければスワップチェーン)。
    /// 影段階は深度だけへ書くため色形式を要らない。
    pub(in crate::renderer) シーンカラー形式: vk::Format,
    pub(in crate::renderer) ディスクリプタlayout: vk::DescriptorSetLayout,
    /// 布専用シャドウ経路のディスクリプタが結ぶフレームユニフォーム。
    /// 描画対象のディスクリプタセットを借りずに束縛先を作るため、この段でバッファを受け取る。
    pub(in crate::renderer) ユニフォーム: &'a フレームユニフォーム一式,
    pub(in crate::renderer) シェーダー: &'a シェーダー束,
    pub(in crate::renderer) 構成: フレーム構成,
}

pub(super) fn 生成する(要求: 生成要求<'_>) -> Result<描画段階資源, レンダラーエラー> {
    let device = 要求.device;
    let シーン = vulkan::pipeline::パイプライン::生成する(
        device,
        要求.シーンカラー形式,
        vulkan::depth::深度形式,
        要求.ディスクリプタlayout,
        &要求.シェーダー.シーン,
    )?;
    let シャドウ = match vulkan::pipeline::シャドウパイプライン::生成する(device, 要求.ディスクリプタlayout, &要求.シェーダー.シャドウ)
    {
        Ok(シャドウ) => シャドウ,
        Err(誤り) => {
            シーン.破棄する(device);
            return Err(誤り);
        }
    };
    match optional_stages::組み立てる(&要求) {
        Ok(任意) => Ok(描画段階資源 {
            シーン,
            シャドウ,
            空: 任意.空,
            大気lut: 任意.大気lut,
            布シャドウ: 任意.布シャドウ,
        }),
        Err(誤り) => {
            シャドウ.破棄する(device);
            シーン.破棄する(device);
            Err(誤り)
        }
    }
}
