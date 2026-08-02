//! 進行中フレームのスロットが、どの資源表世代をまだ使っているかの状態。担当するのは、退役してよいかの判定を
//! GPUのフェンスの実物に依らない純粋な状態遷移として表すことである。
//!
//! フェンスそのものを持たないのは、退役の規律の正しさが「どのスロットが完了を報告したか」だけで決まるためである。
//! 呼び出し元がフェンス待機の後に通過を伝え、この型は誰がまだ保持しているかだけを答える。

use crate::vulkan::sync::{フレームスロット添字, 進行中フレーム数};

use super::generation_id::資源表世代ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum フレーム保持 {
    保持なし,
    進行中 { 世代: 資源表世代ID },
}

pub(in crate::vulkan::material_table) struct フレーム保持状況 {
    スロット別: [フレーム保持; 進行中フレーム数],
}

impl フレーム保持状況 {
    pub(in crate::vulkan::material_table) fn 新規() -> Self {
        Self {
            スロット別: [フレーム保持::保持なし; 進行中フレーム数],
        }
    }

    /// そのスロットで発行するフレームが束縛する世代を記録する。前の記録は、同じスロットのフェンスを待った後にだけ上書きされる。
    pub(in crate::vulkan::material_table) fn 束縛を記録する(
        &mut self, スロット: フレームスロット添字, 世代: 資源表世代ID
    ) {
        self.スロット別[スロット.配列添字()] = フレーム保持::進行中 { 世代 };
    }

    /// そのスロットのフェンス通過を記録する。以後このスロットは何も保持しない。
    pub(in crate::vulkan::material_table) fn 通過を記録する(&mut self, スロット: フレームスロット添字) {
        self.スロット別[スロット.配列添字()] = フレーム保持::保持なし;
    }

    pub(in crate::vulkan::material_table) fn 保持しているか(&self, 世代: 資源表世代ID) -> bool {
        self.スロット別
            .iter()
            .any(|保持| matches!(保持, フレーム保持::進行中 { 世代: 保持中 } if *保持中 == 世代))
    }
}
