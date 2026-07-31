//! 材質スロット1つぶんのGPU資源。担当するのは、そのスロットの材質が要るテクスチャ一式と描画対象シェーダー定数を、
//! 一方の確保が失敗したときにもう一方を取り残さずに確保・解放することである。
//!
//! シェーダー定数をスロットごとに持つのは、ベースカラー係数と金属粗さ係数が材質ごとに違うためである。
//! 同じバッファの先頭が個体変換1件としても読まれる(参照: `crate::vulkan::instance_transform`)が、そこへ書く変換は
//! 描画対象に1つであり、どのスロットの複製を読んでも同じ値になる。
//! 参照: `_doc/設計/マルチマテリアルと材質境界.md`「束縛バックエンドの移行境界」

use ash::vk;
use blitz_math::{ローカル, ワールド, 変換};

use crate::error::レンダラーエラー;
use crate::render_object_material::材質スロット素材;
use crate::vulkan;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::転送実行環境;

pub(in crate::renderer::scene_draw_resources) struct スロット材質資源 {
    スロット番号: u32,
    pub(in crate::renderer::scene_draw_resources) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(in crate::renderer::scene_draw_resources) シェーダー定数: vulkan::object_uniform::描画対象シェーダー定数,
}

impl スロット材質資源 {
    pub(super) fn 生成する(
        問い合わせ: 物理デバイス問い合わせ<'_>,
        device: &GPUデバイス,
        メモリプロパティ: &vk::PhysicalDeviceMemoryProperties,
        転送環境: &転送実行環境,
        ローカルからワールド: 変換<ローカル, ワールド>,
        素材: &材質スロット素材,
    ) -> Result<Self, レンダラーエラー> {
        let テクスチャ =
            vulkan::texture::マテリアルテクスチャ一式::生成する(device, 問い合わせ, メモリプロパティ, 転送環境, 素材.マテリアル())?;
        let シェーダー定数 = match vulkan::object_uniform::描画対象シェーダー定数::生成する(
            device,
            メモリプロパティ,
            ローカルからワールド,
            素材.マテリアル(),
        ) {
            Ok(値) => 値,
            Err(誤り) => {
                テクスチャ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            スロット番号: 素材.スロット番号(),
            テクスチャ,
            シェーダー定数,
        })
    }

    pub(super) fn スロット番号(&self) -> u32 {
        self.スロット番号
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.シェーダー定数.破棄する(device);
        self.テクスチャ.破棄する(device);
    }
}
