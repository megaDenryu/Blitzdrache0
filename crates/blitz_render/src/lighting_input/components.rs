//! ライティング入力の色・強度・環境光係数を検証済み値にする。

use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct 光色([f32; 3]);

impl 光色 {
    pub fn 生成する(r: f32, g: f32, b: f32) -> Result<Self, ライティング入力エラー> {
        let 値 = [r, g, b];
        if 値.iter().any(|成分| !成分.is_finite() || *成分 < 0.0) {
            return Err(ライティング入力エラー::色不正);
        }
        Ok(Self(値))
    }

    pub(crate) fn 配列(self) -> [f32; 3] {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct 光強度(f32);

impl 光強度 {
    pub fn 生成する(値: f32) -> Result<Self, ライティング入力エラー> {
        if !値.is_finite() || 値 < 0.0 {
            return Err(ライティング入力エラー::強度不正);
        }
        Ok(Self(値))
    }

    pub(crate) fn 値(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct 環境光係数(f32);

impl 環境光係数 {
    pub fn 生成する(値: f32) -> Result<Self, ライティング入力エラー> {
        if !値.is_finite() || !(0.0..=1.0).contains(&値) {
            return Err(ライティング入力エラー::環境光係数不正);
        }
        Ok(Self(値))
    }

    pub(crate) fn 値(self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Error, PartialEq, Eq)]
pub enum ライティング入力エラー {
    #[error("光色は有限かつ0以上でなければならない")]
    色不正,
    #[error("光強度は有限かつ0以上でなければならない")]
    強度不正,
    #[error("環境光係数は有限かつ0以上1以下でなければならない")]
    環境光係数不正,
    #[error("方向光の向きは有限かつゼロ長であってはならない")]
    方向不正,
    #[error("点光源の位置は有限でなければならない")]
    点光源位置不正,
    #[error("影変換の全成分は有限でなければならない")]
    影変換不正,
}
