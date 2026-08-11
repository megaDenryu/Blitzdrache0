//! 明るさの圧縮の前にHDR輝度へ掛ける露出倍率。担当するのは、正の有限値だけを受け取ることと、
//! 開発UIのスライダーが動かせる範囲を自分で決めることである。
//!
//! 裸の`f32`で持たないのは、この値と自動露出の補正段(2の冪で表す段数)と露出の経過秒が、どれも`f32`の姿で
//! 同じ経路を流れるためである。倍率と段数を取り違えると絵は破綻せず、明るさが黙って別の値になる。

use std::ops::RangeInclusive;

use thiserror::Error;

/// 露出倍率が受け取れない値だったこと。
#[derive(Debug, Error)]
pub(crate) enum 露出倍率エラー {
    #[error("正の有限値でない: {実測}")]
    正の有限値でない { 実測: f32 },
}

/// 明るさの圧縮の前にHDR輝度へ掛ける倍率。1.0が起動時の既定である。
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(transparent)]
pub(crate) struct 露出倍率(f32);

/// 開発UIのスライダーが動かせる下限と上限。上下2段の明暗を確認できる幅であり、どちらも受理範囲の内側にある。
const スライダーの下限: f32 = 0.25;
const スライダーの上限: f32 = 4.0;

impl 露出倍率 {
    pub(crate) fn 生成する(値: f32) -> Result<Self, 露出倍率エラー> {
        if !値.is_finite() || 値 <= 0.0 {
            return Err(露出倍率エラー::正の有限値でない { 実測: 値 });
        }
        Ok(Self(値))
    }

    /// 起動指定が無いときの倍率。明るさを1つも動かさない。
    pub(crate) fn 既定() -> Self {
        Self(1.0)
    }

    /// 2の冪で表した補正段を掛けた倍率。時刻別固定の枝が天空状態の露出補正段を反映するのに使う。
    pub(crate) fn 補正段を掛ける(self, 補正段: f32) -> Self {
        Self(self.0 * 2.0_f32.powf(補正段))
    }

    /// GPU境界向けの生値取り出し。フレーム描画入力へ載せる1箇所だけが使う。
    pub(crate) fn 値(self) -> f32 {
        self.0
    }

    /// 開発UIのスライダーが直に書き換える生値。eguiが`&mut f32`しか受けないための境界である。
    /// 範囲の外へは動かないことを`スライダーの範囲`が保証する。
    pub(crate) fn スライダーへ貸す(&mut self) -> &mut f32 {
        &mut self.0
    }

    pub(crate) fn スライダーの範囲() -> RangeInclusive<f32> {
        スライダーの下限..=スライダーの上限
    }
}
