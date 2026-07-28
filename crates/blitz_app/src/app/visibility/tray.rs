//! 毎フレーム使い回す可視判定の受け皿。全群の可視IDを連ねた1本の列と、群ごとの選択の一覧を組で持つ。
//! 2つを同じ型が持つのは、選択が指す区間が必ず同じフレームのID列を指すという対応を1箇所で保つためであり、
//! 毎フレームVecを確保して直後に捨てることも避ける。
//! 注意: 描画入力が2本とも借用するため、預けるときも戻すときもこの型ごと動かす。

use blitz_render::可視個体選択;

#[derive(Default)]
pub(in crate::app) struct 可視選択受け皿 {
    id列: Vec<u32>,
    選択一覧: Vec<可視個体選択>,
}

impl 可視選択受け皿 {
    pub(super) fn 生成する() -> Self {
        Self::default()
    }

    pub(super) fn 空にする(&mut self) {
        self.id列.clear();
        self.選択一覧.clear();
    }

    pub(super) fn id列の長さ(&self) -> usize {
        self.id列.len()
    }

    pub(super) fn id列を借りる(&mut self) -> &mut Vec<u32> {
        &mut self.id列
    }

    pub(super) fn 選択を積む(&mut self, 選択: 可視個体選択) {
        self.選択一覧.push(選択);
    }

    pub(in crate::app) fn id列(&self) -> &[u32] {
        &self.id列
    }

    pub(in crate::app) fn 選択一覧(&self) -> &[可視個体選択] {
        &self.選択一覧
    }
}
