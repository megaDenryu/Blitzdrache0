//! 毎フレーム使い回す選択の受け皿。エンジンが書く段別ID受け皿と、群ごとの選択の一覧と、そのフレームの計数を持つ。
//! 3つを同じ型が持つのは、選択が指す区間が必ず同じフレームのID列と区間を指し、計数がその中身を数えた値であるという
//! 対応を1箇所で保つためであり、毎フレームVecを確保して直後に捨てることも避ける。
//! 注意: 描画入力が借用するため、預けるときも戻すときもこの型ごと動かす。預けている間の中身は空であり計数も0である。

use blitz_engine::{可視判定計数, 段別ID受け皿};
use blitz_render::{可視個体選択, 段別描画範囲};

#[derive(Default)]
pub(in crate::app) struct 可視選択受け皿 {
    段別: 段別ID受け皿,
    選択一覧: Vec<可視個体選択>,
    /// そのフレームの全群の計数の合計。判定の内側でしか数えられないため、選択と同じ型が最後の値を保持して報告へ渡す。
    計数: 可視判定計数,
}

impl 可視選択受け皿 {
    pub(super) fn 生成する() -> Self {
        Self::default()
    }

    pub(super) fn 空にする(&mut self) {
        self.段別.空にする();
        self.選択一覧.clear();
        self.計数 = 可視判定計数::default();
    }

    pub(super) fn 計数を据える(&mut self, 計数: 可視判定計数) {
        self.計数 = 計数;
    }

    /// 直近のフレームに全群の判定が数えた計数。レンダラーが積んだ発行から数える計数とは出どころが違い、
    /// 2つを突き合わせることが整合の証拠になる。
    pub(in crate::app) fn 計数(&self) -> 可視判定計数 {
        self.計数
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
