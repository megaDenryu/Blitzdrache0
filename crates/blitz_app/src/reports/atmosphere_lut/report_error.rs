//! 大気のベイク済み画像報告が起こしうる失敗。GPUの初期化と読み戻し、CPU正本の計算、シェーダーの読み込みの3種類に分かれる。

use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum 大気のベイク済み画像報告エラー {
    #[error("GPUでの大気のベイク済み画像生成に失敗した: {0}")]
    GPU生成失敗(#[from] blitz_render::レンダラーエラー),
    #[error("CPU正本の大気計算に失敗した: {0}")]
    CPU計算失敗(#[from] blitz_render::atmosphere::大気数学エラー),
    #[error("埋め込みシェーダーの読み込みに失敗した: {0}")]
    シェーダー読み込み失敗(#[from] crate::error::起動エラー),
}
