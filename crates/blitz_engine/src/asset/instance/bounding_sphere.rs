//! 原型のローカル空間で全メッシュLOD段を覆う球。個体の細かい可視判定はこの球を個体の配置で写して行う。
//! 段ごとに違う球を持たないのは、段が変わっても覆う範囲が広がってはならないためであり、
//! コンパイラが全段の和集合から1つを焼く。

use super::error::インスタンス群エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 境界球 {
    中心: [f32; 3],
    半径: f32,
}

impl 境界球 {
    pub fn 生成する(中心: [f32; 3], 半径: f32) -> Result<Self, インスタンス群エラー> {
        if !中心.iter().all(|値| 値.is_finite()) {
            return Err(インスタンス群エラー::非有限成分 {
                成分: "境界球の中心"
            });
        }
        if !半径.is_finite() || 半径 <= 0.0 {
            return Err(インスタンス群エラー::境界球半径不正);
        }
        Ok(Self { 中心, 半径 })
    }

    pub fn 中心(&self) -> [f32; 3] {
        self.中心
    }

    pub fn 半径(&self) -> f32 {
        self.半径
    }
}
