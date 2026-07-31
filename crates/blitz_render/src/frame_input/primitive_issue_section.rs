//! プリミティブ発行区間: 1つの描画対象のプリミティブ描画発行が、連ねた1本の発行列のどこに置かれているかを指す。
//! 束IDと描画対象添字を持つのは、レンダラーの走査がこの2つで描画対象を識別するためである。
//! 件数を後から足せるのは、受け皿が発行を1件ずつ積むあいだ、同じ描画対象が続くかぎり同じ区間を伸ばすためである。

use crate::draw_bundle_id::描画束ID;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct プリミティブ発行区間 {
    束id: 描画束ID,
    描画対象添字: usize,
    開始: usize,
    件数: usize,
}

impl プリミティブ発行区間 {
    /// 開いた直後の区間は、その位置の1件だけを含む。
    pub(super) fn 生成する(束id: 描画束ID, 描画対象添字: usize, 開始: usize) -> Self {
        Self {
            束id,
            描画対象添字,
            開始,
            件数: 1,
        }
    }

    pub(super) fn 件数を1つ足す(&mut self) {
        self.件数 += 1;
    }

    pub(super) fn 同じ描画対象か(&self, 束id: 描画束ID, 描画対象添字: usize) -> bool {
        self.束id == 束id && self.描画対象添字 == 描画対象添字
    }

    pub(super) fn 開始(&self) -> usize {
        self.開始
    }

    pub(super) fn 終端(&self) -> Option<usize> {
        self.開始.checked_add(self.件数)
    }
}
