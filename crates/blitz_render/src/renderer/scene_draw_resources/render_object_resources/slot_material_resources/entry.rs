//! 材質スロット1つぶんのGPU資源。担当するのは、そのスロットの材質が要るテクスチャ一式を、スロット番号と対にして
//! 確保・解放することである。
//!
//! 係数はここに置かない。ベースカラー係数と金属粗さ係数は描画対象が1本だけ持つ材質レコード列にあり、
//! スロットの添字がその列の添字になる(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「分離の形」)。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「束縛バックエンドの移行境界」

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_object_material::材質スロット素材;
use crate::vulkan;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(in crate::renderer::scene_draw_resources) struct スロット材質資源 {
    スロット番号: u32,
    pub(in crate::renderer::scene_draw_resources) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
}

impl スロット材質資源 {
    pub(super) fn 生成する(
        問い合わせ: 物理デバイス問い合わせ<'_>,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        素材: &材質スロット素材,
    ) -> Result<Self, レンダラーエラー> {
        let テクスチャ =
            vulkan::texture::マテリアルテクスチャ一式::生成する(device, 問い合わせ, メモリプロパティ, 転送環境, 素材.マテリアル())?;
        Ok(Self {
            スロット番号: 素材.スロット番号(),
            テクスチャ,
        })
    }

    pub(super) fn スロット番号(&self) -> u32 {
        self.スロット番号
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.テクスチャ.破棄する(device);
    }
}
