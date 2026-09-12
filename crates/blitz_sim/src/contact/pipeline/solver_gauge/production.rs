//! 本番の構成の解法の計器(判断12・判断13)。何も持たず、何も数えない。試験の構成(親モジュール)から
//! 分けているのは、こちらが空のメソッドの並びであり、最適化で消えることだけが役目だからである。

use crate::contact::normal_tangential_system::{
    接触点集合の法線と接線の連立方程式, 混合連立方程式の受理の結果, 相補条件を満たす有効集合を探した結末, 部分集合を解いた回数,
};

#[cfg(not(test))]
pub(in crate::contact::pipeline) struct 解法の計器;

#[cfg(not(test))]
impl 解法の計器 {
    pub(in crate::contact::pipeline) fn 零から始める() -> Self {
        Self
    }

    /// 1行も積んでいない連立方程式。本番の構成は適用規則を持たないため、本番と同じ適用規則の連立方程式である。
    pub(in crate::contact::pipeline) fn 空の連立方程式を組む(
        &self, 点の数: usize
    ) -> 接触点集合の法線と接線の連立方程式 {
        接触点集合の法線と接線の連立方程式::点の数から空で始める(点の数)
    }

    /// 本番の構成は何も数えない。
    pub(in crate::contact::pipeline) fn 粘着の候補の求解を数える(
        &self,
        _結末: &相補条件を満たす有効集合を探した結末,
        _回数: 部分集合を解いた回数,
    ) {
    }

    /// 本番の構成は何も数えない。
    pub(in crate::contact::pipeline) fn 受理の判定を数える(
        &self,
        _結果: &混合連立方程式の受理の結果,
        _連立方程式: &接触点集合の法線と接線の連立方程式,
    ) {
    }
}
