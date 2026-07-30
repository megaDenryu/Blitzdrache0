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
    #[error("影の注視点の全成分は有限でなければならない")]
    影注視点不正,
    #[error("影の光源距離は有限かつ正でなければならない")]
    影光源距離不正,
    #[error("影の正射影範囲は有限かつ正で、近クリップが遠クリップより手前でなければならない")]
    影正射影範囲不正,
    #[error("実用分割の混合率は有限かつ0以上1以下でなければならない")]
    混合率不正,
    #[error("距離区分の重なり率は有限かつ0.05以上0.10以下でなければならない")]
    重なり率不正,
    #[error("最大影距離は有限かつ正でなければならない")]
    最大影距離不正,
    #[error("キャスター余白は有限かつ0以上でなければならない")]
    キャスター余白不正,
    #[error("カスケードの構築に使うカメラ視錐台が退化している")]
    カスケード視錐台不正,
}
