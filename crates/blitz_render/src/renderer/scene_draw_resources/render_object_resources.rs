//! 1つの描画対象が所有するGPU資源と、その対象の大域の基準原点。
//! 基準原点はGPU資源ではないが、毎フレームのプッシュ定数を作るのに描画対象と1対1で要るため同じ型が持つ。
//! ジオメトリを詳細段ごとに持つのは、全段を読込時にGPUへ載せてLOD切替でGPU再確保を起こさないためである
//! (参照: `_doc/設計/地形とカメラ相対描画.md`「LOD」)。
//! 個体変換は読込時に一度だけ書いて以後変えないため、可視判定やLOD選択で書き直さない。毎フレーム変わるのは可視ID列と、
//! 動く個体を宣言した対象の動く個体の位置だけである
//! (参照: `_doc/設計/植生インスタンスと物量計測.md`「描画発行」)。個体が1体だけの対象も1要素の個体変換バッファを持ち、
//! 他の資源の先頭を個体変換として読むことはない(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「段階導入」)。
//! 材質のGPU資源はこの型が持たない。テクスチャも係数も資源表世代が1つの表として持ち、対象は材質スロット番号ごとの
//! 大域材質IDだけを持つ(参照: `_doc/設計/GPU資源束縛の分離と索引化.md`「材質レコードとテクスチャ台帳」)。
//! 確保の局面は`create`、そのフレームぶんのGPUへの書き込みの局面は`frame_write`、
//! 個体レコードの置き場は`instance_record_storage`、可視ID列の置き場は`visible_id_source`、
//! スロット別の材質IDは`slot_material_ids`、材質スロット番号の解決は`slot_binding`、
//! 書き込む列の中身の検査は`visible_id_content`とその既出記録`seen_record`にある。

mod create;
mod frame_write;
mod geometry_list;
mod instance_record_storage;
mod list;
mod seen_record;
mod shared_single_column;
#[cfg(test)]
mod shared_single_column_tests;
mod slot_binding;
mod slot_material_ids;
mod visible_id_content;
#[cfg(test)]
mod visible_id_content_tests;
mod visible_id_source;

use blitz_math::大域ワールド位置;

use crate::vulkan;
use crate::vulkan::tracked_device::GPUデバイス;
use instance_record_storage::個体レコードの置き場;
use slot_material_ids::スロット別材質ID;
use visible_id_content::可視ID列の内容検査;
use visible_id_source::可視ID列の出どころ;

pub(super) use list::描画対象資源一覧を生成する;

pub(super) struct 描画対象資源 {
    pub(super) 大域の基準原点: 大域ワールド位置,
    /// 詳細段の昇順に並んだ非空のジオメトリ。段の選択はここから1本を選ぶだけであり、確保も解放も伴わない。
    段別ジオメトリ: geometry_list::段別ジオメトリ,
    /// 材質スロットごとの大域材質ID。プリミティブ描画発行が指すスロット番号でここから1件を選ぶ。
    スロット別材質id: スロット別材質ID,
    /// 個体変換の列の置き場。描画が可視ID列を通した添字で1件を選ぶ。動く個体を宣言した対象だけがスロット別の枝になる。
    個体変換: 個体レコードの置き場,
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

    /// 焼かれた段数の内側へ収めた段番号。超える番号を最も粗い段へ丸める規則をここが1箇所で持つため、
    /// ジオメトリの選択とプリミティブ描画発行の絞り込みが別々の段を指すことがない。
    pub(super) fn 有効な段番号(&self, 段番号: usize) -> usize {
        self.段別ジオメトリ.有効な段番号(段番号)
    }

    pub(super) fn 個体数(&self) -> u32 {
        self.内容検査.個体数()
    }

    pub(super) fn 破棄する(&self, device: &GPUデバイス) {
        self.可視id列.破棄する(device);
        self.個体変換.破棄する(device);
        self.段別ジオメトリ.破棄する(device);
    }
}
