//! 派生表現報告が起こしうる失敗。GPUの初期化と読み戻し、CPU正本の畳み込みの入力、シェーダーの読み込みの3種類に分かれる。

use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum 派生表現報告エラー {
    #[error("GPUでの派生表現の生成に失敗した: {0}")]
    GPU生成失敗(#[from] blitz_render::レンダラーエラー),
    #[error("CPU正本の派生表現の計算に失敗した: {0}")]
    CPU計算失敗(#[from] blitz_render::distant_environment::derived::派生表現エラー),
    #[error("埋め込みシェーダーの読み込みに失敗した: {0}")]
    シェーダー読み込み失敗(#[from] crate::error::起動エラー),
}
