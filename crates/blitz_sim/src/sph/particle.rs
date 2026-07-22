//! SPHの1流体粒子。

use super::error::Sph仕様エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 流体粒子 {
    位置: [f32; 3],
    速度: [f32; 3],
    質量: f32,
}

impl 流体粒子 {
    pub fn 生成する(位置: [f32; 3], 速度: [f32; 3], 質量: f32) -> Result<Self, Sph仕様エラー> {
        if 位置.iter().any(|成分| !成分.is_finite()) {
            return Err(Sph仕様エラー::粒子ベクトルが不正 { 項目: "位置" });
        }
        if 速度.iter().any(|成分| !成分.is_finite()) {
            return Err(Sph仕様エラー::粒子ベクトルが不正 { 項目: "速度" });
        }
        if !質量.is_finite() || 質量 <= 0.0 {
            return Err(Sph仕様エラー::粒子質量が不正 { 指定値: 質量 });
        }
        Ok(Self { 位置, 速度, 質量 })
    }

    pub fn 位置(&self) -> [f32; 3] {
        self.位置
    }
    pub fn 速度(&self) -> [f32; 3] {
        self.速度
    }
    pub fn 質量(&self) -> f32 {
        self.質量
    }

    pub(crate) fn 更新する(&self, 位置: [f32; 3], 速度: [f32; 3]) -> Self {
        Self {
            位置, 速度, 質量: self.質量
        }
    }
}
