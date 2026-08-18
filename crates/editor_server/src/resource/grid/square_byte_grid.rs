//! 正方形バイト格子。マザーハイトマップ・チャンクの高さ編集・地表材質の重みが
//! 合成(has-a)で使う共通の土台であり、一辺の頂点数とバイト列の対応を1箇所で守る。
//! JSONへは載らないためts-rsを持たず、Rust側だけの値オブジェクトとして private フィールドで包む。

use super::square_grid_validation::バイト長を検証する;
use crate::resource::validation_error::資源検証エラー;

pub(super) struct 正方形バイト格子 {
    一辺の頂点数: u32,
    バイト列: Vec<u8>,
}

impl 正方形バイト格子 {
    pub(super) fn 生成する(
        一辺の頂点数: u32, 一頂点あたりバイト数: u32, バイト列: Vec<u8>
    ) -> Result<Self, 資源検証エラー> {
        バイト長を検証する(一辺の頂点数, 一頂点あたりバイト数, &バイト列)?;
        Ok(Self {
            一辺の頂点数, バイト列
        })
    }

    pub(super) fn 一辺の頂点数(&self) -> u32 {
        self.一辺の頂点数
    }

    pub(super) fn バイト列(&self) -> &[u8] {
        &self.バイト列
    }

    pub(super) fn バイト列を取り出す(self) -> Vec<u8> {
        self.バイト列
    }
}
