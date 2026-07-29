//! 方向光のカスケードシャドウマップ(判断35・OW5第5段): 2048x2048の層を帯数ぶん重ねた
//! D32_SFLOATの2D配列画像。DEPTH_STENCIL_ATTACHMENT | SAMPLED、デバイスローカル。
//! スワップチェーン再構築とは独立(サイズ固定)のため、生成時に一度だけ確保しリサイズ時に作り直さない。
//!
//! 配列全体を見るビューはシーンの比較サンプリングが使い、層ごとのビューは帯別のシャドウ記録が
//! 深度アタッチメントとして使う。レンダーグラフは配列全体を1資源として保守的に追跡する
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「カスケードシャドウ(CSM)」)。

mod create;
mod sampler;

use ash::vk;

use crate::cascade::帯数;
use crate::error::レンダラーエラー;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const シャドウマップ形式: vk::Format = vk::Format::D32_SFLOAT;
pub(crate) const シャドウマップ一辺: u32 = crate::cascade::帯解像度;

/// 配列画像の層数。帯1つにつき1層である。
pub(crate) fn シャドウマップ層数() -> u32 {
    u32::try_from(帯数).unwrap_or_else(|_| panic!("帯数がu32に収まらない: {帯数}"))
}

pub(crate) struct シャドウマップ {
    pub(crate) 画像: vk::Image,
    /// 全層を1つの2D配列として見るビュー。シーンの比較サンプリングが束縛する。
    pub(crate) 配列ビュー: vk::ImageView,
    /// 帯ごとに1層だけを見るビュー。帯別のシャドウ記録が深度アタッチメントとして束縛する。
    pub(crate) 帯ビュー一覧: [vk::ImageView; 帯数],
    pub(crate) sampler: vk::Sampler,
    memory: vk::DeviceMemory,
}

impl シャドウマップ {
    pub(crate) fn 生成する(
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
    ) -> Result<Self, レンダラーエラー> {
        create::生成する(device, メモリプロパティ)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_sampler(self.sampler, None);
            for &ビュー in &self.帯ビュー一覧 {
                device.destroy_image_view(ビュー, None);
            }
            device.destroy_image_view(self.配列ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
