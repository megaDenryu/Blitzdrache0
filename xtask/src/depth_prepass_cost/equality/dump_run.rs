//! 同値性の検収で撮る1条件1枚ぶんのblitz_app起動。受け取るのは条件とダンプの別とダンプ先、返すものは無い。
//!
//! 費用の計測と別の起動にするのは、1回の起動で読み戻せる画像が1枚だけであり、かつ読み戻しの同期が計測窓を歪めるためである。
//! スモーク実行(`--frames`)で起動するのは、フレームダンプが最終フレームでだけ働く仕組みだからである。

use super::super::schedule::実行条件;
use super::super::world;
use crate::acceptance::{描画フレーム数, 描画検収の実行環境, 書き出しの形式, 検収の実行名};

/// 撮る画像の別。1回の起動で1枚だけ撮る。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ダンプの別 {
    最終深度,
    圧縮前のHDR,
}

impl ダンプの別 {
    /// 器が知る書き出しの形式。選択肢の綴りも拡張子もそちらが持つ。
    pub(super) const fn 書き出しの形式(self) -> 書き出しの形式 {
        match self {
            Self::最終深度 => 書き出しの形式::最終深度,
            Self::圧縮前のHDR => 書き出しの形式::明るさの圧縮前,
        }
    }

    /// 書き出しの基準名に使う前置き。パスはASCIIで保つ。
    pub(super) const fn 実行名の前置き(self) -> &'static str {
        match self {
            Self::最終深度 => "depth",
            Self::圧縮前のHDR => "hdr",
        }
    }

    pub(super) const fn 呼び名(self) -> &'static str {
        match self {
            Self::最終深度 => "最終深度",
            Self::圧縮前のHDR => "提示前HDR",
        }
    }
}

pub(super) fn 撮る(
    実行環境: &描画検収の実行環境,
    条件: &実行条件,
    別: ダンプの別,
    実行名: 検収の実行名,
    一日内秒: Option<&String>,
    フレーム数: 描画フレーム数,
) -> Result<(), String> {
    実行環境.書き出させて報告を採る(
        実行名,
        &world::撮影の起動指定を組み立てる(フレーム数, 条件, 一日内秒),
        別.書き出しの形式(),
    )?;
    Ok(())
}
