//! 開発用UI(egui)描画一式: テクスチャ台帳・ディスクリプタ・パイプライン・
//! フレームごとジオメトリバッファ。常に生成し(F3トグルはblitz_app側の実行有無で
//! 表現)、`フレーム描画入力::UI描画`が`None`の間はグラフへパスを積まない(判断33・34)。

mod descriptor;
mod geometry;
mod pipeline;
mod registry;
mod texture;

use ash::vk;

use crate::error::レンダラーエラー;
use crate::shader_set::シェーダー一式;
use crate::ui_texture_id::UIテクスチャID;
use crate::ui_texture_material::UIテクスチャ素材;
use crate::ui_vertex::UI頂点;
use crate::vulkan::allocator::GPU資源の確保係;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::transfer::ステージング経由の転送係;

pub(crate) struct UIリソース一式 {
    テクスチャ台帳: registry::UIテクスチャレジストリ,
    pipeline: pipeline::UIパイプライン,
    ジオメトリ: geometry::UIジオメトリバッファ,
}

impl UIリソース一式 {
    pub(crate) fn 生成する(
        確保係: &GPU資源の確保係<'_>,
        カラー形式: vk::Format,
        シェーダー: &シェーダー一式,
    ) -> Result<Self, レンダラーエラー> {
        let device = 確保係.論理デバイス();
        let テクスチャ台帳 = registry::UIテクスチャレジストリ::生成する(device)?;
        let pipeline = match pipeline::UIパイプライン::生成する(確保係, カラー形式, テクスチャ台帳.layout(), シェーダー)
        {
            Ok(pipeline) => pipeline,
            Err(誤り) => {
                テクスチャ台帳.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            テクスチャ台帳,
            pipeline,
            ジオメトリ: geometry::UIジオメトリバッファ::生成する(),
        })
    }

    pub(crate) fn テクスチャを反映する(
        &mut self,
        転送係: ステージング経由の転送係<'_>,
        id: UIテクスチャID,
        素材: &UIテクスチャ素材,
    ) -> Result<(), レンダラーエラー> {
        self.テクスチャ台帳.反映する(転送係, id, 素材)
    }

    pub(crate) fn テクスチャを削除する(&mut self, device: &GPUデバイス, id: UIテクスチャID) {
        self.テクスチャ台帳.削除する(device, id);
    }

    pub(crate) fn setを取得する(&self, id: UIテクスチャID) -> vk::DescriptorSet {
        self.テクスチャ台帳.setを取得する(id)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn ジオメトリを書き込む(
        &mut self,
        確保係: &GPU資源の確保係<'_>,
        フレーム添字: フレームスロット添字,
        頂点一覧: &[UI頂点],
        インデックス一覧: &[u32],
    ) -> Result<(vk::Buffer, vk::Buffer), レンダラーエラー> {
        self.ジオメトリ.書き込む(確保係, フレーム添字, 頂点一覧, インデックス一覧)
    }

    pub(crate) fn pipeline_handle(&self) -> vk::Pipeline {
        self.pipeline.handle
    }

    pub(crate) fn pipeline_layout(&self) -> vk::PipelineLayout {
        self.pipeline.layout
    }

    pub(crate) fn 破棄する(&self, device: &GPUデバイス) {
        self.ジオメトリ.破棄する(device);
        self.pipeline.破棄する(device);
        self.テクスチャ台帳.破棄する(device);
    }
}
