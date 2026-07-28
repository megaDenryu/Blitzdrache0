//! 1つの描画対象が所有するGPU資源と、その対象の大域アンカー。
//! アンカーはGPU資源ではないが、毎フレームのプッシュ定数を作るのに描画対象と1対1で要るため同じ型が持つ。
//! ジオメトリを詳細段ごとに持つのは、全段を読込時にGPUへ載せてLOD切替でGPU再確保を起こさないためである
//! (参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」)。
//! 個体変換は読込時に一度だけ書いて以後変えないため、可視判定やLOD選択で書き直さない。毎フレーム変わるのは可視ID列だけである
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」)。
//! 確保の局面は`create`、個体変換の置き場は`instance_source`、可視ID列の置き場は`visible_id_source`にある。

mod create;
mod geometry_list;
mod instance_source;
mod list;
mod shared_single_column;
#[cfg(test)]
mod shared_single_column_tests;
mod visible_id_source;

use blitz_math::大域ワールド位置;

use crate::error::レンダラーエラー;
use crate::terrain_detail::地形詳細段;
use crate::vulkan;
use crate::vulkan::descriptor::描画対象ディスクリプタ参照;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::可視ID列バッファ;
use instance_source::個体変換の出どころ;
use visible_id_source::可視ID列の出どころ;

pub(super) use list::描画対象資源一覧を生成する;

pub(super) struct 描画対象資源 {
    pub(super) 大域アンカー: 大域ワールド位置,
    /// 詳細段の昇順に並んだ非空のジオメトリ。段の選択はここから1本を選ぶだけであり、確保も解放も伴わない。
    段別ジオメトリ: geometry_list::段別ジオメトリ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(super) ユニフォーム: vulkan::object_uniform::描画対象ユニフォーム,
    個体変換: 個体変換の出どころ,
    可視id列: 可視ID列の出どころ,
    /// この対象が持つ個体の数。個体変換列の件数と常に一致し、シーンパスが描く数はこれ以下になる。
    個体数: u32,
}

impl 描画対象資源 {
    /// 要求された段のジオメトリ。焼かれた段数を超える要求は最も粗い段を返す。距離が最も粗い段の閾値より遠いという意味であり、そのとき最も粗い段を描くのが要求どおりの結果だからである。
    pub(super) fn 段を選ぶ(&self, 段: 地形詳細段) -> &vulkan::geometry::ジオメトリバッファ {
        self.段別ジオメトリ.段を選ぶ(段)
    }

    pub(super) fn 個体数(&self) -> u32 {
        self.個体数
    }

    /// ディスクリプタセットへ結ぶ資源の参照。テクスチャ・ユニフォーム・個体変換を所有するのはこの型のため、束ね方を知るのもこの型にする。
    /// 単一個体の可視ID列だけは束が共有するため、束から借りて受け取る。
    pub(super) fn ディスクリプタ参照<'a>(
        &'a self, 束の単一個体列: &可視ID列バッファ
    ) -> 描画対象ディスクリプタ参照<'a> {
        描画対象ディスクリプタ参照 {
            テクスチャ: &self.テクスチャ,
            ユニフォーム: &self.ユニフォーム,
            個体変換: self.個体変換.ディスクリプタ参照(&self.ユニフォーム),
            可視id列: self.可視id列.参照(束の単一個体列),
        }
    }

    /// そのフレームにシーンパスが描く個体の添字を書く。可視個体選択を持たない対象へは呼ばない。
    pub(super) fn 可視id列を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        可視id列: &[u32],
    ) -> Result<(), レンダラーエラー> {
        self.可視id列.書き込む(device, フレーム添字, 可視id列)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.可視id列.破棄する(device);
        self.個体変換.破棄する(device);
        self.ユニフォーム.破棄する(device);
        self.テクスチャ.破棄する(device);
        self.段別ジオメトリ.破棄する(device);
    }
}
