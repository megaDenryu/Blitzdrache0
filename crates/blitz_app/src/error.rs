//! blitz_app（起動バイナリ）の型付きエラー。

use thiserror::Error;

/// 起動から実行までで起こりうる失敗を表す。
#[derive(Debug, Error)]
pub(crate) enum 起動エラー {
    #[error("イベントループの生成・実行に失敗した: {0}")]
    イベントループ失敗(#[from] winit::error::EventLoopError),

    #[error("ウィンドウの生成に失敗した: {0}")]
    ウィンドウ生成失敗(#[from] winit::error::OsError),

    #[error("ウィンドウハンドルの取得に失敗した: {0}")]
    ハンドル取得失敗(#[from] raw_window_handle::HandleError),

    #[error("レンダラーの生成・描画に失敗した: {0}")]
    レンダラー失敗(#[from] blitz_render::レンダラーエラー),

    #[error("クリアカラーの生成に失敗した: {0}")]
    クリアカラー不正(#[from] blitz_render::クリアカラーエラー),

    #[error("シェーダー一式の生成に失敗した: {0}")]
    シェーダー一式不正(#[from] blitz_render::シェーダー一式エラー),

    #[error("--frames引数が不正だった: {0}")]
    フレーム数引数不正(String),

    #[error("--shader-source引数が不正だった: {0}")]
    シェーダーソース引数不正(String),

    #[error("スモークのシェーダー書き換えに失敗した: {0}")]
    シェーダー書き換え失敗(String),

    #[error("スモークのピクセル判定に失敗した: {0}")]
    ピクセル判定失敗(String),
}
