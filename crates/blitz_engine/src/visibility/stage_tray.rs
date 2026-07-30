//! 毎フレーム使い回す選択の受け皿。全群のID列を連ねた1本の列と、全群の段のパス別区間を連ねた1本の列を組で持つ。
//! 2つを同じ型が持つのは、ある群の区間が必ず同じ群のID区間を指すという対応を1箇所で保つためであり、
//! 毎フレームVecを確保して直後に捨てることも避ける。
//! 並べ替えの作業領域も同じ型が持つ。作業領域は群ごとに使い切るが、確保した容量はフレームをまたいで残る。

use blitz_render::段別描画範囲;

use super::count::群のパス別可視数;
use super::stage_bucket::段別並べ替え;
use super::visible_pass_set::可視パス集合;

#[derive(Default)]
pub struct 段別ID受け皿 {
    id列: Vec<u32>,
    段範囲列: Vec<段別描画範囲>,
    並べ替え: 段別並べ替え,
}

impl 段別ID受け皿 {
    pub fn 生成する() -> Self {
        Self::default()
    }

    pub fn 空にする(&mut self) {
        self.id列.clear();
        self.段範囲列.clear();
    }

    pub fn id列(&self) -> &[u32] {
        &self.id列
    }

    pub fn 段範囲列(&self) -> &[段別描画範囲] {
        &self.段範囲列
    }

    pub fn id列の長さ(&self) -> usize {
        self.id列.len()
    }

    pub fn 段範囲列の長さ(&self) -> usize {
        self.段範囲列.len()
    }

    pub(super) fn 群を始める(&mut self, 段数: usize) {
        self.並べ替え.群を始める(段数);
    }

    pub(super) fn 個体を積む(&mut self, 段番号: u8, 可視: 可視パス集合) {
        self.並べ替え.個体を積む(段番号, 可視);
    }

    /// 積んだ判定をパス別・段別の並びへ書き出し、その群のパスごとの可視数を返す。
    pub(super) fn 群を書き出す(&mut self) -> 群のパス別可視数 {
        let パス別可視数 = self.並べ替え.パス別可視数();
        self.並べ替え.書き出す(&mut self.id列, &mut self.段範囲列);
        パス別可視数
    }
}
