//! 太陽の向きを表す単位ベクトル。

use blitz_math::ラジアン;

use super::天空状態エラー;

/// 地表から太陽へ向かう単位ベクトル。ワールド軸は X が東、Y が天頂、Z が南であり、北は Z の負の向きである。
/// 不変条件: 3成分は有限であり、長さが1である(生成時に正規化する)。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 太陽方向 {
    成分: [f32; 3],
}

impl 太陽方向 {
    pub fn 生成する(東: f32, 天頂: f32, 南: f32) -> Result<Self, 天空状態エラー> {
        let 成分 = [東, 天頂, 南];
        let 長さ二乗: f32 = 成分.iter().map(|値| 値 * 値).sum();
        if !長さ二乗.is_finite() || 長さ二乗 <= f32::EPSILON {
            return Err(天空状態エラー::太陽方向が単位ベクトルでない);
        }
        let 逆長さ = 長さ二乗.sqrt().recip();
        Ok(Self {
            成分: 成分.map(|値| 値 * 逆長さ),
        })
    }

    pub fn 成分(&self) -> [f32; 3] {
        self.成分
    }

    /// 地平面からの仰角。天頂成分が単位ベクトルの正弦であることから直接求める。
    pub fn 高度(&self) -> ラジアン {
        ラジアン::生成する(self.成分[1].clamp(-1.0, 1.0).asin())
    }
}
