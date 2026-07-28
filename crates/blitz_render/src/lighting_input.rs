//! レンダラーが1フレームのライティングに必要とする安全な入力契約。

mod components;
mod lights;
mod shadow;

pub use components::{ライティング入力エラー, 光強度, 光色, 環境光係数};
pub use lights::{方向光入力, 点光源入力};
pub use shadow::{影入力, 影正射影範囲};

#[derive(Debug, Clone, Copy)]
pub struct ライティング入力 {
    方向光: 方向光入力,
    点光源: 点光源入力,
    環境光: 環境光係数,
    影: 影入力,
    有効: bool,
}

impl ライティング入力 {
    pub fn 生成する(方向光: 方向光入力, 点光源: 点光源入力, 環境光: 環境光係数, 影: 影入力, 有効: bool) -> Self {
        Self {
            方向光,
            点光源,
            環境光,
            影,
            有効,
        }
    }

    pub(crate) fn 方向光(&self) -> 方向光入力 {
        self.方向光
    }

    pub(crate) fn 点光源(&self) -> 点光源入力 {
        self.点光源
    }

    pub fn 点光源位置(&self) -> blitz_math::大域ワールド位置 {
        self.点光源.位置()
    }

    pub(crate) fn 環境光(&self) -> 環境光係数 {
        self.環境光
    }

    pub(crate) fn 影(&self) -> 影入力 {
        self.影
    }

    pub fn 影注視点(&self) -> blitz_math::大域ワールド位置 {
        self.影.注視点()
    }

    pub(crate) fn 有効か(&self) -> bool {
        self.有効
    }
}
