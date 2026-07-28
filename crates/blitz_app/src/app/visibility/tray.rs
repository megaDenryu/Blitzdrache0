//! 毎フレーム使い回す選択の受け皿。エンジンが書く段別ID受け皿と、群ごとの選択の一覧を組で持つ。
//! 2つを同じ型が持つのは、選択が指す区間が必ず同じフレームのID列と段の範囲を指すという対応を1箇所で保つためであり、
//! 毎フレームVecを確保して直後に捨てることも避ける。
//! 注意: 描画入力が3本とも借用するため、預けるときも戻すときもこの型ごと動かす。

use blitz_engine::段別ID受け皿;
use blitz_render::{可視個体選択, 段別描画範囲};

#[derive(Default)]
pub(in crate::app) struct 可視選択受け皿 {
    段別: 段別ID受け皿,
    選択一覧: Vec<可視個体選択>,
}

impl 可視選択受け皿 {
    pub(super) fn 生成する() -> Self {
        Self::default()
    }

    pub(super) fn 空にする(&mut self) {
        self.段別.空にする();
        self.選択一覧.clear();
    }

    pub(super) fn id列の長さ(&self) -> usize {
        self.段別.id列の長さ()
    }

    pub(super) fn 段範囲列の長さ(&self) -> usize {
        self.段別.段範囲列の長さ()
    }

    pub(super) fn 段別受け皿を借りる(&mut self) -> &mut 段別ID受け皿 {
        &mut self.段別
    }

    pub(super) fn 選択を積む(&mut self, 選択: 可視個体選択) {
        self.選択一覧.push(選択);
    }

    pub(in crate::app) fn id列(&self) -> &[u32] {
        self.段別.id列()
    }

    pub(in crate::app) fn 段範囲列(&self) -> &[段別描画範囲] {
        self.段別.段範囲列()
    }

    pub(in crate::app) fn 選択一覧(&self) -> &[可視個体選択] {
        &self.選択一覧
    }
}
