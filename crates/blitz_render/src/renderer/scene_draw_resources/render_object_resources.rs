//! 1つの描画対象が所有するGPU資源と、その対象の大域アンカー。生成途中の失敗も逆順に解放する。
//! アンカーはGPU資源ではないが、毎フレームのプッシュ定数を作るのに描画対象と1対1で要るため同じ型が持つ。
//! ジオメトリを詳細段ごとに持つのは、全段を読込時にGPUへ載せてLOD切替でGPU再確保を起こさないためである
//! (参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」)。

mod geometry_list;
mod list;

use ash::vk;
use blitz_math::大域ワールド位置;

use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::terrain_detail::地形詳細段;
use crate::vulkan;
use crate::vulkan::descriptor::描画対象ディスクリプタ参照;
use crate::vulkan::gpu_environment::物理デバイス問い合わせ;
use crate::vulkan::tracked_device::GPUデバイス;

pub(super) use list::描画対象資源一覧を生成する;

pub(super) struct 描画対象資源 {
    pub(super) 大域アンカー: 大域ワールド位置,
    /// 詳細段の昇順に並んだ非空のジオメトリ。段の選択はここから1本を選ぶだけであり、確保も解放も伴わない。
    段別ジオメトリ: geometry_list::段別ジオメトリ,
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
        let 段別ジオメトリ = geometry_list::段別ジオメトリ::生成する(device, メモリプロパティ, 転送環境, 素材.段一覧())?;
        let テクスチャ = match vulkan::texture::マテリアルテクスチャ一式::生成する(
            device,
            問い合わせ,
            メモリプロパティ,
            転送環境,
            素材.マテリアル(),
        ) {
            Ok(値) => 値,
            Err(誤り) => {
                段別ジオメトリ.破棄する(device);
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
                段別ジオメトリ.破棄する(device);
                return Err(誤り);
            }
        };
        Ok(Self {
            大域アンカー: 素材.大域アンカー(),
            段別ジオメトリ,
            テクスチャ,
            ユニフォーム,
        })
    }

    /// 要求された段のジオメトリ。焼かれた段数を超える要求は最も粗い段を返す。距離が最も粗い段の閾値より遠いという意味であり、そのとき最も粗い段を描くのが要求どおりの結果だからである。
    pub(super) fn 段を選ぶ(&self, 段: 地形詳細段) -> &vulkan::geometry::ジオメトリバッファ {
        self.段別ジオメトリ.段を選ぶ(段)
    }

    /// ディスクリプタセットへ結ぶ資源の参照。テクスチャとユニフォームを所有するのはこの型のため、束ね方を知るのもこの型にする。
    pub(super) fn ディスクリプタ参照(&self) -> 描画対象ディスクリプタ参照<'_> {
        描画対象ディスクリプタ参照 {
            テクスチャ: &self.テクスチャ,
            ユニフォーム: &self.ユニフォーム,
        }
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.ユニフォーム.破棄する(device);
        self.テクスチャ.破棄する(device);
        self.段別ジオメトリ.破棄する(device);
    }
}
