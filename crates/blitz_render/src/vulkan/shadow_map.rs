//! 方向光の多段シャドウマップ(判断35・OW5第5段): 生成時に決めた一辺の層を距離区分数ぶん重ねた
//! D32_SFLOATの2D配列画像。DEPTH_STENCIL_ATTACHMENT | SAMPLED、デバイスローカル。
//! スワップチェーン再構築とは独立(ウィンドウ寸法に依らない)のため、生成時に一度だけ確保しリサイズ時に作り直さない。
//! `一辺`は生成時の`影の一辺解像度`が決め、以後この型が唯一の保持者になる。グラフの登録寸法・ビューポート・
//! PCFのテクセル尺度がすべてこの1つの値から出る。多段の構築が使う一辺と食い違うと
//! テクセルスナップの刻みが実際のテクセルと合わなくなるため、描く前にレンダラーが両者を突き合わせる。
//!
//! 配列全体を見るビューはシーンの比較サンプリングが使い、層ごとのビューは距離区分別のシャドウ記録が
//! 深度アタッチメントとして使う。レンダーグラフは配列全体を1資源として保守的に追跡する
//! (参照: `_doc/設計/空と時間帯と遠距離シャドウ.md`「多段シャドウ(CSM)」)。

mod create;
mod sampler;

use ash::vk;

use crate::cascade::{影の一辺解像度, 距離区分数};
use crate::error::レンダラーエラー;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const シャドウマップ形式: vk::Format = vk::Format::D32_SFLOAT;

/// 配列画像の層数。距離区分1つにつき1層である。
pub(crate) fn シャドウマップ層数() -> u32 {
    u32::try_from(距離区分数).unwrap_or_else(|_| panic!("距離区分数がu32に収まらない: {距離区分数}"))
}

pub(crate) struct シャドウマップ {
    一辺: 影の一辺解像度, // この資源の一辺
    pub(crate) 画像: vk::Image,
    pub(crate) 配列ビュー: vk::ImageView,                           // 全層を1つの2D配列として見るビュー
    pub(crate) 距離区分別のビュー一覧: [vk::ImageView; 距離区分数], // 距離区分ごとに1層だけを見るビュー
    pub(crate) sampler: vk::Sampler,
    memory: vk::DeviceMemory,
}

impl シャドウマップ {
    pub(crate) fn 生成する(確保係: &GPU資源の確保係<'_>, 一辺: 影の一辺解像度) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係, 一辺)
    }

    pub(crate) fn 一辺(&self) -> 影の一辺解像度 {
        self.一辺
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_sampler(self.sampler, None);
            for &ビュー in &self.距離区分別のビュー一覧 {
                device.destroy_image_view(ビュー, None);
            }
            device.destroy_image_view(self.配列ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
