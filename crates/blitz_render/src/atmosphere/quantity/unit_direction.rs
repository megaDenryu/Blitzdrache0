//! ワールドの向きを表す単位ベクトル。軸は東・天頂・南であり、天頂が+Yである。

use crate::atmosphere::大気数学エラー;

/// 長さ1に正規化済みのワールドの向き。大域原点に依らない量であるため、カメラ相対契約を破らずに運べる。
///
/// 内側をf64で持つのは、この向きが積分の各標本点の位置を決め、惑星半径6.36e6メートルの桁と足し合わされるためである。
/// 入口をf32で受けるのは、呼び出し元がGPU境界の値かエンジンの方針の値であり、どちらもf32だからである。
/// 不変条件: 3成分は有限であり、ベクトルの長さは1である。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 単位方向 {
    成分: [f64; 3],
}

impl 単位方向 {
    pub fn 生成する(東: f32, 天頂: f32, 南: f32) -> Result<Self, 大気数学エラー> {
        let 成分 = [f64::from(東), f64::from(天頂), f64::from(南)];
        let 長さ二乗: f64 = 成分.iter().map(|値| 値 * 値).sum();
        if !長さ二乗.is_finite() || 長さ二乗 <= 0.0 {
            return Err(大気数学エラー::値域外(
                "単位方向の長さ",
                crate::atmosphere::narrowing::実数へ狭める(長さ二乗),
            ));
        }
        let 逆長さ = 長さ二乗.sqrt().recip();
        Ok(Self {
            成分: 成分.map(|値| 値 * 逆長さ),
        })
    }

    pub fn 成分(&self) -> [f32; 3] {
        self.成分.map(crate::atmosphere::narrowing::実数へ狭める)
    }

    pub(in crate::atmosphere) fn 生値(&self) -> [f64; 3] {
        self.成分
    }
}
