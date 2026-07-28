//! 1つの描画対象について、そのフレームの可視ID列と段別描画範囲の中でその対象が使う区間を指す。
//! IDも範囲も所有しないのは、全群のぶんを1本ずつの列へ連ねて毎フレームのヒープ確保をなくすためである。

use crate::draw_bundle_id::描画束ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct 可視個体選択 {
    束id: 描画束ID,
    描画対象添字: usize,
    id列開始: usize,
    id列長: usize,
    段範囲開始: usize,
    段数: usize,
}

impl 可視個体選択 {
    pub fn 生成する(
        束id: 描画束ID, 描画対象添字: usize, id列開始: usize, id列長: usize, 段範囲開始: usize, 段数: usize
    ) -> Self {
        Self {
            束id,
            描画対象添字,
            id列開始,
            id列長,
            段範囲開始,
            段数,
        }
    }

    pub(super) fn 束id(self) -> 描画束ID {
        self.束id
    }

    pub(super) fn 描画対象添字(self) -> usize {
        self.描画対象添字
    }

    pub(super) fn id列開始(self) -> usize {
        self.id列開始
    }

    pub(super) fn id列終端(self) -> Option<usize> {
        self.id列開始.checked_add(self.id列長)
    }

    pub(super) fn 段範囲開始(self) -> usize {
        self.段範囲開始
    }

    pub(super) fn 段範囲終端(self) -> Option<usize> {
        self.段範囲開始.checked_add(self.段数)
    }
}
