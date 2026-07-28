//! 方向光と点光源の型付き入力。

use blitz_math::{メートル, ワールド, 位置, 大域ワールド位置};

use super::{ライティング入力エラー, 光強度, 光色};

#[derive(Debug, Clone, Copy)]
pub struct 方向光入力 {
    方向: [f32; 3],
    色: 光色,
    強度: 光強度,
}

impl 方向光入力 {
    pub fn 生成する(x: f32, y: f32, z: f32, 色: 光色, 強度: 光強度) -> Result<Self, ライティング入力エラー> {
        let 長さ二乗 = x * x + y * y + z * z;
        if !長さ二乗.is_finite() || 長さ二乗 <= f32::EPSILON {
            return Err(ライティング入力エラー::方向不正);
        }
        let 逆長さ = 長さ二乗.sqrt().recip();
        Ok(Self {
            方向: [x * 逆長さ, y * 逆長さ, z * 逆長さ],
            色,
            強度,
        })
    }

    pub(crate) fn 方向(self) -> [f32; 3] {
        self.方向
    }

    pub fn 逆方向へ離れた位置(self, 距離: メートル) -> 位置<ワールド> {
        let 距離 = 距離.値();
        位置::生成する(
            メートル::生成する(-self.方向[0] * 距離),
            メートル::生成する(-self.方向[1] * 距離),
            メートル::生成する(-self.方向[2] * 距離),
        )
    }

    pub(crate) fn 色(self) -> [f32; 3] {
        self.色.配列()
    }

    pub(crate) fn 強度(self) -> f32 {
        self.強度.値()
    }
}

/// 位置は大域ワールド位置で持つ。カメラ相対フレームへの狭めは毎フレームのUBO書き込みが行うため、ここでは大きな座標のまま保つ。
#[derive(Debug, Clone, Copy)]
pub struct 点光源入力 {
    位置: 大域ワールド位置,
    色: 光色,
    強度: 光強度,
}

impl 点光源入力 {
    pub fn 生成する(位置: 大域ワールド位置, 色: 光色, 強度: 光強度) -> Result<Self, ライティング入力エラー> {
        if [位置.x().値(), 位置.y().値(), 位置.z().値()].iter().any(|成分| !成分.is_finite()) {
            return Err(ライティング入力エラー::点光源位置不正);
        }
        Ok(Self { 位置, 色, 強度 })
    }

    pub(crate) fn 位置(self) -> 大域ワールド位置 {
        self.位置
    }

    pub(crate) fn 色(self) -> [f32; 3] {
        self.色.配列()
    }

    pub(crate) fn 強度(self) -> f32 {
        self.強度.値()
    }
}
