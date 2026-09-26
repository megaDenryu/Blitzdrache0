//! 旅行者のキーボード入力と、その解釈(Issue #137)。
//!
//! キーボードに固有の形のままの生入力(M生入力: キーボード歩行入力)から、ゲーム内のコマンド(Mコマンド: 旅行者の意図)を
//! 導出する純粋計算(M解釈関数)を、入力自身のメソッドとして持つ。

use blitz_design::{M不変データ, M生入力, M解釈前の入力, M解釈関数};

use crate::traveler::旅行者の意図;
use crate::walking_direction::歩行方向;

/// キーボードの上下左右キー押下状態を表す生シグナルDTO。ゲーム意味論を持たず、物理的な押下状態のみを保持する。
/// 同じ真偽の4つを位置の引数で受ける生成の口を置かず、何も押していない既定の値からキーごとの口で1つずつ置き換えるのは、取り違えても型が通るためである。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct キーボード歩行入力 {
    上: bool,
    下: bool,
    左: bool,
    右: bool,
}

impl M不変データ for キーボード歩行入力 {}
impl M解釈前の入力 for キーボード歩行入力 {}
impl M生入力 for キーボード歩行入力 {}

impl キーボード歩行入力 {
    pub fn 上を押しているかを置き換えた入力(self, 押しているか: bool) -> Self {
        Self { 上: 押しているか, ..self }
    }

    pub fn 下を押しているかを置き換えた入力(self, 押しているか: bool) -> Self {
        Self { 下: 押しているか, ..self }
    }

    pub fn 左を押しているかを置き換えた入力(self, 押しているか: bool) -> Self {
        Self { 左: 押しているか, ..self }
    }

    pub fn 右を押しているかを置き換えた入力(self, 押しているか: bool) -> Self {
        Self { 右: 押しているか, ..self }
    }

    pub fn 上(&self) -> bool {
        self.上
    }

    pub fn 下(&self) -> bool {
        self.下
    }

    pub fn 左(&self) -> bool {
        self.左
    }

    pub fn 右(&self) -> bool {
        self.右
    }

    // 東西の軸の倒し量(-1・0・1)。右が正であり、左右が両方押されていれば相殺して0である。
    pub(crate) fn 東西の倒し量(&self) -> f32 {
        match (self.右, self.左) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
    }

    // 南北の軸の倒し量(-1・0・1)。上(北)が正であり、上下が両方押されていれば相殺して0である。
    pub(crate) fn 南北の倒し量(&self) -> f32 {
        match (self.上, self.下) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            _ => 0.0,
        }
    }

    /// キーの押下状態から旅行者の歩行の意図を導く。向きが導けなければ静止である。
    pub fn 歩行入力を解釈する(&self) -> 旅行者の意図 {
        match 歩行方向::キー入力から導く(self) {
            Some(方向) => 旅行者の意図::歩く { 方向 },
            None => 旅行者の意図::静止,
        }
    }
}

// 歩行入力を解釈する が解釈の役割(M解釈前の入力 から Mコマンド)を満たすことをコンパイラに確かめさせる。
const _: M解釈関数<キーボード歩行入力, 旅行者の意図> = M解釈関数::生成する(キーボード歩行入力::歩行入力を解釈する);
