//! 高さ場を25メートル刻みの粗い格子として参照する器。

use blitz_engine::height_field::高さ場;

use super::error::遠景コンパイルエラー;

pub(super) const 細分数: u32 = 16;
pub(super) const 粗い刻みメートル: f32 = 25.0;

pub(super) struct 遠景格子 {
    pub(super) 高さ場: 高さ場,
    pub(super) 東セル数: u32,
    pub(super) 南セル数: u32,
}

impl 遠景格子 {
    pub(super) fn 高さ場から作る(高さ場: 高さ場) -> Result<Self, 遠景コンパイルエラー> {
        let 諸元 = 高さ場.諸元();
        let 東区間 = 諸元.東方向の格子点数() - 1;
        let 南区間 = 諸元.南方向の格子点数() - 1;
        let 刻み差 =
            諸元.格子間隔().値() * f32::from(u16::try_from(細分数).map_err(|_| 遠景コンパイルエラー::格子添字表現不能)?) - 粗い刻みメートル;
        if !東区間.is_multiple_of(細分数) || !南区間.is_multiple_of(細分数) || 刻み差.abs() > 1.0e-6 {
            return Err(遠景コンパイルエラー::格子寸法不一致);
        }
        Ok(Self {
            高さ場,
            東セル数: 東区間 / 細分数,
            南セル数: 南区間 / 細分数,
        })
    }

    pub(super) fn 細かい高さ(&self, 東: u32, 南: u32) -> Result<f32, 遠景コンパイルエラー> {
        Ok(self.高さ場.格子点の高さ(東, 南)?.値())
    }

    pub(super) fn 粗い高さ(&self, 東: u32, 南: u32) -> Result<f32, 遠景コンパイルエラー> {
        let 細東 = 東.checked_mul(細分数).ok_or(遠景コンパイルエラー::格子添字表現不能)?;
        let 細南 = 南.checked_mul(細分数).ok_or(遠景コンパイルエラー::格子添字表現不能)?;
        self.細かい高さ(細東, 細南)
    }

    pub(super) fn 頂点添字(&self, 東: u32, 南: u32) -> Result<usize, 遠景コンパイルエラー> {
        let 幅 = self.東セル数 + 1;
        let 添字 = 南
            .checked_mul(幅)
            .and_then(|先頭| 先頭.checked_add(東))
            .ok_or(遠景コンパイルエラー::格子添字表現不能)?;
        usize::try_from(添字).map_err(|_| 遠景コンパイルエラー::格子添字表現不能)
    }
}

pub(super) fn 三角形の高さ(左上: f32, 右上: f32, 左下: f32, 右下: f32, 東比: f32, 南比: f32) -> f32 {
    if 東比 <= 南比 {
        左上 + (左下 - 左上) * 南比 + (右下 - 左下) * 東比
    } else {
        左上 + (右上 - 左上) * 東比 + (右下 - 右上) * 南比
    }
}
