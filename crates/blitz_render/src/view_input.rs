//! 1フレーム内の非空な視点一覧と、各視点のカメラ入力。

use blitz_math::{クリップ, ワールド, 位置, 変換, 大域ワールド位置};

#[derive(Debug, Clone)]
pub struct 描画視点 {
    ビュー射影: 変換<ワールド, クリップ>,
    カメラ位置: 位置<ワールド>,
    カメラ大域原点: 大域ワールド位置,
}

impl 描画視点 {
    pub fn 生成する(
        ビュー射影: 変換<ワールド, クリップ>, カメラ位置: 位置<ワールド>, カメラ大域原点: 大域ワールド位置
    ) -> Self {
        Self {
            ビュー射影,
            カメラ位置,
            カメラ大域原点,
        }
    }

    pub(crate) fn ビュー射影(&self) -> 変換<ワールド, クリップ> {
        self.ビュー射影
    }

    pub(crate) fn カメラ位置(&self) -> 位置<ワールド> {
        self.カメラ位置
    }

    pub fn カメラ大域原点(&self) -> 大域ワールド位置 {
        self.カメラ大域原点
    }
}

#[derive(Debug, Clone)]
pub struct 描画視点一覧 {
    先頭: 描画視点,
    残り: Vec<描画視点>,
}

impl 描画視点一覧 {
    pub fn 生成する(先頭: 描画視点, 残り: Vec<描画視点>) -> Self {
        Self { 先頭, 残り }
    }

    pub fn 視点数(&self) -> usize {
        self.残り.len() + 1
    }

    pub(crate) fn 先頭(&self) -> &描画視点 {
        &self.先頭
    }
}
