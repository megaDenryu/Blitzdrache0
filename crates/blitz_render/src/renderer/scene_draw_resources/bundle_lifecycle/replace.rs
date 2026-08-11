//! 同じ束IDの束を新しい内容へ入れ替える操作。触れるのは`チャンク一覧`と`破棄待ち`の2つだけであり、
//! 追加と解除が持つ規律をそのまま使う。
//!
//! 新しい束のGPU資源を先に作り、成功したときにだけ旧い束を破棄待ちへ移す。追加と解除を呼び出し元が並べる形にすると、
//! 新しい束の確保が失敗したときに旧い束を既に外しており、描くものが1つも無い状態が残る。

use super::super::chunk_draw_resources::チャンク描画資源;
use super::super::create::束追加材料;
use super::super::シーン描画資源;
use crate::draw_bundle_id::描画束ID;
use crate::error::レンダラーエラー;
use crate::render_object_material::描画対象素材;
use crate::vulkan::allocator::GPU資源の確保係;

impl シーン描画資源 {
    pub(in crate::renderer) fn 束を差し替える(
        &mut self,
        確保係: &GPU資源の確保係<'_>,
        材料: 束追加材料<'_>,
        id: 描画束ID,
        描画対象一覧: &[描画対象素材],
    ) -> Result<(), レンダラーエラー> {
        let 新しい束 = チャンク描画資源::生成する(確保係, 材料, id, 描画対象一覧)?;
        self.束を解除予約する(id);
        self.チャンク一覧.push(新しい束);
        Ok(())
    }
}
