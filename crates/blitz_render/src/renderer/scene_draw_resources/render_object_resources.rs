//! 1つの描画対象が所有するGPU資源と、その対象の大域アンカー。
//! アンカーはGPU資源ではないが、毎フレームのプッシュ定数を作るのに描画対象と1対1で要るため同じ型が持つ。
//! ジオメトリを詳細段ごとに持つのは、全段を読込時にGPUへ載せてLOD切替でGPU再確保を起こさないためである
//! (参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」)。
//! 個体変換は読込時に一度だけ書いて以後変えないため、可視判定やLOD選択で書き直さない。毎フレーム変わるのは可視ID列だけである
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」)。
//! 確保の局面は`create`、個体変換の置き場は`instance_source`、可視ID列の置き場は`visible_id_source`、
//! 書き込む列の中身の検査は`visible_id_content`とその既出記録`seen_record`にある。

mod create;
mod geometry_list;
mod instance_source;
mod list;
mod seen_record;
mod shared_single_column;
#[cfg(test)]
mod shared_single_column_tests;
mod visible_id_content;
#[cfg(test)]
mod visible_id_content_tests;
mod visible_id_source;

use blitz_math::大域ワールド位置;

use crate::error::レンダラーエラー;
use crate::visible_instance_selection::個体描画計画;
use crate::vulkan;
use crate::vulkan::descriptor::描画対象ディスクリプタ参照;
use crate::vulkan::sync::フレームスロット添字;
use crate::vulkan::tracked_device::GPUデバイス;
use crate::vulkan::visible_id::可視ID列バッファ;
use instance_source::個体変換の出どころ;
use visible_id_content::可視ID列の内容検査;
use visible_id_source::可視ID列の出どころ;

pub(super) use list::描画対象資源一覧を生成する;

pub(super) struct 描画対象資源 {
    pub(super) 大域アンカー: 大域ワールド位置,
    /// 詳細段の昇順に並んだ非空のジオメトリ。段の選択はここから1本を選ぶだけであり、確保も解放も伴わない。
    段別ジオメトリ: geometry_list::段別ジオメトリ,
    pub(super) テクスチャ: vulkan::texture::マテリアルテクスチャ一式,
    pub(super) シェーダー定数: vulkan::object_uniform::描画対象シェーダー定数,
    個体変換: 個体変換の出どころ,
    可視id列: 可視ID列の出どころ,
    /// 書き込む列の中身がこの対象の個体と整合していることの検査。この対象の個体数(個体変換列の件数と常に一致する)と、
    /// パスごとの重複を見るための既出記録をこれが持つ。
    内容検査: 可視ID列の内容検査,
}

impl 描画対象資源 {
    /// 段番号で参照するジオメトリ。焼かれた段数を超える番号は最も粗い段を返す。距離が最も粗い段の閾値より遠いという意味であり、
    /// そのとき最も粗い段を描くのが要求どおりの結果だからである。
    /// 段の語彙(地形詳細段・個体詳細段)は呼び出し側が持ち、この型へは番号だけが渡る。
    pub(super) fn 段を番号で選ぶ(&self, 段番号: usize) -> &vulkan::geometry::ジオメトリバッファ {
        self.段別ジオメトリ.段番号で選ぶ(段番号)
    }

    pub(super) fn 個体数(&self) -> u32 {
        self.内容検査.個体数()
    }

    /// ディスクリプタセットへ結ぶ資源の参照。テクスチャ・シェーダー定数・個体変換を所有するのはこの型のため、束ね方を知るのもこの型にする。
    /// 単一個体の可視ID列だけは束が共有するため、束から借りて受け取る。
    pub(super) fn ディスクリプタ参照<'a>(
        &'a self, 束の単一個体列: &可視ID列バッファ
    ) -> 描画対象ディスクリプタ参照<'a> {
        描画対象ディスクリプタ参照 {
            テクスチャ: &self.テクスチャ,
            シェーダー定数: &self.シェーダー定数,
            個体変換: self.個体変換.ディスクリプタ参照(&self.シェーダー定数),
            可視id列: self.可視id列.参照(束の単一個体列),
        }
    }

    /// そのフレームに描く個体の添字をパス別・段別の並びで書く。可視個体選択を持たない対象へは呼ばない。
    /// 書き込む前に中身を確かめるのは、この型が対象の個体数を持つ唯一の書き込み境界だからである。
    /// 段別個体数の合計が個体数と一致することが、どのパスからも漏れた個体が無いことの担保になる。
    pub(super) fn 可視id列を書き込む(
        &self,
        device: &ash::Device,
        フレーム添字: フレームスロット添字,
        計画: 個体描画計画<'_>,
    ) -> Result<(), レンダラーエラー> {
        self.内容検査.検査する(計画.id列, 計画.段範囲一覧)?;
        self.可視id列.書き込む(device, フレーム添字, 計画.id列)
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.可視id列.破棄する(device);
        self.個体変換.破棄する(device);
        self.シェーダー定数.破棄する(device);
        self.テクスチャ.破棄する(device);
        self.段別ジオメトリ.破棄する(device);
    }
}
