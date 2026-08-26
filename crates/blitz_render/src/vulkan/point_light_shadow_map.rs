//! 点光源の影の立方体配列(判断l): 影を持てる灯1件につき6層を割り当てた、立方体として標本できる
//! D32_SFLOATの2D配列画像。DEPTH_STENCIL_ATTACHMENT | SAMPLED、デバイスローカル、立方体互換である。
//! スワップチェーン再構築とは独立(ウィンドウ寸法に依らない)のため、生成時に一度だけ確保しリサイズ時に作り直さない。
//!
//! 立方体の配列として見るビューは画素段の比較サンプリングが使い、層ごとのビューは面ごとの影の記録が
//! 深度アタッチメントとして使う。レンダーグラフは配列全体を1資源として保守的に追跡する。
//! 一辺と層の総数は`point_light_shadow`の定数が唯一の出どころであり、この型は値を持ち直さない。
//! 参照: `_doc/設計/クラスタ多光源と点光源の影.md`「判断l」

mod create;
mod initial_layout;
mod sampler;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::point_light_shadow::点光源の影の層の総数;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::tracked_device::GPUデバイス;

pub(crate) const 点光源の影の形式: vk::Format = vk::Format::D32_SFLOAT;

/// 配列画像の層数。多段影の層数と同じく、CPU正本の定数から導く1箇所をここに置く。
pub(crate) fn 点光源の影の層数() -> u32 {
    u32::try_from(点光源の影の層の総数).unwrap_or_else(|_| panic!("点光源の影の層の総数がu32に収まらない: {点光源の影の層の総数}"))
}

pub(crate) struct 点光源の影の立方体配列 {
    pub(crate) 画像: vk::Image,
    pub(crate) 立方体配列ビュー: vk::ImageView, // 全層を1つの立方体の配列として見るビュー。画素段の比較サンプリングが束縛する
    pub(crate) 層別のビュー一覧: Vec<vk::ImageView>, // 層ごとに1層だけを見る2Dビュー。面ごとの影の記録が深度アタッチメントとして束縛する
    pub(crate) sampler: vk::Sampler,
    memory: vk::DeviceMemory,
}

impl 点光源の影の立方体配列 {
    pub(crate) fn 生成する(確保係: &GPU資源の確保係<'_>) -> Result<Self, レンダラーエラー> {
        create::生成する(確保係)
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        // 安全性: 各ハンドルはSelfが唯一の所有者であり、破棄時点でGPU側の使用が
        // device_wait_idle済みであることを呼び出し元が保証する。
        unsafe {
            device.destroy_sampler(self.sampler, None);
            for &ビュー in &self.層別のビュー一覧 {
                device.destroy_image_view(ビュー, None);
            }
            device.destroy_image_view(self.立方体配列ビュー, None);
            device.destroy_image(self.画像, None);
        }
        device.メモリを解放する(self.memory);
    }
}
