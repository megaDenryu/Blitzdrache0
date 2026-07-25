//! 1つの描画対象が所有するGPU資源。生成途中の失敗も逆順に解放する。

mod list;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) use list::描画対象資源一覧を生成する;

pub(super) struct 描画対象資源 {
    pub(super) ジオメトリ: vulkan::geometry::ジオメトリバッファ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(super) ユニフォーム: vulkan::object_uniform::描画対象ユニフォーム,
}

impl 描画対象資源 {
    pub(super) fn 生成する(
        問い合わせ: 物理デバイス問い合わせ<'_>,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &vulkan::transfer::転送実行環境,
        素材: &描画対象素材,
    ) -> Result<Self, レンダラーエラー> {
        let ジオメトリ = vulkan::geometry::ジオメトリバッファ::生成する(
            device,
            メモリプロパティ,
            転送環境,
            素材.頂点一覧(),
            素材.インデックス一覧(),
        )?;
        let テクスチャ = match vulkan::texture::マテリアルテクスチャ一式::生成する(
            device,
            問い合わせ,
            メモリプロパティ,
            転送環境,
            素材.マテリアル(),
        ) {
            Ok(値) => 値,
            Err(誤り) => {
                ジオメトリ.破棄する(device);
                return Err(誤り);
            }
        };
        let ユニフォーム = match vulkan::object_uniform::描画対象ユニフォーム::生成する(
            device,
            メモリプロパティ,
            素材.ローカルからワールド(),
            素材.マテリアル(),
        ) {
            Ok(値) => 値,
            Err(誤り) => {
                テクスチャ.破棄する(device);
                ジオメトリ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            ジオメトリ,
            テクスチャ,
            ユニフォーム,
        })
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.ユニフォーム.破棄する(device);
        self.テクスチャ.破棄する(device);
        self.ジオメトリ.破棄する(device);
    }
}
