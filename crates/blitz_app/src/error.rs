//! blitz_app(起動バイナリ)の型付きエラー。

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

    #[error("粒子シェーダー一式の生成に失敗した: {0}")]
    粒子シェーダー一式不正(#[from] blitz_render::粒子シェーダー一式エラー),

    #[error("粒子素材の生成に失敗した: {0}")]
    粒子素材不正(#[from] blitz_render::粒子素材エラー),

    #[error("表面流初期状態の生成に失敗した: {0}")]
    表面流仕様不正(#[from] blitz_sim::表面流仕様エラー),

    #[error("テクスチャ素材の生成に失敗した: {0}")]
    テクスチャ素材不正(#[from] blitz_render::テクスチャ素材エラー),

    #[error("UIテクスチャ素材の生成に失敗した: {0}")]
    UIテクスチャ素材不正(#[from] blitz_render::UIテクスチャ素材エラー),

    #[error("マテリアル素材の生成に失敗した: {0}")]
    マテリアル素材不正(#[from] blitz_render::マテリアル素材エラー),

    #[error("シーン名の生成に失敗した: {0}")]
    シーン名不正(#[from] blitz_engine::アセットIDエラー),

    #[error("シーンの読込に失敗した: {0}(標準サンプルが無い場合は`cargo xtask fetch-assets`を試すこと)")]
    シーン読込失敗(blitz_engine::アセットエラー),

    #[error("--frames引数が不正だった: {0}")]
    フレーム数引数不正(String),

    #[error("--shader-source引数が不正だった: {0}")]
    シェーダーソース引数不正(String),

    #[error("--scene引数が不正だった: {0}")]
    シーン名引数不正(String),

    #[error("--asset-root引数が不正だった: {0}")]
    アセットルート引数不正(String),

    #[error("--object-count引数が不正だった: {0}")]
    描画対象数引数不正(String),

    #[error("--dump-frame引数が不正だった: {0}")]
    フレームダンプ引数不正(String),

    #[error("--exposure引数が不正だった: {0}")]
    露出引数不正(String),

    #[error("--blend引数が不正だった: {0}")]
    ブレンド引数不正(String),

    #[error("スキンメッシュ素材の生成に失敗した: {0}")]
    スキンメッシュ素材不正(#[from] blitz_render::スキンメッシュ素材エラー),

    #[error("布の生成に失敗した: {0}")]
    布生成失敗(#[from] blitz_sim::布生成エラー),

    #[error("布素材の生成に失敗した: {0}")]
    布素材不正(#[from] blitz_render::布素材エラー),

    #[error("シーンのスキン情報が不整合だった: {0}")]
    スキン整合性不正(String),

    #[error("フレームダンプに失敗した: {0}")]
    フレームダンプ失敗(String),

    #[error("スモークのシェーダー書き換えに失敗した: {0}")]
    シェーダー書き換え失敗(String),

    #[error("スモークのアセット書き換えに失敗した: {0}")]
    アセット書き換え失敗(String),

    #[error("スモークのピクセル判定に失敗した: {0}")]
    ピクセル判定失敗(String),
}
