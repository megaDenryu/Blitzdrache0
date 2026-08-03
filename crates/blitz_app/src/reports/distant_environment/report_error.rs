//! 遠方環境報告が起こしうる失敗。GPUの初期化と読み戻し、CPU正本の計算、方針の組み立て、シェーダーの読み込みの4種類に分かれる。

use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum 遠方環境報告エラー {
    #[error("GPUでの遠方環境生成に失敗した: {0}")]
    GPU生成失敗(#[from] blitz_render::レンダラーエラー),
    #[error("CPU正本の大気計算に失敗した: {0}")]
    CPU計算失敗(#[from] blitz_render::atmosphere::大気数学エラー),
    #[error("対照に使う空描画方針を組めなかった: {0}")]
    方針の組み立て失敗(#[from] blitz_engine::sky::天空状態エラー),
    #[error("埋め込みシェーダーの読み込みに失敗した: {0}")]
    シェーダー読み込み失敗(#[from] crate::error::起動エラー),
}
