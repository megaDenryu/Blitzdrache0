//! ワーカーが返すシーン、読込量、所要時間と型付き失敗。

use std::time::Duration;

use crate::streaming::reset_generation::リセット世代;
use crate::{シーンデータ, チャンク座標, 実行時シーン読込エラー};

#[derive(Debug)]
pub struct チャンク読込成果 {
    pub シーン: シーンデータ,
    pub 読込バイト数: usize,
    pub 所要時間: Duration,
}

#[derive(Debug)]
pub struct チャンク読込完了 {
    チャンク: チャンク座標,
    世代: リセット世代,
    結果: Result<チャンク読込成果, 実行時シーン読込エラー>,
}

impl チャンク読込完了 {
    pub(super) fn 生成する(
        チャンク: チャンク座標, 世代: リセット世代, 結果: Result<チャンク読込成果, 実行時シーン読込エラー>
    ) -> Self {
        Self {
            チャンク, 世代, 結果
        }
    }

    pub fn チャンク(&self) -> チャンク座標 {
        self.チャンク
    }

    /// 読込を投入した時点のリセット世代。座標だけでは、リセットの前後で同じ座標へ並んだ2件の読込を区別できない。
    pub fn 世代(&self) -> リセット世代 {
        self.世代
    }

    pub fn 結果(self) -> Result<チャンク読込成果, 実行時シーン読込エラー> {
        self.結果
    }
}
