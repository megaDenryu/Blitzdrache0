//! 表面流の格子と時間発展を定める検証済み仕様。

use super::error::表面流仕様エラー;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct 表面流仕様 {
    列数: u32,
    行数: u32,
    セル幅: f32,
    時間刻み: f32,
    重力接線成分: [f32; 2],
    速度減衰: f32,
}

impl 表面流仕様 {
    pub fn 生成する(
        格子寸法: [u32; 2],
        セル幅: f32,
        時間刻み: f32,
        重力接線成分: [f32; 2],
        速度減衰: f32,
    ) -> Result<Self, 表面流仕様エラー> {
        let [列数, 行数] = 格子寸法;
        if 列数 < 2 || 行数 < 2 {
            return Err(表面流仕様エラー::格子が小さすぎる { 列数, 行数 });
        }
        if !セル幅.is_finite() || セル幅 <= 0.0 {
            return Err(表面流仕様エラー::セル幅が不正 { 指定値: セル幅 });
        }
        if !時間刻み.is_finite() || 時間刻み <= 0.0 {
            return Err(表面流仕様エラー::時間刻みが不正 { 指定値: 時間刻み });
        }
        if !速度減衰.is_finite() || !(0.0..=1.0).contains(&速度減衰) {
            return Err(表面流仕様エラー::速度減衰が不正 { 指定値: 速度減衰 });
        }
        Ok(Self {
            列数,
            行数,
            セル幅,
            時間刻み,
            重力接線成分,
            速度減衰,
        })
    }

    pub fn 格子寸法(&self) -> [u32; 2] {
        [self.列数, self.行数]
    }
    pub(crate) fn セル幅(&self) -> f32 {
        self.セル幅
    }
    pub(crate) fn 時間刻み(&self) -> f32 {
        self.時間刻み
    }
    pub(crate) fn 重力接線成分(&self) -> [f32; 2] {
        self.重力接線成分
    }
    pub(crate) fn 速度減衰(&self) -> f32 {
        self.速度減衰
    }
}
