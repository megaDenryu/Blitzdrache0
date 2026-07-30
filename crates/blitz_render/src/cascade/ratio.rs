//! 分割の混ぜ方と距離区分の重なりを表す2つの比率の値オブジェクト。どちらも0から1の範囲に意味の制約を持つため、
//! 生のf32でなく検証付きの型で持つ。触れる状態は自分が包む1つの比率だけである。

use crate::lighting_input::ライティング入力エラー;

/// 一様分割と対数分割の混ぜ方。0で一様分割、1で対数分割になる。
#[derive(Debug, Clone, Copy)]
pub struct 実用分割混合率(f32);

impl 実用分割混合率 {
    pub fn 生成する(値: f32) -> Result<Self, ライティング入力エラー> {
        if !値.is_finite() || !(0.0..=1.0).contains(&値) {
            return Err(ライティング入力エラー::混合率不正);
        }
        Ok(Self(値))
    }

    pub(super) fn 値(self) -> f32 {
        self.0
    }
}

/// 隣り合う距離区分が重なる割合。手前の距離区分の深度幅に対する比で表し、その幅ぶん次の距離区分が近側へ伸びる。
#[derive(Debug, Clone, Copy)]
pub struct 距離区分の重なり率(f32);

impl 距離区分の重なり率 {
    pub fn 生成する(値: f32) -> Result<Self, ライティング入力エラー> {
        if !値.is_finite() || !(0.05..=0.10).contains(&値) {
            return Err(ライティング入力エラー::重なり率不正);
        }
        Ok(Self(値))
    }

    pub(super) fn 値(self) -> f32 {
        self.0
    }
}

impl 実用分割混合率 {
    /// 設計正本が初期値として定めた0.6。
    pub(super) fn 既定() -> Self {
        Self(0.6)
    }
}

impl 距離区分の重なり率 {
    /// 設計正本が定めた5パーセントから10パーセントの範囲の中央付近。
    pub(super) fn 既定() -> Self {
        Self(0.07)
    }
}
